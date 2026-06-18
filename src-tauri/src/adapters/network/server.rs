use std::{
    fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        OriginalUri, State,
    },
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{net::TcpListener, sync::watch, time};
use ts_rs::TS;

use crate::{
    application::{
        AppSnapshot, ApplicationService, CommandAck, CommandEnvelope, APPLICATION_PROTOCOL_VERSION,
    },
    ports::AudioBackend,
};

pub const CONTROL_SERVER_PORT: u16 = 17_321;
const SNAPSHOT_INTERVAL: Duration = Duration::from_millis(250);

pub trait NetworkApplication: Send + Sync {
    fn execute(&self, envelope: CommandEnvelope) -> Result<CommandAck, String>;
    fn poll(&self) -> Result<(), String>;
    fn snapshot(&self) -> Result<AppSnapshot, String>;
}

impl<B> NetworkApplication for ApplicationService<B>
where
    B: AudioBackend + Send,
    B::Error: fmt::Display,
{
    fn execute(&self, envelope: CommandEnvelope) -> Result<CommandAck, String> {
        ApplicationService::execute(self, envelope).map_err(|error| error.to_string())
    }

    fn poll(&self) -> Result<(), String> {
        ApplicationService::poll(self)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn snapshot(&self) -> Result<AppSnapshot, String> {
        ApplicationService::snapshot(self).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmbeddedWebAsset {
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub csp_header: Option<String>,
}

pub trait WebAssetProvider: Send + Sync {
    fn get(&self, path: &str) -> Option<EmbeddedWebAsset>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ControlServerInfo {
    pub address: Ipv4Addr,
    pub port: u16,
    pub url: String,
}

impl ControlServerInfo {
    fn new(address: Ipv4Addr, port: u16) -> Self {
        Self {
            address,
            port,
            url: format!("http://{address}:{port}/"),
        }
    }
}

pub struct ControlServer {
    info: ControlServerInfo,
    shutdown: watch::Sender<bool>,
}

impl ControlServer {
    pub fn info(&self) -> &ControlServerInfo {
        &self.info
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown.send(true);
    }
}

impl Drop for ControlServer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub struct ControlServerRuntime {
    server: Option<ControlServer>,
    startup_error: Option<String>,
}

impl ControlServerRuntime {
    pub fn running(server: ControlServer) -> Self {
        Self {
            server: Some(server),
            startup_error: None,
        }
    }

    pub fn unavailable(error: impl Into<String>) -> Self {
        Self {
            server: None,
            startup_error: Some(error.into()),
        }
    }

    pub fn info(&self) -> Option<ControlServerInfo> {
        self.server.as_ref().map(|server| server.info().clone())
    }

    pub fn startup_error(&self) -> Option<&str> {
        self.startup_error.as_deref()
    }
}

#[derive(Debug, Error)]
pub enum ControlServerError {
    #[error("could not bind control server to {address}: {source}")]
    Bind {
        address: SocketAddr,
        #[source]
        source: std::io::Error,
    },
}

pub async fn start_control_server(
    address: Ipv4Addr,
    application: Arc<dyn NetworkApplication>,
    assets: Arc<dyn WebAssetProvider>,
) -> Result<ControlServer, ControlServerError> {
    start_control_server_on(
        SocketAddr::new(IpAddr::V4(address), CONTROL_SERVER_PORT),
        application,
        assets,
    )
    .await
}

async fn start_control_server_on(
    bind_address: SocketAddr,
    application: Arc<dyn NetworkApplication>,
    assets: Arc<dyn WebAssetProvider>,
) -> Result<ControlServer, ControlServerError> {
    let listener =
        TcpListener::bind(bind_address)
            .await
            .map_err(|source| ControlServerError::Bind {
                address: bind_address,
                source,
            })?;
    let local_address = listener
        .local_addr()
        .expect("a bound TCP listener has a local address");
    let address = match local_address.ip() {
        IpAddr::V4(address) => address,
        IpAddr::V6(_) => unreachable!("TASK-006 binds IPv4 only"),
    };
    let (shutdown, mut shutdown_receiver) = watch::channel(false);
    let router = control_router(application, assets);
    tokio::spawn(async move {
        let result = axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                while !*shutdown_receiver.borrow() {
                    if shutdown_receiver.changed().await.is_err() {
                        break;
                    }
                }
            })
            .await;
        if let Err(error) = result {
            eprintln!("control server stopped: {error}");
        }
    });
    Ok(ControlServer {
        info: ControlServerInfo::new(address, local_address.port()),
        shutdown,
    })
}

struct ServerState {
    application: Arc<dyn NetworkApplication>,
    assets: Arc<dyn WebAssetProvider>,
}

fn control_router(
    application: Arc<dyn NetworkApplication>,
    assets: Arc<dyn WebAssetProvider>,
) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/index.html", get(index))
        .route("/health", get(health))
        .route("/ws", get(websocket))
        .fallback(asset)
        .with_state(Arc::new(ServerState {
            application,
            assets,
        }))
}

async fn index(State(state): State<Arc<ServerState>>) -> Response {
    asset_response(&state, "index.html")
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        protocol_version: APPLICATION_PROTOCOL_VERSION,
    })
}

