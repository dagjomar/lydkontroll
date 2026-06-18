# Mobile layout prototypes

Three functional iPhone layouts are available for comparison without changing
branches. The normal URL still uses the current stable layout.

## Open locally with mock audio

Start the frontend:

```text
npm run dev
```

Then open any of these:

- Overlay: `http://127.0.0.1:1420/?preview=mobile&layout=overlay`
- Cues first: `http://127.0.0.1:1420/?preview=mobile&layout=controls`
- Two tabs: `http://127.0.0.1:1420/?preview=mobile&layout=tabs`

The bottom switcher changes layouts while preserving the current mock playback
state. Trigger the same cue several times to compare multiple active sounds.

## Test on the real iPhone control

Append one option to the mobile URL shown by the Mac:

```text
?layout=overlay
?layout=controls
?layout=tabs
```

For example:

```text
http://100.x.y.z:17321/?layout=overlay
```

This uses the real WebSocket, cue library, and playback state. Remove the
`layout` query to return to the current stable production layout.

## What to compare

- How quickly the next cue can be found and triggered.
- Whether multiple active sounds remain understandable.
- Whether stop/fade controls are reachable under pressure.
- Whether connection and master controls are visible often enough.
- How much accidental-tap risk each layout creates.
