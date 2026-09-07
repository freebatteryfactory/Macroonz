//! Fresh test processes load and return retained bytes without executing or reducing a subject.

use super::{
    LIMITS, fixture,
    vector::{InputKind, Vector},
};
use macroonz_harness::report::archive::{read_capsule, retain_capsule};
use std::error::Error;
use std::fmt::Write as _;
use std::io::{Read as _, Write as _};
use std::process::{Command, Stdio};

const PREFIX: &str = "MACROONZ_ARCHIVED_CAPSULE=";

#[test]
#[ignore = "driven by the capsule process-boundary claim"]
fn child_loads_capsule() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(4097)
        .read_to_end(&mut encoded)?;
    let record = read_capsule(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let mut line = String::from(PREFIX);
    for byte in record.encoded() {
        write!(&mut line, "{byte:02x}")?;
    }
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(line.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

fn round_trip(encoded: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new(std::env::current_exe()?)
        .args([
            "--ignored",
            "--exact",
            "archive::process::child_loads_capsule",
            "--nocapture",
        ])
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

#[test]
fn actual_and_independent_capsules_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let capsule = fixture::capsule().map_err(|()| std::io::Error::other("fixture refused"))?;
    let record = retain_capsule(&capsule, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(round_trip(record.encoded())?, record.encoded());
    for typed in [InputKind::Unit, InputKind::Bound] {
        let encoded = Vector::declared(typed).encoded();
        assert_eq!(round_trip(&encoded)?, encoded);
    }
    Ok(())
}
