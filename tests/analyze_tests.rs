use tonemath::analyze::{numeral, parse_key};
use tonemath::chord::Chord;
use tonemath::interval::Interval;
use tonemath::note::Note;
use tonemath::transpose::transpose;

fn note(name: &str) -> Note {
    Note::parse(name).expect("test note must be valid")
}

fn chord(symbol: &str) -> Chord {
    Chord::parse(symbol).expect("test chord must be valid")
}

fn interval(name: &str) -> Interval {
    Interval::parse(name).expect("test interval must be valid")
}

#[test]
fn parses_major_and_natural_minor_keys_with_single_and_double_accidentals() {
    assert_eq!(parse_key("C"), Some((note("C"), false)));
    assert_eq!(parse_key("F#"), Some((note("F#"), false)));
    assert_eq!(parse_key("G##"), Some((note("G##"), false)));
    assert_eq!(parse_key("Bb"), Some((note("Bb"), false)));
    assert_eq!(parse_key("Cbb"), Some((note("Cbb"), false)));
    assert_eq!(parse_key("Am"), Some((note("A"), true)));
    assert_eq!(parse_key("Bbm"), Some((note("Bb"), true)));
    assert_eq!(parse_key("G##m"), Some((note("G##"), true)));
    assert_eq!(parse_key("Cbbm"), Some((note("Cbb"), true)));
}

