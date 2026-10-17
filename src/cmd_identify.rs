use crate::error::TheoryError;
use crate::identify;
use crate::io::{self, Io};
use crate::note::Note;
use crate::options::{self, Spec};
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = options::parse(
        argv,
        &[Spec {
            name: "--limit",
            lo: 1,
            hi: 100,
            fallback: 100,
        }],
    )?;
    let args = parsed.args;

    if args.is_empty() {
        return Err(TheoryError("missing NOTE".to_owned()));
    }

    let words = io::words(io, &args)?;
    if words.len() > 12 {
        return Err(TheoryError("at most 12 notes".to_owned()));
    }

    let mut notes = Vec::with_capacity(words.len());
    for word in &words {
        let note = Note::parse(word)
            .ok_or_else(|| TheoryError(format!("invalid note {}", text::show(word))))?;
        notes.push(note);
    }

    let limit = parsed.values.get("--limit").copied().unwrap_or(100) as usize;
    let matches = identify::identify(&notes);
    if matches.is_empty() {
        return Ok(1);
    }

    let output = matches
        .iter()
        .take(limit)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    io.out(&format!("{output}\n"));
    Ok(0)
}
