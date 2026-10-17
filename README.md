<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# tonemath

tonemath is an offline command-line music theory calculator for musicians, teachers, and songwriters. It spells scales and chords, names intervals, finds matching chords and keys, transposes progressions, and analyzes chords as Roman numerals. It works with note names and integer semitone relationships; it does not model octaves, frequencies, or audio.

[![CI](https://github.com/Gulliby-development/tonemath/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/tonemath/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Install

A Rust toolchain is required. From the project directory, build the optimized executable with:

```text
cargo build --release
```

The executable is `target/release/tonemath`, with the platform's executable suffix where required (for example, `.exe` on Windows). You can also build and run a command in one step with `cargo run --release -- COMMAND`, replacing `COMMAND` with a tonemath command and its arguments.

## Usage

Run `cargo run --release -- --help` to print the command synopsis:

```text
usage: tonemath scale ROOT NAME
       tonemath chord SYMBOL
       tonemath interval A B
       tonemath identify [--limit N] NOTE...
       tonemath keys [--limit N] NOTE...
       tonemath transpose INTERVAL SYMBOL...
       tonemath analyze KEY SYMBOL...
```

For example, `cargo run --release -- scale C major` prints:

```text
scale: C major
notes: C D E F G A B
degrees: 1 2 3 4 5 6 7
steps: 2 2 1 2 2 2 1
```

More examples (run with `cargo run --release -- ...` or the built executable):

```text
$ tonemath chord Cmaj7/E
chord: Cmaj7/E
notes: E G B C
degrees: 3 5 7 1

$ tonemath interval C Cb
C to Cb: d8 (11 semitones)

$ tonemath identify C Eb Gb A
Cdim7
Ebdim7/C
Gbdim7/C
Adim7/C

$ tonemath keys Bb D F A
F major (1 flat)
D minor (1 flat)
Bb major (2 flats)
G minor (2 flats)

$ tonemath transpose -m3 Cmaj7/E
Amaj7/C#

$ tonemath analyze Am Am Dm E7 G Bbmaj7/D
chord     numeral
Am        i
Dm        iv
E7        V7
G         VII
Bbmaj7/D  bIImaj7/4
```

## Notes and intervals

Notes use an uppercase letter from A through G, optionally followed by one or two sharps (`#` or `##`) or flats (`b` or `bb`). Tonemath preserves diatonic spelling, so enharmonic notes can have different names. Interval names combine a quality and a number, such as `M3`, `P5`, or `m2`; prefix an interval with `-` to move downward, as in `-m3`.

For list commands, provide entries as arguments or use a single `-` in place of the list to read it from standard input. Standard input is strict UTF-8; one leading byte-order mark is ignored, and spaces, tabs, carriage returns, and line feeds separate entries. The list limit counts supplied entries, including duplicates: `identify` and `keys` accept at most 12 notes, while `transpose` and `analyze` accept at most 100 chord symbols. The `identify` `--limit` option accepts 1 through 100 and defaults to 100; `keys` accepts 1 through 30 and defaults to 30.

## Commands

`scale ROOT NAME` spells one of the supported scales from a note: `major`, `minor`, `harmonic-minor`, `melodic-minor`, `dorian`, `phrygian`, `lydian`, `mixolydian`, `locrian`, `major-pentatonic`, `minor-pentatonic`, `blues`, or `whole-tone`. It reports the spelled notes, scale degrees, and semitone steps.

`chord SYMBOL` parses a chord and lists its notes and degrees. Omit the quality for a major triad; other supported quality symbols are `m`, `dim`, `aug`, `sus2`, `sus4`, `5`, `6`, `m6`, `7`, `maj7`, `m7`, `mmaj7`, `m7b5`, `dim7`, `add9`, `9`, `maj9`, and `m9`. Add `/BASS` to specify an inversion, such as `Cmaj7/E`; the bass must be a chord tone.

`interval A B` names the interval from note A up to note B and reports its semitone distance.

`identify NOTE...` lists chord names matching the supplied set of notes, including tied matches. Use `--limit N` to cap the results; its range and default are described above.

`keys NOTE...` lists major and minor keys containing every supplied note. Use `--limit N` to cap the results; its range and default are described above.

`transpose INTERVAL SYMBOL...` transposes each chord by the named interval. Prefix the interval with `-` to transpose downward; chord inversions move with their chords.

`analyze KEY SYMBOL...` writes a table of chord names and their Roman numerals in the requested major or minor key. A minor key name ends in `m`, as in `Bbm`.

## Exit codes

The program returns 0 on success. It returns 1 only when `identify` or `keys` finds no matches; in that case, it writes no result lines. It returns 2 for command, option, input, or spelling errors, reported on standard error.

## Development

Run these checks from the project directory:

```text
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## License

MIT, see [LICENSE](LICENSE).

---

Built with BAKED
