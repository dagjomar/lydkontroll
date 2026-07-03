use std::{fs, io::Write, path::Path};

use lydkontroll_lib::{
    adapters::persistence::{JsonLibraryRepository, PersistenceError},
    domain::{AudioFormat, Cue, CueLibrary, CueMode, Scene, LIBRARY_SCHEMA_VERSION},
    ports::{AudioImporter, LibraryRepository},
};
use tempfile::TempDir;

#[test]
fn imports_wav_and_round_trips_scenes_without_source_paths() {
    let temp = TempDir::new().expect("temp directory");
    let source_dir = temp.path().join("operator-files");
    fs::create_dir(&source_dir).expect("source directory");
    let source = source_dir.join("first dance.wav");
    write_test_wav(&source);
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));

    let audio = repository.import(&source).expect("valid WAV should import");
    assert_eq!(audio.format, AudioFormat::Wav);
    assert!(repository
        .paths()
        .audio_dir()
        .join(&audio.file_name)
        .is_file());
    assert!(
        fs::read_dir(repository.paths().staging_dir())
            .expect("staging directory")
            .next()
            .is_none(),
        "successful import should leave no staging files"
    );

    let library = CueLibrary {
        schema_version: LIBRARY_SCHEMA_VERSION,
        event_title: "Første dans".to_owned(),
        audio_files: vec![audio.clone()],
        scenes: vec![Scene {
            id: "scene-1".to_owned(),
            name: "First dance".to_owned(),
            cues: vec![Cue {
                id: "cue-1".to_owned(),
                name: "Start music".to_owned(),
                color: "#6d28d9".to_owned(),
                audio_file_id: audio.id,
                volume: 0.8,
                mode: CueMode::Exclusive,
                fade_ms: 1500,
            }],
        }],
    };

    repository.save(&library).expect("library should save");
    let reopened = JsonLibraryRepository::new(repository.paths().root());
    assert_eq!(reopened.load().expect("library should reopen"), library);

    let json = fs::read_to_string(repository.paths().library()).expect("saved JSON");
    assert!(json.contains("\"schemaVersion\": 1"));
    assert!(!json.contains(source_dir.to_string_lossy().as_ref()));
    assert!(!json.contains(source.to_string_lossy().as_ref()));
}

#[test]
fn imports_mp3_after_decoding_audio_frames() {
    let temp = TempDir::new().expect("temp directory");
    let source = temp.path().join("entrance.mp3");
    fs::write(&source, decode_hex(TEST_MP3_HEX)).expect("MP3 fixture");
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));

    let audio = repository.import(&source).expect("valid MP3 should import");

    assert_eq!(audio.format, AudioFormat::Mp3);
    assert_eq!(audio.original_name, "entrance.mp3");
    assert!(repository
        .paths()
        .audio_dir()
        .join(audio.file_name)
        .is_file());
}

#[test]
fn save_rotates_valid_primary_and_recovery_is_explicit() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());
    let first = CueLibrary::default();
    repository.save(&first).expect("first save");

    let second = CueLibrary {
        scenes: vec![Scene {
            id: "scene-2".to_owned(),
            name: "Speeches".to_owned(),
            cues: Vec::new(),
        }],
        ..CueLibrary::default()
    };
    repository.save(&second).expect("second save");
    assert_eq!(repository.load_backup().expect("valid backup"), first);

    fs::write(repository.paths().library(), b"{ interrupted")
        .expect("simulate interrupted primary write");
    assert!(matches!(
        repository.load(),
        Err(PersistenceError::CorruptLibrary {
            backup_available: true,
            ..
        })
    ));

    let recovered = repository
        .recover_backup()
        .expect("explicit backup recovery");
    assert_eq!(recovered, first);
    assert_eq!(repository.load().expect("recovered primary"), first);
}

#[test]
fn interrupted_rotation_recovers_when_only_backup_and_temporary_file_remain() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());
    repository
        .save(&CueLibrary::default())
        .expect("initial save");
    repository
        .save(&CueLibrary {
            scenes: vec![Scene {
                id: "scene-current".to_owned(),
                name: "Current".to_owned(),
                cues: Vec::new(),
            }],
            ..CueLibrary::default()
        })
        .expect("second save creates backup");

    fs::remove_file(repository.paths().library()).expect("simulate rotated primary");
    fs::write(temp.path().join("library.json.tmp"), b"{ partial")
        .expect("simulate interrupted temporary write");

    assert!(matches!(
        repository.load(),
        Err(PersistenceError::LibraryMissing {
            backup_available: true
        })
    ));
    let recovered = repository.recover_backup().expect("recover valid backup");
    assert_eq!(recovered, CueLibrary::default());
    assert_eq!(repository.load().expect("primary restored"), recovered);
}

