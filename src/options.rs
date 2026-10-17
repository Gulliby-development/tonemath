use std::collections::BTreeMap;

use crate::error::TheoryError;
use crate::text::show;

pub struct Spec {
    pub name: &'static str,
    pub lo: i64,
    pub hi: i64,
    pub fallback: i64,
}

pub struct Parsed {
    pub values: BTreeMap<String, i64>,
    pub args: Vec<String>,
}

pub fn parse(argv: &[String], specs: &[Spec]) -> Result<Parsed, TheoryError> {
    let mut args = Vec::new();
    let mut provided = BTreeMap::new();
    let mut index = 0;

    while index < argv.len() {
        let argument = &argv[index];
        if !argument.starts_with("--") {
            args.push(argument.clone());
            index += 1;
            continue;
        }

        let (name, value) = match argument.split_once('=') {
            Some((name, value)) => (name, Some(value.to_owned())),
            None => (argument.as_str(), None),
        };

        if !specs.iter().any(|spec| spec.name == name) {
            return Err(TheoryError(format!("unknown option {}", show(argument))));
        }

        let value = match value {
            Some(value) => value,
            None => {
                index += 1;
                match argv.get(index) {
                    Some(value) => value.clone(),
                    None => {
                        return Err(TheoryError(format!("{name} needs a value")));
                    }
                }
            }
        };
        provided.insert(name.to_owned(), value);
        index += 1;
    }

    let mut values = BTreeMap::new();
    for spec in specs {
        let value = match provided.get(spec.name) {
            Some(value) => {
                if !valid_integer(value) {
                    return Err(invalid_value(spec, value));
                }
                let parsed = value
                    .parse::<i64>()
                    .map_err(|_| invalid_value(spec, value))?;
                if !(spec.lo..=spec.hi).contains(&parsed) {
                    return Err(invalid_value(spec, value));
                }
                parsed
            }
            None => spec.fallback,
        };
        values.insert(spec.name.to_owned(), value);
    }

    Ok(Parsed { values, args })
}

fn valid_integer(value: &str) -> bool {
    if value == "0" {
        return true;
    }

    let bytes = value.as_bytes();
    (1..=6).contains(&bytes.len())
        && bytes[0].is_ascii_digit()
        && bytes[0] != b'0'
        && bytes.iter().all(u8::is_ascii_digit)
}

fn invalid_value(spec: &Spec, value: &str) -> TheoryError {
    TheoryError(format!(
        "invalid {} {} (expected an integer from {} to {})",
        spec.name,
        show(value),
        spec.lo,
        spec.hi
    ))
}
