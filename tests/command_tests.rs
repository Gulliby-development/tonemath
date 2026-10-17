use std::cell::{Cell, RefCell};

use tonemath::cmd_analyze;
use tonemath::cmd_chord;
use tonemath::cmd_identify;
use tonemath::cmd_interval;
use tonemath::cmd_keys;
use tonemath::cmd_scale;
use tonemath::cmd_transpose;
use tonemath::error::TheoryError;
use tonemath::io::Io;

type Command = fn(&[String], &dyn Io) -> Result<i32, TheoryError>;

const ONE_HUNDRED_CHORDS: &[u8] = concat!(
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C "
)
.as_bytes();

const ONE_HUNDRED_ONE_CHORDS: &[u8] = concat!(
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C C C C C C C C C C ",
    "C"
)
.as_bytes();

const ONE_HUNDRED_TRANSPOSED: &str = concat!(
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D ",
    "D D D D D D D D D D\n"
);

const ONE_HUNDRED_ANALYSIS_ROWS: &str = concat!(
    "chord  numeral\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n",
    "C      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\nC      I\n"
);

struct MemoryIo {
    stdin: Vec<u8>,
    reads: Cell<usize>,
    outs: RefCell<Vec<String>>,
    errs: RefCell<Vec<String>>,
}

impl MemoryIo {
    fn new(stdin: &[u8]) -> Self {
        Self {
            stdin: stdin.to_vec(),
            reads: Cell::new(0),
            outs: RefCell::new(Vec::new()),
            errs: RefCell::new(Vec::new()),
        }
    }
}

impl Io for MemoryIo {
    fn read_stdin(&self) -> Vec<u8> {
        self.reads.set(self.reads.get() + 1);
        self.stdin.clone()
    }

    fn out(&self, text: &str) {
        self.outs.borrow_mut().push(text.to_owned());
    }

