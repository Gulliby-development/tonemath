use tonemath::chord::{Chord, QUALITIES};
use tonemath::note::Note;

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn assert_quality(symbol: &str, degrees: &[&str], expected_notes: &[&str]) {
    let chord_text = format!("C{symbol}");
    let chord = Chord::parse(&chord_text).unwrap();

    assert_eq!(chord.name(), chord_text);
    assert_eq!(chord.quality, symbol);
    assert_eq!(chord.root, Note::parse("C").unwrap());
    assert_eq!(chord.bass, None);
    assert_eq!(chord.tokens(), degrees.to_vec());

    let names = chord
        .notes()
        .unwrap()
        .iter()
        .map(Note::name)
        .collect::<Vec<_>>();
    assert_eq!(names, strings(expected_notes));
}

#[test]
fn qualities_table_has_every_literal_symbol_degree_string_and_order() {
    assert_eq!(
        QUALITIES,
        [
            ("", "1 3 5"),
            ("m", "1 b3 5"),
            ("dim", "1 b3 b5"),
            ("aug", "1 3 #5"),
            ("sus2", "1 2 5"),
            ("sus4", "1 4 5"),
            ("5", "1 5"),
            ("6", "1 3 5 6"),
            ("m6", "1 b3 5 6"),
            ("7", "1 3 5 b7"),
            ("maj7", "1 3 5 7"),
            ("m7", "1 b3 5 b7"),
            ("mmaj7", "1 b3 5 7"),
            ("m7b5", "1 b3 b5 b7"),
            ("dim7", "1 b3 b5 bb7"),
            ("add9", "1 3 5 9"),
            ("9", "1 3 5 b7 9"),
            ("maj9", "1 3 5 7 9"),
            ("m9", "1 b3 5 b7 9"),
        ]
    );
}

#[test]
fn major_quality_has_literal_tokens_and_notes() {
    assert_quality("", &["1", "3", "5"], &["C", "E", "G"]);
}

#[test]
fn minor_quality_has_literal_tokens_and_notes() {
    assert_quality("m", &["1", "b3", "5"], &["C", "Eb", "G"]);
}

#[test]
fn diminished_quality_has_literal_tokens_and_notes() {
    assert_quality("dim", &["1", "b3", "b5"], &["C", "Eb", "Gb"]);
}

#[test]
fn augmented_quality_has_literal_tokens_and_notes() {
    assert_quality("aug", &["1", "3", "#5"], &["C", "E", "G#"]);
}

#[test]
fn suspended_second_quality_has_literal_tokens_and_notes() {
    assert_quality("sus2", &["1", "2", "5"], &["C", "D", "G"]);
}

#[test]
fn suspended_fourth_quality_has_literal_tokens_and_notes() {
    assert_quality("sus4", &["1", "4", "5"], &["C", "F", "G"]);
}

#[test]
fn power_chord_quality_has_literal_tokens_and_notes() {
    assert_quality("5", &["1", "5"], &["C", "G"]);
}

#[test]
fn sixth_quality_has_literal_tokens_and_notes() {
    assert_quality("6", &["1", "3", "5", "6"], &["C", "E", "G", "A"]);
}

#[test]
fn minor_sixth_quality_has_literal_tokens_and_notes() {
    assert_quality("m6", &["1", "b3", "5", "6"], &["C", "Eb", "G", "A"]);
}

#[test]
fn dominant_seventh_quality_has_literal_tokens_and_notes() {
    assert_quality("7", &["1", "3", "5", "b7"], &["C", "E", "G", "Bb"]);
}

#[test]
fn major_seventh_quality_has_literal_tokens_and_notes() {
    assert_quality("maj7", &["1", "3", "5", "7"], &["C", "E", "G", "B"]);
}

#[test]
fn minor_seventh_quality_has_literal_tokens_and_notes() {
    assert_quality("m7", &["1", "b3", "5", "b7"], &["C", "Eb", "G", "Bb"]);
}

#[test]
fn minor_major_seventh_quality_has_literal_tokens_and_notes() {
    assert_quality("mmaj7", &["1", "b3", "5", "7"], &["C", "Eb", "G", "B"]);
}

#[test]
fn half_diminished_quality_has_literal_tokens_and_notes() {
    assert_quality("m7b5", &["1", "b3", "b5", "b7"], &["C", "Eb", "Gb", "Bb"]);
}

