use tonemath::note::Note;
use tonemath::scale::{SCALES, spell};

fn assert_scale_names(root_text: &str, scale_name: &str, expected: &[&str]) {
    let root = Note::parse(root_text).unwrap();
    let notes = spell(&root, scale_name).unwrap();
    let names = notes.iter().map(Note::name).collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(names, expected);
}

fn expected_scale_names() -> Vec<String> {
    vec![
        "major",
        "minor",
        "harmonic-minor",
        "melodic-minor",
        "dorian",
        "phrygian",
        "lydian",
        "mixolydian",
        "locrian",
        "major-pentatonic",
        "minor-pentatonic",
        "blues",
        "whole-tone",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn scales_table_has_every_name_degree_string_and_order() {
    assert_eq!(
        SCALES,
        [
            ("major", "1 2 3 4 5 6 7"),
            ("minor", "1 2 b3 4 5 b6 b7"),
            ("harmonic-minor", "1 2 b3 4 5 b6 7"),
            ("melodic-minor", "1 2 b3 4 5 6 7"),
            ("dorian", "1 2 b3 4 5 6 b7"),
            ("phrygian", "1 b2 b3 4 5 b6 b7"),
            ("lydian", "1 2 3 #4 5 6 7"),
            ("mixolydian", "1 2 3 4 5 6 b7"),
            ("locrian", "1 b2 b3 4 b5 b6 b7"),
            ("major-pentatonic", "1 2 3 5 6"),
            ("minor-pentatonic", "1 b3 4 5 b7"),
            ("blues", "1 b3 4 b5 5 b7"),
            ("whole-tone", "1 2 3 #4 #5 b7"),
        ]
    );
}

#[test]
fn major_scale_spells_every_degree() {
    assert_scale_names("C", "major", &["C", "D", "E", "F", "G", "A", "B"]);
}

#[test]
fn minor_scale_spells_every_degree() {
    assert_scale_names("C", "minor", &["C", "D", "Eb", "F", "G", "Ab", "Bb"]);
}

#[test]
fn harmonic_minor_scale_spells_every_degree() {
    assert_scale_names(
        "C",
        "harmonic-minor",
        &["C", "D", "Eb", "F", "G", "Ab", "B"],
    );
}

#[test]
fn melodic_minor_scale_spells_every_degree() {
    assert_scale_names("C", "melodic-minor", &["C", "D", "Eb", "F", "G", "A", "B"]);
}

#[test]
fn dorian_scale_spells_every_degree() {
    assert_scale_names("C", "dorian", &["C", "D", "Eb", "F", "G", "A", "Bb"]);
}

#[test]
fn phrygian_scale_spells_every_degree() {
    assert_scale_names("C", "phrygian", &["C", "Db", "Eb", "F", "G", "Ab", "Bb"]);
}

#[test]
fn lydian_scale_spells_every_degree() {
    assert_scale_names("C", "lydian", &["C", "D", "E", "F#", "G", "A", "B"]);
}

#[test]
fn mixolydian_scale_spells_every_degree() {
    assert_scale_names("C", "mixolydian", &["C", "D", "E", "F", "G", "A", "Bb"]);
}

#[test]
fn locrian_scale_spells_every_degree() {
    assert_scale_names("C", "locrian", &["C", "Db", "Eb", "F", "Gb", "Ab", "Bb"]);
}

#[test]
fn major_pentatonic_scale_spells_every_degree() {
    assert_scale_names("C", "major-pentatonic", &["C", "D", "E", "G", "A"]);
}

#[test]
fn minor_pentatonic_scale_spells_every_degree() {
    assert_scale_names("C", "minor-pentatonic", &["C", "Eb", "F", "G", "Bb"]);
}

#[test]
fn blues_scale_spells_every_degree() {
    assert_scale_names("C", "blues", &["C", "Eb", "F", "Gb", "G", "Bb"]);
}

#[test]
fn whole_tone_scale_spells_every_degree() {
    assert_scale_names("C", "whole-tone", &["C", "D", "E", "F#", "G#", "Bb"]);
}

#[test]
fn major_scale_spells_double_sharp_root_without_changing_letter_steps() {
    assert_scale_names(
        "C##",
        "major",
        &["C##", "D##", "E##", "F##", "G##", "A##", "B##"],
    );
}

#[test]
fn major_scale_stops_at_the_first_unspellable_degree() {
    let root = Note::parse("G##").unwrap();
    assert_eq!(
        spell(&root, "major").err().map(|error| error.0),
        Some("cannot spell G## up by 7".to_owned())
    );
}

#[test]
fn unknown_scale_sanitizes_unicode_and_control_text_and_lists_names_in_order() {
    let root = Note::parse("C").unwrap();
    assert_eq!(
        spell(&root, "unknown\u{00e9}\u{0000}")
            .err()
            .map(|error| error.0),
        Some(
            "unknown scale \"unknown??\" (expected major, minor, harmonic-minor, melodic-minor, dorian, phrygian, lydian, mixolydian, locrian, major-pentatonic, minor-pentatonic, blues or whole-tone)"
                .to_owned()
        )
    );
}

#[test]
fn empty_scale_name_reports_the_complete_expected_name_list() {
    let root = Note::parse("C").unwrap();
    assert_eq!(
        spell(&root, "").err().map(|error| error.0),
        Some(
            "unknown scale \"\" (expected major, minor, harmonic-minor, melodic-minor, dorian, phrygian, lydian, mixolydian, locrian, major-pentatonic, minor-pentatonic, blues or whole-tone)"
                .to_owned()
        )
    );
}

#[test]
fn malformed_public_root_fields_return_an_error_without_panicking() {
    let invalid_letter = Note {
        letter: u8::MAX,
        acc: i8::MIN,
    };
    assert!(spell(&invalid_letter, "major").is_err());

    let invalid_accidental = Note {
        letter: 0,
        acc: i8::MIN,
    };
    assert!(spell(&invalid_accidental, "major").is_err());
}

#[test]
fn scale_expected_names_match_the_literal_table_names() {
    let names = SCALES
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(names, expected_scale_names());
}
