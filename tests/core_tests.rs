use std::cell::Cell;

use tonemath::error::TheoryError;
use tonemath::io::{Io, words};
use tonemath::note::{NATURAL, Note};
use tonemath::options::{Parsed, Spec, parse};
use tonemath::text::{or_list, plural, render_table, show};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn option_specs() -> [Spec; 2] {
    [
        Spec {
            name: "--limit",
            lo: 1,
            hi: 100,
            fallback: 100,
        },
        Spec {
            name: "--other",
            lo: 1,
            hi: 30,
            fallback: 30,
        },
    ]
}

fn parse_options(argv: &[&str]) -> Result<Parsed, TheoryError> {
    parse(&strings(argv), &option_specs())
}

fn check_note(text: &str, letter: u8, acc: i8, name: &str, pc: i32) {
    let note = Note::parse(text).unwrap();
    assert_eq!(note, Note { letter, acc });
    assert_eq!(note.name(), name);
    assert_eq!(note.pc(), pc);
}

struct MemoryIo {
    stdin: Vec<u8>,
    reads: Cell<usize>,
    outs: Cell<usize>,
    errs: Cell<usize>,
}

impl MemoryIo {
    fn new(stdin: &[u8]) -> Self {
        Self {
            stdin: stdin.to_vec(),
            reads: Cell::new(0),
            outs: Cell::new(0),
            errs: Cell::new(0),
        }
    }
}

impl Io for MemoryIo {
    fn read_stdin(&self) -> Vec<u8> {
        self.reads.set(self.reads.get() + 1);
        self.stdin.clone()
    }

    fn out(&self, _text: &str) {
        self.outs.set(self.outs.get() + 1);
    }

    fn err(&self, _text: &str) {
        self.errs.set(self.errs.get() + 1);
    }
}

#[test]
fn theory_error_has_the_required_debug_clone_and_equality_contract() {
    let error = TheoryError("problem".to_owned());
    let clone = error.clone();
    assert_eq!(error, clone);
    assert_eq!(format!("{error:?}"), "TheoryError(\"problem\")");
}

#[test]
fn show_wraps_empty_text_in_double_quotes() {
    assert_eq!(show(""), "\"\"");
}

#[test]
fn show_preserves_printable_ascii_boundaries() {
    assert_eq!(show("!~"), "\"!~\"");
}

#[test]
fn show_keeps_printable_quotes_and_backslashes_literally() {
    assert_eq!(show("a\"b\\c"), "\"a\"b\\c\"");
}

#[test]
fn show_replaces_each_non_printable_or_non_ascii_scalar_once() {
    assert_eq!(show("A B\né😀"), "\"A?B???\"");
    assert_eq!(show("\0\t\r\n"), "\"????\"");
}

#[test]
fn or_list_handles_zero_and_one_item() {
    assert_eq!(or_list(&[]), "");
    assert_eq!(or_list(&strings(&["A"])), "A");
}

#[test]
fn or_list_handles_two_three_and_four_items() {
    assert_eq!(or_list(&strings(&["A", "B"])), "A or B");
    assert_eq!(or_list(&strings(&["A", "B", "C"])), "A, B or C");
    assert_eq!(or_list(&strings(&["A", "B", "C", "D"])), "A, B, C or D");
}

#[test]
fn plural_selects_singular_only_for_one() {
    assert_eq!(plural(0, "sharp", "sharps"), "0 sharps");
    assert_eq!(plural(1, "sharp", "sharps"), "1 sharp");
    assert_eq!(plural(2, "sharp", "sharps"), "2 sharps");
    assert_eq!(plural(0, "semitone", "semitones"), "0 semitones");
}

#[test]
fn plural_formats_the_architecture_appropriate_usize_maximum() {
    let expected = if usize::BITS == 64 {
        "18446744073709551615 items"
    } else {
        "4294967295 items"
    };
    assert_eq!(plural(usize::MAX, "item", "items"), expected);
}

#[test]
fn render_table_aligns_columns_to_the_widest_cell() {
    assert_eq!(
        render_table(
            &["chord", "numeral"],
            &[strings(&["C", "I"]), strings(&["F#dim7", "#ivdim7"]),],
        ),
        strings(&["chord   numeral", "C       I", "F#dim7  #ivdim7"])
    );
}

