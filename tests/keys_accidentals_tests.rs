use std::cell::RefCell;

use tonemath::cli;
use tonemath::io::Io;

struct MemoryIo {
    stdin: Vec<u8>,
    outs: RefCell<Vec<String>>,
    errs: RefCell<Vec<String>>,
}

impl MemoryIo {
    fn new(stdin: &str) -> Self {
        Self {
            stdin: stdin.as_bytes().to_vec(),
            outs: RefCell::new(Vec::new()),
            errs: RefCell::new(Vec::new()),
        }
    }
}

impl Io for MemoryIo {
    fn read_stdin(&self) -> Vec<u8> {
        self.stdin.clone()
    }

    fn out(&self, text: &str) {
        self.outs.borrow_mut().push(text.to_owned());
    }

    fn err(&self, text: &str) {
        self.errs.borrow_mut().push(text.to_owned());
    }
}

fn keys(args: &[&str], stdin: &str) -> (i32, String, String) {
    let io = MemoryIo::new(stdin);
    let mut argv = vec!["keys".to_owned()];
    argv.extend(args.iter().map(|argument| (*argument).to_owned()));
    let status = cli::run(&argv, &io);
    let out = io.outs.borrow().concat();
    let err = io.errs.borrow().concat();
    (status, out, err)
}

#[test]
fn the_default_keeps_every_key() {
    let all = keys(&["C"], "");
    assert_eq!(all.0, 0);
    assert_eq!(keys(&["--max-accidentals", "7", "C"], ""), all);
}

#[test]
fn zero_keeps_only_the_keys_without_accidentals() {
    let (status, out, err) = keys(
        &["--max-accidentals", "0", "C", "D", "E", "F", "G", "A", "B"],
        "",
    );
    assert_eq!(status, 0);
    assert_eq!(out, "C major (no accidentals)\nA minor (no accidentals)\n");
    assert_eq!(err, "");
}

#[test]
fn one_keeps_keys_with_at_most_one_sharp_or_flat() {
    let (status, out, err) = keys(&["--max-accidentals", "1", "C", "E", "G"], "");
    assert_eq!(status, 0);
    assert_eq!(
        out,
        "C major (no accidentals)\nA minor (no accidentals)\nG major (1 sharp)\nF major (1 flat)\nE minor (1 sharp)\nD minor (1 flat)\n"
    );
    assert_eq!(err, "");
}

#[test]
fn flat_keys_count_like_sharp_keys() {
    let (status, out, _) = keys(&["--max-accidentals", "1", "Bb", "D", "F", "A"], "");
    assert_eq!(status, 0);
    assert_eq!(out, "F major (1 flat)\nD minor (1 flat)\n");
}

#[test]
fn a_filter_that_leaves_no_key_is_the_no_match_result() {
    let (status, out, err) = keys(&["--max-accidentals", "0", "C#"], "");
    assert_eq!(status, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

#[test]
fn the_limit_applies_after_the_filter() {
    let (status, out, _) = keys(&["--max-accidentals", "1", "--limit", "3", "C"], "");
    assert_eq!(status, 0);
    assert_eq!(
        out,
        "C major (no accidentals)\nA minor (no accidentals)\nG major (1 sharp)\n"
    );
}

#[test]
fn the_equals_form_and_the_last_value_win() {
    let (status, out, _) = keys(
        &["--max-accidentals=0", "--max-accidentals=1", "C", "E", "G"],
        "",
    );
    assert_eq!(status, 0);
    assert!(out.ends_with("D minor (1 flat)\n"));
    assert_eq!(out.lines().count(), 6);
}

#[test]
fn notes_can_come_from_standard_input() {
    let (status, out, _) = keys(&["--max-accidentals", "0", "-"], "C D E\nF G A B\n");
    assert_eq!(status, 0);
    assert_eq!(out, "C major (no accidentals)\nA minor (no accidentals)\n");
}

#[test]
fn values_outside_zero_to_seven_are_errors() {
    for value in ["8", "-1", "01", "x", "", "1.5"] {
        let (status, out, err) = keys(&["--max-accidentals", value, "C"], "");
        assert_eq!(status, 2, "value {value:?}");
        assert_eq!(out, "");
        assert_eq!(
            err,
            format!(
                "tonemath: invalid --max-accidentals \"{value}\" (expected an integer from 0 to 7)\n"
            ),
            "value {value:?}"
        );
    }
}

#[test]
fn a_missing_value_is_an_error() {
    let (status, out, err) = keys(&["C", "--max-accidentals"], "");
    assert_eq!(status, 2);
    assert_eq!(out, "");
    assert_eq!(err, "tonemath: --max-accidentals needs a value\n");
}

#[test]
fn other_commands_do_not_know_the_option() {
    let io = MemoryIo::new("");
    let argv = ["identify", "--max-accidentals", "1", "C", "E", "G"].map(str::to_owned);
    assert_eq!(cli::run(&argv, &io), 2);
    assert!(io.outs.borrow().is_empty());
    assert_eq!(
        io.errs.borrow().concat(),
        "tonemath: unknown option \"--max-accidentals\"\n"
    );
}
