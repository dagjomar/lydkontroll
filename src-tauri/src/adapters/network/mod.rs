//! Fail-closed Tailscale discovery and Axum control transport.

mod discovery;
mod server;

pub use discovery::{
    discover_tailscale_ipv4, local_ipv4_addresses, CommandOutput, ProcessRunner,
    SystemProcessRunner, TailscaleDiscoveryError, DEFAULT_TAILSCALE_TIMEOUT,
};
pub use server::{
    start_control_server, ControlServer, ControlServerError, ControlServerInfo,
    ControlServerRuntime, EmbeddedWebAsset, NetworkApplication, WebAssetProvider,
    CONTROL_SERVER_PORT,
};