async fn asset(State(state): State<Arc<ServerState>>, OriginalUri(uri): OriginalUri) -> Response {
    asset_response(&state, uri.path().trim_start_matches('/'))
}

fn asset_response(state: &ServerState, path: &str) -> Response {
    let Some(asset) = state.assets.get(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let mut response = Response::new(Body::from(asset.bytes));
    let headers = response.headers_mut();
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    if let Ok(content_type) = HeaderValue::from_str(&asset.mime_type) {
        headers.insert(header::CONTENT_TYPE, content_type);
    }
    if let Some(csp) = asset
        .csp_header
        .and_then(|value| HeaderValue::from_str(&value).ok())
    {
        headers.insert(header::CONTENT_SECURITY_POLICY, csp);
    }
    response
}

async fn websocket(websocket: WebSocketUpgrade, State(state): State<Arc<ServerState>>) -> Response {
    websocket
        .on_upgrade(move |socket| websocket_session(socket, state.application.clone()))
        .into_response()
}

async fn websocket_session(mut socket: WebSocket, application: Arc<dyn NetworkApplication>) {
    let snapshot = match application.snapshot() {
        Ok(snapshot) => snapshot,
        Err(error) => {
            let _ = send_server_message(
                &mut socket,
                &ServerMessage::ProtocolError { message: error },
            )
            .await;
            return;
        }
    };
    let mut last_revision = snapshot.revision;
    if send_server_message(&mut socket, &ServerMessage::Snapshot { snapshot })
        .await
        .is_err()
    {
        return;
    }

    let mut interval = time::interval(SNAPSHOT_INTERVAL);
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            frame = socket.next() => {
                let Some(frame) = frame else {
                    break;
                };
                match frame {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<CommandEnvelope>(&text) {
                            Ok(envelope) => {
                                match application.execute(envelope) {
                                    Ok(acknowledgement) => {
                                        if send_server_message(
                                            &mut socket,
                                            &ServerMessage::Acknowledgement { acknowledgement },
                                        )
                                        .await
                                        .is_err()
                                        {
                                            break;
                                        }
                                        if let Ok(snapshot) = application.snapshot() {
                                            last_revision = snapshot.revision;
                                            if send_server_message(
                                                &mut socket,
                                                &ServerMessage::Snapshot { snapshot },
                                            )
                                            .await
                                            .is_err()
                                            {
                                                break;
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        if send_server_message(
                                            &mut socket,
                                            &ServerMessage::ProtocolError { message: error },
                                        )
                                        .await
                                        .is_err()
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                if send_server_message(
                                    &mut socket,
                                    &ServerMessage::ProtocolError {
                                        message: format!("invalid command envelope: {error}"),
                                    },
                                )
                                .await
                                .is_err()
                                {
                                    break;
                                }
                            }
                        }
                    }
                    Ok(Message::Binary(_)) => {
                        if send_server_message(
                            &mut socket,
                            &ServerMessage::ProtocolError {
                                message: "binary WebSocket messages are not supported".to_owned(),
                            },
                        )
                        .await
                        .is_err()
                        {
                            break;
                        }
                    }
                    Ok(Message::Ping(payload)) => {
                        if socket.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Ok(Message::Pong(_)) => {}
                    Ok(Message::Close(_)) | Err(_) => break,
                }
            }
            _ = interval.tick() => {
                if application.poll().is_err() {
                    continue;
                }
                if let Ok(snapshot) = application.snapshot() {
                    if snapshot.revision > last_revision {
                        last_revision = snapshot.revision;
                        if send_server_message(
                            &mut socket,
                            &ServerMessage::Snapshot { snapshot },
                        )
                        .await
                        .is_err()
                        {
                            break;
                        }
                    }
                }
            }
        }
    }
}

async fn send_server_message(
    socket: &mut WebSocket,
    message: &ServerMessage,
) -> Result<(), axum::Error> {
    let json = serde_json::to_string(message).expect("server messages are serializable");
    socket.send(Message::Text(json.into())).await
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    protocol_version: u32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ServerMessage {
    Snapshot { snapshot: AppSnapshot },
    Acknowledgement { acknowledgement: CommandAck },
    ProtocolError { message: String },
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Read, Write},
        path::Path,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{connect_async, tungstenite};
    use uuid::Uuid;

    use crate::{
        application::{Command, CommandEnvelope},
        domain::CueLibrary,
        ports::{AudioBackendEvent, BackendPlaybackId},
    };

    use super::*;

    #[derive(Debug, Default)]
    struct FakeBackend {
        stop_all_calls: Arc<AtomicUsize>,
    }

    impl AudioBackend for FakeBackend {
        type Error = &'static str;

        fn play(
            &mut self,
            _managed_path: &Path,
            _volume: f32,
        ) -> Result<BackendPlaybackId, Self::Error> {
            Ok("backend".to_owned())
        }

        fn stop(
            &mut self,
            _backend_id: &BackendPlaybackId,
            _fade: Duration,
        ) -> Result<(), Self::Error> {
            self.stop_all_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn set_master_volume(&mut self, _volume: f32) -> Result<(), Self::Error> {
            Ok(())
        }

        fn poll_events(&mut self) -> Vec<AudioBackendEvent> {
            Vec::new()
        }
    }

    fn application() -> Arc<ApplicationService<FakeBackend>> {
        Arc::new(ApplicationService::new(
            CueLibrary::default(),
            "/managed",
            FakeBackend::default(),
        ))
    }

    #[derive(Debug, Default)]
    struct FakeAssets;

    impl WebAssetProvider for FakeAssets {
        fn get(&self, path: &str) -> Option<EmbeddedWebAsset> {
            match path {
                "index.html" => Some(EmbeddedWebAsset {
                    bytes: b"<!doctype html><title>Lydkontroll</title>".to_vec(),
                    mime_type: "text/html".to_owned(),
                    csp_header: Some("default-src 'self'".to_owned()),
                }),
                "assets/app.js" => Some(EmbeddedWebAsset {
                    bytes: b"console.log('ready')".to_vec(),
                    mime_type: "text/javascript".to_owned(),
                    csp_header: None,
                }),
                _ => None,
            }
        }
    }

    async fn test_server(application: Arc<dyn NetworkApplication>) -> (ControlServer, String) {
        let server = start_control_server_on(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            application,
            Arc::new(FakeAssets),
        )
        .await
        .expect("test server");
        let url = format!("ws://127.0.0.1:{}/ws", server.info().port);
        (server, url)
    }

    async fn read_json(
        socket: &mut tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    ) -> serde_json::Value {
        let message = socket
            .next()
            .await
            .expect("server message")
            .expect("valid WebSocket message");
        serde_json::from_str(message.to_text().expect("text message")).expect("JSON message")
    }

    async fn http_get(port: u16, path: &str) -> String {
        let path = path.to_owned();
        tokio::task::spawn_blocking(move || {
            let mut stream =
                std::net::TcpStream::connect((Ipv4Addr::LOCALHOST, port)).expect("HTTP connect");
            write!(
                stream,
                "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
            )
            .expect("HTTP request");
            let mut response = String::new();
            stream.read_to_string(&mut response).expect("HTTP response");
            response
        })
        .await
        .expect("HTTP task")
    }

    #[tokio::test]
    async fn embedded_root_and_health_are_served() {
        let (server, _url) = test_server(application()).await;

        let root = http_get(server.info().port, "/").await;
        assert!(root.starts_with("HTTP/1.1 200 OK"));
        assert!(root.contains("<title>Lydkontroll</title>"));
        assert!(root.contains("cache-control: no-store"));
        assert!(root.contains("content-security-policy: default-src 'self'"));

        let javascript = http_get(server.info().port, "/assets/app.js").await;
        assert!(javascript.starts_with("HTTP/1.1 200 OK"));
        assert!(javascript.contains("content-type: text/javascript"));
        assert!(javascript.contains("console.log('ready')"));

        let health = http_get(server.info().port, "/health").await;
        assert!(health.starts_with("HTTP/1.1 200 OK"));
        assert!(health.contains(r#""status":"ok""#));
        assert!(health.contains(r#""protocolVersion":1"#));
    }

    #[tokio::test]
    async fn websocket_sends_snapshot_acknowledgement_and_idempotent_retry() {
        let application = application();
        let (_server, url) = test_server(application.clone()).await;
        let (mut socket, _) = connect_async(&url).await.expect("connect");

        let initial = read_json(&mut socket).await;
        assert_eq!(initial["type"], "snapshot");
        assert_eq!(initial["snapshot"]["revision"], 0);

        let command_id = Uuid::new_v4().to_string();
        let command = CommandEnvelope {
            protocol_version: APPLICATION_PROTOCOL_VERSION,
            command_id: command_id.clone(),
            command: Command::SetMasterVolume { volume: 0.4 },
        };
        socket
            .send(tungstenite::Message::Text(
                serde_json::to_string(&command)
                    .expect("command JSON")
                    .into(),
            ))
            .await
            .expect("send command");
        let acknowledgement = read_json(&mut socket).await;
        let snapshot = read_json(&mut socket).await;
        assert_eq!(acknowledgement["type"], "acknowledgement");
        assert_eq!(acknowledgement["acknowledgement"]["commandId"], command_id);
        assert_eq!(snapshot["snapshot"]["revision"], 1);

        socket
            .send(tungstenite::Message::Text(
                serde_json::to_string(&command)
                    .expect("command JSON")
                    .into(),
            ))
            .await
            .expect("retry command");
        let duplicate = read_json(&mut socket).await;
        let duplicate_snapshot = read_json(&mut socket).await;
        assert_eq!(duplicate, acknowledgement);
        assert_eq!(duplicate_snapshot["snapshot"]["revision"], 1);
    }

    #[tokio::test]
    async fn malformed_message_is_recoverable_and_reconnect_gets_latest_snapshot() {
        let application = application();
        let (_server, url) = test_server(application.clone()).await;
        let (mut first, _) = connect_async(&url).await.expect("connect");
        let _ = read_json(&mut first).await;
        first
            .send(tungstenite::Message::Text("{bad json".into()))
            .await
            .expect("send malformed message");
        assert_eq!(read_json(&mut first).await["type"], "protocolError");

        application
            .execute(CommandEnvelope {
                protocol_version: APPLICATION_PROTOCOL_VERSION,
                command_id: Uuid::new_v4().to_string(),
                command: Command::SetMasterVolume { volume: 0.25 },
            })
            .expect("advance state");
        first.close(None).await.expect("close first connection");

        let (mut reconnected, _) = connect_async(&url).await.expect("reconnect");
        let latest = read_json(&mut reconnected).await;
        assert_eq!(latest["type"], "snapshot");
        assert_eq!(latest["snapshot"]["revision"], 1);
        assert_eq!(latest["snapshot"]["masterVolume"], 0.25);
    }

    #[tokio::test]
    async fn graceful_shutdown_releases_listener() {
        let (server, _url) = test_server(application()).await;
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), server.info().port);
        server.shutdown();
        drop(server);

        let mut rebound = None;
        for _ in 0..20 {
            match TcpListener::bind(address).await {
                Ok(listener) => {
                    rebound = Some(listener);
                    break;
                }
                Err(_) => time::sleep(Duration::from_millis(25)).await,
            }
        }
        assert!(
            rebound.is_some(),
            "graceful shutdown should release the port"
        );
    }

    #[tokio::test]
    async fn occupied_address_returns_typed_bind_failure() {
        let occupied = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .expect("occupied listener");
        let address = occupied.local_addr().expect("occupied address");

        let result = start_control_server_on(address, application(), Arc::new(FakeAssets)).await;

        assert!(matches!(
            result,
            Err(ControlServerError::Bind {
                address: failed_address,
                ..
            }) if failed_address == address
        ));
    }
}
