# Plan: TASK-019 — Phase-sensitive audio playback

Status: accepted
Updated: 2026-06-25

## Problem

Some MP3 files become hollow or cancel through the event mixer's line input,
while the same playback sounds correct through its CD input. The app currently
performs ordinary decoded stereo playback with volume only. A correction cannot
be designed safely until the source channel format and physical signal path are
measured independently.

## Current Evidence

- The symptom depends on mixer input choice: CD input works, line input does not.
- Logic Pro Direction Mixer appeared to help when `Input` changed from `LR` to
  `MS`.
- The apparent failure after exporting was caused by an unsaved app
  configuration that continued to reference the old managed file.
- After saving the configuration, the newly corrected Logic export played
  perfectly through the app.
- `src-tauri/src/adapters/audio/mod.rs` uses Kira's default backend and
  `StreamingSoundData::from_file`, then applies only cue/master volume.
- `PLAN.md` intentionally uses the selected macOS system output and an analog
  cable to the mixer.

## Resolved Questions

- The affected source required correction during Logic file preparation.
- Logic successfully rendered the correction into the new exported file.
- The app plays that corrected export properly once the cue selects it and the
  configuration is saved.
- No app-side channel transform or hardware-path workaround is currently
  required.

## Options Considered

### Option A: Correct the physical signal path

- Benefits: Preserves normal stereo files and avoids hidden DSP.
- Costs and risks: Requires the correct stereo-to-dual-mono or stereo mixer
  input and an event-hardware check.

### Option B: Normalize affected files during import

- Benefits: Produces standard playback with no live operator control.
- Costs and risks: Destructive unless the original managed copy is retained;
  detection and re-import behavior must be explicit.

### Option C: Store a per-file or per-cue channel transform

- Benefits: Handles exceptional sources without changing normal cues.
- Costs and risks: Expands domain/protocol/UI/audio-backend contracts and risks
  choosing the wrong transform under pressure.

### Option D: Add a global output transform

- Benefits: Simple operator control if the entire physical path requires it.
- Costs and risks: Can damage every normal file and hide a cable/input problem.

## Decision

Use source-file correction in Logic and export a standard playback-ready file.
After import, select the new managed file on the cue and save the
configuration before testing. Do not add app-side per-file, per-cue, or global
phase/M/S processing because the corrected export already plays properly
through the existing audio path.

## Implementation Slices

1. Correct affected source audio in Logic.
2. Render the correction into a new exported file.
3. Import and select the new file, then save the app configuration.
4. Play the saved cue through the event path to confirm the export.

## Test Strategy

- Automated: None required; no software change was selected.
- Manual: Correct in Logic, export, import/select, save configuration, and play
  the saved cue.
- Failure modes: Testing before saving and unknowingly playing the old managed
  file; exporting without rendering the Logic correction.

## Rollback or Recovery

Keep the original source and prior export. If a corrected export is unsuitable,
select the previous managed file and save the configuration again.

## Ready Checklist

- [x] Open questions are resolved or explicitly deferred.
- [x] Decisions and tradeoffs are recorded.
- [x] Slices are small and dependency ordered.
- [x] Tests cover the important failure modes.
- [x] The parent task is complete without product-code implementation.
