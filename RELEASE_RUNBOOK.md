# Release and Event Runbook

This runbook ties one Apple Silicon application build to automated checks,
target-hardware rehearsal evidence, recovery steps, and the event-day sequence.
Do not commit generated `.app` bundles, audio, local app data, or credentials.

## 1. Build the Candidate

Use the event Mac and a clean Git worktree:

```bash
npm ci
npm run release:build
```

The command runs the frontend build/tests, lint and generated-binding checks,
Rust formatting/Clippy/tests, a tracked-file safety check, and the Tauri release
build. The expected artifact is:

```text
src-tauri/target/release/bundle/macos/Lydkontroll.app
```

Record the exact candidate:

```bash
git rev-parse HEAD
node -p "require('./package.json').version"
file src-tauri/target/release/lydkontroll
shasum -a 256 src-tauri/target/release/lydkontroll
```

The `file` result must include `arm64`. Keep the previous rehearsed application
until the replacement passes every gate below.

## 2. Rehearsal Record

Copy this section into `.agents/REHEARSAL.md` for each release candidate.

```text
# Rehearsal Record

Date/time:
Operator:
Git commit:
App version:
macOS version:
Mac model:
Artifact path:
Executable SHA-256:
iPhone model/iOS:
Tailscale Mac/iPhone versions:
Analog output/cable/mixer:
Production library backup location:

Automated release check: PASS / FAIL
Packaged app launches: PASS / FAIL
Packaged Tailscale discovery and QR: PASS / FAIL
Managed MP3 survives source move/relaunch: PASS / FAIL
Managed WAV survives source move/relaunch: PASS / FAIL
Overlap/exclusive/retrigger: PASS / FAIL
Cue stop/fade and stop/fade all: PASS / FAIL
Master volume and scene changes: PASS / FAIL
Safari lock/reopen reconnect: PASS / FAIL
iPhone airplane mode/reconnect: PASS / FAIL
iPhone Tailscale quit/restart: PASS / FAIL
Mac network loss leaves local control/audio: PASS / FAIL
Analog cable loss/recovery: PASS / FAIL
macOS output switch/recovery: PASS / FAIL
60-minute run without crash/sleep/drift: PASS / FAIL
Event-day checklist dry run: PASS / FAIL

Blocking failures:
Recovery notes:
Final result: PASS / FAIL
```

## 3. Target-Hardware Rehearsal

Prepare the exact event setup:

- Mac connected to power with sleep disabled for the event window.
- Production cue library imported, saved, and backed up.
- Analog cable connected through the intended adapter/mixer.
- macOS system output set to the analog destination.
- Mac on the intended Wi-Fi with Tailscale connected.
- iPhone on cellular data with Wi-Fi disabled and Tailscale connected.

Run preflight. Missing files or control-server unavailability block the
rehearsal. Confirm the manual output warning, run the three-second test, scan
the displayed QR code, and then operate for at least 60 uninterrupted minutes.

Exercise both MP3 and WAV cues and verify:

- overlap, exclusive playback, and retrigger;
- cue stop/fade, stop/fade all, and master volume;
- scene changes while audio continues;
- editing, relaunch, and managed files after moving the original sources.

While audio is active, deliberately:

1. lock Safari and reopen it;
2. toggle iPhone airplane mode, restore cellular/Tailscale, and reconnect;
3. quit and restart Tailscale on the iPhone;
4. remove and restore Mac Wi-Fi/Tailscale after confirming audio and local
   controls remain available;
5. disconnect and restore the analog cable;
6. switch the macOS output away from and back to the analog destination.

Release is blocked by a crash, audio interruption caused by phone/network loss,
loss of Mac-local control, wrong cue semantics, missing persisted configuration,
binding outside Tailscale, or output failure that cannot be recovered with the
steps below.

## 4. Recovery

### Mobile control is unavailable

Continue from the Mac. Do not restart the application while audio is active.
When idle, restore cellular/Tailscale, reopen the current QR URL, and confirm a
fresh snapshot before using the phone again.

### Control server is unavailable

Keep using the Mac. When idle, restart Tailscale and then Lydkontroll. Refresh
preflight and scan the newly displayed QR code. Never substitute a LAN or
wildcard bind.

### Audio output is wrong or silent

Stop playback, verify cable/mixer power and levels, select the analog output in
macOS System Settings, refresh preflight, and run the three-second test. Keep
the Mac's local controls visible during recovery.

### Configuration is missing or corrupt

Stop editing and preserve the app-data directory. Keep the production library
backup and `library.json.bak`; recover the backup before importing or saving
more cues.

## 5. Event-Day Checklist

### Before guests arrive

- [ ] Use only the rehearsed commit and executable checksum.
- [ ] Connect Mac power and disable sleep.
- [ ] Connect and secure the analog cable; select the correct macOS output.
- [ ] Start Tailscale on Mac and iPhone.
- [ ] Open Lydkontroll and confirm the production scenes/cues.
- [ ] Pass preflight and the three-second test.
- [ ] Scan the current QR code from the iPhone on cellular data.
- [ ] Trigger one non-critical cue from the phone and stop/fade it from the Mac.
- [ ] Keep the Mac editor/control view open as the fallback.
- [ ] Put the previous rehearsed build and library backup somewhere accessible.

### During the event

- [ ] Treat the Mac as authoritative; use the phone only when status is fresh.
- [ ] Do not restart the app, Tailscale, or output hardware during active audio.
- [ ] If the phone disconnects, continue locally and recover it only when safe.

### After the event

- [ ] Stop/fade all playback.
- [ ] Save the final library and preserve the rehearsal/event notes.
- [ ] Quit the app before disconnecting output hardware.
