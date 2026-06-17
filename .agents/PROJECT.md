# Project Context

## Mission

Build a reliable Tauri 2 sound-control application for a wedding. An Apple
Silicon Mac plays local MP3/WAV files through the selected macOS output, while
an iPhone controls cues over Tailscale.

## Product Invariants

- Playback must continue if the phone, WebSocket, Wi-Fi, or Tailscale connection
  drops.
- The Mac must always retain local control.
- Audio files stay on the Mac and are copied into application-managed storage.
- The control server binds only to the active Tailscale address on port `17321`.
- Tailscale is the access-control boundary for version one.
- Commands use unique IDs, acknowledgements, and authoritative playback state.
- Overlap, exclusive playback, retrigger, stop, and fade behavior must be
  deterministic.
- Scene changes do not stop already-playing audio.
- The preflight view must expose network, output, and file problems before the
  event.

## Engineering Boundaries

- React renders controls and views; it does not own playback or networking
  behavior.
- Rust owns the audio engine, persistence, command handling, and control server.
- Shared protocol and state types need one documented source of truth.
- Static web assets belong in `public/`; user audio never does.
- The first supported targets are Apple Silicon macOS and iPhone Safari.

## Delivery Strategy

Build in vertical, verifiable slices:

1. settle architecture, protocol ownership, and core library choices;
2. scaffold a runnable Tauri shell and test harness;
3. implement persistence and managed audio import;
4. implement and test the audio engine;
5. expose one authoritative application state and command path;
6. add the Tailscale-only HTTP/WebSocket control server;
7. build desktop editing/local control and mobile control views;
8. add preflight, reconnect hardening, packaging, and rehearsal checks.

Reliability at the event matters more than framework novelty or broad platform
support.
