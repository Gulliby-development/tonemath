use crate::error::TheoryError;
use crate::interval::Interval;
use crate::io::Io;
use crate::note::Note;
use crate::options;
use crate::scale::{self, SCALES};
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, TheoryError> {
    let parsed = options::parse(argv, &[])?;
    let args = parsed.args;

    let root_text = args
        .first()
        .ok_or_else(|| TheoryError("missing ROOT".to_owned()))?;
    let name = args
        .get(1)
        .ok_or_else(|| TheoryError("missing NAME".to_owned()))?;
    if args.len() > 2 {
        return Err(TheoryError("too many arguments".to_owned()));
    }

    let root = Note::parse(root_text)
        .ok_or_else(|| TheoryError(format!("invalid note {}", text::show(root_text))))?;
    let notes = scale::spell(&root, name)?;
    let Some((_, degree_text)) = SCALES.iter().find(|(scale_name, _)| *scale_name == name) else {
        return Err(TheoryError(format!("unknown scale {}", text::show(name))));
    };

    let degrees: Vec<&str> = degree_text.split_ascii_whitespace().collect();
    let sizes = degrees
        .iter()
        .map(|token| {
            Interval::degree(token)
                .map(|degree| degree.semis)
                .ok_or_else(|| TheoryError(format!("invalid scale degree {token}")))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let steps = sizes
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .chain(std::iter::once(12 - sizes.last().copied().unwrap_or(12)))
        .map(|step| step.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let output = [
        format!("scale: {root_text} {name}"),
        format!(
            "notes: {}",
            notes.iter().map(Note::name).collect::<Vec<_>>().join(" ")
        ),
        format!("degrees: {}", degrees.join(" ")),
        format!("steps: {steps}"),
    ]
    .join("\n")
        + "\n";
    io.out(&output);
    Ok(0)
}