    fn err(&self, text: &str) {
        self.errs.borrow_mut().push(text.to_owned());
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn assert_output(command: Command, argv: &[&str], expected: &str) {
    let io = MemoryIo::new(&[]);
    assert_eq!(command(&strings(argv), &io), Ok(0));
    assert_eq!(&*io.outs.borrow(), &[expected.to_owned()]);
    assert!(io.errs.borrow().is_empty());
}

fn assert_error(command: Command, argv: &[&str], expected: &str) {
    let io = MemoryIo::new(&[]);
    assert_eq!(
        command(&strings(argv), &io),
        Err(TheoryError(expected.to_owned()))
    );
    assert!(io.outs.borrow().is_empty());
    assert!(io.errs.borrow().is_empty());
}

fn assert_empty_result(command: Command, argv: &[&str], status: i32) {
    let io = MemoryIo::new(&[]);
    assert_eq!(command(&strings(argv), &io), Ok(status));
    assert!(io.outs.borrow().is_empty());
    assert!(io.errs.borrow().is_empty());
}

#[test]
fn scale_spells_every_named_scale_with_exact_steps() {
    assert_output(
        cmd_scale::run,
        &["C", "major"],
        "scale: C major\nnotes: C D E F G A B\ndegrees: 1 2 3 4 5 6 7\nsteps: 2 2 1 2 2 2 1\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "minor"],
        "scale: C minor\nnotes: C D Eb F G Ab Bb\ndegrees: 1 2 b3 4 5 b6 b7\nsteps: 2 1 2 2 1 2 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "harmonic-minor"],
        "scale: C harmonic-minor\nnotes: C D Eb F G Ab B\ndegrees: 1 2 b3 4 5 b6 7\nsteps: 2 1 2 2 1 3 1\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "melodic-minor"],
        "scale: C melodic-minor\nnotes: C D Eb F G A B\ndegrees: 1 2 b3 4 5 6 7\nsteps: 2 1 2 2 2 2 1\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "dorian"],
        "scale: C dorian\nnotes: C D Eb F G A Bb\ndegrees: 1 2 b3 4 5 6 b7\nsteps: 2 1 2 2 2 1 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "phrygian"],
        "scale: C phrygian\nnotes: C Db Eb F G Ab Bb\ndegrees: 1 b2 b3 4 5 b6 b7\nsteps: 1 2 2 2 1 2 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "lydian"],
        "scale: C lydian\nnotes: C D E F# G A B\ndegrees: 1 2 3 #4 5 6 7\nsteps: 2 2 2 1 2 2 1\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "mixolydian"],
        "scale: C mixolydian\nnotes: C D E F G A Bb\ndegrees: 1 2 3 4 5 6 b7\nsteps: 2 2 1 2 2 1 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "locrian"],
        "scale: C locrian\nnotes: C Db Eb F Gb Ab Bb\ndegrees: 1 b2 b3 4 b5 b6 b7\nsteps: 1 2 2 1 2 2 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "major-pentatonic"],
        "scale: C major-pentatonic\nnotes: C D E G A\ndegrees: 1 2 3 5 6\nsteps: 2 2 3 2 3\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "minor-pentatonic"],
        "scale: C minor-pentatonic\nnotes: C Eb F G Bb\ndegrees: 1 b3 4 5 b7\nsteps: 3 2 2 3 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "blues"],
        "scale: C blues\nnotes: C Eb F Gb G Bb\ndegrees: 1 b3 4 b5 5 b7\nsteps: 3 2 1 1 3 2\n",
    );
    assert_output(
        cmd_scale::run,
        &["C", "whole-tone"],
        "scale: C whole-tone\nnotes: C D E F# G# Bb\ndegrees: 1 2 3 #4 #5 b7\nsteps: 2 2 2 2 2 2\n",
    );
}

#[test]
fn scale_reports_arity_options_note_and_scale_errors_in_contract_order() {
    assert_error(cmd_scale::run, &[], "missing ROOT");
    assert_error(cmd_scale::run, &["C"], "missing NAME");
    assert_error(
        cmd_scale::run,
        &["C", "missing", "extra"],
        "too many arguments",
    );
    assert_error(cmd_scale::run, &["--help"], "unknown option \"--help\"");
    assert_error(cmd_scale::run, &["--ARG"], "unknown option \"--ARG\"");
    assert_error(cmd_scale::run, &["X", "major"], "invalid note \"X\"");
    assert_error(
        cmd_scale::run,
        &["C", "bad"],
        "unknown scale \"bad\" (expected major, minor, harmonic-minor, melodic-minor, dorian, phrygian, lydian, mixolydian, locrian, major-pentatonic, minor-pentatonic, blues or whole-tone)",
    );
    assert_error(
        cmd_scale::run,
        &["G##", "major"],
        "cannot spell G## up by 7",
    );
}

#[test]
fn chord_renders_every_quality_with_its_literal_spelling() {
    assert_output(
        cmd_chord::run,
        &["C"],
        "chord: C\nnotes: C E G\ndegrees: 1 3 5\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cm"],
        "chord: Cm\nnotes: C Eb G\ndegrees: 1 b3 5\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cdim"],
        "chord: Cdim\nnotes: C Eb Gb\ndegrees: 1 b3 b5\n",
    );
    assert_output(
        cmd_chord::run,
        &["Caug"],
        "chord: Caug\nnotes: C E G#\ndegrees: 1 3 #5\n",
    );
    assert_output(
        cmd_chord::run,
        &["Csus2"],
        "chord: Csus2\nnotes: C D G\ndegrees: 1 2 5\n",
    );
    assert_output(
        cmd_chord::run,
        &["Csus4"],
        "chord: Csus4\nnotes: C F G\ndegrees: 1 4 5\n",
    );
    assert_output(
        cmd_chord::run,
        &["C5"],
        "chord: C5\nnotes: C G\ndegrees: 1 5\n",
    );
    assert_output(
        cmd_chord::run,
        &["C6"],
        "chord: C6\nnotes: C E G A\ndegrees: 1 3 5 6\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cm6"],
        "chord: Cm6\nnotes: C Eb G A\ndegrees: 1 b3 5 6\n",
    );
    assert_output(
        cmd_chord::run,
        &["C7"],
        "chord: C7\nnotes: C E G Bb\ndegrees: 1 3 5 b7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cmaj7"],
        "chord: Cmaj7\nnotes: C E G B\ndegrees: 1 3 5 7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cm7"],
        "chord: Cm7\nnotes: C Eb G Bb\ndegrees: 1 b3 5 b7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cmmaj7"],
        "chord: Cmmaj7\nnotes: C Eb G B\ndegrees: 1 b3 5 7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cm7b5"],
        "chord: Cm7b5\nnotes: C Eb Gb Bb\ndegrees: 1 b3 b5 b7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cdim7"],
        "chord: Cdim7\nnotes: C Eb Gb Bbb\ndegrees: 1 b3 b5 bb7\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cadd9"],
        "chord: Cadd9\nnotes: C E G D\ndegrees: 1 3 5 9\n",
    );
    assert_output(
        cmd_chord::run,
        &["C9"],
        "chord: C9\nnotes: C E G Bb D\ndegrees: 1 3 5 b7 9\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cmaj9"],
        "chord: Cmaj9\nnotes: C E G B D\ndegrees: 1 3 5 7 9\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cm9"],
        "chord: Cm9\nnotes: C Eb G Bb D\ndegrees: 1 b3 5 b7 9\n",
    );
}

