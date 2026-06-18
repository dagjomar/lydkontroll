import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { Command } from "../generated/Command";
import type { CommandAck } from "../generated/CommandAck";
import type { CueLibrary } from "../generated/CueLibrary";
import type { ManagedAudioFile } from "../generated/ManagedAudioFile";

export interface DesktopApi {
  getSnapshot(): Promise<AppSnapshot>;
  execute(command: Command): Promise<CommandAck>;
  saveLibrary(library: CueLibrary): Promise<AppSnapshot>;
  importAudio(): Promise<ManagedAudioFile | null>;
}

const tauriDesktopApi: DesktopApi = {
  getSnapshot() {
    return invoke<AppSnapshot>("get_snapshot");
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
    : tauriDesktopApi;

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
