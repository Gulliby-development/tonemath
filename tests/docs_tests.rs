use std::cell::RefCell;

use tonemath::cli;
use tonemath::io::Io;

const README: &str = include_str!("../README.md");

struct MemoryIo {
    outs: RefCell<Vec<String>>,
    errs: RefCell<Vec<String>>,
}

impl MemoryIo {
    fn new() -> Self {
        Self {
            outs: RefCell::new(Vec::new()),
            errs: RefCell::new(Vec::new()),
        }
    }
}

impl Io for MemoryIo {
    fn read_stdin(&self) -> Vec<u8> {
        Vec::new()
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

fn extract_text_block_after(marker: &str) -> String {
    let normalized_readme = README.replace("\r\n", "\n");
    let marker_count = normalized_readme.match_indices(marker).count();
    assert_eq!(
        marker_count, 1,
        "README marker must occur exactly once: {marker}"
    );

    let marker_end = normalized_readme.find(marker).unwrap() + marker.len();
    let after_marker = &normalized_readme[marker_end..];
    let opening = after_marker
        .find("```text\n")
        .expect("README marker must be followed by a text code block");
    assert!(
        after_marker[..opening].trim().is_empty(),
        "the marked README example must be the next content block"
    );

    let block_start = opening + "```text\n".len();
    let after_opening = &after_marker[block_start..];
    let closing = after_opening
        .find("\n```")
        .expect("README text code block must have a closing fence");
    let block = after_opening[..closing].to_owned();
    let complete_fence = format!("```text\n{block}\n```");
    assert_eq!(
        normalized_readme.matches(&complete_fence).count(),
        1,
        "README example code block must be unambiguous"
    );
    block
}

#[test]
fn readme_usage_block_matches_contract_and_cli_help() {
    let usage = extract_text_block_after(
        "Run `cargo run --release -- --help` to print the command synopsis:",
    );
    let expected_usage = concat!(
        "usage: tonemath scale ROOT NAME\n",
        "       tonemath chord SYMBOL\n",
        "       tonemath interval A B\n",
        "       tonemath identify [--limit N] NOTE...\n",
        "       tonemath keys [--limit N] NOTE...\n",
        "       tonemath transpose INTERVAL SYMBOL...\n",
        "       tonemath analyze KEY SYMBOL..."
    );
    assert_eq!(usage, expected_usage);

    let io = MemoryIo::new();
    assert_eq!(cli::run(&strings(&["--help"]), &io), 0);
    assert_eq!(&*io.outs.borrow(), &[format!("{usage}\n")]);
    assert!(io.errs.borrow().is_empty());
}

#[test]
fn readme_scale_block_matches_contract_and_cli_output() {
    let scale_output =
        extract_text_block_after("For example, `cargo run --release -- scale C major` prints:");
    let expected_scale_output = concat!(
        "scale: C major\n",
        "notes: C D E F G A B\n",
        "degrees: 1 2 3 4 5 6 7\n",
        "steps: 2 2 1 2 2 2 1"
    );
    assert_eq!(scale_output, expected_scale_output);

    let io = MemoryIo::new();
    assert_eq!(cli::run(&strings(&["scale", "C", "major"]), &io), 0);
    assert_eq!(&*io.outs.borrow(), &[format!("{scale_output}\n")]);
    assert!(io.errs.borrow().is_empty());
}
