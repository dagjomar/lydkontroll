# Design- og innholdsbrief for Lydkontroll

Dette dokumentet er en kildebasert overlevering til designer eller Google
Stitch. Det beskriver produktet slik det finnes i dag, og skiller faste rammer
fra visuelle valg som kan utforskes. Endelig design og tekst skal godkjennes av
produkteier før nettstedet bygges.

## Produktet kort fortalt

**Lydkontroll** er en norsk, lokal-først lydkontroll for arrangementer. En
Apple Silicon-Mac spiller lokale MP3- og WAV-filer gjennom macOS' valgte
lydutgang. Operatøren kan styre alt fra Mac-en og kan i tillegg bruke en iPhone
med Safari over Tailscale som fjernkontroll.

Målgruppen er toastmastere, teknikere og andre som må starte, stoppe og fade
lydinnslag presist uten å være bundet til Mac-en. Produktløftet er ro og
kontroll i et øyeblikk der feil blir synlige: lyden ligger og spilles lokalt,
Mac-en beholder kontrollen, og avspilling fortsetter selv om telefonen eller
nettforbindelsen faller ut.

Lydkontroll er arrangementsgenerisk, ikke et bryllupsprodukt. Operatøren kan
sette en egen arrangementstittel; standard og reserveverdi er «Mitt
arrangement». Første lansering er på norsk.

## Et typisk arbeidsforløp

1. Operatøren importerer lydfiler til appens administrerte lagring på Mac-en.
2. Lydinnslag organiseres som cues i scener, for eksempel etter taler eller
   programposter.
3. Hvert cue får navn, farge, fil, volum, avspillingsmodus og fade-tid.
4. Før arrangementet viser preflight nettverk, lydutgang og manglende filer,
   samt QR-kode og adresse til mobilkontrollen.
5. Under arrangementet startes cues fra Mac eller iPhone. Flere lyder kan
   overlappe, mens eksklusive cues stopper eller fader aktive lyder først.
6. Operatøren kan alltid stoppe eller fade alt og styre mastervolumet lokalt.

## Det som finnes nå

- Lokal import, redigering, lagring og gjenåpning av scener og cues.
- Lokal avspilling av MP3/WAV med overlapp, eksklusiv modus, retrigger, stopp,
  fade, cue-volum og mastervolum.
- Mac-grensesnitt for oppsett, avspilling, aktive lyder og preflight.
- iPhone/Safari-grensesnitt med store cue-knapper, scener, aktive lyder,
  mastervolum, stopp alt, fade alt og tilkoblingsstatus.
- Autoritativ synkronisering og automatisk ny tilkobling mellom Mac og mobil.
- Lydfiler forlater ikke Mac-en; mobilen sender bare kommandoer og mottar
  status.

## Krav og ærlige begrensninger

- Støttet mål er Apple Silicon macOS. Mobilkontroll er laget for iPhone/Safari.
- Mac-kontroll og avspilling virker uten telefon, Tailscale eller internett.
- Mobilkontroll i versjon én krever Tailscale på både Mac og iPhone. Vanlig
  lokalnett, skyrelay, Android, Windows og Intel Mac støttes ikke.
- Tailscale-medlemskap er tilgangskontrollen; appen har ikke eget passord eller
  PIN. Personer med tilgang til riktig tailnet kan potensielt nå kontrollen.
- Lydutgang velges i macOS Systeminnstillinger.
- Det finnes foreløpig ingen offentlig, signert eller notarisert app for
  nedlasting. GitHub-repositoriet tilbyr kildekode og byggeinstruksjoner.
- Prosjektet er **kildetilgjengelig**, ikke OSI-åpen kildekode. Egen kode er
  lisensiert under PolyForm Noncommercial 1.0.0. Kommersiell bruk krever egen
  avtale; fremtidige kommersielle bygg eller tjenester er ikke lovet.

Nettstedet skal aldri antyde App Store-tilgjengelighet, bred
plattformstøtte, vanlig LAN-kontroll, skylagring, automatisk nettverksfallback
eller en ferdig internasjonal utgave.

