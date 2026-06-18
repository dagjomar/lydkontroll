import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { CommandAck } from "../generated/CommandAck";
import type { ControlServerInfo } from "../generated/ControlServerInfo";
import type { CueLibrary } from "../generated/CueLibrary";
import type { ManagedAudioFile } from "../generated/ManagedAudioFile";
import { createUuid } from "./uuid";

export interface DesktopApi {
  mode: "desktop" | "mobile";
  getSnapshot(): Promise<AppSnapshot>;
  refreshPreflight(): Promise<AppSnapshot>;
  getControlServerInfo(): Promise<ControlServerInfo | null>;
  execute(command: Command): Promise<CommandAck>;
  saveLibrary(library: CueLibrary): Promise<AppSnapshot>;
  importAudio(): Promise<ManagedAudioFile | null>;
  subscribeConnection?(
    listener: (status: ConnectionStatus) => void,
  ): () => void;
}

export type ConnectionStatus = "connecting" | "connected" | "reconnecting";

const tauriDesktopApi: DesktopApi = {
  mode: "desktop",
  getSnapshot() {
    return invoke<AppSnapshot>("get_snapshot");
  },
  refreshPreflight() {
    return invoke<AppSnapshot>("refresh_preflight");
  },
  getControlServerInfo() {
    return invoke<ControlServerInfo | null>("get_control_server_info");
  },
  execute(command) {
    return invoke<CommandAck>("execute_desktop_command", {
      envelope: {
        protocolVersion: 1,
        commandId: createUuid(),
        command,
      },
    });
  },
  saveLibrary(library) {
    return invoke<AppSnapshot>("save_library", { library });
  },
  async importAudio() {
    const sourcePath = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Lydfiler", extensions: ["mp3", "wav"] }],
    });
    if (!sourcePath) {
      return null;
    }
    return invoke<ManagedAudioFile>("import_audio", { sourcePath });
  },
};

const previewMode = new URLSearchParams(window.location.search).get("preview");
export const desktopApi =
  previewMode === "mobile"
    ? createPreviewApi("mobile")
    : previewMode === "1"
      ? createPreviewApi("desktop")
      : isTauriRuntime()
        ? tauriDesktopApi
        : createRemoteApi();

function isTauriRuntime(): boolean {
  return "__TAURI_INTERNALS__" in window;
}

type ServerMessage =
  | { type: "snapshot"; snapshot: AppSnapshot }
  | { type: "acknowledgement"; acknowledgement: CommandAck }
  | { type: "protocolError"; message: string };

export function createRemoteApi(): DesktopApi {
  let socket: WebSocket | null = null;
  let snapshot: AppSnapshot | null = null;
  let reconnectTimer: number | null = null;
  let reconnectDelayMs = 250;
  let connectionStatus: ConnectionStatus = "connecting";
  const connectionListeners = new Set<(status: ConnectionStatus) => void>();
  const snapshotWaiters = new Set<{
    resolve: (value: AppSnapshot) => void;
    reject: (reason: Error) => void;
  }>();
  const acknowledgementWaiters = new Map<
    string,
    {
      resolve: (value: CommandAck) => void;
      reject: (reason: Error) => void;
      timeout: number;
    }
  >();

  function connect(): WebSocket {
    if (
      socket?.readyState === WebSocket.OPEN ||
      socket?.readyState === WebSocket.CONNECTING
    ) {
      return socket;
    }
    const scheme = window.location.protocol === "https:" ? "wss:" : "ws:";
    const nextSocket = new WebSocket(`${scheme}//${window.location.host}/ws`);
    socket = nextSocket;
    nextSocket.addEventListener("open", () => {
      if (socket !== nextSocket) {
        return;
      }
      reconnectDelayMs = 250;
    });
    nextSocket.addEventListener("message", (event) => {
      if (socket !== nextSocket) {
        return;
      }
      try {
        const message = JSON.parse(String(event.data)) as ServerMessage;
        if (message.type === "snapshot") {
          snapshot = message.snapshot;
          setConnectionStatus("connected");
          for (const waiter of snapshotWaiters) {
            waiter.resolve(message.snapshot);
          }
          snapshotWaiters.clear();
          return;
        }
        if (message.type === "acknowledgement") {
          const waiter = acknowledgementWaiters.get(
            message.acknowledgement.commandId,
          );
          if (waiter) {
            window.clearTimeout(waiter.timeout);
            acknowledgementWaiters.delete(message.acknowledgement.commandId);
            waiter.resolve(message.acknowledgement);
          }
          return;
        }
        rejectAll(new Error(message.message));
      } catch {
        rejectAll(new Error("Mac-en sendte et ugyldig svar."));
      }
    });
    nextSocket.addEventListener("close", () => {
      if (socket !== nextSocket) {
        return;
      }
      socket = null;
      snapshot = null;
      setConnectionStatus("reconnecting");
      rejectAll(new Error("Forbindelsen til Mac-en ble brutt."));
      if (reconnectTimer === null) {
        reconnectTimer = window.setTimeout(() => {
          reconnectTimer = null;
          connect();
        }, reconnectDelayMs);
        reconnectDelayMs = Math.min(reconnectDelayMs * 2, 5000);
      }
    });
    nextSocket.addEventListener("error", () => {
      if (socket === nextSocket) {
        nextSocket.close();
      }
    });
    return nextSocket;
  }

  function setConnectionStatus(status: ConnectionStatus) {
    if (status === connectionStatus) {
      return;
    }
    connectionStatus = status;
    for (const listener of connectionListeners) {
      listener(status);
    }
  }

  function rejectAll(error: Error) {
    for (const waiter of snapshotWaiters) {
      waiter.reject(error);
    }
    snapshotWaiters.clear();
    for (const waiter of acknowledgementWaiters.values()) {
      window.clearTimeout(waiter.timeout);
      waiter.reject(error);
    }
    acknowledgementWaiters.clear();
  }

  async function getSnapshot(): Promise<AppSnapshot> {
    connect();
    if (snapshot) {
      return snapshot;
    }
    return new Promise<AppSnapshot>((resolve, reject) => {
      snapshotWaiters.add({ resolve, reject });
    });
  }

  return {
    mode: "mobile",
    getSnapshot,
    refreshPreflight: getSnapshot,
    subscribeConnection(listener) {
      connectionListeners.add(listener);
      listener(connectionStatus);
      return () => connectionListeners.delete(listener);
    },
    async getControlServerInfo() {
      return {
        address: window.location.hostname,
        port: Number(
          window.location.port || (location.protocol === "https:" ? 443 : 80),
        ),
        url: window.location.origin,
      };
    },
    async execute(command) {
      const activeSocket = connect();
      if (activeSocket.readyState !== WebSocket.OPEN) {
        await getSnapshot();
      }
      if (socket?.readyState !== WebSocket.OPEN) {
        throw new Error("Kan ikke koble til Mac-en.");
      }
      const commandId = createUuid();
      const acknowledgement = new Promise<CommandAck>((resolve, reject) => {
        const timeout = window.setTimeout(() => {
          acknowledgementWaiters.delete(commandId);
          reject(new Error("Mac-en svarte ikke på kommandoen."));
        }, 5000);
        acknowledgementWaiters.set(commandId, { resolve, reject, timeout });
      });
      socket.send(
        JSON.stringify({
          protocolVersion: 1,
          commandId,
          command,
        }),
      );
      return acknowledgement;
    },
    async saveLibrary() {
      throw new Error("Oppsettet kan bare redigeres på Mac-en.");
    },
    async importAudio() {
      throw new Error("Lydfiler kan bare importeres på Mac-en.");
    },
  };
}

