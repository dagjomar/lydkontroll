import { useCallback, useEffect, useMemo, useState } from "react";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { ConnectionStatus, DesktopApi } from "../services/desktopApi";

export type MobileLayout = "stable" | "overlay" | "controls" | "tabs";

interface MobileControlProps {
  api: DesktopApi;
  pollIntervalMs?: number;
  layout?: MobileLayout;
}

type CommandStatus =
  | { state: "idle"; message: string }
  | { state: "pending"; message: string }
  | { state: "success"; message: string }
  | { state: "error"; message: string };

export function MobileControl({
  api,
  pollIntervalMs = 500,
  layout: requestedLayout,
}: MobileControlProps) {
  const [snapshot, setSnapshot] = useState<AppSnapshot | null>(null);
  const [connection, setConnection] = useState<ConnectionStatus>("connecting");
  const [selectedSceneId, setSelectedSceneId] = useState<string | null>(null);
  const [layout, setLayout] = useState<MobileLayout>(
    requestedLayout ?? initialMobileLayout(),
  );
  const [mobileTab, setMobileTab] = useState<"cues" | "status">("cues");
  const [commandStatus, setCommandStatus] = useState<CommandStatus>({
    state: "idle",
    message: "Klar for kommando",
  });

  const refresh = useCallback(async () => {
    try {
      const next = await api.getSnapshot();
      setSnapshot(next);
    } catch (error) {
      setCommandStatus({
        state: "error",
        message: errorMessage(error),
      });
    }
  }, [api]);

  useEffect(() => {
    const unsubscribe = api.subscribeConnection?.((status) => {
      setConnection(status);
      if (status === "connected") {
        void refresh();
      }
    });
    void refresh();
    if (pollIntervalMs <= 0) {
      return unsubscribe;
    }
    const timer = window.setInterval(() => void refresh(), pollIntervalMs);
    return () => {
      unsubscribe?.();
      window.clearInterval(timer);
    };
  }, [api, pollIntervalMs, refresh]);

  useEffect(() => {
    if (!snapshot?.scenes.length) {
      setSelectedSceneId(null);
      return;
    }
    if (!snapshot.scenes.some((scene) => scene.id === selectedSceneId)) {
      setSelectedSceneId(snapshot.scenes[0].id);
    }
  }, [selectedSceneId, snapshot]);

  const selectedScene = useMemo(
    () =>
      snapshot?.scenes.find((scene) => scene.id === selectedSceneId) ?? null,
    [selectedSceneId, snapshot],
  );
  const controlsDisabled =
    connection !== "connected" || commandStatus.state === "pending";

  const run = useCallback(
    async (command: Command, label: string) => {
      if (connection !== "connected" || commandStatus.state === "pending") {
        return;
      }
      setCommandStatus({ state: "pending", message: `Sender: ${label}` });
      try {
        const acknowledgement = await api.execute(command);
        if (acknowledgement.outcome.status === "failure") {
          setCommandStatus({
            state: "error",
            message: commandErrorMessage(acknowledgement.outcome.error),
          });
          return;
        }
        setCommandStatus({
          state: "success",
          message: `Bekreftet · revisjon ${acknowledgement.outcome.revision}`,
        });
        await refresh();
      } catch (error) {
        setCommandStatus({ state: "error", message: errorMessage(error) });
      }
    },
    [api, commandStatus.state, connection, refresh],
  );

  if (!snapshot) {
    return (
      <main className="mobile-shell mobile-loading">
        <StatusBar connection={connection} commandStatus={commandStatus} />
        <p>Henter lydkontrollen fra Mac-en …</p>
      </main>
    );
  }

  const cueNames = new Map(
    snapshot.scenes.flatMap((scene) =>
      scene.cues.map((cue) => [cue.id, cue.name] as const),
    ),
  );
  const previewSwitcher =
    requestedLayout === undefined && isLayoutPrototypeSession();

  return (
    <main
      className={`mobile-shell mobile-layout-${layout}`}
      data-mobile-layout={layout}
      data-testid="mobile-layout-root"
    >
      {layout === "tabs" ? (
        <TabbedHeader connection={connection} />
      ) : (
        <MobileHeader connection={connection} commandStatus={commandStatus} />
      )}

      {layout === "stable" ? (
        <>
          <MasterControls
            snapshot={snapshot}
            controlsDisabled={controlsDisabled}
            run={run}
          />
          <ActivePlayback
            snapshot={snapshot}
            cueNames={cueNames}
            controlsDisabled={controlsDisabled}
            run={run}
            fixed
          />
          <CueControls
            snapshot={snapshot}
            selectedScene={selectedScene}
            selectedSceneId={selectedSceneId}
            controlsDisabled={controlsDisabled}
            setSelectedSceneId={setSelectedSceneId}
            run={run}
          />
        </>
      ) : null}

      {layout === "overlay" ? (
        <>
          <MasterControls
            snapshot={snapshot}
            controlsDisabled={controlsDisabled}
            run={run}
          />
          <CueControls
            snapshot={snapshot}
            selectedScene={selectedScene}
            selectedSceneId={selectedSceneId}
            controlsDisabled={controlsDisabled}
            setSelectedSceneId={setSelectedSceneId}
            run={run}
          />
          {snapshot.activePlayback.length ? (
            <ActivePlayback
              snapshot={snapshot}
              cueNames={cueNames}
              controlsDisabled={controlsDisabled}
              run={run}
              overlay
            />
          ) : null}
        </>
      ) : null}

      {layout === "controls" ? (
        <>
          <CueControls
            snapshot={snapshot}
            selectedScene={selectedScene}
            selectedSceneId={selectedSceneId}
            controlsDisabled={controlsDisabled}
            setSelectedSceneId={setSelectedSceneId}
            run={run}
          />
          <section className="mobile-global-card" aria-label="Global kontroll">
            <ActivePlayback
              snapshot={snapshot}
              cueNames={cueNames}
              controlsDisabled={controlsDisabled}
              run={run}
            />
            <MasterControls
              snapshot={snapshot}
              controlsDisabled={controlsDisabled}
              run={run}
              embedded
            />
          </section>
        </>
      ) : null}

      {layout === "tabs" ? (
        <>
          <nav className="mobile-tabs" aria-label="Mobilvisning">
            <button
              className={mobileTab === "cues" ? "selected" : ""}
              type="button"
              onClick={() => setMobileTab("cues")}
            >
              Cues
            </button>
            <button
              className={mobileTab === "status" ? "selected" : ""}
              type="button"
              onClick={() => setMobileTab("status")}
            >
              Status
            </button>
          </nav>
          {mobileTab === "cues" ? (
            <>
              <ActivePlayback
                snapshot={snapshot}
                cueNames={cueNames}
                controlsDisabled={controlsDisabled}
                run={run}
                compact
              />
              <CueControls
                snapshot={snapshot}
                selectedScene={selectedScene}
                selectedSceneId={selectedSceneId}
                controlsDisabled={controlsDisabled}
                setSelectedSceneId={setSelectedSceneId}
                run={run}
              />
            </>
          ) : (
            <section className="mobile-status-page" aria-label="Status og valg">
              <StatusBar
                connection={connection}
                commandStatus={commandStatus}
              />
              <MasterControls
                snapshot={snapshot}
                controlsDisabled={controlsDisabled}
                run={run}
              />
              <div className="mobile-future-card">
                <p className="eyebrow">Neste steg</p>
                <h2>Preflight og innstillinger</h2>
                <p>
                  Plass for lydutgang, Tailscale, filkontroll og testavspilling.
                </p>
              </div>
            </section>
          )}
        </>
      ) : null}

      {previewSwitcher ? (
        <LayoutSwitcher
          layout={layout}
          onChange={(nextLayout) => {
            setLayout(nextLayout);
            setMobileTab("cues");
            const url = new URL(window.location.href);
            url.searchParams.set("layout", nextLayout);
            window.history.replaceState({}, "", url);
          }}
        />
      ) : null}
    </main>
  );
}

