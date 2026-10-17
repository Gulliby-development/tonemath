use crate::error::TheoryError;
use crate::note::{NATURAL, Note};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub steps: u32,
    pub semis: i32,
    pub label: String,
}

const PERFECT_QUALITIES: [(&str, i32); 5] = [("dd", -2), ("AA", 2), ("d", -1), ("P", 0), ("A", 1)];
const MAJOR_QUALITIES: [(&str, i32); 6] = [
    ("dd", -3),
    ("AA", 2),
    ("d", -2),
    ("m", -1),
    ("M", 0),
    ("A", 1),
];

impl Interval {
    pub fn parse(text: &str) -> Option<Interval> {
        let bytes = text.as_bytes();

        for (quality, _) in PERFECT_QUALITIES.iter().chain(MAJOR_QUALITIES.iter()) {
            if !bytes.starts_with(quality.as_bytes()) {
                continue;
            }

            let number = parse_number(&bytes[quality.len()..])?;
            let perfect = is_perfect(number);
            let offset = quality_offset(quality, perfect)?;
            return Some(make_interval(number, offset, text));
        }

        None
    }

    pub fn degree(text: &str) -> Option<Interval> {
        let bytes = text.as_bytes();
        let first = *bytes.first()?;
        let accidental = match first {
            b'#' | b'b' => first,
            _ => 0,
        };
        let mut prefix_len = 0;
        if accidental != 0 {
            while bytes.get(prefix_len) == Some(&accidental) {
                prefix_len += 1;
            }
            if prefix_len > 2 {
                return None;
            }
            if bytes
                .get(prefix_len)
                .is_some_and(|next| *next == b'#' || *next == b'b')
            {
                return None;
            }
        }

        let number = parse_number(&bytes[prefix_len..])?;
        let offset = match (accidental, prefix_len) {
            (b'#', 1) => 1,
            (b'#', 2) => 2,
            (b'b', 1) => -1,
            (b'b', 2) => -2,
            (0, 0) => 0,
            _ => return None,
        };
        Some(make_interval(number, offset, text))
    }

    pub fn signed(text: &str) -> Option<(Interval, bool)> {
        if let Some(name) = text.strip_prefix('-') {
            Self::parse(name).map(|interval| (interval, true))
        } else {
            Self::parse(text).map(|interval| (interval, false))
        }
    }
}

pub fn signed(text: &str) -> Option<(Interval, bool)> {
    Interval::signed(text)
}

pub fn up(note: &Note, interval: &Interval) -> Result<Note, TheoryError> {
    spell_directionally(note, interval, false)
}

pub fn down(note: &Note, interval: &Interval) -> Result<Note, TheoryError> {
    spell_directionally(note, interval, true)
}

pub fn name_between(from: &Note, to: &Note) -> Option<(String, i32)> {
    if from.letter >= 7 || to.letter >= 7 {
        return None;
    }

    let steps = (u32::from(to.letter) + 7 - u32::from(from.letter)) % 7;
    let semis = (to.pc() - from.pc()).rem_euclid(12);
    let offset = normalize_accidental(semis - NATURAL[steps as usize]);
    let mut number = steps + 1;
    if steps == 0 && offset < 0 {
        number = 8;
    }
    let quality = quality_for_offset(number, offset)?;
    Some((format!("{quality}{number}"), semis))
}

fn parse_number(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() || bytes.len() > 2 || bytes[0] == b'0' {
        return None;
    }

    let mut number = 0_u32;
    for byte in bytes {
        if !byte.is_ascii_digit() {
            return None;
        }
        number = number * 10 + u32::from(*byte - b'0');
    }

    (1..=15).contains(&number).then_some(number)
}

fn is_perfect(number: u32) -> bool {
    matches!(number, 1 | 4 | 5 | 8 | 11 | 12 | 15)
}

fn quality_offset(quality: &str, perfect: bool) -> Option<i32> {
    let qualities = if perfect {
        &PERFECT_QUALITIES[..]
    } else {
        &MAJOR_QUALITIES[..]
    };
    qualities
        .iter()
        .find_map(|(name, offset)| (*name == quality).then_some(*offset))
}

fn quality_for_offset(number: u32, offset: i32) -> Option<&'static str> {
    let qualities = if is_perfect(number) {
        &PERFECT_QUALITIES[..]
    } else {
        &MAJOR_QUALITIES[..]
    };
    qualities
        .iter()
        .find_map(|(quality, value)| (*value == offset).then_some(*quality))
}

fn make_interval(number: u32, offset: i32, label: &str) -> Interval {
    let zero_based = number - 1;
    let simple = (zero_based % 7) as usize;
    let octaves = zero_based / 7;
    let semis = NATURAL[simple] + (octaves as i32 * 12) + offset;
    Interval {
        steps: zero_based,
        semis,
        label: label.to_owned(),
    }
}

fn spell_directionally(
    note: &Note,
    interval: &Interval,
    is_down: bool,
) -> Result<Note, TheoryError> {
    let direction = if is_down { "down" } else { "up" };
    if note.letter >= 7 {
        return Err(cannot_spell(note, interval, direction));
    }

    let step_remainder = interval.steps % 7;
    let target_letter = if is_down {
        (u32::from(note.letter) + 7 - step_remainder) % 7
    } else {
        (u32::from(note.letter) + step_remainder) % 7
    } as u8;

    let signed_semis = if is_down {
        -i64::from(interval.semis)
    } else {
        i64::from(interval.semis)
    };
    let target_pc = (i64::from(note.pc()) + signed_semis).rem_euclid(12) as i32;
    let accidental = normalize_accidental(target_pc - NATURAL[usize::from(target_letter)]);
    if !(-2..=2).contains(&accidental) {
        return Err(cannot_spell(note, interval, direction));
    }

    Ok(Note {
        letter: target_letter,
        acc: accidental as i8,
    })
}

fn normalize_accidental(difference: i32) -> i32 {
    let normalized = difference.rem_euclid(12);
    if normalized > 5 {
        normalized - 12
    } else {
        normalized
    }
}

fn cannot_spell(note: &Note, interval: &Interval, direction: &str) -> TheoryError {
    TheoryError(format!(
        "cannot spell {} {direction} by {}",
        note.name(),
        interval.label
    ))
}
