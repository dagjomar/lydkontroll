import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { CommandAck } from "../generated/CommandAck";
import type { ControlServerInfo } from "../generated/ControlServerInfo";
import type { CueLibrary } from "../generated/CueLibrary";
import type { ManagedAudioFile } from "../generated/ManagedAudioFile";

export interface DesktopApi {
  getSnapshot(): Promise<AppSnapshot>;
  getControlServerInfo(): Promise<ControlServerInfo | null>;
  execute(command: Command): Promise<CommandAck>;
  saveLibrary(library: CueLibrary): Promise<AppSnapshot>;
  importAudio(): Promise<ManagedAudioFile | null>;
}

const tauriDesktopApi: DesktopApi = {
  getSnapshot() {
    return invoke<AppSnapshot>("get_snapshot");
  },
  getControlServerInfo() {
    return invoke<ControlServerInfo | null>("get_control_server_info");
  },
  execute(command) {
    return invoke<CommandAck>("execute_desktop_command", {
      envelope: {
        protocolVersion: 1,
        commandId: crypto.randomUUID(),
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

export const desktopApi =
  new URLSearchParams(window.location.search).get("preview") === "1"
    ? createPreviewApi()
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

function createRemoteApi(): DesktopApi {
  let socket: WebSocket | null = null;
  let snapshot: AppSnapshot | null = null;
  let reconnectTimer: number | null = null;
  let reconnectDelayMs = 250;
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
    socket = new WebSocket(`${scheme}//${window.location.host}/ws`);
    socket.addEventListener("open", () => {
      reconnectDelayMs = 250;
    });
    socket.addEventListener("message", (event) => {
      try {
        const message = JSON.parse(String(event.data)) as ServerMessage;
        if (message.type === "snapshot") {
          snapshot = message.snapshot;
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
    socket.addEventListener("close", () => {
      socket = null;
      rejectAll(new Error("Forbindelsen til Mac-en ble brutt."));
      if (reconnectTimer === null) {
        reconnectTimer = window.setTimeout(() => {
          reconnectTimer = null;
          connect();
        }, reconnectDelayMs);
        reconnectDelayMs = Math.min(reconnectDelayMs * 2, 5000);
      }
    });
    socket.addEventListener("error", () => {
      socket?.close();
    });
    return socket;
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
    getSnapshot,
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
      const commandId = crypto.randomUUID();
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

function createPreviewApi(): DesktopApi {
  let snapshot: AppSnapshot = {
    revision: 0,
    scenes: [],
    audioFiles: [],
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
    async getSnapshot() {
      return snapshot;
    },
    async getControlServerInfo() {
      return null;
    },
    async execute(command) {
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        masterVolume:
          command.type === "setMasterVolume"
            ? command.volume
            : snapshot.masterVolume,
      };
      return {
        protocolVersion: 1,
        commandId: crypto.randomUUID(),
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
        id: crypto.randomUUID(),
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