## Merkevare og visuell retning

Uttrykket skal være varmt, mørkt, rolig og operatørrettet. Det skal føles mer
som et pålitelig kontrollbord i dempet arrangementslys enn som en aggressiv
DJ-app eller et romantisk bryllupstema. Store, tydelige handlinger og synlig
systemstatus er viktigere enn dekorasjon.

### Fargepalett med roller

| Rolle           | Kildeverdi            | Bruk                                  |
| --------------- | --------------------- | ------------------------------------- |
| Hovedbakgrunn   | `#11100e`             | Nær-svart, varm sidebakgrunn          |
| Ikonbakgrunn    | `#181512`             | Den avrundede appflaten               |
| Feltbakgrunn    | `#191715`             | Input og mørke innfelte flater        |
| Kortflate       | `#211e1b`             | Paneler og statuskort                 |
| Kontrollflate   | `#302b27`             | Sekundære knapper                     |
| Kontroll hover  | `#413a34`             | Tydelig, dempet hoverrespons          |
| Primær tekst    | `#f8f4ec`             | Varm kremhvit tekst                   |
| Sekundær tekst  | `#cfc8bd`             | Forklaringer og sammendrag            |
| Dempet tekst    | `#a9a096` / `#bdb4a8` | Metadata og mindre viktige fakta      |
| Primær handling | `#e0a77f`             | Terrakotta/fersken for play og CTA    |
| Aksent/eyebrow  | `#dcae8c`             | Små merkevaremarkører                 |
| Fokus           | `#efc6a8`             | Synlig tastaturfokus, 3 px            |
| Standard cue    | `#d88c68`             | Standard identitetsfarge for nye cues |
| Klar/suksess    | `#79c58d`             | Preflight og tilkobling som er klar   |
| Advarsel        | `#e5a85a`             | Forhold som krever oppmerksomhet      |
| Feil/fare       | `#df776f` / `#7e302d` | Blokkering, feil og destruktive valg  |

Den svake terrakottafargede radialgløden øverst i appen kan brukes som et
gjenkjennelig bakgrunnsmotiv. Statusfargene skal beholde semantikken og aldri
være eneste informasjonsbærer.

### Typografi, form og bevegelse

- Displayoverskrifter bruker Georgia/Times-karakter: redaksjonell, rolig serif
  med tett bokstavmellomrom. Brødtekst og kontroller bruker Inter/system-sans
  for rask avlesning.
- Kort og kontroller har myke hjørner, hovedsakelig `0.65–1.2rem`; kompakte
  statuskontroller kan være helt pilleformede.
- Flater skilles med lavkontrast hvite rammer og små tonetrinn, ikke tunge
  skygger. Luft og tydelig gruppering skal bære hierarkiet.
- Bevegelse skal være sparsom, rask og funksjonell. Unngå autoplay-animasjoner,
  parallax og effekter som konkurrerer med status eller CTA.
- Behold store berøringsmål, sterk fokusmarkering, god kontrast, lesbar norsk
  tekst og redusert-bevegelse-vennlig oppførsel. Ikke legg tekst i bilder.

### Logo og ikon

Bruk den nøytrale, avrundede bølgeformen fra `src-tauri/icons/icon.svg`: varm
mørk kvadratflate med kremfarget waveform og runde linjeender. Ikke gjeninnfør
hjerte, navn på personer eller bryllupssymbolikk. På nettstedet kan symbolet stå
alene eller sammen med ordmerket «Lydkontroll», men selve formen og den rolige
karakteren skal bevares.

## Foreslått informasjonsarkitektur

### 1. Hero: kontroll når øyeblikket teller

Forklar produktet i én setning og vis Mac og iPhone i samme reelle situasjon.
Foreslått arbeidsoverskrift: **«Riktig lyd. Riktig øyeblikk.»** Støtteteksten
skal si at Mac-en spiller lokale lydfiler, mens operatøren styrer fra Mac eller
iPhone over Tailscale. Primær CTA bør foreløpig være kildekode/bygging på
GitHub, ikke «Last ned».

### 2. Problemet og gevinsten