#[test]
fn diminished_seventh_quality_has_literal_tokens_and_double_flat_note() {
    assert_quality("dim7", &["1", "b3", "b5", "bb7"], &["C", "Eb", "Gb", "Bbb"]);
}

#[test]
fn added_ninth_quality_has_literal_tokens_and_notes() {
    assert_quality("add9", &["1", "3", "5", "9"], &["C", "E", "G", "D"]);
}

#[test]
fn dominant_ninth_quality_has_literal_tokens_and_notes() {
    assert_quality(
        "9",
        &["1", "3", "5", "b7", "9"],
        &["C", "E", "G", "Bb", "D"],
    );
}

#[test]
fn major_ninth_quality_has_literal_tokens_and_notes() {
    assert_quality(
        "maj9",
        &["1", "3", "5", "7", "9"],
        &["C", "E", "G", "B", "D"],
    );
}

#[test]
fn minor_ninth_quality_has_literal_tokens_and_notes() {
    assert_quality(
        "m9",
        &["1", "b3", "5", "b7", "9"],
        &["C", "Eb", "G", "Bb", "D"],
    );
}

#[test]
fn greedy_roots_and_quality_symbols_preserve_their_spelling() {
    let bb = Chord::parse("Bb").unwrap();
    assert_eq!(bb.name(), "Bb");
    assert_eq!(bb.root, Note::parse("Bb").unwrap());
    assert_eq!(bb.quality, "");
    assert_eq!(
        bb.notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["Bb", "D", "F"])
    );

    let bb_minor_seventh = Chord::parse("Bbm7").unwrap();
    assert_eq!(bb_minor_seventh.name(), "Bbm7");
    assert_eq!(bb_minor_seventh.root, Note::parse("Bb").unwrap());
    assert_eq!(bb_minor_seventh.quality, "m7");
    assert_eq!(
        bb_minor_seventh
            .notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["Bb", "Db", "F", "Ab"])
    );

    let cb_power = Chord::parse("Cb5").unwrap();
    assert_eq!(cb_power.name(), "Cb5");
    assert_eq!(cb_power.root, Note::parse("Cb").unwrap());
    assert_eq!(cb_power.quality, "5");
    assert_eq!(
        cb_power
            .notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["Cb", "Gb"])
    );

    let double_sharp_root = Chord::parse("C##").unwrap();
    assert_eq!(double_sharp_root.name(), "C##");
    assert_eq!(double_sharp_root.root, Note::parse("C##").unwrap());
    assert_eq!(
        double_sharp_root
            .notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["C##", "E##", "G##"])
    );
}

#[test]
fn enharmonic_basses_are_accepted_and_keep_their_original_spelling() {
    let flat_bass = Chord::parse("C/Fb").unwrap();
    assert_eq!(flat_bass.name(), "C/Fb");
    assert_eq!(flat_bass.bass, Some(Note::parse("Fb").unwrap()));

    let sharp_bass = Chord::parse("C/B#").unwrap();
    assert_eq!(sharp_bass.name(), "C/B#");
    assert_eq!(sharp_bass.bass, Some(Note::parse("B#").unwrap()));
}

#[test]
fn double_sharp_root_with_enharmonic_bass_spells_the_chord() {
    let chord = Chord::parse("G##/Db").unwrap();
    assert_eq!(chord.name(), "G##/Db");
    assert_eq!(chord.root, Note::parse("G##").unwrap());
    assert_eq!(chord.bass, Some(Note::parse("Db").unwrap()));

    let notes = chord.notes().unwrap();
    assert_eq!(
        notes.iter().map(Note::name).collect::<Vec<_>>(),
        strings(&["G##", "B##", "D##"])
    );
}

#[test]
fn nonmember_bass_is_reported_only_after_valid_chord_grammar() {
    assert_eq!(
        Chord::parse("C/D").err().map(|error| error.0),
        Some("invalid chord \"C/D\" (the bass is not a chord note)".to_owned())
    );
    assert_eq!(
        Chord::parse("G##/D").err().map(|error| error.0),
        Some("invalid chord \"G##/D\" (the bass is not a chord note)".to_owned())
    );
    assert_eq!(
        Chord::parse("G##/Hx").err().map(|error| error.0),
        Some("invalid chord \"G##/Hx\"".to_owned())
    );
}