#[test]
fn chord_rotates_notes_and_degree_tokens_together_after_spelling() {
    assert_output(
        cmd_chord::run,
        &["Cmaj7/E"],
        "chord: Cmaj7/E\nnotes: E G B C\ndegrees: 3 5 7 1\n",
    );
    assert_output(
        cmd_chord::run,
        &["G##/Db"],
        "chord: G##/Db\nnotes: B## D## G##\ndegrees: 3 5 1\n",
    );
    assert_output(
        cmd_chord::run,
        &["Cmaj7/Fb"],
        "chord: Cmaj7/Fb\nnotes: E G B C\ndegrees: 3 5 7 1\n",
    );
    assert_output(
        cmd_chord::run,
        &["Bbdim7"],
        "chord: Bbdim7\nnotes: Bb Db Fb Abb\ndegrees: 1 b3 b5 bb7\n",
    );
}

#[test]
fn chord_reports_invalid_arity_grammar_bass_and_first_spelling_failure() {
    assert_error(cmd_chord::run, &[], "missing SYMBOL");
    assert_error(
        cmd_chord::run,
        &["--limit=2"],
        "unknown option \"--limit=2\"",
    );
    assert_error(cmd_chord::run, &["C", "extra"], "too many arguments");
    assert_error(
        cmd_chord::run,
        &["C/D"],
        "invalid chord \"C/D\" (the bass is not a chord note)",
    );
    assert_error(cmd_chord::run, &["C###"], "invalid chord \"C###\"");
    assert_error(cmd_chord::run, &["TEXT"], "invalid chord \"TEXT\"");
    assert_error(cmd_chord::run, &["G##maj7"], "cannot spell G## up by 7");
}

#[test]
fn interval_prints_compound_directional_and_singular_or_plural_distances() {
    assert_output(cmd_interval::run, &["C", "G"], "C to G: P5 (7 semitones)\n");
    assert_output(
        cmd_interval::run,
        &["C", "Cb"],
        "C to Cb: d8 (11 semitones)\n",
    );
    assert_output(
        cmd_interval::run,
        &["B#", "C"],
        "B# to C: d2 (0 semitones)\n",
    );
    assert_output(
        cmd_interval::run,
        &["C", "C#"],
        "C to C#: A1 (1 semitone)\n",
    );
    assert_output(cmd_interval::run, &["C", "D"], "C to D: M2 (2 semitones)\n");
}

