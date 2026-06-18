import { useCallback, useEffect, useMemo, useState } from "react";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { ConnectionStatus, DesktopApi } from "../services/desktopApi";

interface MobileControlProps {
  api: DesktopApi;
  pollIntervalMs?: number;
}

type CommandStatus =
  | { state: "idle"; message: string }
  | { state: "pending"; message: string }
  | { state: "success"; message: string }
  | { state: "error"; message: string };

export function MobileControl({
  api,
  pollIntervalMs = 500,
}: MobileControlProps) {
  const [snapshot, setSnapshot] = useState<AppSnapshot | null>(null);
  const [connection, setConnection] = useState<ConnectionStatus>("connecting");
  const [selectedSceneId, setSelectedSceneId] = useState<string | null>(null);
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

  return (
    <main className="mobile-shell">
      <header className="mobile-header">
        <div>
          <p className="eyebrow">Marius + Wenche</p>
          <h1>Lydkontroll</h1>
        </div>
        <StatusBar connection={connection} commandStatus={commandStatus} />
      </header>

      <section className="mobile-master" aria-label="Hovedkontroller">
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

      {snapshot.activePlayback.length ? (
        <section className="mobile-active" aria-label="Aktiv lyd">
          <h2>Spiller nå</h2>
          {snapshot.activePlayback.map((playback) => (
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
          ))}
        </section>
      ) : null}

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
    </main>
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
