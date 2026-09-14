//! Shared subprocess byte transport, without an archive reader or semantic judge.

use std::error::Error;
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

const PREFIX: &str = "MACROONZ_ARCHIVED_RECORD=";

pub(crate) fn publish(encoded: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut line = String::from(PREFIX);
    for byte in encoded {
        write!(&mut line, "{byte:02x}")?;
    }
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(line.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

pub(crate) fn round_trip(encoded: &[u8], child_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new(std::env::current_exe()?)
        .args(["--ignored", "--exact", child_name, "--nocapture"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("missing child stdin"))?;
    stdin.write_all(encoded)?;
    drop(stdin);
    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "archive child failed: {}",
            String::from_utf8_lossy(&output.stderr),
        ))
        .into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    let hex = stdout
        .lines()
        .find_map(|line| line.split_once(PREFIX).map(|(_, hex)| hex))
        .ok_or_else(|| std::io::Error::other("missing archive output"))?;
    let (pairs, remainder) = hex.as_bytes().as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(std::io::Error::other("partial hex byte").into());
    }
    pairs
        .iter()
        .map(|pair| Ok(u8::from_str_radix(core::str::from_utf8(pair)?, 16)?))
        .collect()
}
