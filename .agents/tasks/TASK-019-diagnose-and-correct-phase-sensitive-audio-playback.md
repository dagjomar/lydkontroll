---
id: TASK-019
title: Diagnose and correct phase-sensitive audio playback
status: done
priority: P0
type: bug
owner: unassigned
depends_on: []
plan: .agents/plans/TASK-019-phase-sensitive-audio-playback.md
updated: 2026-06-25
---

# TASK-019: Diagnose and correct phase-sensitive audio playback

## Context

Some imported MP3 files initially sounded thin, hollow, or nearly cancelled.
A Logic Pro workaround used Direction Mixer with `Input: MS`. The apparent
failure of the corrected export in the app was later traced to the desktop
configuration not having been saved, so the cue was still playing the old
managed file.

The current app opens managed audio through Kira and the selected macOS system
output with volume adjustment only. It does not select a special output,
downmix channels, invert polarity, or apply stereo-width or M/S processing.
The input-dependent symptom therefore needs a controlled diagnosis across the
source file, app decoder, macOS output, cable wiring, and mixer input behavior
before adding a correction control.

## Outcome

Affected files play intelligibly and consistently through the event's actual
Mac-to-mixer signal path, with any required correction applied at the safest
scope and normal stereo files left unchanged.

## Scope

- Capture at least one affected MP3 and one known-good stereo control file.
- Compare playback from the app and a trusted macOS player through both the
  mixer's CD input and line input using the exact event cable path.
- Inspect channel count, channel layout, polarity/correlation, and whether the
  source appears to contain ordinary L/R stereo or encoded M/S material.
- Verify whether the line input and cable are stereo, mono, balanced, or
  summing/subtracting the Mac's two output channels.
- Decide from evidence whether the correction belongs in cabling/mixer setup,
  import-time normalization, managed-file metadata, a per-cue option, or a
  global output mode.
- If an app change is selected, preserve deterministic playback and document
  the chosen channel transform in the Rust-owned audio boundary.

## Non-goals

- Adding a speculative phase, mono, or M/S switch before the failure is
  reproduced and classified.
- Building a general-purpose audio editor or mastering suite.
- Changing macOS output-device selection inside the app.
- Modifying every imported file when only specific sources are affected.

## Acceptance Criteria

- [x] The reported failure was reproduced with the original affected export.
- [x] Saving the configuration and playing the newly exported file confirmed
      that the app reproduces the corrected Logic output properly.
- [x] The chosen correction scope is documented with reasons, including why it
      is per-file/per-cue, global, import-time, or external to the app.
- [x] The corrected export is clearly audible through the user's playback path.
- [x] No product-code regression coverage is required because no app channel
      transform is needed.
- [x] Operator guidance explains the required Logic export and configuration
      save/reselection workflow.
      setting before the event.

## Validation

```text
manual: correct the affected source in Logic and export a new audio file
manual: import/select the new file, save the configuration, and play the cue
manual: confirm the corrected export plays properly through the app
python3 scripts/ralph.py check
```

## Notes

- Priority P0 because destructive channel cancellation on the event signal path
  can make a successfully triggered cue unusable.
- Screenshot evidence: Logic Pro Direction Mixer appeared to correct one file
  when its input interpretation was changed from `LR` to `MS`, with direction
  `0.0°` and spread `1.00`.
- A stereo Mac output connected by a stereo TRS cable to a balanced mono line
  input can be interpreted as hot/cold and subtract right from left. This is a
  hypothesis to test, not an accepted diagnosis.
- Planning must choose the correction only after controlled hardware and file
  evidence; do not assume a global app transform.
- Feedback intake is complete; controlled file and hardware A/B must resolve
  correction scope before implementation.
- 2026-06-25: User confirmed the corrected Logic export plays perfectly in the
  app after saving the configuration. The prior test had unknowingly played the
  old managed file because the new cue configuration was not saved.
- Resolution: treat the phase/channel issue as source-file preparation. Apply
  the required correction in Logic, render it into the exported file, import
  and select that file, then save the app configuration. Do not add a global or
  per-cue phase/M/S toggle.
