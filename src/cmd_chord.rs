use crate::chord::Chord;
use crate::error::TheoryError;
use crate::io::Io;
use crate::note::Note;
use crate::options;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = options::parse(argv, &[])?;
    let args = parsed.args;

    let symbol = args
        .first()
        .ok_or_else(|| TheoryError("missing SYMBOL".to_owned()))?;
    if args.len() > 1 {
        return Err(TheoryError("too many arguments".to_owned()));
    }

    let chord = Chord::parse(symbol)?;
    let mut notes = chord.notes()?;
    let mut degrees = chord.tokens();
    if let Some(bass) = chord.bass
        && let Some(start) = notes.iter().position(|note| note.pc() == bass.pc())
    {
        notes.rotate_left(start);
        degrees.rotate_left(start);
    }

    let output = [
        format!("chord: {}", chord.name()),
        format!(
            "notes: {}",
            notes.iter().map(Note::name).collect::<Vec<_>>().join(" ")
        ),
        format!("degrees: {}", degrees.join(" ")),
    ]
    .join("\n")
        + "\n";
    io.out(&output);
    Ok(0)
}