#[test]
fn interval_reports_missing_extra_invalid_and_unnameable_arguments_in_order() {
    assert_error(cmd_interval::run, &[], "missing A");
    assert_error(cmd_interval::run, &["C"], "missing B");
    assert_error(cmd_interval::run, &["C", "D", "E"], "too many arguments");
    assert_error(cmd_interval::run, &["X", "D"], "invalid note \"X\"");
    assert_error(cmd_interval::run, &["C", "X"], "invalid note \"X\"");
    assert_error(
        cmd_interval::run,
        &["C##", "Dbb"],
        "no name for the interval from C## to Dbb",
    );
}

#[test]
fn identify_preserves_the_supplied_chord_examples_and_tied_order() {
    assert_output(cmd_identify::run, &["C", "E", "G", "A"], "C6\nAm7/C\n");
    assert_output(
        cmd_identify::run,
        &["C", "Eb", "Gb", "A"],
        "Cdim7\nEbdim7/C\nGbdim7/C\nAdim7/C\n",
    );
    assert_output(
        cmd_identify::run,
        &["--limit", "2", "C", "Eb", "Gb", "A"],
        "Cdim7\nEbdim7/C\n",
    );
    assert_output(
        cmd_identify::run,
        &["--limit=99", "--limit=2", "C", "Eb", "Gb", "A"],
        "Cdim7\nEbdim7/C\n",
    );
}

#[test]
fn identify_keeps_one_candidate_per_pitch_class_and_first_input_spellings() {
    assert_output(
        cmd_identify::run,
        &["C", "E", "G", "A", "C", "C"],
        "C6\nAm7/C\n",
    );
    assert_output(
        cmd_identify::run,
        &["Bb", "Db", "Fb", "Abb"],
        "Bbdim7\nDbdim7/Bb\nFbdim7/Bb\nAbbdim7/Bb\n",
    );
}

#[test]
fn identify_returns_one_without_output_when_no_chord_matches() {
    assert_empty_result(cmd_identify::run, &["C", "D"], 1);
    assert_empty_result(cmd_identify::run, &["C"], 1);
}

#[test]
fn identify_observes_option_errors_bounds_and_deferred_last_value_wins() {
    assert_error(cmd_identify::run, &["--limit"], "--limit needs a value");
    assert_error(
        cmd_identify::run,
        &["--limit=0", "--limit"],
        "--limit needs a value",
    );
    assert_error(
        cmd_identify::run,
        &["--limit", "--help"],
        "invalid --limit \"--help\" (expected an integer from 1 to 100)",
    );
    assert_error(
        cmd_identify::run,
        &["--limit=0"],
        "invalid --limit \"0\" (expected an integer from 1 to 100)",
    );
    assert_error(
        cmd_identify::run,
        &["--limit=00"],
        "invalid --limit \"00\" (expected an integer from 1 to 100)",
    );
    assert_error(
        cmd_identify::run,
        &["--limit=101"],
        "invalid --limit \"101\" (expected an integer from 1 to 100)",
    );
    assert_error(
        cmd_identify::run,
        &["--wat", "C"],
        "unknown option \"--wat\"",
    );
    assert_error(
        cmd_identify::run,
        &["--help", "C"],
        "unknown option \"--help\"",
    );
    assert_error(
        cmd_identify::run,
        &["--limit=0", "X"],
        "invalid --limit \"0\" (expected an integer from 1 to 100)",
    );
}

