use crate::error::TheoryError;
use crate::interval::{self, Interval};
use crate::note::Note;
use crate::text;

pub const QUALITIES: [(&str, &str); 19] = [
    ("", "1 3 5"),
    ("m", "1 b3 5"),
    ("dim", "1 b3 b5"),
    ("aug", "1 3 #5"),
    ("sus2", "1 2 5"),
    ("sus4", "1 4 5"),
    ("5", "1 5"),
    ("6", "1 3 5 6"),
    ("m6", "1 b3 5 6"),
    ("7", "1 3 5 b7"),
    ("maj7", "1 3 5 7"),
    ("m7", "1 b3 5 b7"),
    ("mmaj7", "1 b3 5 7"),
    ("m7b5", "1 b3 b5 b7"),
    ("dim7", "1 b3 b5 bb7"),
    ("add9", "1 3 5 9"),
    ("9", "1 3 5 b7 9"),
    ("maj9", "1 3 5 7 9"),
    ("m9", "1 b3 5 b7 9"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub quality: String,
    pub bass: Option<Note>,
}

impl Chord {
    pub fn parse(text: &str) -> Result<Chord, TheoryError> {
        let bytes = text.as_bytes();
        let Some(first) = bytes.first().copied() else {
            return Err(invalid_chord(text));
        };
        if !matches!(first, b'A'..=b'G') {
            return Err(invalid_chord(text));
        }

        let accidental = bytes
            .get(1)
            .copied()
            .filter(|byte| matches!(byte, b'#' | b'b'));
        let accidental_count = match accidental {
            Some(mark) if bytes.get(2) == Some(&mark) => 2,
            Some(_) => 1,
            None => 0,
        };
        let root_end = 1 + accidental_count;

        let mut slash = None;
        for (index, byte) in bytes.iter().enumerate() {
            if *byte == b'/' {
                if slash.is_some() {
                    return Err(invalid_chord(text));
                }
                slash = Some(index);
            }
        }

        let (head_end, bass_text) = match slash {
            Some(index) => (index, Some(&text[index + 1..])),
            None => (text.len(), None),
        };
        if root_end > head_end {
            return Err(invalid_chord(text));
        }

        let Some(root) = Note::parse(&text[..root_end]) else {
            return Err(invalid_chord(text));
        };
        let quality = &text[root_end..head_end];
        let Some((_, degrees)) = quality_degrees(quality) else {
            return Err(invalid_chord(text));
        };

        let bass = match bass_text {
            Some(value) => {
                let Some(note) = Note::parse(value) else {
                    return Err(invalid_chord(text));
                };
                Some(note)
            }
            None => None,
        };

        if let Some(bass_note) = bass {
            let mut is_chord_note = false;
            for token in degrees.split_ascii_whitespace() {
                let Some(degree) = Interval::degree(token) else {
                    return Err(invalid_chord(text));
                };
                let pitch_class = (root.pc() + degree.semis).rem_euclid(12);
                if pitch_class == bass_note.pc() {
                    is_chord_note = true;
                    break;
                }
            }
            if !is_chord_note {
                return Err(TheoryError(format!(
                    "invalid chord {} (the bass is not a chord note)",
                    text::show(text)
                )));
            }
        }

        Ok(Chord {
            root,
            quality: quality.to_owned(),
            bass,
        })
    }

    pub fn name(&self) -> String {
        let mut name = format!("{}{}", self.root.name(), self.quality);
        if let Some(bass) = self.bass {
            name.push('/');
            name.push_str(&bass.name());
        }
        name
    }

    pub fn tokens(&self) -> Vec<&'static str> {
        let Some((_, degrees)) = quality_degrees(&self.quality) else {
            return Vec::new();
        };
        degrees.split_ascii_whitespace().collect()
    }

    pub fn notes(&self) -> Result<Vec<Note>, TheoryError> {
        if quality_degrees(&self.quality).is_none() {
            return Err(TheoryError(format!(
                "unknown chord quality {}",
                text::show(&self.quality)
            )));
        }

        self.tokens()
            .into_iter()
            .map(|token| {
                let degree = Interval::degree(token)
                    .ok_or_else(|| TheoryError(format!("invalid chord degree {token}")))?;
                interval::up(&self.root, &degree)
            })
            .collect()
    }
}

fn quality_degrees(quality: &str) -> Option<(&'static str, &'static str)> {
    QUALITIES
        .iter()
        .find(|(symbol, _)| *symbol == quality)
        .copied()
}

fn invalid_chord(text: &str) -> TheoryError {
    TheoryError(format!("invalid chord {}", text::show(text)))
}
