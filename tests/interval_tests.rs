use tonemath::error::TheoryError;
use tonemath::interval::{self, Interval};
use tonemath::note::Note;

fn check_interval(text: &str, steps: u32, semis: i32) {
    let interval = Interval::parse(text).expect("expected a valid interval");
    assert_eq!(interval.steps, steps);
    assert_eq!(interval.semis, semis);
    assert_eq!(interval.label, text);
}

fn check_degree(text: &str, steps: u32, semis: i32) {
    let interval = Interval::degree(text).expect("expected a valid degree");
    assert_eq!(interval.steps, steps);
    assert_eq!(interval.semis, semis);
    assert_eq!(interval.label, text);
}

fn note(text: &str) -> Note {
    Note::parse(text).expect("expected a valid note")
}

#[test]
fn perfect_unison_qualities_have_their_exact_offsets() {
    check_interval("dd1", 0, -2);
    check_interval("d1", 0, -1);
    check_interval("P1", 0, 0);
    check_interval("A1", 0, 1);
    check_interval("AA1", 0, 2);
}

#[test]
fn perfect_fourth_qualities_have_their_exact_offsets() {
    check_interval("dd4", 3, 3);
    check_interval("d4", 3, 4);
    check_interval("P4", 3, 5);
    check_interval("A4", 3, 6);
    check_interval("AA4", 3, 7);
}

#[test]
fn perfect_fifth_qualities_have_their_exact_offsets() {
    check_interval("dd5", 4, 5);
    check_interval("d5", 4, 6);
    check_interval("P5", 4, 7);
    check_interval("A5", 4, 8);
    check_interval("AA5", 4, 9);
}

#[test]
fn octave_qualities_have_their_exact_offsets() {
    check_interval("dd8", 7, 10);
    check_interval("d8", 7, 11);
    check_interval("P8", 7, 12);
    check_interval("A8", 7, 13);
    check_interval("AA8", 7, 14);
}

#[test]
fn perfect_eleventh_qualities_have_their_exact_offsets() {
    check_interval("dd11", 10, 15);
    check_interval("d11", 10, 16);
    check_interval("P11", 10, 17);
    check_interval("A11", 10, 18);
    check_interval("AA11", 10, 19);
}

#[test]
fn perfect_twelfth_qualities_have_their_exact_offsets() {
    check_interval("dd12", 11, 17);
    check_interval("d12", 11, 18);
    check_interval("P12", 11, 19);
    check_interval("A12", 11, 20);
    check_interval("AA12", 11, 21);
}

#[test]
fn perfect_fifteenth_qualities_have_their_exact_offsets() {
    check_interval("dd15", 14, 22);
    check_interval("d15", 14, 23);
    check_interval("P15", 14, 24);
    check_interval("A15", 14, 25);
    check_interval("AA15", 14, 26);
}

#[test]
fn major_second_qualities_have_their_exact_offsets() {
    check_interval("dd2", 1, -1);
    check_interval("d2", 1, 0);
    check_interval("m2", 1, 1);
    check_interval("M2", 1, 2);
    check_interval("A2", 1, 3);
    check_interval("AA2", 1, 4);
}

#[test]
fn major_third_qualities_have_their_exact_offsets() {
    check_interval("dd3", 2, 1);
    check_interval("d3", 2, 2);
    check_interval("m3", 2, 3);
    check_interval("M3", 2, 4);
    check_interval("A3", 2, 5);
    check_interval("AA3", 2, 6);
}

#[test]
fn major_sixth_qualities_have_their_exact_offsets() {
    check_interval("dd6", 5, 6);
    check_interval("d6", 5, 7);
    check_interval("m6", 5, 8);
    check_interval("M6", 5, 9);
    check_interval("A6", 5, 10);
    check_interval("AA6", 5, 11);
}

#[test]
fn major_seventh_qualities_have_their_exact_offsets() {
    check_interval("dd7", 6, 8);
    check_interval("d7", 6, 9);
    check_interval("m7", 6, 10);
    check_interval("M7", 6, 11);
    check_interval("A7", 6, 12);
    check_interval("AA7", 6, 13);
}

#[test]
fn major_ninth_qualities_have_their_exact_offsets() {
    check_interval("dd9", 8, 11);
    check_interval("d9", 8, 12);
    check_interval("m9", 8, 13);
    check_interval("M9", 8, 14);
    check_interval("A9", 8, 15);
    check_interval("AA9", 8, 16);
}

