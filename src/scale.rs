use crate::error::TheoryError;
use crate::interval::{self, Interval};
use crate::note::Note;
use crate::text;

pub const SCALES: [(&str, &str); 13] = [
    ("major", "1 2 3 4 5 6 7"),
    ("minor", "1 2 b3 4 5 b6 b7"),
    ("harmonic-minor", "1 2 b3 4 5 b6 7"),
    ("melodic-minor", "1 2 b3 4 5 6 7"),
    ("dorian", "1 2 b3 4 5 6 b7"),
    ("phrygian", "1 b2 b3 4 5 b6 b7"),
    ("lydian", "1 2 3 #4 5 6 7"),
    ("mixolydian", "1 2 3 4 5 6 b7"),
    ("locrian", "1 b2 b3 4 b5 b6 b7"),
    ("major-pentatonic", "1 2 3 5 6"),
    ("minor-pentatonic", "1 b3 4 5 b7"),
    ("blues", "1 b3 4 b5 5 b7"),
    ("whole-tone", "1 2 3 #4 #5 b7"),
];

pub fn spell(root: &Note, name: &str) -> Result<Vec<Note>, TheoryError> {
    let Some((_, degrees)) = SCALES.iter().find(|(scale_name, _)| *scale_name == name) else {
        let names = SCALES
            .iter()
            .map(|(scale_name, _)| (*scale_name).to_owned())
            .collect::<Vec<_>>();
        return Err(TheoryError(format!(
            "unknown scale {} (expected {})",
            text::show(name),
            text::or_list(&names)
        )));
    };

    degrees
        .split_ascii_whitespace()
        .map(|token| {
            let degree =
                Interval::degree(token).expect("scale table contains only valid degree tokens");
            interval::up(root, &degree)
        })
        .collect()
}