Beskriv stresset ved mapper, mediespillere og en Mac som står feil plassert.
Vis gevinsten: organiserte cues, store trykkflater og lokal kontroll som blir
igjen når mobilen mister kontakt.

### 3. Funksjonene i grupper

- **Forbered:** importer filer, bygg scener og cues, angi volum og fade.
- **Gjennomfør:** overlap, eksklusiv avspilling, retrigger, stopp/fade alt.
- **Beveg deg:** bruk en iPhone-kontroll uten at lyd sendes over nettet.
- **Sjekk før start:** preflight, filstatus, lydutgang og tilkobling.

### 4. Slik virker det

En enkel tretrinnsfigur: lokale filer på Mac → lokal lydmotor → valgfrie
kommandoer/status over Tailscale til iPhone. Gjør det visuelt utvetydig at
lydfilene og avspillingen forblir på Mac-en.

### 5. Tillit og robusthet

Fremhev lokal avspilling, Mac som permanent reservekontroll, deterministiske
avspillingsregler, gjenoppkobling og synlig preflight. Bruk konkrete fakta, ikke
ord som «feilfri», «garantert» eller «profesjonell standard».

### 6. Krav og sikkerhetsmodell

Oppgi Apple Silicon Mac, iPhone/Safari og Tailscale for mobilkontroll. Forklar
kort at desktop fungerer uten nett og at tailnet er tilgangsgrensen.

### 7. Kilde og lisens

Forklar at kildekoden kan inspiseres og bygges fra GitHub under PolyForm
Noncommercial. Skill tydelig mellom ikke-kommersiell bruk og eventuell separat
kommersiell lisens. Ikke presenter dette som «open source».

### 8. Avsluttende CTA

Inviter til å se kildekoden, lese byggeveiledningen eller følge utviklingen.
Endelig CTA, URL-struktur og eventuell kommersiell kontakt avgjøres av eier før
implementering.

## Skjermbilde- og assetliste

Bruk ekte produktbilder med fiktive, arrangementsgeneriske cue-navn og uten
persondata eller lisensierte lydfiler.

1. **Mac-oversikt:** scener og fargede cue-kort, med aktive transportkontroller.
   Skal kommunisere oversikt og lokal betjening.
2. **Mac-redigering:** cue med fil, volum, modus og fade. Skal vise at oppsettet
   kan forberedes presist.
3. **Kompakt preflight:** grønn status og eventuelt åpent detaljpanel med QR.
   Skal vise kontroll før dørene åpnes, ikke love automatisk lydverifisering.
4. **iPhone cue-rutenett:** store knapper, scenevalg og stabil toppstatus i en
   realistisk Safari-viewport. Skal kommunisere trygg én-håndsbetjening.
5. **iPhone med aktiv lyd:** «Spiller nå»-overlay, mastervolum, stopp/fade alt.
   Skal vise at aktive lyder er synlige uten at cue-posisjonene hopper.
6. **Mac + iPhone sammen:** én komposisjon som viser at mobilen kontrollerer,
   mens Mac-en spiller. Unngå grafikk som ser ut som lydstreaming.
7. **Appikon:** eksporter den nøytrale waveform-ressursen rent, med beholdt
   avrundet form og transparent utside.

For hvert skjermbilde bør lys, utsnitt og skala være konsistent. Ta også en
smal mobilvariant og en bred desktopvariant uten enhetsrammer, slik at designet
kan bruke dem responsivt.

## Kopierbar brief til Google Stitch