#[test]
fn major_tenth_qualities_have_their_exact_offsets() {
    check_interval("dd10", 9, 13);
    check_interval("d10", 9, 14);
    check_interval("m10", 9, 15);
    check_interval("M10", 9, 16);
    check_interval("A10", 9, 17);
    check_interval("AA10", 9, 18);
}

#[test]
fn major_thirteenth_qualities_have_their_exact_offsets() {
    check_interval("dd13", 12, 18);
    check_interval("d13", 12, 19);
    check_interval("m13", 12, 20);
    check_interval("M13", 12, 21);
    check_interval("A13", 12, 22);
    check_interval("AA13", 12, 23);
}

#[test]
fn major_fourteenth_qualities_have_their_exact_offsets() {
    check_interval("dd14", 13, 20);
    check_interval("d14", 13, 21);
    check_interval("m14", 13, 22);
    check_interval("M14", 13, 23);
    check_interval("A14", 13, 24);
    check_interval("AA14", 13, 25);
}

#[test]
fn interval_parser_rejects_wrong_type_bounds_and_malformed_text() {
    assert_eq!(Interval::parse(""), None);
    assert_eq!(Interval::parse("M5"), None);
    assert_eq!(Interval::parse("P3"), None);
    assert_eq!(Interval::parse("m4"), None);
    assert_eq!(Interval::parse("P0"), None);
    assert_eq!(Interval::parse("P01"), None);
    assert_eq!(Interval::parse("P16"), None);
    assert_eq!(Interval::parse("P999999999999999999999"), None);
    assert_eq!(Interval::parse("P１"), None);
    assert_eq!(Interval::parse(" P5"), None);
    assert_eq!(Interval::parse("P5x"), None);
    assert_eq!(Interval::parse("p5"), None);
    assert_eq!(Interval::parse("P♮5"), None);
}

#[test]
fn degrees_preserve_tokens_and_apply_accidental_offsets() {
    check_degree("1", 0, 0);
    check_degree("b3", 2, 3);
    check_degree("#4", 3, 6);
    check_degree("bb7", 6, 9);
    check_degree("b9", 8, 13);
    check_degree("##15", 14, 26);
}

#[test]
fn degrees_accept_zero_one_or_two_identical_accidentals() {
    check_degree("b1", 0, -1);
    check_degree("bb1", 0, -2);
    check_degree("#1", 0, 1);
    check_degree("##1", 0, 2);
}

#[test]
fn degree_parser_rejects_mixed_excessive_and_out_of_range_tokens() {
    assert_eq!(Interval::degree(""), None);
    assert_eq!(Interval::degree("b#3"), None);
    assert_eq!(Interval::degree("#b3"), None);
    assert_eq!(Interval::degree("###3"), None);
    assert_eq!(Interval::degree("bbb3"), None);
    assert_eq!(Interval::degree("b03"), None);
    assert_eq!(Interval::degree("16"), None);
    assert_eq!(Interval::degree("♯3"), None);
    assert_eq!(Interval::degree("b３"), None);
}

#[test]
fn signed_intervals_keep_the_unsigned_label_and_direction() {
    let (interval, down) = interval::signed("-m3").expect("expected a signed interval");
    assert_eq!(interval.steps, 2);
    assert_eq!(interval.semis, 3);
    assert_eq!(interval.label, "m3");
    assert!(down);

    let (interval, down) = interval::signed("M3").expect("expected an upward interval");
    assert_eq!(interval.steps, 2);
    assert_eq!(interval.semis, 4);
    assert_eq!(interval.label, "M3");
    assert!(!down);
}

#[test]
fn signed_parser_rejects_multiple_signs_and_plus_signs() {
    assert_eq!(interval::signed("--m3"), None);
    assert_eq!(interval::signed("+M3"), None);
    assert_eq!(interval::signed("-"), None);
    assert_eq!(interval::signed("-P3"), None);
}

#[test]
fn intervals_spell_up_and_down_with_letter_and_pitch_class_motion() {
    assert_eq!(
        interval::up(&note("C"), &Interval::parse("M3").unwrap())
            .unwrap()
            .name(),
        "E"
    );
    assert_eq!(
        interval::down(&note("C"), &Interval::parse("M3").unwrap())
            .unwrap()
            .name(),
        "Ab"
    );
    assert_eq!(
        interval::up(&note("B"), &Interval::parse("m2").unwrap())
            .unwrap()
            .name(),
        "C"
    );
    assert_eq!(
        interval::up(&note("C"), &Interval::parse("M9").unwrap())
            .unwrap()
            .name(),
        "D"
    );
    assert_eq!(
        interval::down(&note("C"), &Interval::parse("M9").unwrap())
            .unwrap()
            .name(),
        "Bb"
    );
}

