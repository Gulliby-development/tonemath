use crate::error::TheoryError;
use crate::io::Io;
use crate::text;

pub const USAGE: &str = "usage: tonemath scale ROOT NAME
       tonemath chord SYMBOL
       tonemath interval A B
       tonemath identify [--limit N] NOTE...
       tonemath keys [--limit N] NOTE...
       tonemath transpose INTERVAL SYMBOL...
       tonemath analyze KEY SYMBOL...";

pub fn run(args: &[String], io: &dyn Io) -> i32 {
    if args.is_empty() {
        return report_error(
            TheoryError(
                "missing command (scale, chord, interval, identify, keys, transpose or analyze)"
                    .to_owned(),
            ),
            io,
        );
    }

    if args.len() == 1 && args[0] == "--help" {
        io.out(&format!("{USAGE}\n"));
        return 0;
    }

    let command = &args[0];
    let remaining = &args[1..];
    let result = match command.as_str() {
        "scale" => crate::cmd_scale::run(remaining, io),
        "chord" => crate::cmd_chord::run(remaining, io),
        "interval" => crate::cmd_interval::run(remaining, io),
        "identify" => crate::cmd_identify::run(remaining, io),
        "keys" => crate::cmd_keys::run(remaining, io),
        "transpose" => crate::cmd_transpose::run(remaining, io),
        "analyze" => crate::cmd_analyze::run(remaining, io),
        _ => {
            return report_error(
                TheoryError(format!("unknown command {}", text::show(command))),
                io,
            );
        }
    };

    match result {
        Ok(status) => status,
        Err(error) => report_error(error, io),
    }
}

fn report_error(error: TheoryError, io: &dyn Io) -> i32 {
    io.err(&format!("tonemath: {}\n", error.0));
    2
}