#[test]
fn identify_reports_missing_stdin_note_and_invalid_note_without_partial_output() {
    assert_error(cmd_identify::run, &[], "missing NOTE");
    let empty = MemoryIo::new(b" \t\r\n");
    assert_eq!(
        cmd_identify::run(&strings(&["-"]), &empty),
        Err(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(empty.reads.get(), 1);
    assert!(empty.outs.borrow().is_empty());
    assert!(empty.errs.borrow().is_empty());

    assert_error(cmd_identify::run, &["C", "nø"], "invalid note \"n?\"");
}

#[test]
fn identify_expands_a_single_dash_with_bom_crlf_and_strict_utf8() {
    let stdin = MemoryIo::new(b"\xEF\xBB\xBFC E\r\nG A\r\n");
    assert_eq!(cmd_identify::run(&strings(&["-"]), &stdin), Ok(0));
    assert_eq!(&*stdin.outs.borrow(), &["C6\nAm7/C\n".to_owned()]);
    assert!(stdin.errs.borrow().is_empty());
    assert_eq!(stdin.reads.get(), 1);

    let bad_utf8 = MemoryIo::new(&[0x43, 0x20, 0xFF, 0x0A]);
    assert_eq!(
        cmd_identify::run(&strings(&["-"]), &bad_utf8),
        Err(TheoryError("<stdin>: input is not valid UTF-8".to_owned()))
    );
    assert!(bad_utf8.outs.borrow().is_empty());
    assert!(bad_utf8.errs.borrow().is_empty());
    assert_eq!(bad_utf8.reads.get(), 1);

    let mixed_dash = MemoryIo::new(b"C E G A");
    assert_eq!(
        cmd_identify::run(&strings(&["C", "-"]), &mixed_dash),
        Err(TheoryError("invalid note \"-\"".to_owned()))
    );
    assert_eq!(mixed_dash.reads.get(), 0);
}

#[test]
fn identify_and_keys_enforce_literal_twelve_and_thirteen_note_boundaries() {
    let twelve_notes = MemoryIo::new(b"C C# D D# E F F# G G# A A# B");
    assert_eq!(cmd_identify::run(&strings(&["-"]), &twelve_notes), Ok(1));
    assert!(twelve_notes.outs.borrow().is_empty());
    assert_eq!(twelve_notes.reads.get(), 1);

    let thirteen_notes = MemoryIo::new(b"X C# D D# E F F# G G# A A# B C");
    assert_eq!(
        cmd_identify::run(&strings(&["-"]), &thirteen_notes),
        Err(TheoryError("at most 12 notes".to_owned()))
    );
    assert!(thirteen_notes.outs.borrow().is_empty());
    assert_eq!(thirteen_notes.reads.get(), 1);

    let twelve_keys = MemoryIo::new(b"C C# D D# E F F# G G# A A# B");
    assert_eq!(cmd_keys::run(&strings(&["-"]), &twelve_keys), Ok(1));
    assert!(twelve_keys.outs.borrow().is_empty());
    assert_eq!(twelve_keys.reads.get(), 1);

    let thirteen_keys = MemoryIo::new(b"X C# D D# E F F# G G# A A# B C");
    assert_eq!(
        cmd_keys::run(&strings(&["-"]), &thirteen_keys),
        Err(TheoryError("at most 12 notes".to_owned()))
    );
    assert!(thirteen_keys.outs.borrow().is_empty());
    assert_eq!(thirteen_keys.reads.get(), 1);

    assert_error(cmd_keys::run, &["X"], "invalid note \"X\"");
}

#[test]
fn keys_prints_the_requested_matches_and_ordered_limit() {
    assert_output(
        cmd_keys::run,
        &["Bb", "D", "F", "A"],
        "F major (1 flat)\nD minor (1 flat)\nBb major (2 flats)\nG minor (2 flats)\n",
    );
    assert_output(
        cmd_keys::run,
        &["--limit", "4", "F#", "A#", "C#", "E#"],
        "Db major (5 flats)\nBb minor (5 flats)\nF# major (6 sharps)\nGb major (6 flats)\n",
    );
    assert_empty_result(cmd_keys::run, &["C", "C#", "D", "D#", "E", "F"], 1);
}

#[test]
fn keys_reports_limit_missing_unknown_and_out_of_range_errors() {
    assert_error(cmd_keys::run, &[], "missing NOTE");
    assert_error(cmd_keys::run, &["--limit"], "--limit needs a value");
    assert_error(
        cmd_keys::run,
        &["--limit=31", "C"],
        "invalid --limit \"31\" (expected an integer from 1 to 30)",
    );
    assert_error(
        cmd_keys::run,
        &["--limit=0", "C"],
        "invalid --limit \"0\" (expected an integer from 1 to 30)",
    );
    assert_error(
        cmd_keys::run,
        &["--limit=bad", "--wat"],
        "unknown option \"--wat\"",
    );
    assert_error(
        cmd_keys::run,
        &["--limit", "0", "C"],
        "invalid --limit \"0\" (expected an integer from 1 to 30)",
    );
}

#[test]
fn transpose_moves_chords_up_and_down_including_the_bass() {
    assert_output(
        cmd_transpose::run,
        &["M2", "C", "Am", "F", "G7"],
        "D Bm G A7\n",
    );
    assert_output(cmd_transpose::run, &["-m3", "Cmaj7/E"], "Amaj7/C#\n");
    assert_output(cmd_transpose::run, &["d1", "C"], "Cb\n");
}

#[test]
fn transpose_checks_missing_input_invalid_interval_and_first_chord_errors() {
    assert_error(cmd_transpose::run, &[], "missing INTERVAL");
    assert_error(cmd_transpose::run, &["M2"], "missing SYMBOL");
    assert_error(cmd_transpose::run, &["M5", "C"], "invalid interval \"M5\"");
    assert_error(
        cmd_transpose::run,
        &["M7", "G##"],
        "cannot spell G## up by M7",
    );
    assert_error(
        cmd_transpose::run,
        &["M7", "G##", "not-a-chord"],
        "cannot spell G## up by M7",
    );
    assert_error(
        cmd_transpose::run,
        &["M2", "C", "not-a-chord"],
        "invalid chord \"not-a-chord\"",
    );
    assert_error(
        cmd_transpose::run,
        &["--limit=2"],
        "unknown option \"--limit=2\"",
    );
}

#[test]
fn transpose_expands_one_hundred_chords_and_rejects_one_hundred_one_before_interval_parse() {
    let hundred = MemoryIo::new(ONE_HUNDRED_CHORDS);
    assert_eq!(cmd_transpose::run(&strings(&["M2", "-"]), &hundred), Ok(0));
    assert_eq!(
        &*hundred.outs.borrow(),
        &[ONE_HUNDRED_TRANSPOSED.to_owned()]
    );
    assert!(hundred.errs.borrow().is_empty());
    assert_eq!(hundred.reads.get(), 1);

    let one_hundred_one = MemoryIo::new(ONE_HUNDRED_ONE_CHORDS);
    assert_eq!(
        cmd_transpose::run(&strings(&["M99", "-"]), &one_hundred_one),
        Err(TheoryError("at most 100 chords".to_owned()))
    );
    assert!(one_hundred_one.outs.borrow().is_empty());
    assert!(one_hundred_one.errs.borrow().is_empty());
    assert_eq!(one_hundred_one.reads.get(), 1);
}

#[test]
fn analyze_prints_exact_major_and_minor_tables() {
    assert_output(
        cmd_analyze::run,
        &["C", "C", "Am", "F", "G7", "Bb", "F#dim7", "C/E"],
        "chord   numeral\nC       I\nAm      vi\nF       IV\nG7      V7\nBb      bVII\nF#dim7  #ivdim7\nC/E     I/3\n",
    );
    assert_output(
        cmd_analyze::run,
        &["Am", "Am", "Dm", "E7", "G", "Bbmaj7/D"],
        "chord     numeral\nAm        i\nDm        iv\nE7        V7\nG         VII\nBbmaj7/D  bIImaj7/4\n",
    );
    assert_output(
        cmd_analyze::run,
        &["Bbm", "Db", "Gb7", "Cb/Eb"],
        "chord  numeral\nDb     III\nGb7    VI7\nCb/Eb  bII/4\n",
    );
}

#[test]
fn analyze_validates_key_before_chords_and_chord_spelling_before_later_symbols() {
    assert_error(cmd_analyze::run, &[], "missing KEY");
    assert_error(cmd_analyze::run, &["C"], "missing SYMBOL");
    assert_error(
        cmd_analyze::run,
        &["Cm", "bad-key"],
        "invalid chord \"bad-key\"",
    );
    assert_error(
        cmd_analyze::run,
        &["not-a-key", "C"],
        "invalid key \"not-a-key\"",
    );
    assert_error(
        cmd_analyze::run,
        &["C", "G##maj7", "not-a-chord"],
        "cannot spell G## up by 7",
    );
    assert_error(
        cmd_analyze::run,
        &["C", "G##maj7"],
        "cannot spell G## up by 7",
    );
    assert_error(
        cmd_analyze::run,
        &["--limit=2"],
        "unknown option \"--limit=2\"",
    );
}

#[test]
fn analyze_expands_one_hundred_chords_and_rejects_one_hundred_one_before_key_parse() {
    let hundred = MemoryIo::new(ONE_HUNDRED_CHORDS);
    assert_eq!(cmd_analyze::run(&strings(&["C", "-"]), &hundred), Ok(0));
    assert_eq!(
        &*hundred.outs.borrow(),
        &[ONE_HUNDRED_ANALYSIS_ROWS.to_owned()]
    );
    assert!(hundred.errs.borrow().is_empty());
    assert_eq!(hundred.reads.get(), 1);

    let one_hundred_one = MemoryIo::new(ONE_HUNDRED_ONE_CHORDS);
    assert_eq!(
        cmd_analyze::run(&strings(&["not-a-key", "-"]), &one_hundred_one),
        Err(TheoryError("at most 100 chords".to_owned()))
    );
    assert!(one_hundred_one.outs.borrow().is_empty());
    assert!(one_hundred_one.errs.borrow().is_empty());
    assert_eq!(one_hundred_one.reads.get(), 1);
}

#[test]
fn identify_honors_the_minimum_and_maximum_limit_values() {
    assert_output(
        cmd_identify::run,
        &["--limit=1", "C", "Eb", "Gb", "A"],
        "Cdim7\n",
    );
    assert_output(
        cmd_identify::run,
        &["--limit=100", "C", "Eb", "Gb", "A"],
        "Cdim7\nEbdim7/C\nGbdim7/C\nAdim7/C\n",
    );
}

#[test]
fn keys_limit_one_keeps_the_first_sorted_key() {
    assert_output(
        cmd_keys::run,
        &["--limit=1", "F#", "A#", "C#", "E#"],
        "Db major (5 flats)\n",
    );
}

#[test]
fn scale_rejects_options_before_missing_arguments() {
    assert_error(cmd_scale::run, &["--limit"], "unknown option \"--limit\"");
    assert_error(
        cmd_scale::run,
        &["X", "unknown", "extra"],
        "too many arguments",
    );
}

#[test]
fn transpose_reads_the_chord_list_before_parsing_its_interval() {
    let empty_stdin = MemoryIo::new(b" \t\r\n");
    assert_eq!(
        cmd_transpose::run(&strings(&["M5", "-"]), &empty_stdin),
        Err(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(empty_stdin.reads.get(), 1);
    assert!(empty_stdin.outs.borrow().is_empty());
    assert!(empty_stdin.errs.borrow().is_empty());
}

#[test]
fn analyze_reads_only_the_list_after_the_key_and_before_key_parsing() {
    let empty_stdin = MemoryIo::new(b"");
    assert_eq!(
        cmd_analyze::run(&strings(&["not-a-key", "-"]), &empty_stdin),
        Err(TheoryError("<stdin>: no input".to_owned()))
    );
    assert_eq!(empty_stdin.reads.get(), 1);
    assert!(empty_stdin.outs.borrow().is_empty());
    assert!(empty_stdin.errs.borrow().is_empty());
}

#[test]
fn keys_accepts_the_inclusive_limit_of_thirty_and_keeps_full_order() {
    assert_output(
        cmd_keys::run,
        &["--limit=30", "F#", "A#", "C#", "E#"],
        "Db major (5 flats)\nBb minor (5 flats)\nF# major (6 sharps)\nGb major (6 flats)\nD# minor (6 sharps)\nEb minor (6 flats)\nC# major (7 sharps)\nA# minor (7 sharps)\n",
    );
}

#[test]
fn analyze_accepts_a_single_chord_and_renders_the_header() {
    assert_output(cmd_analyze::run, &["C", "C"], "chord  numeral\nC      I\n");
}