#[test]
fn diminished_unisons_with_negative_semitones_spell_in_both_directions() {
    let diminished_unison = Interval::parse("d1").unwrap();
    assert_eq!(diminished_unison.semis, -1);
    assert_eq!(
        interval::up(&note("C"), &diminished_unison).unwrap().name(),
        "Cb"
    );
    assert_eq!(
        interval::down(&note("C"), &diminished_unison)
            .unwrap()
            .name(),
        "C#"
    );
}

#[test]
fn doubly_augmented_unisons_spell_double_accidentals_in_both_directions() {
    let augmented_unison = Interval::parse("AA1").unwrap();
    assert_eq!(
        interval::up(&note("C"), &augmented_unison).unwrap().name(),
        "C##"
    );
    assert_eq!(
        interval::down(&note("C"), &augmented_unison)
            .unwrap()
            .name(),
        "Cbb"
    );
}

#[test]
fn spelling_errors_are_exact_and_include_the_direction_and_original_label() {
    let major_seventh = Interval::parse("M7").unwrap();
    assert_eq!(
        interval::up(&note("G##"), &major_seventh),
        Err(TheoryError("cannot spell G## up by M7".to_owned()))
    );

    let augmented_unison = Interval::parse("A1").unwrap();
    assert_eq!(
        interval::down(&note("Cbb"), &augmented_unison),
        Err(TheoryError("cannot spell Cbb down by A1".to_owned()))
    );

    let seventh_degree = Interval::degree("7").unwrap();
    assert_eq!(
        interval::up(&note("G##"), &seventh_degree),
        Err(TheoryError("cannot spell G## up by 7".to_owned()))
    );
}

#[test]
fn name_between_reports_perfect_fifth_and_unison() {
    assert_eq!(
        interval::name_between(&note("C"), &note("G")),
        Some(("P5".to_owned(), 7))
    );
    assert_eq!(
        interval::name_between(&note("C"), &note("C")),
        Some(("P1".to_owned(), 0))
    );
}

#[test]
fn name_between_uses_descending_unison_numbers_for_flattened_octaves() {
    assert_eq!(
        interval::name_between(&note("C"), &note("Cb")),
        Some(("d8".to_owned(), 11))
    );
    assert_eq!(
        interval::name_between(&note("C"), &note("Cbb")),
        Some(("dd8".to_owned(), 10))
    );
}

#[test]
fn name_between_handles_enharmonic_notes_and_augmented_unisons() {
    assert_eq!(
        interval::name_between(&note("B#"), &note("C")),
        Some(("d2".to_owned(), 0))
    );
    assert_eq!(
        interval::name_between(&note("C"), &note("C#")),
        Some(("A1".to_owned(), 1))
    );
    assert_eq!(interval::name_between(&note("C##"), &note("Dbb")), None);
}

#[test]
fn invalid_public_note_fields_do_not_panic_or_get_a_musical_interval_name() {
    let invalid = Note { letter: 7, acc: 3 };
    let interval = Interval::parse("M3").unwrap();
    assert_eq!(
        interval::up(&invalid, &interval),
        Err(TheoryError("cannot spell ?### up by M3".to_owned()))
    );
    assert_eq!(
        interval::down(&invalid, &interval),
        Err(TheoryError("cannot spell ?### down by M3".to_owned()))
    );
    assert_eq!(interval::name_between(&invalid, &note("C")), None);
    assert_eq!(interval::name_between(&note("C"), &invalid), None);
}

#[test]
fn extreme_public_interval_fields_do_not_overflow_spelling_arithmetic() {
    let start = note("C");
    let largest_steps_and_smallest_size = Interval {
        steps: u32::MAX,
        semis: i32::MIN,
        label: "extreme".to_owned(),
    };
    let smallest_steps_and_largest_size = Interval {
        steps: 0,
        semis: i32::MAX,
        label: "extreme".to_owned(),
    };

    let first_up = interval::up(&start, &largest_steps_and_smallest_size);
    let first_down = interval::down(&start, &largest_steps_and_smallest_size);
    let second_up = interval::up(&start, &smallest_steps_and_largest_size);
    let second_down = interval::down(&start, &smallest_steps_and_largest_size);
    assert!(first_up.is_ok() || first_up.is_err());
    assert!(first_down.is_ok() || first_down.is_err());
    assert!(second_up.is_ok() || second_up.is_err());
    assert!(second_down.is_ok() || second_down.is_err());
}