type RunCommand = (command: Command, label: string) => Promise<void>;

function MobileHeader({
  connection,
  commandStatus,
}: {
  connection: ConnectionStatus;
  commandStatus: CommandStatus;
}) {
  return (
    <header className="mobile-header">
      <div>
        <p className="eyebrow">Marius + Wenche</p>
        <h1>Lydkontroll</h1>
      </div>
      <StatusBar connection={connection} commandStatus={commandStatus} />
    </header>
  );
}

function TabbedHeader({ connection }: { connection: ConnectionStatus }) {
  return (
    <header className="mobile-header mobile-header-compact">
      <div>
        <p className="eyebrow">Marius + Wenche</p>
        <h1>Lydkontroll</h1>
      </div>
      <span className="mobile-connection-pill">
        <span className={`connection-dot ${connection}`} aria-hidden="true" />
        {connection === "connected" ? "Tilkoblet" : "Kobler til"}
      </span>
    </header>
  );
}

function MasterControls({
  snapshot,
  controlsDisabled,
  run,
  embedded = false,
}: {
  snapshot: AppSnapshot;
  controlsDisabled: boolean;
  run: RunCommand;
  embedded?: boolean;
}) {
  return (
    <section
      className={embedded ? "mobile-master embedded" : "mobile-master"}
      aria-label="Hovedkontroller"
    >
      <label>
        Mastervolum {Math.round(snapshot.masterVolume * 100)} %
        <input
          aria-label="Mastervolum"
          type="range"
          min="0"
          max="1"
          step="0.05"
          value={snapshot.masterVolume}
          disabled={controlsDisabled}
          onChange={(event) =>
            void run(
              {
                type: "setMasterVolume",
                volume: Number(event.target.value),
              },
              "mastervolum",
            )
          }
        />
      </label>
      <div className="mobile-emergency">
        <button
          type="button"
          disabled={controlsDisabled}
          onClick={() =>
            void run({ type: "fadeAll", durationMs: 2000 }, "fade alt")
          }
        >
          Fade alt
        </button>
        <button
          className="danger"
          type="button"
          disabled={controlsDisabled}
          onClick={() => void run({ type: "stopAll" }, "stopp alt")}
        >
          Stopp alt
        </button>
      </div>
    </section>
  );
}

