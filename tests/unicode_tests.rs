#![cfg(feature = "unicode")]

use std::cell::RefCell;
use std::collections::HashSet;

use tonemath::cli;
use tonemath::io::Io;
use tonemath::note::Note;

struct MemoryIo {
    outs: RefCell<Vec<String>>,
}

impl MemoryIo {
    fn new() -> Self {
        Self {
            outs: RefCell::new(Vec::new()),
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

    fn err(&self, _text: &str) {}
}

fn run_cli(args: &[&str]) -> (i32, String) {
    let io = MemoryIo::new();
    let args: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
    let exit_code = cli::run(&args, &io);
    let output = io.outs.borrow().concat();
    (exit_code, output)
}

#[test]
fn note_names_use_the_music_symbols() {
    let examples = [
        ("C", "C"),
        ("F#", "F\u{266F}"),
        ("Bb", "B\u{266D}"),
        ("C##", "C\u{266F}\u{266F}"),
        ("Ebb", "E\u{266D}\u{266D}"),
    ];
    for (spelling, expected) in examples {
        let note = Note::parse(spelling).expect("example spelling must parse");
        assert_eq!(note.name(), expected, "input spelling: {spelling}");
    }

    let letters = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    for (letter_index, letter) in letters.into_iter().enumerate() {
        for accidental in i8::MIN..=i8::MAX {
            let note = Note {
                letter: letter_index as u8,
                acc: accidental,
            };
            let symbol = if accidental < 0 {
                '\u{266D}'
            } else {
                '\u{266F}'
            };
            let repeated_symbols = symbol
                .to_string()
                .repeat(usize::from(accidental.unsigned_abs()));
            let expected = format!("{letter}{repeated_symbols}");
            let input_accidentals = if accidental < 0 {
                "b".repeat(usize::from(accidental.unsigned_abs()))
            } else {
                "#".repeat(usize::from(accidental.unsigned_abs()))
            };
            let input = format!("{letter}{input_accidentals}");
            assert_eq!(
                note.name(),
                expected,
                "failed note-name property for input {input:?} (accidental count {accidental})"
            );
        }
    }
}

#[test]
fn scale_prints_symbols_and_still_reads_ascii() {
    let (exit_code, output) = run_cli(&["scale", "F#", "major"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "scale: F# major\n",
            "notes: F\u{266F} G\u{266F} A\u{266F} B C\u{266F} D\u{266F} E\u{266F}\n",
            "degrees: 1 2 3 4 5 6 7\n",
            "steps: 2 2 1 2 2 2 1\n"
        )
    );

    let (exit_code, output) = run_cli(&["scale", "Bb", "major"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "scale: Bb major\n",
            "notes: B\u{266D} C D E\u{266D} F G A\n",
            "degrees: 1 2 3 4 5 6 7\n",
            "steps: 2 2 1 2 2 2 1\n"
        )
    );
}

#[test]
fn chord_with_a_flat_bass() {
    let (exit_code, output) = run_cli(&["chord", "Bbmaj7/D"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "chord: B\u{266D}maj7/D\n",
            "notes: D F A B\u{266D}\n",
            "degrees: 3 5 7 1\n"
        )
    );

    let (exit_code, output) = run_cli(&["chord", "C##7"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "chord: C\u{266F}\u{266F}7\n",
            "notes: C\u{266F}\u{266F} E\u{266F}\u{266F} G\u{266F}\u{266F} B\u{266F}\n",
            "degrees: 1 3 5 b7\n"
        )
    );

    let (exit_code, output) = run_cli(&["identify", "C", "Eb", "Gb", "A"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "Cdim7\n",
            "E\u{266D}dim7/C\n",
            "G\u{266D}dim7/C\n",
            "Adim7/C\n"
        )
    );

    let (exit_code, output) = run_cli(&["keys", "Bb", "D", "F", "A"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "F major (1 flat)\n",
            "D minor (1 flat)\n",
            "B\u{266D} major (2 flats)\n",
            "G minor (2 flats)\n"
        )
    );

    let (exit_code, output) = run_cli(&["analyze", "Bbm", "Bbm", "Ebm", "F7"]);
    assert_eq!(exit_code, 0);
    assert_eq!(
        output,
        concat!(
            "chord  numeral\n",
            "B\u{266D}m    i\n",
            "E\u{266D}m    iv\n",
            "F7     V7\n"
        )
    );

    let (exit_code, output) = run_cli(&["analyze", "Am", "Bbmaj7/D"]);
    assert_eq!(exit_code, 0);
    assert!(
        output.contains("bIImaj7/4"),
        "Roman numeral must remain ASCII: {output:?}"
    );
}

#[test]
fn transpose_keeps_the_symbols_in_each_chord() {
    let (exit_code, output) = run_cli(&["transpose", "-m3", "Cmaj7/E"]);
    assert_eq!(exit_code, 0);
    assert_eq!(output, "Amaj7/C\u{266F}\n");
}

#[test]
fn the_symbols_are_not_accepted_as_input() {
    let (exit_code, output) = run_cli(&["chord", "C\u{266F}"]);
    assert_eq!(exit_code, 2);
    assert!(output.is_empty(), "invalid input must not produce stdout");

    let (exit_code, output) = run_cli(&["interval", "C", "Cb"]);
    assert_eq!(exit_code, 0);
    assert_eq!(output, "C to Cb: d8 (11 semitones)\n");

    let mut seed = 0x9E37_79B9_7F4A_7C15_u64;
    let mut spellings = HashSet::new();
    let letters = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    for repetitions in 1..=128 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let letter = letters[(seed as usize) % letters.len()];
        let musical_sign = if seed & 1 == 0 {
            '\u{266F}'
        } else {
            '\u{266D}'
        };
        let ascii_sign = match repetitions % 4 {
            0 => "#",
            1 => "b",
            2 => "",
            _ => "##",
        };
        let spelling = format!(
            "{letter}{ascii_sign}{}",
            musical_sign.to_string().repeat(repetitions)
        );
        assert!(
            spellings.insert(spelling.clone()),
            "generator produced a duplicate spelling: {spelling:?}"
        );
        let (exit_code, _) = run_cli(&["chord", &spelling]);
        assert_eq!(
            exit_code, 2,
            "musical-sign input must be rejected: {spelling:?}"
        );
    }
    assert_eq!(spellings.len(), 128, "generator must cover distinct inputs");
}
