use crate::chord::Chord;
use crate::note::{NATURAL, Note};

/// Parses a tonic note name with an optional natural-minor suffix.
pub fn parse_key(text: &str) -> Option<(Note, bool)> {
    let (tonic_text, minor) = match text.strip_suffix('m') {
        Some(tonic_text) => (tonic_text, true),
        None => (text, false),
    };
    Note::parse(tonic_text).map(|tonic| (tonic, minor))
}

/// Formats a chord as a roman numeral relative to a major or natural-minor key.
pub fn numeral(tonic: &Note, minor: bool, chord: &Chord) -> String {
    const ROMAN: [&str; 7] = ["I", "II", "III", "IV", "V", "VI", "VII"];
    const MINOR: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

    let (root_degree, root_alteration) = degree(tonic, &chord.root, minor, &NATURAL, &MINOR);
    let is_minor_quality = chord.tokens().contains(&"b3");
    let roman = if is_minor_quality {
        ROMAN[root_degree].to_ascii_lowercase()
    } else {
        ROMAN[root_degree].to_owned()
    };

    let mut result = alteration_text(root_alteration);
    result.push_str(&roman);
    result.push_str(quality_suffix(&chord.quality));

    if let Some(bass) = chord.bass {
        let (bass_degree, bass_alteration) = degree(tonic, &bass, minor, &NATURAL, &MINOR);
        result.push('/');
        result.push_str(&alteration_text(bass_alteration));
        result.push_str(&(bass_degree + 1).to_string());
    }

    result
}

fn degree(
    tonic: &Note,
    note: &Note,
    minor: bool,
    major_sizes: &[i32; 7],
    minor_sizes: &[i32; 7],
) -> (usize, i32) {
    // Public Note fields permit malformed letters; reduce them before indexing so
    // numeral remains panic-free even for values outside the documented range.
    let degree = (i32::from(note.letter) - i32::from(tonic.letter)).rem_euclid(7) as usize;
    let semitones = (note.pc() - tonic.pc()).rem_euclid(12);
    let expected = if minor {
        minor_sizes[degree]
    } else {
        major_sizes[degree]
    };
    (degree, normalize_alteration(semitones - expected))
}

fn normalize_alteration(alteration: i32) -> i32 {
    let normalized = alteration.rem_euclid(12);
    if normalized > 5 {
        normalized - 12
    } else {
        normalized
    }
}

fn alteration_text(alteration: i32) -> String {
    if alteration < 0 {
        "b".repeat(alteration.unsigned_abs() as usize)
    } else {
        "#".repeat(alteration as usize)
    }
}

fn quality_suffix(quality: &str) -> &str {
    match quality {
        "" | "m" => "",
        "m6" => "6",
        "m7" => "7",
        "mmaj7" => "maj7",
        "m7b5" => "7b5",
        "m9" => "9",
        symbol => symbol,
    }
}