function createPreviewApi(mode: "desktop" | "mobile"): DesktopApi {
  let snapshot: AppSnapshot = {
    revision: 0,
    scenes: [
      {
        id: "preview-scene",
        name: "Middag",
        cues: [
          {
            id: "preview-intro",
            name: "Introduksjon",
            color: "#d88c68",
            audioFileId: "preview-audio",
            volume: 0.8,
            mode: "overlap",
            fadeMs: 500,
          },
          {
            id: "preview-dance",
            name: "Første dans",
            color: "#7f9c87",
            audioFileId: "preview-audio",
            volume: 0.9,
            mode: "exclusive",
            fadeMs: 1000,
          },
        ],
      },
    ],
    audioFiles: [
      {
        id: "preview-audio",
        fileName: "preview.wav",
        originalName: "forhandsvisning.wav",
        format: "wav",
        byteLength: 44,
      },
    ],
    activePlayback: [],
    pendingCueId: null,
    masterVolume: 1,
    preflight: {
      controlServer: { status: "unknown" },
      audioOutput: { status: "ready" },
      audioFiles: { status: "ready" },
    },
    errors: [],
  };
  return {
    mode,
    subscribeConnection(listener) {
      listener("connected");
      return () => undefined;
    },
    async getSnapshot() {
      return snapshot;
    },
    async refreshPreflight() {
      return snapshot;
    },
    async getControlServerInfo() {
      return null;
    },
    async execute(command) {
      const activePlayback =
        command.type === "triggerCue"
          ? [
              ...snapshot.activePlayback,
              {
                id: createUuid(),
                cueId: command.cueId,
                volume:
                  snapshot.scenes
                    .flatMap((scene) => scene.cues)
                    .find((cue) => cue.id === command.cueId)?.volume ?? 1,
                fadeMs:
                  snapshot.scenes
                    .flatMap((scene) => scene.cues)
                    .find((cue) => cue.id === command.cueId)?.fadeMs ?? 500,
                status: "playing" as const,
              },
            ]
          : command.type === "stopPlayback"
            ? snapshot.activePlayback.filter(
                (playback) => playback.id !== command.playbackId,
              )
            : command.type === "fadePlayback"
              ? snapshot.activePlayback.map((playback) =>
                  playback.id === command.playbackId
                    ? { ...playback, status: "fading" as const }
                    : playback,
                )
              : command.type === "stopAll"
                ? []
                : command.type === "fadeAll"
                  ? snapshot.activePlayback.map((playback) => ({
                      ...playback,
                      status: "fading" as const,
                    }))
                  : snapshot.activePlayback;
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        activePlayback,
        masterVolume:
          command.type === "setMasterVolume"
            ? command.volume
            : snapshot.masterVolume,
      };
      return {
        protocolVersion: 1,
        commandId: createUuid(),
        outcome: { status: "success", revision: snapshot.revision },
      };
    },
    async saveLibrary(library) {
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        scenes: library.scenes,
        audioFiles: library.audioFiles,
      };
      return snapshot;
    },
    async importAudio() {
      const audio: ManagedAudioFile = {
        id: createUuid(),
        fileName: "preview.wav",
        originalName: "forhandsvisning.wav",
        format: "wav",
        byteLength: 44,
      };
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        audioFiles: [...snapshot.audioFiles, audio],
      };
      return audio;
    },
  };
}