#[test]
fn malformed_chord_grammar_returns_the_exact_complete_input_error() {
    assert_eq!(
        Chord::parse("").err().map(|error| error.0),
        Some("invalid chord \"\"".to_owned())
    );
    assert_eq!(
        Chord::parse("H").err().map(|error| error.0),
        Some("invalid chord \"H\"".to_owned())
    );
    assert_eq!(
        Chord::parse("c").err().map(|error| error.0),
        Some("invalid chord \"c\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C###").err().map(|error| error.0),
        Some("invalid chord \"C###\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C#b").err().map(|error| error.0),
        Some("invalid chord \"C#b\"".to_owned())
    );
    assert_eq!(
        Chord::parse("Cfoo").err().map(|error| error.0),
        Some("invalid chord \"Cfoo\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C/").err().map(|error| error.0),
        Some("invalid chord \"C/\"".to_owned())
    );
    assert_eq!(
        Chord::parse("/C").err().map(|error| error.0),
        Some("invalid chord \"/C\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C/E/G").err().map(|error| error.0),
        Some("invalid chord \"C/E/G\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C/Efoo").err().map(|error| error.0),
        Some("invalid chord \"C/Efoo\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C major").err().map(|error| error.0),
        Some("invalid chord \"C?major\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C/Dx").err().map(|error| error.0),
        Some("invalid chord \"C/Dx\"".to_owned())
    );
}

#[test]
fn malformed_unicode_and_control_text_is_sanitized_in_the_full_error() {
    assert_eq!(
        Chord::parse("C♯").err().map(|error| error.0),
        Some("invalid chord \"C?\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C/é").err().map(|error| error.0),
        Some("invalid chord \"C/?\"".to_owned())
    );
    assert_eq!(
        Chord::parse("C\0").err().map(|error| error.0),
        Some("invalid chord \"C?\"".to_owned())
    );
}

#[test]
fn invalid_chord_text_has_no_small_arbitrary_length_limit() {
    let text = concat!(
        "C",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    );
    assert!(Chord::parse(text).is_err());
}

#[test]
fn c_major_seventh_keeps_tokens_notes_and_name_unrotated_with_bass() {
    let chord = Chord::parse("Cmaj7/E").unwrap();
    assert_eq!(chord.name(), "Cmaj7/E");
    assert_eq!(chord.tokens(), ["1", "3", "5", "7"]);
    assert_eq!(
        chord
            .notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["C", "E", "G", "B"])
    );
}

#[test]
fn notes_stop_at_the_first_unspellable_degree_without_returning_partial_notes() {
    let chord = Chord::parse("G##maj7").unwrap();
    assert_eq!(
        chord.notes().err().map(|error| error.0),
        Some("cannot spell G## up by 7".to_owned())
    );
}

#[test]
fn unspellable_double_flat_diminished_seventh_stops_at_its_first_degree() {
    let chord = Chord::parse("Cbbdim7").unwrap();
    assert_eq!(
        chord.notes().err().map(|error| error.0),
        Some("cannot spell Cbb up by b3".to_owned())
    );
}

#[test]
fn unknown_manually_constructed_quality_has_empty_tokens_and_exact_error() {
    let chord = Chord {
        root: Note::parse("C").unwrap(),
        quality: "unknowné".to_owned(),
        bass: None,
    };
    assert!(chord.tokens().is_empty());
    assert_eq!(
        chord.notes().err().map(|error| error.0),
        Some("unknown chord quality \"unknown?\"".to_owned())
    );
}

#[test]
fn malformed_public_chord_fields_do_not_panic() {
    let invalid_letter = Chord {
        root: Note {
            letter: u8::MAX,
            acc: i8::MIN,
        },
        quality: String::new(),
        bass: None,
    };
    assert!(invalid_letter.notes().is_err());
    let _ = invalid_letter.name();

    let invalid_bass = Chord {
        root: Note::parse("C").unwrap(),
        quality: String::new(),
        bass: Some(Note {
            letter: u8::MAX,
            acc: i8::MIN,
        }),
    };
    let _ = invalid_bass.name();
    assert_eq!(
        invalid_bass
            .notes()
            .unwrap()
            .iter()
            .map(Note::name)
            .collect::<Vec<_>>(),
        strings(&["C", "E", "G"])
    );
}