function ActivePlayback({
  snapshot,
  cueNames,
  controlsDisabled,
  run,
  fixed = false,
  overlay = false,
  compact = false,
}: {
  snapshot: AppSnapshot;
  cueNames: Map<string, string>;
  controlsDisabled: boolean;
  run: RunCommand;
  fixed?: boolean;
  overlay?: boolean;
  compact?: boolean;
}) {
  const classNames = [
    "mobile-active",
    fixed ? "fixed" : "",
    overlay ? "overlay" : "",
    compact ? "compact" : "",
  ]
    .filter(Boolean)
    .join(" ");
  return (
    <section className={classNames} aria-label="Aktiv lyd">
      <h2>Spiller nå</h2>
      <div className="mobile-active-list">
        {snapshot.activePlayback.length ? (
          snapshot.activePlayback.map((playback) => (
            <article key={playback.id}>
              <span>
                {cueNames.get(playback.cueId) ?? "Ukjent cue"} ·{" "}
                {playback.status === "fading" ? "fader" : "spiller"}
              </span>
              <button
                type="button"
                disabled={controlsDisabled}
                onClick={() =>
                  void run(
                    {
                      type: "fadePlayback",
                      playbackId: playback.id,
                    },
                    `fade ${cueNames.get(playback.cueId) ?? "cue"}`,
                  )
                }
              >
                Fade
              </button>
              <button
                type="button"
                disabled={controlsDisabled}
                onClick={() =>
                  void run(
                    {
                      type: "stopPlayback",
                      playbackId: playback.id,
                    },
                    `stopp ${cueNames.get(playback.cueId) ?? "cue"}`,
                  )
                }
              >
                Stopp
              </button>
            </article>
          ))
        ) : (
          <p className="mobile-active-empty">Ingen lyd spiller.</p>
        )}
      </div>
    </section>
  );
}