```text
Lag et første designkonsept for en responsiv, norsk markedsføringsside for
produktet «Lydkontroll».

PRODUKTFAKTA SOM IKKE SKAL ENDRES:
Lydkontroll er en norsk, lokal-først lydkontroll for live-arrangementer. En
Apple Silicon-Mac spiller lokale MP3/WAV-filer gjennom macOS' valgte lydutgang.
Operatøren kan importere lyd, organisere cues i scener, spille overlappende
eller eksklusive cues, retrigge, styre volum og stoppe eller fade. Mac-en har
alltid lokal kontroll, og avspilling fortsetter om telefon eller nett faller ut.
Valgfri mobilkontroll bruker iPhone/Safari over Tailscale. Lydfilene forblir på
Mac-en; mobilen sender kommandoer og mottar status. Versjon én støtter ikke
Android, Windows, Intel Mac, vanlig LAN, skyrelay eller App Store-nedlasting.
Det finnes foreløpig ingen offentlig signert/notarisert appbundle.
Kildekoden er tilgjengelig på GitHub under PolyForm Noncommercial 1.0.0;
prosjektet er source-available/kildetilgjengelig, ikke open source. Kommersiell
bruk krever separat lisens. Ikke lov at kommersielle bygg eller tjenester finnes.

FAST MERKEVARE:
Navnet er «Lydkontroll». Produktet er arrangementsgenerisk og norsk-først.
Tonen skal være rolig, presis, varm og pålitelig – ikke en neonpreget DJ-app og
ikke romantisk bryllupsdesign. Bruk en nøytral avrundet waveform som symbol,
uten hjerte. Visuell kjerne: varm nær-svart #11100e, mørke flater #191715 og
#211e1b, kremtekst #f8f4ec, terrakotta handling #e0a77f, samt grønn #79c58d,
amber #e5a85a og rød #df776f kun som tydelige statusroller. Kombiner en rolig
serif i store overskrifter med Inter/system-sans i brødtekst og kontroller.
Bruk avrundede kort, lavkontrast rammer, god luft, høy lesbarhet, tydelig
tastaturfokus og store berøringsmål. Status må aldri formidles bare med farge.

SIDESTRUKTUR:
1) Hero med arbeidstittelen «Riktig lyd. Riktig øyeblikk.», kort forklaring,
Mac+iPhone-produktbilde og CTA til GitHub/kildekode (ikke «Last ned»).
2) Problem og gevinst for toastmaster eller arrangementsoperatør.
3) Funksjoner gruppert som Forbered, Gjennomfør, Beveg deg og Sjekk før start.
4) Enkel «Slik virker det»-figur: filer og avspilling på Mac, bare
kommando/status over Tailscale til iPhone.
5) Robusthet: lokal avspilling, Mac-reserve, gjenoppkobling og preflight.
6) Krav og sikkerhetsmodell.
7) Kildetilgjengelighet og ikke-kommersiell lisens.
8) Avsluttende GitHub/følg-utviklingen-CTA.

DESIGNFRIHET:
Utforsk komposisjon, seksjonsrytme, produktbildeplassering, diskrete
waveform-motiver, kortlayout og hvordan en varm radial glød kan gi dybde.
Foreslå gjerne bedre norsk mikrocopy, men merk den som forslag. Hold animasjon
sparsom og funksjonell. Design både bred desktop og 390 px mobil. Unngå
prisplaner, testimonials, kundelogoer, oppdiktede måltall og funksjonsløfter.

Konseptet er et forslag til eiergjennomgang, ikke et ferdig godkjent design.
```

## Kilder og driftssjekk

Produktpåstander skal ved senere revisjoner kontrolleres mot:

- `PLAN.md` og `.agents/PROJECT.md`: produktmål, invariants og målplattformer.
- `README.md`, `SUPPORT.md` og `SECURITY.md`: offentlig funksjon, begrensninger
  og sikkerhetsmodell.
- `.agents/DECISIONS.md`, særlig ADR-014 og ADR-015: navn, norsk-first,
  arrangementstittel, ikon og lisensposisjon.
- `src/styles.css`: implementert palett, typografi, former og responsivitet.
- `src/components/Shell.tsx`: standard cue-farge og desktoparbeidsflyt.
- `src/components/MobileControl.tsx`: mobilfunksjoner og statuspresentasjon.
- `src-tauri/icons/icon.svg`: kanonisk waveform-symbol.

Design og tekst er bevisst et første forslag. Produkteier må godkjenne Stitch-
konsept, endelig ordlyd, publiserbare skjermbilder, CTA og publiseringsvalg før
`TASK-030` bygger nettstedet.