#[test]
fn corrupt_and_future_schemas_are_typed_errors() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());

    fs::write(repository.paths().library(), b"not json").expect("corrupt primary");
    assert!(matches!(
        repository.load(),
        Err(PersistenceError::CorruptLibrary {
            backup_available: false,
            ..
        })
    ));

    fs::write(
        repository.paths().library(),
        br#"{"schemaVersion":2,"scenes":[],"audioFiles":[]}"#,
    )
    .expect("future primary");
    assert!(matches!(
        repository.load(),
        Err(PersistenceError::UnsupportedSchemaVersion {
            found: 2,
            supported: 1,
            ..
        })
    ));
}

#[test]
fn schema_one_without_event_title_loads_with_reusable_default() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());
    fs::write(
        repository.paths().library(),
        br#"{"schemaVersion":1,"scenes":[],"audioFiles":[]}"#,
    )
    .expect("legacy schema-one library");

    let loaded = repository.load().expect("legacy library should load");

    assert_eq!(loaded.event_title, "Mitt arrangement");
    assert!(loaded.scenes.is_empty());
    assert!(loaded.audio_files.is_empty());
}

#[test]
fn missing_managed_files_are_reported_without_losing_metadata() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());
    fs::create_dir_all(repository.paths().audio_dir()).expect("audio directory");
    fs::write(
        repository.paths().library(),
        br#"{
          "schemaVersion": 1,
          "scenes": [],
          "audioFiles": [{
            "id": "audio-1",
            "fileName": "audio-1.wav",
            "originalName": "missing.wav",
            "format": "wav",
            "byteLength": 44
          }]
        }"#,
    )
    .expect("library JSON");

    assert!(matches!(
        repository.load(),
        Err(PersistenceError::MissingManagedFile { file_name })
            if file_name == "audio-1.wav"
    ));
    let metadata = repository
        .load_metadata()
        .expect("valid metadata remains available for recovery");
    assert_eq!(metadata.audio_files[0].original_name, "missing.wav");
}

#[test]
fn cue_references_and_managed_filenames_cannot_escape_storage() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path());
    fs::create_dir_all(repository.paths().audio_dir()).expect("audio directory");
    fs::write(
        repository.paths().library(),
        br#"{
          "schemaVersion": 1,
          "scenes": [],
          "audioFiles": [{
            "id": "audio-1",
            "fileName": "../outside.wav",
            "originalName": "outside.wav",
            "format": "wav",
            "byteLength": 44
          }]
        }"#,
    )
    .expect("library JSON");
    assert!(matches!(
        repository.load(),
        Err(PersistenceError::InvalidManagedFileName { .. })
    ));

    fs::write(
        repository.paths().library(),
        br##"{
          "schemaVersion": 1,
          "scenes": [{
            "id": "scene-1",
            "name": "Scene",
            "cues": [{
              "id": "cue-1",
              "name": "Missing",
              "color": "#000000",
              "audioFileId": "unknown",
              "volume": 1.0,
              "mode": "overlap",
              "fadeMs": 0
            }]
          }],
          "audioFiles": []
        }"##,
    )
    .expect("library JSON");
    assert!(matches!(
        repository.load(),
        Err(PersistenceError::MissingAudioReference {
            cue_id,
            audio_file_id
        }) if cue_id == "cue-1" && audio_file_id == "unknown"
    ));
}

#[test]
fn invalid_audio_is_recoverable_and_staging_is_cleaned() {
    let temp = TempDir::new().expect("temp directory");
    let source = temp.path().join("broken.mp3");
    fs::write(&source, b"this is not an mp3").expect("invalid source");
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));

    assert!(matches!(
        repository.import(&source),
        Err(PersistenceError::InvalidAudio { .. })
    ));
    assert!(
        fs::read_dir(repository.paths().staging_dir())
            .expect("staging directory")
            .next()
            .is_none(),
        "failed import should remove staged files"
    );
}

#[test]
fn missing_sources_and_unsupported_extensions_are_typed() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));
    let missing = temp.path().join("missing.wav");
    assert!(matches!(
        repository.import(&missing),
        Err(PersistenceError::MissingSourceFile { .. })
    ));

    let text = temp.path().join("notes.txt");
    fs::write(&text, b"not audio").expect("text source");
    assert!(matches!(
        repository.import(&text),
        Err(PersistenceError::UnsupportedAudioFormat { extension })
            if extension == "txt"
    ));
}

