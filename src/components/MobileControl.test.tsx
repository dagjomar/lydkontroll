import {
  act,
  fireEvent,
  render,
  screen,
  waitFor,
} from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { expect, test, vi } from "vitest";
import type { AppSnapshot } from "../generated/AppSnapshot";
import type { CommandAck } from "../generated/CommandAck";
import type { ConnectionStatus, DesktopApi } from "../services/desktopApi";
import { MobileControl } from "./MobileControl";

test("shows touch controls and suppresses duplicate taps while awaiting acknowledgement", async () => {
  const user = userEvent.setup();
  const pending = deferred<CommandAck>();
  const harness = createHarness();
  harness.execute.mockReturnValueOnce(pending.promise);

  render(<MobileControl api={harness.api} pollIntervalMs={0} />);
  const cue = await screen.findByRole("button", { name: /Første dans/ });

  await user.click(cue);
  await user.click(cue);

  expect(harness.execute).toHaveBeenCalledTimes(1);
  expect(screen.getByText("Sender: spill Første dans")).toBeVisible();
  expect(cue).toBeDisabled();

  pending.resolve(successAck(4));
  expect(await screen.findByText("Bekreftet · revisjon 4")).toBeVisible();
});

test("disables stale controls while reconnecting and renders the fresh snapshot", async () => {
  const harness = createHarness();
  render(<MobileControl api={harness.api} pollIntervalMs={0} />);

  expect(
    await screen.findByRole("button", { name: /Første dans/ }),
  ).toBeEnabled();

  act(() => harness.setConnection("reconnecting"));
  expect(screen.getByText("Kobler til igjen")).toBeVisible();
  expect(screen.getByRole("button", { name: /Første dans/ })).toBeDisabled();

  act(() => {
    harness.setSnapshot({
      ...baseSnapshot,
      revision: 8,
      scenes: [
        {
          id: "scene-2",
          name: "Fest",
          cues: [
            {
              ...baseSnapshot.scenes[0].cues[0],
              id: "cue-2",
              name: "Dans",
            },
          ],
        },
      ],
    });
    harness.setConnection("connected");
  });

  expect(await screen.findByRole("button", { name: /Dans/ })).toBeEnabled();
  await waitFor(() =>
    expect(
      screen.queryByRole("button", { name: /Første dans/ }),
    ).not.toBeInTheDocument(),
  );
});

test("shows an expanding playback overlay only while audio is active", async () => {
  const harness = createHarness();
  render(<MobileControl api={harness.api} pollIntervalMs={0} />);

  const cue = await screen.findByRole("button", { name: /Første dans/ });
  expect(
    screen.queryByRole("region", { name: "Aktiv lyd" }),
  ).not.toBeInTheDocument();

  act(() => {
    harness.setSnapshot({
      ...baseSnapshot,
      revision: 4,
      activePlayback: [
        {
          id: "playback-1",
          cueId: "cue-1",
          volume: 0.8,
          fadeMs: 500,
          status: "playing",
        },
        {
          id: "playback-2",
          cueId: "cue-1",
          volume: 0.8,
          fadeMs: 500,
          status: "fading",
        },
      ],
    });
    harness.setConnection("connected");
  });

  expect(await screen.findByText("Første dans · spiller")).toBeVisible();
  expect(screen.getByText("Første dans · fader")).toBeVisible();
  expect(screen.getByRole("button", { name: /Første dans/ })).toBe(cue);
  expect(cue).toBeEnabled();

  act(() => {
    harness.setSnapshot({ ...baseSnapshot, revision: 5 });
    harness.setConnection("connected");
  });

  await waitFor(() =>
    expect(
      screen.queryByRole("region", { name: "Aktiv lyd" }),
    ).not.toBeInTheDocument(),
  );
  expect(screen.getByRole("button", { name: /Første dans/ })).toBe(cue);
});

test("keeps master volume draggable and coalesces changes while awaiting acknowledgement", async () => {
  const first = deferred<CommandAck>();
  const second = deferred<CommandAck>();
  const harness = createHarness();
  harness.execute
    .mockReturnValueOnce(first.promise)
    .mockReturnValueOnce(second.promise);

  render(<MobileControl api={harness.api} pollIntervalMs={0} />);
  const slider = await screen.findByRole("slider", { name: "Mastervolum" });

  fireEvent.input(slider, { target: { value: "0.8" } });
  fireEvent.input(slider, { target: { value: "0.6" } });
  fireEvent.input(slider, { target: { value: "0.4" } });

  expect(slider).toBeEnabled();
  expect(slider).toHaveValue("0.4");
  expect(harness.execute).toHaveBeenCalledTimes(1);
  expect(harness.execute).toHaveBeenLastCalledWith({
    type: "setMasterVolume",
    volume: 0.8,
  });

  first.resolve(successAck(4));
  await waitFor(() => expect(harness.execute).toHaveBeenCalledTimes(2));
  expect(harness.execute).toHaveBeenLastCalledWith({
    type: "setMasterVolume",
    volume: 0.4,
  });
  expect(slider).toBeEnabled();
  expect(slider).toHaveValue("0.4");

  second.resolve(successAck(5));
  expect(
    await screen.findByText(/Mastervolum 40 % · revisjon 5/),
  ).toBeVisible();
  expect(slider).toHaveValue("0.4");
});

test("shows authoritative master volume changes while locally idle", async () => {
  const harness = createHarness();
  render(<MobileControl api={harness.api} pollIntervalMs={0} />);
  const slider = await screen.findByRole("slider", { name: "Mastervolum" });

  act(() => {
    harness.setSnapshot({
      ...baseSnapshot,
      revision: 4,
      masterVolume: 0.35,
    });
    harness.setConnection("connected");
  });

  await waitFor(() => expect(slider).toHaveValue("0.35"));
});

const baseSnapshot: AppSnapshot = {
  revision: 3,
  scenes: [
    {
      id: "scene-1",
      name: "Middag",
      cues: [
        {
          id: "cue-1",
          name: "Første dans",
          color: "#d88c68",
          audioFileId: "audio-1",
          volume: 0.8,
          mode: "overlap",
          fadeMs: 500,
        },
      ],
    },
  ],
  audioFiles: [],
  activePlayback: [],
  pendingCueId: null,
  masterVolume: 1,
  preflight: {
    controlServer: { status: "ready" },
    audioOutput: { status: "ready" },
    audioFiles: { status: "ready" },
  },
  errors: [],
};

function createHarness() {
  let snapshot = baseSnapshot;
  let listener: ((status: ConnectionStatus) => void) | null = null;
  const execute = vi.fn<DesktopApi["execute"]>(async () => successAck(4));
  const api: DesktopApi = {
    mode: "mobile",
    getSnapshot: vi.fn(async () => snapshot),
    refreshPreflight: vi.fn(async () => snapshot),
    getControlServerInfo: vi.fn(async () => null),
    execute,
    saveLibrary: vi.fn(async () => snapshot),
    importAudio: vi.fn(async () => null),
    deleteManagedAudio: vi.fn(async () => snapshot),
    subscribeConnection(nextListener) {
      listener = nextListener;
      nextListener("connected");
      return () => {
        listener = null;
      };
    },
  };
  return {
    api,
    execute,
    setConnection(status: ConnectionStatus) {
      listener?.(status);
    },
    setSnapshot(next: AppSnapshot) {
      snapshot = next;
    },
  };
}

function successAck(revision: number): CommandAck {
  return {
    protocolVersion: 1,
    commandId: "command-1",
    outcome: { status: "success", revision },
  };
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((nextResolve) => {
    resolve = nextResolve;
  });
  return { promise, resolve };
}
