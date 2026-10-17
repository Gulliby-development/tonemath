use std::cell::{Cell, RefCell};
use std::io::Write;
use std::process::{Command, Stdio};

use tonemath::cli::{self, USAGE};
use tonemath::io::Io;

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

fn run(args: &[&str], io: &MemoryIo) -> i32 {
    cli::run(&strings(args), io)
}

fn assert_success(args: &[&str], expected: &str) {
    let io = MemoryIo::new(&[]);
    assert_eq!(run(args, &io), 0);
    assert_eq!(&*io.outs.borrow(), &[expected.to_owned()]);
    assert!(io.errs.borrow().is_empty());
    assert_eq!(io.reads.get(), 0);
}

fn assert_error(args: &[&str], message: &str) {
    let io = MemoryIo::new(&[]);
    assert_eq!(run(args, &io), 2);
    assert!(io.outs.borrow().is_empty());
    assert_eq!(&*io.errs.borrow(), &[format!("tonemath: {message}\n")]);
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn usage_is_exact_and_sole_help_writes_it_once() {
    let expected = "usage: tonemath scale ROOT NAME\n       tonemath chord SYMBOL\n       tonemath interval A B\n       tonemath identify [--limit N] NOTE...\n       tonemath keys [--limit N] NOTE...\n       tonemath transpose INTERVAL SYMBOL...\n       tonemath analyze KEY SYMBOL...";
    assert_eq!(USAGE, expected);

    let io = MemoryIo::new(&[]);
    assert_eq!(run(&["--help"], &io), 0);
    assert_eq!(&*io.outs.borrow(), &[format!("{expected}\n")]);
    assert!(io.errs.borrow().is_empty());
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn no_arguments_reports_the_complete_missing_command_error() {
    assert_error(
        &[],
        "missing command (scale, chord, interval, identify, keys, transpose or analyze)",
    );
}

#[test]
fn unknown_command_sanitizes_unicode_and_control_text() {
    assert_error(&["møde\0"], "unknown command \"m?de?\"");
}

#[test]
fn cli_dispatches_all_seven_commands_and_preserves_exact_output() {
    assert_success(
        &["scale", "C", "major"],
        "scale: C major\nnotes: C D E F G A B\ndegrees: 1 2 3 4 5 6 7\nsteps: 2 2 1 2 2 2 1\n",
    );
    assert_success(
        &["chord", "Bbdim7"],
        "chord: Bbdim7\nnotes: Bb Db Fb Abb\ndegrees: 1 b3 b5 bb7\n",
    );
    assert_success(&["interval", "C", "C#"], "C to C#: A1 (1 semitone)\n");
    assert_success(&["identify", "C", "E", "G", "A"], "C6\nAm7/C\n");
    assert_success(
        &["keys", "Bb", "D", "F", "A"],
        "F major (1 flat)\nD minor (1 flat)\nBb major (2 flats)\nG minor (2 flats)\n",
    );
    assert_success(&["transpose", "M2", "C", "Am"], "D Bm\n");
    assert_success(
        &["analyze", "C", "C", "Am"],
        "chord  numeral\nC      I\nAm     vi\n",
    );
}

#[test]
fn empty_identification_and_key_results_return_one_without_stream_calls() {
    let identify = MemoryIo::new(&[]);
    assert_eq!(run(&["identify", "C", "D"], &identify), 1);
    assert!(identify.outs.borrow().is_empty());
    assert!(identify.errs.borrow().is_empty());

    let keys = MemoryIo::new(&[]);
    assert_eq!(run(&["keys", "C", "C#", "D", "D#", "E", "F"], &keys), 1);
    assert!(keys.outs.borrow().is_empty());
    assert!(keys.errs.borrow().is_empty());
}

#[test]
fn help_is_special_only_as_the_entire_argument_list() {
    assert_error(&["--help", "scale"], "unknown command \"--help\"");
    assert_error(&["scale", "--help"], "unknown option \"--help\"");
    assert_error(&["identify", "--help", "C"], "unknown option \"--help\"");
}

#[test]
fn options_are_checked_before_missing_arguments_and_later_unknown_options() {
    assert_error(
        &["identify", "--limit=0"],
        "invalid --limit \"0\" (expected an integer from 1 to 100)",
    );
    assert_error(
        &["keys", "--limit=31"],
        "invalid --limit \"31\" (expected an integer from 1 to 30)",
    );
    assert_error(
        &["identify", "--limit=bad", "--wat"],
        "unknown option \"--wat\"",
    );
    assert_error(
        &["identify", "--limit", "--help", "C"],
        "invalid --limit \"--help\" (expected an integer from 1 to 100)",
    );
    assert_error(&["scale", "--limit=2"], "unknown option \"--limit=2\"");
}

#[test]
fn fixed_arity_commands_check_missing_arguments_then_excess_arguments() {
    assert_error(&["scale"], "missing ROOT");
    assert_error(&["scale", "C"], "missing NAME");
    assert_error(&["scale", "X", "bad", "extra"], "too many arguments");
    assert_error(&["chord"], "missing SYMBOL");
    assert_error(&["chord", "C", "extra"], "too many arguments");
    assert_error(&["interval"], "missing A");
    assert_error(&["interval", "C"], "missing B");
    assert_error(&["interval", "C", "D", "E"], "too many arguments");
}

#[test]
fn scale_errors_are_reported_through_the_cli_without_partial_output() {
    assert_error(&["scale", "X", "major"], "invalid note \"X\"");
    assert_error(
        &["scale", "C", "unknown"],
        "unknown scale \"unknown\" (expected major, minor, harmonic-minor, melodic-minor, dorian, phrygian, lydian, mixolydian, locrian, major-pentatonic, minor-pentatonic, blues or whole-tone)",
    );
    assert_error(&["scale", "G##", "major"], "cannot spell G## up by 7");
}

#[test]
fn interval_errors_are_reported_in_note_order() {
    assert_error(&["interval", "X", "D"], "invalid note \"X\"");
    assert_error(&["interval", "C", "X"], "invalid note \"X\"");
    assert_error(
        &["interval", "C##", "Dbb"],
        "no name for the interval from C## to Dbb",
    );
}

#[test]
fn chord_grammar_bass_and_spelling_errors_reach_standard_error_once() {
    assert_error(
        &["chord", "C/D"],
        "invalid chord \"C/D\" (the bass is not a chord note)",
    );
    assert_error(&["chord", "C###"], "invalid chord \"C###\"");
    assert_error(&["chord", "G##maj7"], "cannot spell G## up by 7");
}

#[test]
fn list_commands_report_missing_and_invalid_notes_with_sanitization() {
    assert_error(&["identify"], "missing NOTE");
    assert_error(&["keys"], "missing NOTE");
    assert_error(&["identify", "nø"], "invalid note \"n?\"");
    assert_error(&["keys", "X"], "invalid note \"X\"");
}

#[test]
fn list_count_errors_precede_note_parsing_at_thirteen_words() {
    assert_error(
        &[
            "identify", "X", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C",
        ],
        "at most 12 notes",
    );
    assert_error(
        &[
            "keys", "X", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C", "C",
        ],
        "at most 12 notes",
    );
}

#[test]
fn transpose_and_analyze_validate_missing_arguments_and_primary_inputs() {
    assert_error(&["transpose"], "missing INTERVAL");
    assert_error(&["transpose", "M2"], "missing SYMBOL");
    assert_error(&["transpose", "M5", "C"], "invalid interval \"M5\"");
    assert_error(
        &["transpose", "M2", "not-a-chord"],
        "invalid chord \"not-a-chord\"",
    );
    assert_error(&["analyze"], "missing KEY");
    assert_error(&["analyze", "C"], "missing SYMBOL");
    assert_error(&["analyze", "not-a-key", "C"], "invalid key \"not-a-key\"");
    assert_error(
        &["analyze", "C", "not-a-chord"],
        "invalid chord \"not-a-chord\"",
    );
}

#[test]
fn chord_list_expansion_and_count_precede_interval_or_key_parsing() {
    let empty_transpose_stdin = MemoryIo::new(b" \t\r\n");
    assert_eq!(run(&["transpose", "M5", "-"], &empty_transpose_stdin), 2);
    assert!(empty_transpose_stdin.outs.borrow().is_empty());
    assert_eq!(
        &*empty_transpose_stdin.errs.borrow(),
        &["tonemath: <stdin>: no input\n".to_owned()]
    );
    assert_eq!(empty_transpose_stdin.reads.get(), 1);

    let empty_analyze_stdin = MemoryIo::new(b"");
    assert_eq!(run(&["analyze", "not-a-key", "-"], &empty_analyze_stdin), 2);
    assert_eq!(
        &*empty_analyze_stdin.errs.borrow(),
        &["tonemath: <stdin>: no input\n".to_owned()]
    );
    assert_eq!(empty_analyze_stdin.reads.get(), 1);
}

#[test]
fn stdin_accepts_one_leading_bom_and_crlf_and_reads_once() {
    let io = MemoryIo::new(b"\xEF\xBB\xBFC E\r\nG A\r\n");
    assert_eq!(run(&["identify", "-"], &io), 0);
    assert_eq!(&*io.outs.borrow(), &["C6\nAm7/C\n".to_owned()]);
    assert!(io.errs.borrow().is_empty());
    assert_eq!(io.reads.get(), 1);
}

#[test]
fn stdin_rejects_exact_malformed_utf8_bytes_without_partial_output() {
    let io = MemoryIo::new(&[0x43, 0x20, 0xFF, 0x0A]);
    assert_eq!(run(&["identify", "-"], &io), 2);
    assert!(io.outs.borrow().is_empty());
    assert_eq!(
        &*io.errs.borrow(),
        &["tonemath: <stdin>: input is not valid UTF-8\n".to_owned()]
    );
    assert_eq!(io.reads.get(), 1);
}

#[test]
fn stdin_empty_and_separator_only_inputs_report_no_input() {
    let empty = MemoryIo::new(&[]);
    assert_eq!(run(&["identify", "-"], &empty), 2);
    assert_eq!(
        &*empty.errs.borrow(),
        &["tonemath: <stdin>: no input\n".to_owned()]
    );
    assert_eq!(empty.reads.get(), 1);

    let separators = MemoryIo::new(b" \t\r\n");
    assert_eq!(run(&["keys", "-"], &separators), 2);
    assert_eq!(
        &*separators.errs.borrow(),
        &["tonemath: <stdin>: no input\n".to_owned()]
    );
    assert_eq!(separators.reads.get(), 1);
}

#[test]
fn stdin_splits_only_ascii_separators_and_removes_only_one_bom() {
    let non_breaking_space = MemoryIo::new("C\u{00a0}E".as_bytes());
    assert_eq!(run(&["identify", "-"], &non_breaking_space), 2);
    assert_eq!(
        &*non_breaking_space.errs.borrow(),
        &["tonemath: invalid note \"C?E\"\n".to_owned()]
    );
    assert_eq!(non_breaking_space.reads.get(), 1);

    let nul = MemoryIo::new(b"C\0E");
    assert_eq!(run(&["identify", "-"], &nul), 2);
    assert_eq!(
        &*nul.errs.borrow(),
        &["tonemath: invalid note \"C?E\"\n".to_owned()]
    );
    assert_eq!(nul.reads.get(), 1);

    let double_bom = MemoryIo::new(b"\xEF\xBB\xBF\xEF\xBB\xBFC E");
    assert_eq!(run(&["identify", "-"], &double_bom), 2);
    assert_eq!(
        &*double_bom.errs.borrow(),
        &["tonemath: invalid note \"?C\"\n".to_owned()]
    );
    assert_eq!(double_bom.reads.get(), 1);
}

#[test]
fn a_dash_mixed_with_note_arguments_is_not_stdin_expansion() {
    let io = MemoryIo::new(b"C E G A");
    assert_eq!(run(&["identify", "C", "-"], &io), 2);
    assert_eq!(
        &*io.errs.borrow(),
        &["tonemath: invalid note \"-\"\n".to_owned()]
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn only_the_malformed_utf8_contract_starts_the_program() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tonemath"))
        .args(["identify", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(&[0x43, 0x20, 0xFF, 0x0A]).unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        output.stderr,
        b"tonemath: <stdin>: input is not valid UTF-8\n"
    );
}