#[test]
fn deletes_only_unreferenced_managed_audio_and_persists_the_result() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));
    let first_source = temp.path().join("version.wav");
    let second_source = temp.path().join("other").join("version.wav");
    fs::create_dir_all(second_source.parent().expect("source parent")).expect("source directory");
    write_test_wav(&first_source);
    write_test_wav(&second_source);
    let referenced = repository.import(&first_source).expect("first import");
    let obsolete = repository.import(&second_source).expect("second import");
    let library = CueLibrary {
        schema_version: LIBRARY_SCHEMA_VERSION,
        event_title: "Fest".to_owned(),
        scenes: vec![Scene {
            id: "scene-1".to_owned(),
            name: "Fest".to_owned(),
            cues: vec![Cue {
                id: "cue-1".to_owned(),
                name: "Introduksjon".to_owned(),
                color: "#ffffff".to_owned(),
                audio_file_id: referenced.id.clone(),
                volume: 1.0,
                mode: CueMode::Overlap,
                fade_ms: 500,
            }],
        }],
        audio_files: vec![referenced.clone(), obsolete.clone()],
    };
    repository.save(&library).expect("library save");

    let blocked = repository.delete_managed_audio(&library, &referenced.id);
    assert!(matches!(
        blocked,
        Err(PersistenceError::ManagedAudioFileInUse {
            cue_names,
            ..
        }) if cue_names == "Introduksjon"
    ));
    assert!(repository
        .paths()
        .audio_dir()
        .join(&referenced.file_name)
        .is_file());

    let updated = repository
        .delete_managed_audio(&library, &obsolete.id)
        .expect("unreferenced file deletes");
    assert_eq!(updated.audio_files, vec![referenced.clone()]);
    assert!(!repository
        .paths()
        .audio_dir()
        .join(&obsolete.file_name)
        .exists());
    assert_eq!(repository.load().expect("reopened library"), updated);
}

#[test]
fn failed_delete_save_restores_the_managed_file_and_metadata() {
    let temp = TempDir::new().expect("temp directory");
    let repository = JsonLibraryRepository::new(temp.path().join("app-data"));
    let source = temp.path().join("obsolete.wav");
    write_test_wav(&source);
    let audio = repository.import(&source).expect("audio import");
    let library = CueLibrary {
        audio_files: vec![audio.clone()],
        ..CueLibrary::default()
    };
    repository.save(&library).expect("library save");
    fs::create_dir(repository.paths().root().join("library.json.tmp"))
        .expect("block temporary library file");

    assert!(repository
        .delete_managed_audio(&library, &audio.id)
        .is_err());
    assert!(repository
        .paths()
        .audio_dir()
        .join(&audio.file_name)
        .is_file());
    assert_eq!(repository.load().expect("unchanged library"), library);
}

fn write_test_wav(path: &Path) {
    let samples = [0_i16, 1000, -1000, 0];
    let data_size = (samples.len() * size_of::<i16>()) as u32;
    let mut file = fs::File::create(path).expect("WAV fixture");
    file.write_all(b"RIFF").expect("RIFF");
    file.write_all(&(36 + data_size).to_le_bytes())
        .expect("RIFF size");
    file.write_all(b"WAVEfmt ").expect("WAVE fmt");
    file.write_all(&16_u32.to_le_bytes()).expect("fmt size");
    file.write_all(&1_u16.to_le_bytes()).expect("PCM");
    file.write_all(&1_u16.to_le_bytes()).expect("mono");
    file.write_all(&8_000_u32.to_le_bytes())
        .expect("sample rate");
    file.write_all(&16_000_u32.to_le_bytes())
        .expect("byte rate");
    file.write_all(&2_u16.to_le_bytes()).expect("block align");
    file.write_all(&16_u16.to_le_bytes())
        .expect("bits per sample");
    file.write_all(b"data").expect("data marker");
    file.write_all(&data_size.to_le_bytes()).expect("data size");
    for sample in samples {
        file.write_all(&sample.to_le_bytes()).expect("sample");
    }
    file.sync_all().expect("flush WAV fixture");
}

fn decode_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("ASCII hex");
            u8::from_str_radix(text, 16).expect("valid hex")
        })
        .collect()
}

// Three short MPEG-2.5 Layer III silence frames generated for this test.
const TEST_MP3_HEX: &str = "ffe338c40000000348000000004c414d45332e313030555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555554c414d45332e31303055555555555555555555555555555555555555555555555555555555555555555555555555555555555555ffe338c4340000034800000000555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555554c414d45332e31303055555555555555555555555555555555555555555555555555555555555555555555555555555555555555ffe338c43400000348000000005555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555555";
