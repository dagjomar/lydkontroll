# Tauri-basert lydkontroll for bryllupet

## Sammendrag

Bygg en Tauri 2-app for Apple Silicon Mac som spiller lokale MP3/WAV-filer gjennom macOS-systemutgangen. En iPhone styrer appen direkte over Tailscale, uten Bluetooth, stedets LAN eller ekstern skyserver.

## Implementasjon

- React/TypeScript-grensesnitt med redigeringsmodus og lokal reservekontroll.
- Rust-lydmotor med støtte for samtidige lyder, eksklusive cues, volum, stopp og fade.
- Cue-oppsett lagres lokalt med navn, farge, lydfil, volum, avspillingsmodus og fade-tid.
- Importerte lydfiler kopieres til appens lokale datamappe for å unngå brutte filreferanser.
- Cues organiseres i scener, eksempelvis per tale eller innslag.
- Rust kjører en lokal HTTP/WebSocket-server kun på Mac-ens Tailscale-adresse, port `17321`.
- Appen viser QR-kode og URL til mobilkontrollen.
- Mobilgrensesnittet viser scener, store cue-knapper, aktive lyder, mastervolum, «stopp alt», «fade alt» og forbindelsesstatus.
- WebSocket-kommandoer får unik ID, kvittering og oppdatert avspillingsstatus.
- Hvis Tailscale eller mobilen faller ut, fortsetter lyd og lokal kontroll på Mac-en uforstyrret.
- En preflight-visning kontrollerer Tailscale, lydutgang og alle lydfiler før arrangementet.

## Avspillingsregler

- «Overlapp»-cue kan spille samtidig med andre cues.
- «Eksklusiv»-cue fader eller stopper aktive cues før den starter.
- Ny triggering av samme cue starter den på nytt.
- Scenebytte påvirker ikke lyd som allerede spiller.
- Appen bruker macOS’ valgte systemutgang; lydutgang velges før festen i Systeminnstillinger.
- «Fade alt» bruker to sekunder som standard, mens individuelle cues kan ha egen fade-tid.

## Tester

- Import, lagring og gjenåpning av cue-oppsett.
- MP3/WAV-avspilling, overlapp, eksklusiv modus, stopp og fade.
- Synkronisering mellom lokale og mobile kontroller.
- Frakobling og automatisk gjenkobling av iPhone.
- Avvisning av kontrollserver når Tailscale ikke er tilgjengelig.
- Manglende eller ugyldige lydfiler vises tydelig uten appkrasj.
- Manuell generalprøve med iPhone på mobilnett, Mac på Wi-Fi og analog lydutgang.

## Forutsetninger

- Tailscale er installert og innlogget på både Mac og iPhone.
- Første versjon støtter macOS på Apple Silicon og iPhone/Safari.
- Tailscale er eneste tilgangskontroll; ingen ekstra PIN eller passord.
- Mobilen brukes kun til kontroll—lydfilene sendes aldri over nettverket.
