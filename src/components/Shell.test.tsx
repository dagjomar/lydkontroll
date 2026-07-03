import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
  within,
} from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, expect, test, vi } from "vitest";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { CommandAck } from "../generated/CommandAck";
import type { CueLibrary } from "../generated/CueLibrary";
import type { ManagedAudioFile } from "../generated/ManagedAudioFile";
import type { DesktopApi } from "../services/desktopApi";
import { Shell } from "./Shell";

vi.mock("qrcode", () => ({
  default: {
    toDataURL: vi.fn(async () => "data:image/png;base64,preflight"),
  },
}));

afterEach(() => {
  vi.useRealTimers();
});

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

test("persists a reusable event title and shows its empty fallback", async () => {
  const user = userEvent.setup();
  const harness = createHarness();

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  const title = await screen.findByLabelText("Arrangementstittel");
  expect(screen.getByText("Mitt arrangement")).toBeVisible();

  await user.clear(title);
  expect(screen.getByText("Mitt arrangement")).toBeVisible();
  await user.type(title, "Sommerfesten");
  await user.click(screen.getByRole("button", { name: "Lagre oppsett" }));

  expect(harness.saveLibrary).toHaveBeenCalledWith(
    expect.objectContaining({ eventTitle: "Sommerfesten" }),
  );
  expect(screen.getByText("Sommerfesten")).toBeVisible();
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

test("lists duplicate-name imports and deletes only an unreferenced confirmed file", async () => {
  const user = userEvent.setup();
  const referenced = {
    ...audioFile,
    fileName: "11111111-1111-1111-1111-111111111111.wav",
  };
  const obsolete = {
    ...audioFile,
    id: "audio-2",
    fileName: "22222222-2222-2222-2222-222222222222.wav",
  };
  const harness = createHarness({
    scenes: [
      {
        id: "scene-1",
        name: "Fest",
        cues: [
          {
            id: "cue-1",
            name: "Introduksjon",
            color: "#d88c68",
            audioFileId: referenced.id,
            volume: 0.8,
            mode: "overlap",
            fadeMs: 500,
          },
        ],
      },
    ],
    audioFiles: [referenced, obsolete],
  });
  const confirm = vi.spyOn(window, "confirm").mockReturnValue(true);

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await user.click(
    await screen.findByText("Administrerte lydfiler", { exact: false }),
  );

  const audioLibrary = screen
    .getByText("Administrerte lydfiler", { exact: false })
    .closest("details");
  expect(audioLibrary).not.toBeNull();
  expect(
    within(audioLibrary as HTMLElement).getAllByText("first-dance.wav"),
  ).toHaveLength(2);
  expect(screen.getByText("Brukes av: Introduksjon")).toBeVisible();
  expect(
    screen.getByRole("button", {
      name: "Slett first-dance.wav (11111111.wav)",
    }),
  ).toBeDisabled();

  await user.click(
    screen.getByRole("button", {
      name: "Slett first-dance.wav (22222222.wav)",
    }),
  );

  expect(confirm).toHaveBeenCalledWith(
    "Slett first-dance.wav permanent fra administrert lagring?",
  );
  expect(harness.deleteManagedAudio).toHaveBeenCalledWith("audio-2");
  expect(await screen.findByText("first-dance.wav er slettet.")).toBeVisible();
  expect(
    within(audioLibrary as HTMLElement).getAllByText("first-dance.wav"),
  ).toHaveLength(1);
  confirm.mockRestore();
});

test("cancelling managed audio deletion changes nothing", async () => {
  const user = userEvent.setup();
  const harness = createHarness({ audioFiles: [audioFile] });
  const confirm = vi.spyOn(window, "confirm").mockReturnValue(false);

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await user.click(
    await screen.findByText("Administrerte lydfiler", { exact: false }),
  );
  await user.click(
    screen.getByRole("button", {
      name: "Slett first-dance.wav (audio-1.wav)",
    }),
  );

  expect(harness.deleteManagedAudio).not.toHaveBeenCalled();
  expect(screen.getByText("first-dance.wav")).toBeVisible();
  confirm.mockRestore();
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

test("shows blocking failures, manual checks, and the mobile access QR", async () => {
  const user = userEvent.setup();
  const harness = createHarness(
    {
      preflight: {
        controlServer: { status: "ready" },
        audioOutput: {
          status: "warning",
          message: "Kontroller valgt lydutgang.",
        },
        audioFiles: {
          status: "unavailable",
          message: "Mangler administrert lydfil for: Introduksjon.",
        },
      },
    },
    {
      address: "100.64.0.10",
      port: 17321,
      url: "http://100.64.0.10:17321/",
    },
  );

  render(<Shell api={harness.api} pollIntervalMs={0} />);

  const status = await screen.findByRole("button", {
    name: /Tiltak kreves Vis innstillinger/,
  });
  expect(status).toHaveAttribute("aria-expanded", "false");
  expect(screen.queryByText("1 blokkering må løses.")).not.toBeInTheDocument();

  await user.click(status);
  expect(await screen.findByText("1 blokkering må løses.")).toBeVisible();
  expect(screen.getByText("Kontroller valgt lydutgang.")).toBeVisible();
  expect(
    screen.getByText("Mangler administrert lydfil for: Introduksjon."),
  ).toBeVisible();
  expect(await screen.findByAltText("QR-kode for mobilkontroll")).toBeVisible();
  expect(
    screen.getByRole("link", { name: "http://100.64.0.10:17321/" }),
  ).toBeVisible();

  const refreshCount = vi.mocked(harness.api.refreshPreflight).mock.calls
    .length;
  await user.click(screen.getByRole("button", { name: "Kjør sjekk på nytt" }));
  expect(harness.api.refreshPreflight).toHaveBeenCalledTimes(refreshCount + 1);
});

test("keeps ready preflight details collapsed until requested", async () => {
  const user = userEvent.setup();
  const harness = createHarness();

  render(<Shell api={harness.api} pollIntervalMs={0} />);

  const status = await screen.findByRole("button", {
    name: /System klart Vis innstillinger/,
  });
  expect(status).toHaveAttribute("aria-expanded", "false");
  expect(screen.queryByRole("heading", { name: "Preflight" })).toBeNull();

  await user.click(status);
  expect(status).toHaveAttribute("aria-expanded", "true");
  expect(screen.getByRole("heading", { name: "Preflight" })).toBeVisible();

  await user.click(status);
  expect(screen.queryByRole("heading", { name: "Preflight" })).toBeNull();
});

test("safe test play triggers a saved cue and fades it after three seconds", async () => {
  vi.useFakeTimers();
  const harness = createHarness({
    scenes: [
      {
        id: "scene-1",
        name: "Fest",
        cues: [
          {
            id: "cue-1",
            name: "Introduksjon",
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

  render(<Shell api={harness.api} pollIntervalMs={0} />);
  await act(async () => {
    await Promise.resolve();
  });
  fireEvent.click(
    screen.getByRole("button", {
      name: /System klart Vis innstillinger/,
    }),
  );
  fireEvent.click(screen.getByRole("button", { name: "Spill i 3 sekunder" }));
  await act(async () => {
    await Promise.resolve();
  });

  expect(harness.execute).toHaveBeenCalledWith({
    type: "triggerCue",
    cueId: "cue-1",
  });

  await act(async () => {
    await vi.advanceTimersByTimeAsync(3000);
  });
  expect(harness.execute).toHaveBeenCalledWith({
    type: "fadePlayback",
    playbackId: "test-playback",
  });
});

const audioFile: ManagedAudioFile = {
  id: "audio-1",
  fileName: "audio-1.wav",
  originalName: "first-dance.wav",
  format: "wav",
  byteLength: 44,
};

function createHarness(
  overrides: Partial<AppSnapshot> = {},
  controlServerInfo: Awaited<
    ReturnType<DesktopApi["getControlServerInfo"]>
  > = null,
): {
  api: DesktopApi;
  execute: ReturnType<typeof vi.fn<DesktopApi["execute"]>>;
  saveLibrary: ReturnType<typeof vi.fn<DesktopApi["saveLibrary"]>>;
  deleteManagedAudio: ReturnType<
    typeof vi.fn<DesktopApi["deleteManagedAudio"]>
  >;
} {
  let snapshot: AppSnapshot = {
    revision: 0,
    eventTitle: "Mitt arrangement",
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
  const execute = vi.fn<DesktopApi["execute"]>(async (command) => {
    if (command.type === "triggerCue") {
      snapshot = {
        ...snapshot,
        activePlayback: [
          {
            id: "test-playback",
            cueId: command.cueId,
            volume: 0.8,
            fadeMs: 500,
            status: "playing",
          },
        ],
      };
    }
    return successAck();
  });
  const saveLibrary = vi.fn<DesktopApi["saveLibrary"]>(async (library) => {
    snapshot = {
      ...snapshot,
      revision: snapshot.revision + 1,
      eventTitle: library.eventTitle,
      scenes: library.scenes,
      audioFiles: library.audioFiles,
    };
    return snapshot;
  });
  const deleteManagedAudio = vi.fn<DesktopApi["deleteManagedAudio"]>(
    async (audioFileId) => {
      snapshot = {
        ...snapshot,
        revision: snapshot.revision + 1,
        audioFiles: snapshot.audioFiles.filter(
          (audio) => audio.id !== audioFileId,
        ),
      };
      return snapshot;
    },
  );
  const api: DesktopApi = {
    mode: "desktop",
    getSnapshot: vi.fn(async () => snapshot),
    refreshPreflight: vi.fn(async () => snapshot),
    getControlServerInfo: vi.fn(async () => controlServerInfo),
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
    deleteManagedAudio,
  };
  return { api, execute, saveLibrary, deleteManagedAudio };
}

function successAck(): CommandAck {
  return {
    protocolVersion: 1,
    commandId: "command-1",
    outcome: { status: "success", revision: 1 },
  };
}