#[test]
fn render_table_counts_unicode_scalars_for_width() {
    assert_eq!(
        render_table(&["x", "y"], &[strings(&["é", "😀"])]),
        strings(&["x  y", "é  😀"])
    );
}

#[test]
fn render_table_with_no_header_or_rows_returns_one_empty_line() {
    assert_eq!(render_table(&[], &[]), strings(&[""]));
}

#[test]
fn render_table_fills_a_missing_ragged_cell_without_trailing_spaces() {
    let rendered = render_table(&["a", "b"], &[strings(&["x"])]);
    assert_eq!(rendered, strings(&["a  b", "x"]));
    assert!(!rendered[0].ends_with(' '));
    assert!(!rendered[1].ends_with(' '));
}

#[test]
fn render_table_keeps_columns_from_rows_wider_than_the_header() {
    assert_eq!(
        render_table(&["a"], &[strings(&["x", "yz"])]),
        strings(&["a", "x  yz"])
    );
}

#[test]
fn parse_options_returns_fallbacks_for_absent_values() {
    let parsed = parse_options(&[]).unwrap();
    assert_eq!(parsed.values.len(), 2);
    assert_eq!(parsed.values.get("--limit"), Some(&100));
    assert_eq!(parsed.values.get("--other"), Some(&30));
    assert_eq!(parsed.args, strings(&[]));
}

#[test]
fn parse_options_keeps_non_options_in_order() {
    let parsed = parse_options(&["C", "-", "-m3"]).unwrap();
    assert_eq!(parsed.args, strings(&["C", "-", "-m3"]));
    assert_eq!(parsed.values.get("--limit"), Some(&100));
    assert_eq!(parsed.values.get("--other"), Some(&30));
}

#[test]
fn parse_options_accepts_inline_values_and_uses_the_last_repetition() {
    let parsed = parse_options(&["C", "--limit=2", "--limit", "4"]).unwrap();
    assert_eq!(parsed.values.get("--limit"), Some(&4));
    assert_eq!(parsed.values.get("--other"), Some(&30));
    assert_eq!(parsed.args, strings(&["C"]));
}

#[test]
fn parse_options_ignores_an_earlier_invalid_repetition_when_final_value_is_valid() {
    let parsed = parse_options(&["--limit=bad", "--limit=3"]).unwrap();
    assert_eq!(parsed.values.get("--limit"), Some(&3));
}

