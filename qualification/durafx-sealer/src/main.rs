//! Seals or verifies one explicitly declared qualification evidence tree.

mod arguments;
mod manifest;
mod seal;
mod storage;
mod verify;

use std::io::{Read, Write};
use std::process::ExitCode;

const MAX_REQUEST_BYTES: usize = 16_384;
const READ_LIMIT_BYTES: u64 = 16_385;

fn main() -> ExitCode {
    match read_request().and_then(|request| execute(&request)) {
        Ok(message) => write_success(&message),
        Err(refusal) => write_refusal(&refusal),
    }
}

fn read_request() -> Result<Vec<u8>, String> {
    let mut request = Vec::new();
    std::io::stdin()
        .take(READ_LIMIT_BYTES)
        .read_to_end(&mut request)
        .map_err(|error| format!("cannot read request from standard input: {error}"))?;
    if request.len() > MAX_REQUEST_BYTES {
        return Err(format!(
            "request exceeds the {MAX_REQUEST_BYTES}-byte input limit"
        ));
    }
    Ok(request)
}

fn execute(request: &[u8]) -> Result<String, String> {
    match arguments::parse(request)? {
        arguments::Command::Seal(declaration) => seal::run(&declaration).and_then(path_message),
        arguments::Command::Verify { repository, run } => {
            verify::run(&repository, &run)?;
            let run_text = run
                .to_str()
                .ok_or_else(|| "verified run path is not UTF-8".to_owned())?;
            Ok(format!("verified {run_text}"))
        }
    }
}

fn path_message(path: std::path::PathBuf) -> Result<String, String> {
    path.into_os_string()
        .into_string()
        .map_err(|_| "sealed run path is not UTF-8".to_owned())
}

fn write_success(message: &str) -> ExitCode {
    if writeln!(std::io::stdout(), "{message}").is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn write_refusal(refusal: &str) -> ExitCode {
    if writeln!(std::io::stderr(), "error: {refusal}").is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::FAILURE
}
