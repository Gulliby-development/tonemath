use crate::analyze;
use crate::chord::Chord;
use crate::error::TheoryError;
use crate::io::{self, Io};
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = crate::options::parse(argv, &[])?;
    let args = parsed.args;

    if args.is_empty() {
        return Err(TheoryError("missing KEY".to_owned()));
    }
    if args.len() == 1 {
        return Err(TheoryError("missing SYMBOL".to_owned()));
    }

    let words = io::words(io, &args[1..])?;
    if words.len() > 100 {
        return Err(TheoryError("at most 100 chords".to_owned()));
    }

    let (tonic, minor) = analyze::parse_key(&args[0])
        .ok_or_else(|| TheoryError(format!("invalid key {}", text::show(&args[0]))))?;

    let mut rows = Vec::with_capacity(words.len());
    for word in &words {
        let chord = Chord::parse(word)?;
        chord.notes()?;
        rows.push(vec![chord.name(), analyze::numeral(&tonic, minor, &chord)]);
    }

    let lines = text::render_table(&["chord", "numeral"], &rows);
    io.out(&format!("{}\n", lines.join("\n")));
    Ok(0)
}
