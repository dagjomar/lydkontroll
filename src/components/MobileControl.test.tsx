import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
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

test("keeps the playback region mounted while playback starts and stops", async () => {
  const harness = createHarness();
  render(<MobileControl api={harness.api} pollIntervalMs={0} />);

  const cue = await screen.findByRole("button", { name: /Første dans/ });
  const playbackRegion = screen.getByRole("region", { name: "Aktiv lyd" });
  expect(screen.getByText("Ingen lyd spiller.")).toBeVisible();

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
  expect(screen.getByRole("region", { name: "Aktiv lyd" })).toBe(
    playbackRegion,
  );
  expect(screen.getByRole("button", { name: /Første dans/ })).toBe(cue);

  act(() => {
    harness.setSnapshot({ ...baseSnapshot, revision: 5 });
    harness.setConnection("connected");
  });

  expect(await screen.findByText("Ingen lyd spiller.")).toBeVisible();
  expect(screen.getByRole("region", { name: "Aktiv lyd" })).toBe(
    playbackRegion,
  );
  expect(screen.getByRole("button", { name: /Første dans/ })).toBe(cue);
});

test.each(["overlay", "controls", "tabs"] as const)(
  "keeps shared cue commands functional in the %s prototype",
  async (layout) => {
    const user = userEvent.setup();
    const harness = createHarness();
    render(
      <MobileControl api={harness.api} pollIntervalMs={0} layout={layout} />,
    );

    await user.click(
      await screen.findByRole("button", { name: /Første dans/ }),
    );

    expect(harness.execute).toHaveBeenCalledWith({
      type: "triggerCue",
      cueId: "cue-1",
    });
    cleanup();
  },
);

test("switches between all preview layouts without changing checkout", async () => {
  const user = userEvent.setup();
  const previousUrl = window.location.href;
  window.history.replaceState({}, "", "/?layout=overlay");
  const harness = createHarness();
  render(<MobileControl api={harness.api} pollIntervalMs={0} />);

  expect(await screen.findByTestId("mobile-layout-root")).toHaveAttribute(
    "data-mobile-layout",
    "overlay",
  );
  await user.click(screen.getByRole("button", { name: "2 · Cues først" }));
  expect(screen.getByTestId("mobile-layout-root")).toHaveAttribute(
    "data-mobile-layout",
    "controls",
  );
  await user.click(screen.getByRole("button", { name: "3 · Faner" }));
  expect(screen.getByTestId("mobile-layout-root")).toHaveAttribute(
    "data-mobile-layout",
    "tabs",
  );
  await user.click(screen.getByRole("button", { name: "Status" }));
  expect(screen.getByRole("region", { name: "Status og valg" })).toBeVisible();

  window.history.replaceState({}, "", previousUrl);
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
    getControlServerInfo: vi.fn(async () => null),
    execute,
    saveLibrary: vi.fn(async () => snapshot),
    importAudio: vi.fn(async () => null),
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