#[test]
fn parse_options_validates_the_final_repetition() {
    assert_eq!(
        parse_options(&["--limit=2", "--limit=bad"]).err(),
        Some(TheoryError(
            "invalid --limit \"bad\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_reports_unknown_names_immediately() {
    assert_eq!(
        parse_options(&["--limit=bad", "--wat=x"]).err(),
        Some(TheoryError("unknown option \"--wat=x\"".to_owned()))
    );
    assert_eq!(
        parse_options(&["--"]).err(),
        Some(TheoryError("unknown option \"--\"".to_owned()))
    );
    assert_eq!(
        parse_options(&["--help"]).err(),
        Some(TheoryError("unknown option \"--help\"".to_owned()))
    );
}

#[test]
fn parse_options_sanitizes_unknown_option_text() {
    assert_eq!(
        parse_options(&["--what=é"]).err(),
        Some(TheoryError("unknown option \"--what=?\"".to_owned()))
    );
}

#[test]
fn parse_options_reports_missing_values_before_deferred_validation() {
    assert_eq!(
        parse_options(&["--limit=bad", "--other"]).err(),
        Some(TheoryError("--other needs a value".to_owned()))
    );
    assert_eq!(
        parse_options(&["--limit"]).err(),
        Some(TheoryError("--limit needs a value".to_owned()))
    );
}

#[test]
fn parse_options_consumes_an_option_looking_value_as_the_value() {
    assert_eq!(
        parse_options(&["--limit", "--other"]).err(),
        Some(TheoryError(
            "invalid --limit \"--other\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_validates_final_values_in_spec_order() {
    assert_eq!(
        parse_options(&["--other=bad", "--limit=bad"]).err(),
        Some(TheoryError(
            "invalid --limit \"bad\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_splits_inline_values_at_only_the_first_equals_sign() {
    assert_eq!(
        parse_options(&["--limit=2=3"]).err(),
        Some(TheoryError(
            "invalid --limit \"2=3\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_accepts_inclusive_option_bounds() {
    let parsed = parse_options(&["--limit=1", "--other=30"]).unwrap();
    assert_eq!(parsed.values.get("--limit"), Some(&1));
    assert_eq!(parsed.values.get("--other"), Some(&30));

    let parsed = parse_options(&["--limit=100", "--other=1"]).unwrap();
    assert_eq!(parsed.values.get("--limit"), Some(&100));
    assert_eq!(parsed.values.get("--other"), Some(&1));
}

#[test]
fn parse_options_accepts_zero_and_six_ascii_digits_when_in_range() {
    let specs = [Spec {
        name: "--number",
        lo: 0,
        hi: 999_999,
        fallback: 42,
    }];
    let zero = parse(&strings(&["--number=0"]), &specs).unwrap();
    assert_eq!(zero.values.get("--number"), Some(&0));

    let upper = parse(&strings(&["--number=999999"]), &specs).unwrap();
    assert_eq!(upper.values.get("--number"), Some(&999_999));
}

#[test]
fn parse_options_rejects_values_outside_one_to_thirty() {
    let specs = [Spec {
        name: "--number",
        lo: 1,
        hi: 30,
        fallback: 15,
    }];
    assert_eq!(
        parse(&strings(&["--number=0"]), &specs).err(),
        Some(TheoryError(
            "invalid --number \"0\" (expected an integer from 1 to 30)".to_owned()
        ))
    );
    assert_eq!(
        parse(&strings(&["--number=31"]), &specs).err(),
        Some(TheoryError(
            "invalid --number \"31\" (expected an integer from 1 to 30)".to_owned()
        ))
    );
}

#[test]
fn parse_options_rejects_empty_leading_zero_and_signed_values() {
    assert_eq!(
        parse_options(&["--limit="]).err(),
        Some(TheoryError(
            "invalid --limit \"\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=00"]).err(),
        Some(TheoryError(
            "invalid --limit \"00\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=01"]).err(),
        Some(TheoryError(
            "invalid --limit \"01\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=+1"]).err(),
        Some(TheoryError(
            "invalid --limit \"+1\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=-1"]).err(),
        Some(TheoryError(
            "invalid --limit \"-1\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_rejects_whitespace_non_ascii_digits_and_too_many_digits() {
    assert_eq!(
        parse_options(&["--limit= 1"]).err(),
        Some(TheoryError(
            "invalid --limit \"?1\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=1 "]).err(),
        Some(TheoryError(
            "invalid --limit \"1?\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=١"]).err(),
        Some(TheoryError(
            "invalid --limit \"?\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
    assert_eq!(
        parse_options(&["--limit=1000000"]).err(),
        Some(TheoryError(
            "invalid --limit \"1000000\" (expected an integer from 1 to 100)".to_owned()
        ))
    );
}

#[test]
fn parse_options_rejects_malformed_values_with_zero_to_999999_bounds() {
    let specs = [Spec {
        name: "--number",
        lo: 0,
        hi: 999_999,
        fallback: 0,
    }];
    let parse_number = |value: &str| parse(&strings(&[&format!("--number={value}")]), &specs).err();

    assert_eq!(
        parse_number(""),
        Some(TheoryError(
            "invalid --number \"\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("00"),
        Some(TheoryError(
            "invalid --number \"00\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("01"),
        Some(TheoryError(
            "invalid --number \"01\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("+1"),
        Some(TheoryError(
            "invalid --number \"+1\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("-1"),
        Some(TheoryError(
            "invalid --number \"-1\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number(" 1"),
        Some(TheoryError(
            "invalid --number \"?1\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("1 "),
        Some(TheoryError(
            "invalid --number \"1?\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("١"),
        Some(TheoryError(
            "invalid --number \"?\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
    assert_eq!(
        parse_number("1000000"),
        Some(TheoryError(
            "invalid --number \"1000000\" (expected an integer from 0 to 999999)".to_owned()
        ))
    );
}

#[test]
fn parse_options_does_not_validate_an_absent_fallback() {
    let specs = [Spec {
        name: "--number",
        lo: 1,
        hi: 30,
        fallback: 999,
    }];
    let parsed = parse(&strings(&[]), &specs).unwrap();
    assert_eq!(parsed.values.get("--number"), Some(&999));
}

#[test]
fn words_passes_arguments_through_without_reading_or_writing() {
    let io = MemoryIo::new(b"unused");
    assert_eq!(words(&io, &[]).unwrap(), strings(&[]));
    assert_eq!(
        words(&io, &["C".to_owned(), "-".to_owned()]).unwrap(),
        strings(&["C", "-"])
    );
    assert_eq!(words(&io, &["C E".to_owned()]).unwrap(), strings(&["C E"]));
    assert_eq!(io.reads.get(), 0);
    assert_eq!(io.outs.get(), 0);
    assert_eq!(io.errs.get(), 0);
}

#[test]
fn words_splits_only_ascii_space_tab_cr_and_lf_and_reads_once() {
    let io = MemoryIo::new(b" C\tE\r\nG A \n");
    assert_eq!(
        words(&io, &["-".to_owned()]).unwrap(),
        strings(&["C", "E", "G", "A"])
    );
    assert_eq!(io.reads.get(), 1);
    assert_eq!(io.outs.get(), 0);
    assert_eq!(io.errs.get(), 0);
}

#[test]
fn words_skips_exactly_one_leading_bom_and_accepts_crlf() {
    let io = MemoryIo::new("\u{feff}C E\r\nG".as_bytes());
    assert_eq!(
        words(&io, &["-".to_owned()]).unwrap(),
        strings(&["C", "E", "G"])
    );
    assert_eq!(io.reads.get(), 1);
    assert_eq!(io.outs.get(), 0);
    assert_eq!(io.errs.get(), 0);
}

#[test]
fn words_reports_invalid_utf8_from_exact_input_bytes() {
    let io = MemoryIo::new(&[0x43, 0x20, 0xFF, 0x0A]);
    assert_eq!(
        words(&io, &["-".to_owned()]).err(),
        Some(TheoryError("<stdin>: input is not valid UTF-8".to_owned()))
    );
    assert_eq!(io.reads.get(), 1);
    assert_eq!(io.outs.get(), 0);
    assert_eq!(io.errs.get(), 0);
}

#[test]
fn words_reports_empty_separators_and_bom_only_stdin() {
    let empty = MemoryIo::new(&[]);
    assert_eq!(
        words(&empty, &["-".to_owned()]).err(),
        Some(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(empty.reads.get(), 1);
    assert_eq!(empty.outs.get(), 0);
    assert_eq!(empty.errs.get(), 0);

    let spaces = MemoryIo::new(b" \t\r\n ");
    assert_eq!(
        words(&spaces, &["-".to_owned()]).err(),
        Some(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(spaces.reads.get(), 1);
    assert_eq!(spaces.outs.get(), 0);
    assert_eq!(spaces.errs.get(), 0);

    let bom = MemoryIo::new(&[0xEF, 0xBB, 0xBF]);
    assert_eq!(
        words(&bom, &["-".to_owned()]).err(),
        Some(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(bom.reads.get(), 1);
    assert_eq!(bom.outs.get(), 0);
    assert_eq!(bom.errs.get(), 0);
}

#[test]
fn words_preserves_a_second_or_interior_bom() {
    let doubled = MemoryIo::new("\u{feff}\u{feff}C".as_bytes());
    assert_eq!(
        words(&doubled, &["-".to_owned()]).unwrap(),
        strings(&["\u{feff}C"])
    );

    let interior = MemoryIo::new("C\u{feff}E".as_bytes());
    assert_eq!(
        words(&interior, &["-".to_owned()]).unwrap(),
        strings(&["C\u{feff}E"])
    );
    assert_eq!(doubled.reads.get(), 1);
    assert_eq!(doubled.outs.get(), 0);
    assert_eq!(doubled.errs.get(), 0);
    assert_eq!(interior.reads.get(), 1);
    assert_eq!(interior.outs.get(), 0);
    assert_eq!(interior.errs.get(), 0);
}

#[test]
fn words_preserves_non_ascii_and_control_characters_not_in_the_separator_set() {
    let io = MemoryIo::new("C\u{00a0}E\u{000b}F\u{000c}G\0A".as_bytes());
    assert_eq!(
        words(&io, &["-".to_owned()]).unwrap(),
        strings(&["C\u{00a0}E\u{000b}F\u{000c}G\0A"])
    );
    assert_eq!(io.reads.get(), 1);
    assert_eq!(io.outs.get(), 0);
    assert_eq!(io.errs.get(), 0);
}

#[test]
fn natural_pitch_classes_are_exact() {
    assert_eq!(NATURAL, [0, 2, 4, 5, 7, 9, 11]);
}

#[test]
fn note_parses_and_names_all_c_accidentals() {
    check_note("C", 0, 0, "C", 0);
    check_note("C#", 0, 1, "C#", 1);
    check_note("C##", 0, 2, "C##", 2);
    check_note("Cb", 0, -1, "Cb", 11);
    check_note("Cbb", 0, -2, "Cbb", 10);
}

#[test]
fn note_parses_and_names_all_d_accidentals() {
    check_note("D", 1, 0, "D", 2);
    check_note("D#", 1, 1, "D#", 3);
    check_note("D##", 1, 2, "D##", 4);
    check_note("Db", 1, -1, "Db", 1);
    check_note("Dbb", 1, -2, "Dbb", 0);
}

#[test]
fn note_parses_and_names_all_e_accidentals() {
    check_note("E", 2, 0, "E", 4);
    check_note("E#", 2, 1, "E#", 5);
    check_note("E##", 2, 2, "E##", 6);
    check_note("Eb", 2, -1, "Eb", 3);
    check_note("Ebb", 2, -2, "Ebb", 2);
}

#[test]
fn note_parses_and_names_all_f_accidentals() {
    check_note("F", 3, 0, "F", 5);
    check_note("F#", 3, 1, "F#", 6);
    check_note("F##", 3, 2, "F##", 7);
    check_note("Fb", 3, -1, "Fb", 4);
    check_note("Fbb", 3, -2, "Fbb", 3);
}

#[test]
fn note_parses_and_names_all_g_accidentals() {
    check_note("G", 4, 0, "G", 7);
    check_note("G#", 4, 1, "G#", 8);
    check_note("G##", 4, 2, "G##", 9);
    check_note("Gb", 4, -1, "Gb", 6);
    check_note("Gbb", 4, -2, "Gbb", 5);
}

#[test]
fn note_parses_and_names_all_a_accidentals() {
    check_note("A", 5, 0, "A", 9);
    check_note("A#", 5, 1, "A#", 10);
    check_note("A##", 5, 2, "A##", 11);
    check_note("Ab", 5, -1, "Ab", 8);
    check_note("Abb", 5, -2, "Abb", 7);
}

#[test]
fn note_parses_and_names_all_b_accidentals() {
    check_note("B", 6, 0, "B", 11);
    check_note("B#", 6, 1, "B#", 0);
    check_note("B##", 6, 2, "B##", 1);
    check_note("Bb", 6, -1, "Bb", 10);
    check_note("Bbb", 6, -2, "Bbb", 9);
}

#[test]
fn note_rejects_malformed_case_accidental_and_unicode_forms() {
    assert_eq!(Note::parse(""), None);
    assert_eq!(Note::parse("H"), None);
    assert_eq!(Note::parse("c"), None);
    assert_eq!(Note::parse("C#b"), None);
    assert_eq!(Note::parse("C###"), None);
    assert_eq!(Note::parse("Cbbb"), None);
    assert_eq!(Note::parse(" C"), None);
    assert_eq!(Note::parse("C♯"), None);
}

#[test]
fn note_is_copy_clone_debug_and_equality_capable() {
    let note = Note { letter: 0, acc: 1 };
    let copied = note;
    let cloned = note.clone();
    assert_eq!(note, copied);
    assert_eq!(note, cloned);
    assert_eq!(format!("{note:?}"), "Note { letter: 0, acc: 1 }");
}
