use crate::chord::Chord;
use crate::error::TheoryError;
use crate::interval::Interval;
use crate::io::{self, Io};
use crate::text;
use crate::transpose;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = crate::options::parse(argv, &[])?;
    let args = parsed.args;

    if args.is_empty() {
        return Err(TheoryError("missing INTERVAL".to_owned()));
    }
    if args.len() == 1 {
        return Err(TheoryError("missing SYMBOL".to_owned()));
    }

    let words = io::words(io, &args[1..])?;
    if words.len() > 100 {
        return Err(TheoryError("at most 100 chords".to_owned()));
    }

    let (interval, down) = Interval::signed(&args[0])
        .ok_or_else(|| TheoryError(format!("invalid interval {}", text::show(&args[0]))))?;

    let mut names = Vec::with_capacity(words.len());
    for word in &words {
        let chord = Chord::parse(word)?;
        let transposed = transpose::transpose(&chord, &interval, down)?;
        names.push(transposed.name());
    }

    io.out(&format!("{}\n", names.join(" ")));
    Ok(0)
}
