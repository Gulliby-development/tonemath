use crate::chord::Chord;
use crate::error::TheoryError;
use crate::interval::{self, Interval};

/// Transposes a chord's root and optional bass without changing its quality.
pub fn transpose(chord: &Chord, iv: &Interval, down: bool) -> Result<Chord, TheoryError> {
    let root = move_note(&chord.root, iv, down)?;
    let bass = chord
        .bass
        .map(|note| move_note(&note, iv, down))
        .transpose()?;

    Ok(Chord {
        root,
        quality: chord.quality.clone(),
        bass,
    })
}

fn move_note(
    note: &crate::note::Note,
    iv: &Interval,
    down: bool,
) -> Result<crate::note::Note, TheoryError> {
    if down {
        interval::down(note, iv)
    } else {
        interval::up(note, iv)
    }
}
