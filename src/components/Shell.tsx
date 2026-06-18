import { useCallback, useEffect, useMemo, useState } from "react";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { Cue } from "../generated/Cue";
import type { CueLibrary } from "../generated/CueLibrary";
import type { Scene } from "../generated/Scene";
import { desktopApi, type DesktopApi } from "../services/desktopApi";
import { createUuid } from "../services/uuid";

interface ShellProps {
  api?: DesktopApi;
  pollIntervalMs?: number;
}

const DEFAULT_COLOR = "#d88c68";

export function Shell({ api = desktopApi, pollIntervalMs = 500 }: ShellProps) {
  const [snapshot, setSnapshot] = useState<AppSnapshot | null>(null);
  const [draft, setDraft] = useState<CueLibrary | null>(null);
  const [selectedSceneId, setSelectedSceneId] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const next = await api.getSnapshot();
      setSnapshot(next);
      setDraft((current) =>
        current
          ? {
              ...current,
              audioFiles: next.audioFiles,
            }
          : libraryFromSnapshot(next),
      );
    } catch (error) {
      setMessage(errorMessage(error));
    }
  }, [api]);

  useEffect(() => {
    void refresh();
    if (pollIntervalMs <= 0) {
      return;
    }
    const timer = window.setInterval(() => void refresh(), pollIntervalMs);
    return () => window.clearInterval(timer);
  }, [pollIntervalMs, refresh]);

  useEffect(() => {
    if (!draft?.scenes.length) {
      setSelectedSceneId(null);
      return;
    }
    if (!draft.scenes.some((scene) => scene.id === selectedSceneId)) {
      setSelectedSceneId(draft.scenes[0].id);
    }
  }, [draft, selectedSceneId]);

  const selectedScene = useMemo(
    () => draft?.scenes.find((scene) => scene.id === selectedSceneId) ?? null,
    [draft, selectedSceneId],
  );

  const run = useCallback(
    async (command: Command) => {
      setMessage(null);
      try {
        const acknowledgement = await api.execute(command);
        if (acknowledgement.outcome.status === "failure") {
          setMessage(commandErrorMessage(acknowledgement.outcome.error));
        }
        await refresh();
      } catch (error) {
        setMessage(errorMessage(error));
      }
    },
    [api, refresh],
  );

  async function persist() {
    if (!draft) {
      return;
    }
    setBusy(true);
    setMessage(null);
    try {
      const next = await api.saveLibrary(draft);
      setSnapshot(next);
      setDraft(libraryFromSnapshot(next));
      setMessage("Oppsettet er lagret.");
    } catch (error) {
      setMessage(errorMessage(error));
    } finally {
      setBusy(false);
    }
  }

  async function importAudio() {
    setBusy(true);
    setMessage(null);
    try {
      const imported = await api.importAudio();
      if (imported) {
        await refresh();
        setMessage(`${imported.originalName} er importert.`);
      }
    } catch (error) {
      setMessage(errorMessage(error));
    } finally {
      setBusy(false);
    }
  }

  function updateScenes(updater: (scenes: Scene[]) => Scene[]) {
    setDraft((current) =>
      current ? { ...current, scenes: updater(current.scenes) } : current,
    );
  }

  function addScene() {
    const scene: Scene = {
      id: createUuid(),
      name: "Ny scene",
      cues: [],
    };
    updateScenes((scenes) => [...scenes, scene]);
    setSelectedSceneId(scene.id);
  }

  function updateScene(sceneId: string, updater: (scene: Scene) => Scene) {
    updateScenes((scenes) =>
      scenes.map((scene) => (scene.id === sceneId ? updater(scene) : scene)),
    );
  }

  function removeScene(sceneId: string) {
    updateScenes((scenes) => scenes.filter((scene) => scene.id !== sceneId));
  }

  function moveScene(sceneId: string, direction: -1 | 1) {
    updateScenes((scenes) => moveItem(scenes, sceneId, direction));
  }

  function addCue(sceneId: string) {
    const audio = draft?.audioFiles[0];
    if (!audio) {
      setMessage("Importer en MP3- eller WAV-fil før du lager en cue.");
      return;
    }
    const cue: Cue = {
      id: createUuid(),
      name: "Ny cue",
      color: DEFAULT_COLOR,
      audioFileId: audio.id,
      volume: 0.8,
      mode: "overlap",
      fadeMs: 500,
    };
    updateScene(sceneId, (scene) => ({
      ...scene,
      cues: [...scene.cues, cue],
    }));
  }

  function updateCue(
    sceneId: string,
    cueId: string,
    updater: (cue: Cue) => Cue,
  ) {
    updateScene(sceneId, (scene) => ({
      ...scene,
      cues: scene.cues.map((cue) => (cue.id === cueId ? updater(cue) : cue)),
    }));
  }

  function removeCue(sceneId: string, cueId: string) {
    updateScene(sceneId, (scene) => ({
      ...scene,
      cues: scene.cues.filter((cue) => cue.id !== cueId),
    }));
  }

  function moveCue(sceneId: string, cueId: string, direction: -1 | 1) {
    updateScene(sceneId, (scene) => ({
      ...scene,
      cues: moveItem(scene.cues, cueId, direction),
    }));
  }

  if (!snapshot || !draft) {
    return (
      <main className="loading-shell">
        <p>{message ?? "Åpner lydkontrollen …"}</p>
      </main>
    );
  }

  const visibleErrors = [
    ...(message ? [message] : []),
    ...snapshot.errors.map((error) => error.message),
  ];

  return (
    <main className="operator-shell">
      <header className="topbar">
        <div>
          <p className="eyebrow">Marius + Wenche</p>
          <h1>Lydkontroll</h1>
        </div>
        <div className="topbar-actions">
          <button type="button" onClick={addScene}>
            Ny scene
          </button>
          <button
            type="button"
            onClick={() => void importAudio()}
            disabled={busy}
          >
            Importer lyd
          </button>
          <button
            className="primary"
            type="button"
            onClick={() => void persist()}
            disabled={busy}
          >
            Lagre oppsett
          </button>
        </div>
      </header>

      {visibleErrors.length ? (
        <section className="error-stack" aria-label="Meldinger">
          {visibleErrors.map((error, index) => (
            <p key={`${error}-${index}`}>{error}</p>
          ))}
        </section>
      ) : null}

      <section className="transport" aria-label="Hovedkontroller">
        <label>
          Mastervolum
          <input
            aria-label="Mastervolum"
            type="range"
            min="0"
            max="1"
            step="0.05"
            value={snapshot.masterVolume}
            onChange={(event) =>
              void run({
                type: "setMasterVolume",
                volume: Number(event.target.value),
              })
            }
          />
        </label>
        <button
          type="button"
          onClick={() => void run({ type: "fadeAll", durationMs: 2000 })}
        >
          Fade alt
        </button>
        <button
          className="danger"
          type="button"
          onClick={() => void run({ type: "stopAll" })}
        >
          Stopp alt
        </button>
      </section>

      <div className="workspace">
        <aside className="scene-list" aria-label="Scener">
          {draft.scenes.length ? (
            draft.scenes.map((scene, index) => (
              <div
                className={
                  scene.id === selectedSceneId
                    ? "scene-row selected"
                    : "scene-row"
                }
                key={scene.id}
              >
                <button
                  className="scene-select"
                  type="button"
                  onClick={() => setSelectedSceneId(scene.id)}
                >
                  {scene.name}
                </button>
                <div className="mini-actions">
                  <button
                    aria-label={`Flytt ${scene.name} opp`}
                    type="button"
                    disabled={index === 0}
                    onClick={() => moveScene(scene.id, -1)}
                  >
                    ↑
                  </button>
                  <button
                    aria-label={`Flytt ${scene.name} ned`}
                    type="button"
                    disabled={index === draft.scenes.length - 1}
                    onClick={() => moveScene(scene.id, 1)}
                  >
                    ↓
                  </button>
                </div>
              </div>
            ))
          ) : (
            <p className="empty">Lag den første scenen.</p>
          )}
        </aside>

        <section className="editor">
          {selectedScene ? (
            <>
              <div className="scene-heading">
                <input
                  aria-label="Scenenavn"
                  value={selectedScene.name}
                  onChange={(event) =>
                    updateScene(selectedScene.id, (scene) => ({
                      ...scene,
                      name: event.target.value,
                    }))
                  }
                />
                <button
                  type="button"
                  onClick={() => removeScene(selectedScene.id)}
                >
                  Slett scene
                </button>
                <button
                  className="primary"
                  type="button"
                  onClick={() => addCue(selectedScene.id)}
                >
                  Ny cue
                </button>
              </div>

              <div className="cue-grid">
                {selectedScene.cues.map((cue, index) => {
                  const active = snapshot.activePlayback.filter(
                    (playback) => playback.cueId === cue.id,
                  );
                  return (
                    <article
                      className="cue-card"
                      key={cue.id}
                      style={{ borderTopColor: cue.color }}
                    >
                      <div className="cue-card-heading">
                        <input
                          aria-label={`Navn på cue ${index + 1}`}
                          value={cue.name}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              name: event.target.value,
                            }))
                          }
                        />
                        <input
                          aria-label={`Farge for ${cue.name}`}
                          type="color"
                          value={cue.color}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              color: event.target.value,
                            }))
                          }
                        />
                      </div>

                      <label>
                        Lydfil
                        <select
                          value={cue.audioFileId}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              audioFileId: event.target.value,
                            }))
                          }
                        >
                          {draft.audioFiles.map((audio) => (
                            <option key={audio.id} value={audio.id}>
                              {audio.originalName}
                            </option>
                          ))}
                        </select>
                      </label>
                      <label>
                        Modus
                        <select
                          value={cue.mode}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              mode: event.target.value as Cue["mode"],
                            }))
                          }
                        >
                          <option value="overlap">Overlapp</option>
                          <option value="exclusive">Eksklusiv</option>
                        </select>
                      </label>
                      <label>
                        Volum {Math.round(cue.volume * 100)} %
                        <input
                          type="range"
                          min="0"
                          max="1"
                          step="0.05"
                          value={cue.volume}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              volume: Number(event.target.value),
                            }))
                          }
                        />
                      </label>
                      <label>
                        Fade (ms)
                        <input
                          type="number"
                          min="0"
                          step="100"
                          value={cue.fadeMs}
                          onChange={(event) =>
                            updateCue(selectedScene.id, cue.id, (current) => ({
                              ...current,
                              fadeMs: Math.max(0, Number(event.target.value)),
                            }))
                          }
                        />
                      </label>

                      <div className="cue-actions">
                        <button
                          className="play"
                          type="button"
                          onClick={() =>
                            void run({ type: "triggerCue", cueId: cue.id })
                          }
                        >
                          Spill {cue.name}
                        </button>
                        <button
                          type="button"
                          disabled={index === 0}
                          onClick={() => moveCue(selectedScene.id, cue.id, -1)}
                        >
                          ↑
                        </button>
                        <button
                          type="button"
                          disabled={index === selectedScene.cues.length - 1}
                          onClick={() => moveCue(selectedScene.id, cue.id, 1)}
                        >
                          ↓
                        </button>
                        <button
                          type="button"
                          onClick={() => removeCue(selectedScene.id, cue.id)}
                        >
                          Slett
                        </button>
                      </div>

                      {active.map((playback) => (
                        <div className="active-cue" key={playback.id}>
                          <span>
                            {playback.status === "fading" ? "Fader" : "Spiller"}
                          </span>
                          <button
                            type="button"
                            onClick={() =>
                              void run({
                                type: "fadePlayback",
                                playbackId: playback.id,
                              })
                            }
                          >
                            Fade
                          </button>
                          <button
                            type="button"
                            onClick={() =>
                              void run({
                                type: "stopPlayback",
                                playbackId: playback.id,
                              })
                            }
                          >
                            Stopp
                          </button>
                        </div>
                      ))}
                    </article>
                  );
                })}
              </div>
            </>
          ) : (
            <div className="empty-state">
              <h2>Ingen scene valgt</h2>
              <p>Lag en scene, importer lyd og bygg den første cue-knappen.</p>
            </div>
          )}
        </section>
      </div>
    </main>
  );
}

function libraryFromSnapshot(snapshot: AppSnapshot): CueLibrary {
  return {
    schemaVersion: 1,
    scenes: snapshot.scenes,
    audioFiles: snapshot.audioFiles,
  };
}

function moveItem<T extends { id: string }>(
  items: T[],
  id: string,
  direction: -1 | 1,
): T[] {
  const index = items.findIndex((item) => item.id === id);
  const target = index + direction;
  if (index < 0 || target < 0 || target >= items.length) {
    return items;
  }
  const next = [...items];
  [next[index], next[target]] = [next[target], next[index]];
  return next;
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function commandErrorMessage(error: { code: string }): string {
  switch (error.code) {
    case "unknownCue":
      return "Cue-en finnes ikke lenger. Lagre oppsettet og prøv igjen.";
    case "missingAudioMetadata":
      return "Cue-en mangler en gyldig importert lydfil.";
    case "unknownPlayback":
      return "Lyden er allerede ferdig eller stoppet.";
    case "audioBackend":
      return "Lydutgangen kunne ikke utføre kommandoen.";
    default:
      return "Kommandoen ble avvist.";
  }
}
