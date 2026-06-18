import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test, vi } from "vitest";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { CommandAck } from "../generated/CommandAck";
import type { CueLibrary } from "../generated/CueLibrary";
import type { ManagedAudioFile } from "../generated/ManagedAudioFile";
import type { DesktopApi } from "../services/desktopApi";
import { Shell } from "./Shell";

test("creates, edits, reorders, and saves scenes", async () => {
  const user = userEvent.setup();
  const harness = createHarness();

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await screen.findByRole("button", { name: "Ny scene" });

  await user.click(screen.getByRole("button", { name: "Ny scene" }));
  await user.clear(screen.getByLabelText("Scenenavn"));
  await user.type(screen.getByLabelText("Scenenavn"), "Middag");
  await user.click(screen.getByRole("button", { name: "Ny scene" }));
  await user.click(screen.getByRole("button", { name: "Flytt Ny scene opp" }));
  await user.click(screen.getByRole("button", { name: "Lagre oppsett" }));

  await waitFor(() => expect(harness.saveLibrary).toHaveBeenCalled());
  const saved = harness.saveLibrary.mock.calls[0][0] as CueLibrary;
  expect(saved.scenes.map((scene) => scene.name)).toEqual([
    "Ny scene",
    "Middag",
  ]);
});

test("imports audio, configures a cue, and triggers shared playback", async () => {
  const user = userEvent.setup();
  const harness = createHarness();

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await screen.findByRole("button", { name: "Importer lyd" });

  await user.click(screen.getByRole("button", { name: "Ny scene" }));
  await user.click(screen.getByRole("button", { name: "Importer lyd" }));
  expect(
    await screen.findByText("first-dance.wav er importert."),
  ).toBeVisible();
  await user.click(screen.getByRole("button", { name: "Ny cue" }));
  await user.clear(screen.getByLabelText("Navn på cue 1"));
  await user.type(screen.getByLabelText("Navn på cue 1"), "Første dans");
  await user.selectOptions(screen.getByLabelText("Modus"), "exclusive");
  await user.click(screen.getByRole("button", { name: "Lagre oppsett" }));
  await user.click(screen.getByRole("button", { name: "Spill Første dans" }));

  expect(harness.execute).toHaveBeenCalledWith(
    expect.objectContaining({ type: "triggerCue" }),
  );
});

test("shows recoverable command failures to the operator", async () => {
  const user = userEvent.setup();
  const harness = createHarness({
    scenes: [
      {
        id: "scene-1",
        name: "Fest",
        cues: [
          {
            id: "cue-1",
            name: "Intro",
            color: "#d88c68",
            audioFileId: "audio-1",
            volume: 0.8,
            mode: "overlap",
            fadeMs: 500,
          },
        ],
      },
    ],
    audioFiles: [audioFile],
  });
  harness.execute.mockResolvedValueOnce({
    protocolVersion: 1,
    commandId: "command-1",
    outcome: {
      status: "failure",
      revision: 0,
      error: {
        code: "audioBackend",
        message: "output disappeared",
      },
    },
  });

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await user.click(await screen.findByRole("button", { name: "Spill Intro" }));

  expect(
    await screen.findByText("Lydutgangen kunne ikke utføre kommandoen."),
  ).toBeVisible();
});

const audioFile: ManagedAudioFile = {
  id: "audio-1",
  fileName: "audio-1.wav",
  originalName: "first-dance.wav",
  format: "wav",
  byteLength: 44,
};

function createHarness(overrides: Partial<AppSnapshot> = {}): {
  api: DesktopApi;
  execute: ReturnType<typeof vi.fn<DesktopApi["execute"]>>;
  saveLibrary: ReturnType<typeof vi.fn<DesktopApi["saveLibrary"]>>;
} {
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
    ...overrides,
  };
  const execute = vi.fn<DesktopApi["execute"]>(async () => successAck());
  const saveLibrary = vi.fn<DesktopApi["saveLibrary"]>(async (library) => {
    snapshot = {
      ...snapshot,
      revision: snapshot.revision + 1,
      scenes: library.scenes,
      audioFiles: library.audioFiles,
    };
    return snapshot;
  });
  const api: DesktopApi = {
    getSnapshot: vi.fn(async () => snapshot),
    getControlServerInfo: vi.fn(async () => null),
    execute,
    saveLibrary,
    importAudio: vi.fn(async () => {
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        audioFiles: [audioFile],
      };
      return audioFile;
    }),
  };
  return { api, execute, saveLibrary };
}

function successAck(): CommandAck {
  return {
    protocolVersion: 1,
    commandId: "command-1",
    outcome: { status: "success", revision: 1 },
  };
}
