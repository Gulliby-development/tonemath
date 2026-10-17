use crate::note::Note;
use crate::scale;
use crate::text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Key {
    pub tonic: Note,
    pub minor: bool,
    pub accidentals: i32,
}

impl Key {
    /// Formats this key signature without a trailing line feed.
    pub fn line(&self) -> String {
        let mode = if self.minor { "minor" } else { "major" };
        let count = self.accidentals.unsigned_abs() as usize;
        let description = if count == 0 {
            "no accidentals".to_owned()
        } else if self.accidentals < 0 {
            text::plural(count, "flat", "flats")
        } else {
            text::plural(count, "sharp", "sharps")
        };

        format!("{} {mode} ({description})", self.tonic.name())
    }
}

/// Returns every catalogue key containing all the input pitch classes.
pub fn matching(notes: &[Note]) -> Vec<Key> {
    let mut matches = catalogue()
        .into_iter()
        .filter_map(|(tonic, minor)| {
            let scale_name = if minor { "minor" } else { "major" };
            let spelled = scale::spell(&tonic, scale_name)
                .expect("catalogue keys must have spellable scales");

            let contains_all = notes.iter().all(|note| {
                spelled
                    .iter()
                    .any(|scale_note| scale_note.pc() == note.pc())
            });
            if !contains_all {
                return None;
            }

            let accidentals = spelled
                .iter()
                .map(|note| i32::from(note.acc.signum()))
                .sum();

            Some(Key {
                tonic,
                minor,
                accidentals,
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by_key(|key| {
        let sign_order = if key.accidentals > 0 { 0 } else { 1 };
        (key.accidentals.unsigned_abs(), key.minor, sign_order)
    });
    matches
}

fn catalogue() -> Vec<(Note, bool)> {
    const MAJOR_TONICS: [&str; 15] = [
        "C", "G", "D", "A", "E", "B", "F#", "C#", "F", "Bb", "Eb", "Ab", "Db", "Gb", "Cb",
    ];
    const MINOR_TONICS: [&str; 15] = [
        "A", "E", "B", "F#", "C#", "G#", "D#", "A#", "D", "G", "C", "F", "Bb", "Eb", "Ab",
    ];

    let mut keys = Vec::with_capacity(30);
    keys.extend(MAJOR_TONICS.into_iter().map(|tonic| {
        (
            Note::parse(tonic).expect("catalogue contains valid tonic notes"),
            false,
        )
    }));
    keys.extend(MINOR_TONICS.into_iter().map(|tonic| {
        (
            Note::parse(tonic).expect("catalogue contains valid tonic notes"),
            true,
        )
    }));
    keys
}
