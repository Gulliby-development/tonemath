use crate::error::TheoryError;
use crate::interval;
use crate::io::Io;
use crate::note::Note;
use crate::options;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = options::parse(argv, &[])?;
    let args = parsed.args;

    let from_text = args
        .first()
        .ok_or_else(|| TheoryError("missing A".to_owned()))?;
    let to_text = args
        .get(1)
        .ok_or_else(|| TheoryError("missing B".to_owned()))?;
    if args.len() > 2 {
        return Err(TheoryError("too many arguments".to_owned()));
    }

    let from = Note::parse(from_text)
        .ok_or_else(|| TheoryError(format!("invalid note {}", text::show(from_text))))?;
    let to = Note::parse(to_text)
        .ok_or_else(|| TheoryError(format!("invalid note {}", text::show(to_text))))?;
    let (name, semitones) = interval::name_between(&from, &to).ok_or_else(|| {
        TheoryError(format!(
            "no name for the interval from {from_text} to {to_text}"
        ))
    })?;

    let distance = text::plural(semitones as usize, "semitone", "semitones");
    io.out(&format!("{from_text} to {to_text}: {name} ({distance})\n"));
    Ok(0)
}
