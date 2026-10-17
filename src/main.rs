use std::io::{Read, Write};

use tonemath::cli;
use tonemath::io::Io;

struct StdioIo;

impl Io for StdioIo {
    fn read_stdin(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let _ = std::io::stdin().read_to_end(&mut bytes);
        bytes
    }

    fn out(&self, text: &str) {
        let mut stdout = std::io::stdout().lock();
        let _ = stdout.write_all(text.as_bytes());
        let _ = stdout.flush();
    }

    fn err(&self, text: &str) {
        let mut stderr = std::io::stderr().lock();
        let _ = stderr.write_all(text.as_bytes());
        let _ = stderr.flush();
    }
}

fn main() {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();
    std::process::exit(cli::run(&args, &StdioIo));
}