#[test]
fn rejects_invalid_key_grammar_unicode_and_long_malformed_text() {
    assert_eq!(parse_key(""), None);
    assert_eq!(parse_key("H"), None);
    assert_eq!(parse_key("c"), None);
    assert_eq!(parse_key("C#b"), None);
    assert_eq!(parse_key("C###"), None);
    assert_eq!(parse_key("CmM"), None);
    assert_eq!(parse_key("Cmm"), None);
    assert_eq!(parse_key("Cmajor"), None);
    assert_eq!(parse_key(" C"), None);
    assert_eq!(parse_key("C "), None);
    assert_eq!(parse_key("C/E"), None);
    assert_eq!(parse_key("C♯"), None);
    let long_invalid = concat!(
        "Cxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    );
    assert_eq!(parse_key(long_invalid), None);
}

#[test]
fn numerals_match_all_supplied_major_key_examples() {
    let tonic = note("C");
    assert_eq!(numeral(&tonic, false, &chord("C")), "I");
    assert_eq!(numeral(&tonic, false, &chord("Am")), "vi");
    assert_eq!(numeral(&tonic, false, &chord("F")), "IV");
    assert_eq!(numeral(&tonic, false, &chord("G7")), "V7");
    assert_eq!(numeral(&tonic, false, &chord("Bb")), "bVII");
    assert_eq!(numeral(&tonic, false, &chord("F#dim7")), "#ivdim7");
    assert_eq!(numeral(&tonic, false, &chord("C/E")), "I/3");
}

#[test]
fn numerals_match_all_supplied_natural_minor_key_examples() {
    let tonic = note("A");
    assert_eq!(numeral(&tonic, true, &chord("Am")), "i");
    assert_eq!(numeral(&tonic, true, &chord("Dm")), "iv");
    assert_eq!(numeral(&tonic, true, &chord("E7")), "V7");
    assert_eq!(numeral(&tonic, true, &chord("G")), "VII");
    assert_eq!(numeral(&tonic, true, &chord("Bbmaj7/D")), "bIImaj7/4");
}

#[test]
fn applies_the_case_and_suffix_rule_to_all_nineteen_qualities() {
    let tonic = note("C");
    assert_eq!(numeral(&tonic, false, &chord("C")), "I");
    assert_eq!(numeral(&tonic, false, &chord("Cm")), "i");
    assert_eq!(numeral(&tonic, false, &chord("Cdim")), "idim");
    assert_eq!(numeral(&tonic, false, &chord("Caug")), "Iaug");
    assert_eq!(numeral(&tonic, false, &chord("Csus2")), "Isus2");
    assert_eq!(numeral(&tonic, false, &chord("Csus4")), "Isus4");
    assert_eq!(numeral(&tonic, false, &chord("C5")), "I5");
    assert_eq!(numeral(&tonic, false, &chord("C6")), "I6");
    assert_eq!(numeral(&tonic, false, &chord("Cm6")), "i6");
    assert_eq!(numeral(&tonic, false, &chord("C7")), "I7");
    assert_eq!(numeral(&tonic, false, &chord("Cmaj7")), "Imaj7");
    assert_eq!(numeral(&tonic, false, &chord("Cm7")), "i7");
    assert_eq!(numeral(&tonic, false, &chord("Cmmaj7")), "imaj7");
    assert_eq!(numeral(&tonic, false, &chord("Cm7b5")), "i7b5");
    assert_eq!(numeral(&tonic, false, &chord("Cdim7")), "idim7");
    assert_eq!(numeral(&tonic, false, &chord("Cadd9")), "Iadd9");
    assert_eq!(numeral(&tonic, false, &chord("C9")), "I9");
    assert_eq!(numeral(&tonic, false, &chord("Cmaj9")), "Imaj9");
    assert_eq!(numeral(&tonic, false, &chord("Cm9")), "i9");
}

#[test]
fn formats_double_root_alterations_and_enharmonic_letter_degrees() {
    let tonic = note("C");
    assert_eq!(numeral(&tonic, false, &chord("Dbb")), "bbII");
    assert_eq!(numeral(&tonic, false, &chord("G#")), "#V");
    assert_eq!(numeral(&tonic, false, &chord("Ab")), "bVI");
    assert_eq!(numeral(&tonic, false, &chord("B#")), "#VII");
}

#[test]
fn formats_the_positive_five_alteration_boundary_for_a_minor_key() {
    let tonic = note("Cbb");
    assert_eq!(numeral(&tonic, true, &chord("E##m")), "#####iii");
}

#[test]
fn normalizes_the_negative_six_boundary_for_public_note_fields() {
    let tonic = note("C");
    let altered_root = Chord {
        root: Note { letter: 0, acc: 6 },
        quality: String::new(),
        bass: None,
    };
    assert_eq!(numeral(&tonic, false, &altered_root), "bbbbbbI");
}

#[test]
fn formats_altered_numeric_bass_degrees_without_using_roman_numerals() {
    assert_eq!(numeral(&note("C"), false, &chord("C/B#")), "I/#7");
    assert_eq!(numeral(&note("C"), false, &chord("C/Dbb")), "I/bb2");
    assert_eq!(numeral(&note("A"), true, &chord("Am/C")), "i/3");
}

#[test]
fn numerals_do_not_require_other_chord_tones_to_be_spellable() {
    let unspellable = chord("G##maj7");
    assert!(unspellable.notes().is_err());
    assert_eq!(numeral(&note("C"), false, &unspellable), "##Vmaj7");
}

#[test]
fn transposes_the_supplied_major_second_progression_upward() {
    let iv = interval("M2");
    let c = transpose(&chord("C"), &iv, false).unwrap();
    let am = transpose(&chord("Am"), &iv, false).unwrap();
    let f = transpose(&chord("F"), &iv, false).unwrap();
    let g7 = transpose(&chord("G7"), &iv, false).unwrap();
    assert_eq!(
        vec![c, am, f, g7],
        vec![chord("D"), chord("Bm"), chord("G"), chord("A7")]
    );
}

#[test]
fn transposes_down_a_minor_third_including_the_inversion_bass() {
    assert_eq!(
        transpose(&chord("Cmaj7/E"), &interval("m3"), true)
            .unwrap()
            .name(),
        "Amaj7/C#"
    );
}

#[test]
fn perfect_unison_preserves_double_sharp_spelling_and_quality() {
    let original = chord("G##mmaj7");
    assert_eq!(transpose(&original, &interval("P1"), false), Ok(original));
}

#[test]
fn handles_compound_intervals_and_negative_semitone_interval_sizes() {
    assert_eq!(
        transpose(&chord("C/E"), &interval("M9"), false)
            .unwrap()
            .name(),
        "D/F#"
    );
    assert_eq!(
        transpose(&chord("C"), &interval("d1"), false)
            .unwrap()
            .name(),
        "Cb"
    );
    assert_eq!(
        transpose(&chord("C"), &interval("d1"), true)
            .unwrap()
            .name(),
        "C#"
    );
}

#[test]
fn transposes_double_accidentals_and_keeps_enharmonic_bass_spelling_directional() {
    assert_eq!(
        transpose(&chord("G##/Db"), &interval("M2"), false)
            .unwrap()
            .name(),
        "A##/Eb"
    );
    assert_eq!(
        transpose(&chord("C/B#"), &interval("M2"), false)
            .unwrap()
            .name(),
        "D/C##"
    );
}

#[test]
fn preserves_quality_and_bass_presence_without_revalidating_bass_membership() {
    let manually_constructed = Chord {
        root: note("G##"),
        quality: "mmaj7".to_owned(),
        bass: Some(note("Cbb")),
    };
    let expected = Chord {
        root: note("G##"),
        quality: "mmaj7".to_owned(),
        bass: Some(note("Cbb")),
    };
    assert_eq!(
        transpose(&manually_constructed, &interval("P1"), false),
        Ok(expected)
    );
}

#[test]
fn returns_the_exact_root_spelling_errors_for_both_directions() {
    assert_eq!(
        transpose(&chord("G##"), &interval("M7"), false)
            .err()
            .map(|error| error.0),
        Some("cannot spell G## up by M7".to_owned())
    );
    assert_eq!(
        transpose(&chord("Cbb"), &interval("M3"), true)
            .err()
            .map(|error| error.0),
        Some("cannot spell Cbb down by M3".to_owned())
    );
}

#[test]
fn returns_the_exact_bass_spelling_error_after_a_successful_root() {
    let invalid_bass_member = Chord {
        root: note("C"),
        quality: String::new(),
        bass: Some(note("G##")),
    };
    assert_eq!(
        transpose(&invalid_bass_member, &interval("M7"), false)
            .err()
            .map(|error| error.0),
        Some("cannot spell G## up by M7".to_owned())
    );
}

#[test]
fn a_chord_with_two_failing_notes_reports_the_root_failure_first() {
    let root_and_bass_fail = Chord {
        root: note("D##"),
        quality: String::new(),
        bass: Some(note("G##")),
    };
    assert_eq!(
        transpose(&root_and_bass_fail, &interval("M7"), false)
            .err()
            .map(|error| error.0),
        Some("cannot spell D## up by M7".to_owned())
    );
}

#[test]
fn returns_the_exact_error_for_a_double_sharp_root_with_its_own_bass() {
    assert_eq!(
        transpose(&chord("G##/G##"), &interval("M7"), false)
            .err()
            .map(|error| error.0),
        Some("cannot spell G## up by M7".to_owned())
    );
}

#[test]
fn malformed_public_fields_and_extreme_interval_values_do_not_panic() {
    let malformed_quality = Chord {
        root: Note {
            letter: u8::MAX,
            acc: i8::MAX,
        },
        quality: "mystery".to_owned(),
        bass: None,
    };
    assert_eq!(
        numeral(&note("C"), false, &malformed_quality),
        "##IVmystery"
    );

    let extreme_interval = Interval {
        steps: u32::MAX,
        semis: i32::MIN,
        label: "extreme".to_owned(),
    };
    assert_eq!(
        transpose(&chord("C"), &extreme_interval, false)
            .unwrap()
            .name(),
        "Fb"
    );

    let malformed_note_chord = Chord {
        root: Note {
            letter: u8::MAX,
            acc: i8::MIN,
        },
        quality: "unknown".to_owned(),
        bass: Some(Note {
            letter: u8::MAX,
            acc: i8::MAX,
        }),
    };
    assert!(transpose(&malformed_note_chord, &interval("P1"), false).is_err());
}