function CueControls({
  snapshot,
  selectedScene,
  selectedSceneId,
  controlsDisabled,
  setSelectedSceneId,
  run,
}: {
  snapshot: AppSnapshot;
  selectedScene: AppSnapshot["scenes"][number] | null;
  selectedSceneId: string | null;
  controlsDisabled: boolean;
  setSelectedSceneId: (sceneId: string) => void;
  run: RunCommand;
}) {
  return (
    <>
      <nav className="mobile-scenes" aria-label="Scener">
        {snapshot.scenes.map((scene) => (
          <button
            className={scene.id === selectedSceneId ? "selected" : ""}
            key={scene.id}
            type="button"
            onClick={() => setSelectedSceneId(scene.id)}
          >
            {scene.name}
          </button>
        ))}
      </nav>
      <section className="mobile-cues" aria-label="Cues">
        {selectedScene?.cues.map((cue) => {
          const active = snapshot.activePlayback.some(
            (playback) => playback.cueId === cue.id,
          );
          return (
            <button
              className={active ? "mobile-cue active" : "mobile-cue"}
              key={cue.id}
              type="button"
              disabled={controlsDisabled}
              style={{ borderColor: cue.color }}
              onClick={() =>
                void run(
                  { type: "triggerCue", cueId: cue.id },
                  `spill ${cue.name}`,
                )
              }
            >
              <span>{cue.name}</span>
              <small>{active ? "Spiller" : "Trykk for å spille"}</small>
            </button>
          );
        })}
        {!selectedScene?.cues.length ? (
          <p className="empty">Denne scenen har ingen cues.</p>
        ) : null}
      </section>
    </>
  );
}

function LayoutSwitcher({
  layout,
  onChange,
}: {
  layout: MobileLayout;
  onChange: (layout: MobileLayout) => void;
}) {
  const options: Array<[Exclude<MobileLayout, "stable">, string]> = [
    ["overlay", "1 · Overlay"],
    ["controls", "2 · Cues først"],
    ["tabs", "3 · Faner"],
  ];
  return (
    <nav className="mobile-layout-switcher" aria-label="Layoutforslag">
      {options.map(([value, label]) => (
        <button
          className={layout === value ? "selected" : ""}
          key={value}
          type="button"
          onClick={() => onChange(value)}
        >
          {label}
        </button>
      ))}
    </nav>
  );
}

function StatusBar({
  connection,
  commandStatus,
}: {
  connection: ConnectionStatus;
  commandStatus: CommandStatus;
}) {
  const connectionLabel =
    connection === "connected"
      ? "Tilkoblet"
      : connection === "reconnecting"
        ? "Kobler til igjen"
        : "Kobler til";
  return (
    <section className="mobile-status" aria-label="Tilkoblingsstatus">
      <span className={`connection-dot ${connection}`} aria-hidden="true" />
      <strong>{connectionLabel}</strong>
      <span aria-live="polite">{commandStatus.message}</span>
    </section>
  );
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : "Noe gikk galt.";
}

function commandErrorMessage(error: { code: string }): string {
  switch (error.code) {
    case "unknownCue":
      return "Cue-en finnes ikke lenger.";
    case "missingAudioMetadata":
      return "Cue-en mangler en gyldig lydfil.";
    case "unknownPlayback":
      return "Lyden er allerede ferdig eller stoppet.";
    case "audioBackend":
      return "Lydutgangen kunne ikke utføre kommandoen.";
    default:
      return "Kommandoen ble avvist.";
  }
}

function prototypeLayoutFromUrl(): Exclude<MobileLayout, "stable"> | null {
  const layout = new URLSearchParams(window.location.search).get("layout");
  return layout === "overlay" || layout === "controls" || layout === "tabs"
    ? layout
    : null;
}

function isLayoutPrototypeSession(): boolean {
  return (
    new URLSearchParams(window.location.search).get("preview") === "mobile" ||
    prototypeLayoutFromUrl() !== null
  );
}

function initialMobileLayout(): MobileLayout {
  const layout = prototypeLayoutFromUrl();
  if (layout) {
    return layout;
  }
  return new URLSearchParams(window.location.search).get("preview") === "mobile"
    ? "overlay"
    : "stable";
}
