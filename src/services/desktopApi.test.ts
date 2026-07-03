import { afterEach, expect, test, vi } from "vitest";
import type { AppSnapshot } from "../generated/AppSnapshot";
import { createRemoteApi, type ConnectionStatus } from "./desktopApi";

afterEach(() => {
  FakeWebSocket.instances = [];
  vi.unstubAllGlobals();
});

test("invalidates disconnected state and ignores snapshots from stale sockets", async () => {
  vi.stubGlobal("WebSocket", FakeWebSocket);
  const api = createRemoteApi();
  const statuses: ConnectionStatus[] = [];
  api.subscribeConnection?.((status) => statuses.push(status));

  const initial = api.getSnapshot();
  const first = FakeWebSocket.instances[0];
  first.open();
  first.message(snapshotMessage(3, "Første dans"));
  expect((await initial).revision).toBe(3);

  first.close();
  const reconnected = api.getSnapshot();
  const second = FakeWebSocket.instances[1];
  first.message(snapshotMessage(99, "Gammel socket"));

  let settled = false;
  void reconnected.then(() => {
    settled = true;
  });
  await Promise.resolve();
  expect(settled).toBe(false);

  second.open();
  second.message(snapshotMessage(4, "Ny autoritativ"));
  const fresh = await reconnected;
  expect(fresh.revision).toBe(4);
  expect(fresh.scenes[0].cues[0].name).toBe("Ny autoritativ");
  expect(statuses).toEqual([
    "connecting",
    "connected",
    "reconnecting",
    "connected",
  ]);
});

class FakeWebSocket {
  static CONNECTING = 0;
  static OPEN = 1;
  static CLOSING = 2;
  static CLOSED = 3;
  static instances: FakeWebSocket[] = [];

  readyState = FakeWebSocket.CONNECTING;
  sent: string[] = [];
  private listeners = new Map<string, Array<(event: MessageEvent) => void>>();

  constructor(readonly url: string) {
    FakeWebSocket.instances.push(this);
  }

  addEventListener(type: string, listener: (event: MessageEvent) => void) {
    const listeners = this.listeners.get(type) ?? [];
    listeners.push(listener);
    this.listeners.set(type, listeners);
  }

  send(value: string) {
    this.sent.push(value);
  }

  open() {
    this.readyState = FakeWebSocket.OPEN;
    this.emit("open", new MessageEvent("open"));
  }

  message(data: string) {
    this.emit("message", new MessageEvent("message", { data }));
  }

  close() {
    this.readyState = FakeWebSocket.CLOSED;
    this.emit("close", new MessageEvent("close"));
  }

  private emit(type: string, event: MessageEvent) {
    for (const listener of this.listeners.get(type) ?? []) {
      listener(event);
    }
  }
}

function snapshotMessage(revision: number, cueName: string): string {
  const snapshot: AppSnapshot = {
    revision,
    eventTitle: "Mitt arrangement",
    scenes: [
      {
        id: "scene-1",
        name: "Middag",
        cues: [
          {
            id: "cue-1",
            name: cueName,
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
  return JSON.stringify({ type: "snapshot", snapshot });
}
