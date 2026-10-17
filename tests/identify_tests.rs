use tonemath::identify::identify;
use tonemath::keys::{Key, matching};
use tonemath::note::Note;

fn notes(names: &[&str]) -> Vec<Note> {
    names
        .iter()
        .map(|name| Note::parse(name).expect("test note must be valid"))
        .collect()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn key(tonic: &str, minor: bool, accidentals: i32) -> Key {
    Key {
        tonic: Note::parse(tonic).expect("test tonic must be valid"),
        minor,
        accidentals,
    }
}

#[test]
fn identifies_c_major_sixth_and_inverted_a_minor_seventh() {
    assert_eq!(
        identify(&notes(&["C", "E", "G", "A"])),
        strings(&["C6", "Am7/C"])
    );
}

#[test]
fn identifies_all_four_diminished_seventh_ties_in_input_root_order() {
    assert_eq!(
        identify(&notes(&["C", "Eb", "Gb", "A"])),
        strings(&["Cdim7", "Ebdim7/C", "Gbdim7/C", "Adim7/C"])
    );
}

#[test]
fn diminished_seventh_tie_prefix_preserves_candidate_order() {
    let candidates = identify(&notes(&["C", "Eb", "Gb", "A"]));
    assert_eq!(candidates[..2], strings(&["Cdim7", "Ebdim7/C"]));
}

#[test]
fn returns_no_chord_for_an_unmatched_pitch_class_set() {
    assert_eq!(identify(&notes(&["C", "D"])), Vec::<String>::new());
}

#[test]
fn identifies_a_chord_in_inversion_using_first_input_spelling() {
    assert_eq!(identify(&notes(&["E", "G", "C"])), strings(&["C/E"]));
}

#[test]
fn duplicate_and_enharmonic_notes_do_not_add_roots_or_chord_tones() {
    assert_eq!(
        identify(&notes(&["C", "E", "G", "C", "B#"])),
        strings(&["C"])
    );
}

#[test]
fn first_enharmonic_spelling_is_preserved_as_the_root() {
    assert_eq!(identify(&notes(&["B#", "E", "G", "C"])), strings(&["B#"]));
}

#[test]
fn identifies_every_pitch_class_match_without_spelling_chord_tones() {
    assert_eq!(
        identify(&notes(&["G##", "C#", "E#"])),
        strings(&["G##aug", "C#aug/G##", "E#aug/G##"])
    );
}

#[test]
fn suspended_chord_matches_follow_quality_table_order() {
    assert_eq!(
        identify(&notes(&["C", "D", "G"])),
        strings(&["Csus2", "Gsus4/C"])
    );
}

#[test]
fn empty_identification_input_has_no_matches() {
    assert_eq!(identify(&[]), Vec::<String>::new());
}

#[test]
fn matches_the_four_flat_key_examples_in_catalogue_order() {
    assert_eq!(
        matching(&notes(&["Bb", "D", "F", "A"]))
            .iter()
            .map(Key::line)
            .collect::<Vec<_>>(),
        strings(&[
            "F major (1 flat)",
            "D minor (1 flat)",
            "Bb major (2 flats)",
            "G minor (2 flats)",
        ])
    );
}

#[test]
fn returns_no_keys_when_the_input_contains_too_many_distinct_pitch_classes() {
    assert_eq!(
        matching(&notes(&["C", "C#", "D", "D#", "E", "F"])),
        Vec::<Key>::new()
    );
}

#[test]
fn lists_all_eight_keys_for_the_f_sharp_dominant_seventh_pitch_set() {
    assert_eq!(
        matching(&notes(&["F#", "A#", "C#", "E#"]))
            .iter()
            .map(Key::line)
            .collect::<Vec<_>>(),
        strings(&[
            "Db major (5 flats)",
            "Bb minor (5 flats)",
            "F# major (6 sharps)",
            "Gb major (6 flats)",
            "D# minor (6 sharps)",
            "Eb minor (6 flats)",
            "C# major (7 sharps)",
            "A# minor (7 sharps)",
        ])
    );
}

#[test]
fn key_limit_prefix_starts_with_five_flats_then_f_sharp_and_g_flat() {
    let lines = matching(&notes(&["F#", "A#", "C#", "E#"]))
        .iter()
        .map(Key::line)
        .collect::<Vec<_>>();
    assert_eq!(
        lines[..4],
        strings(&[
            "Db major (5 flats)",
            "Bb minor (5 flats)",
            "F# major (6 sharps)",
            "Gb major (6 flats)",
        ])
    );
}

#[test]
fn duplicate_and_enharmonic_input_notes_leave_key_membership_unchanged() {
    assert_eq!(
        matching(&notes(&["C", "E", "G", "C", "B#"])),
        matching(&notes(&["C", "E", "G"]))
    );
}

#[test]
fn empty_key_input_returns_the_exact_thirty_key_catalogue() {
    assert_eq!(
        matching(&[]),
        vec![
            key("C", false, 0),
            key("A", true, 0),
            key("G", false, 1),
            key("F", false, -1),
            key("E", true, 1),
            key("D", true, -1),
            key("D", false, 2),
            key("Bb", false, -2),
            key("B", true, 2),
            key("G", true, -2),
            key("A", false, 3),
            key("Eb", false, -3),
            key("F#", true, 3),
            key("C", true, -3),
            key("E", false, 4),
            key("Ab", false, -4),
            key("C#", true, 4),
            key("F", true, -4),
            key("B", false, 5),
            key("Db", false, -5),
            key("G#", true, 5),
            key("Bb", true, -5),
            key("F#", false, 6),
            key("Gb", false, -6),
            key("D#", true, 6),
            key("Eb", true, -6),
            key("C#", false, 7),
            key("Cb", false, -7),
            key("A#", true, 7),
            key("Ab", true, -7),
        ]
    );
}

#[test]
fn all_catalogue_key_lines_have_exact_tonics_modes_counts_and_order() {
    assert_eq!(
        matching(&[]).iter().map(Key::line).collect::<Vec<_>>(),
        strings(&[
            "C major (no accidentals)",
            "A minor (no accidentals)",
            "G major (1 sharp)",
            "F major (1 flat)",
            "E minor (1 sharp)",
            "D minor (1 flat)",
            "D major (2 sharps)",
            "Bb major (2 flats)",
            "B minor (2 sharps)",
            "G minor (2 flats)",
            "A major (3 sharps)",
            "Eb major (3 flats)",
            "F# minor (3 sharps)",
            "C minor (3 flats)",
            "E major (4 sharps)",
            "Ab major (4 flats)",
            "C# minor (4 sharps)",
            "F minor (4 flats)",
            "B major (5 sharps)",
            "Db major (5 flats)",
            "G# minor (5 sharps)",
            "Bb minor (5 flats)",
            "F# major (6 sharps)",
            "Gb major (6 flats)",
            "D# minor (6 sharps)",
            "Eb minor (6 flats)",
            "C# major (7 sharps)",
            "Cb major (7 flats)",
            "A# minor (7 sharps)",
            "Ab minor (7 flats)",
        ])
    );
}

#[test]
fn key_line_handles_zero_singular_plural_and_never_adds_a_line_feed() {
    let zero = key("C", false, 0).line();
    let one_sharp = key("G", false, 1).line();
    let one_flat = key("F", false, -1).line();
    let many_sharps = key("D", false, 2).line();
    let many_flats = key("Bb", false, -2).line();

    assert_eq!(zero, "C major (no accidentals)");
    assert_eq!(one_sharp, "G major (1 sharp)");
    assert_eq!(one_flat, "F major (1 flat)");
    assert_eq!(many_sharps, "D major (2 sharps)");
    assert_eq!(many_flats, "Bb major (2 flats)");
    assert!(!zero.contains('\n'));
    assert!(!one_sharp.contains('\n'));
    assert!(!one_flat.contains('\n'));
    assert!(!many_sharps.contains('\n'));
    assert!(!many_flats.contains('\n'));
}

#[test]
fn key_line_formats_the_minimum_i32_accidental_count_without_overflow() {
    assert_eq!(
        key("C", false, i32::MIN).line(),
        "C major (2147483648 flats)"
    );
}

#[test]
fn malformed_public_notes_do_not_panic_in_identification_or_key_matching() {
    let malformed = Note {
        letter: u8::MAX,
        acc: i8::MIN,
    };
    assert!(identify(&[malformed]).len() <= 228);
    assert!(matching(&[malformed]).len() <= 30);

    let other_malformed = Note {
        letter: u8::MAX,
        acc: i8::MAX,
    };
    assert!(identify(&[other_malformed]).len() <= 228);
    assert!(matching(&[other_malformed]).len() <= 30);
}
