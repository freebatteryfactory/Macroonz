//! Fresh test processes load and return retained bytes without executing or reducing a subject.

use super::{
    LIMITS, fixture,
    vector::{InputKind, Vector},
};
use macroonz_harness::report::archive::{read_capsule, read_trial, retain_capsule, retain_trial};
use std::error::Error;
use std::fmt::Write as _;
use std::io::{Read as _, Write as _};
use std::process::{Command, Stdio};

const PREFIX: &str = "MACROONZ_ARCHIVED_RECORD=";

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
    publish(record.encoded())
}

fn publish(encoded: &[u8]) -> Result<(), Box<dyn Error>> {
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

fn round_trip(encoded: &[u8], child_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
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

#[test]
fn actual_and_independent_capsules_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let capsule = fixture::capsule().map_err(|()| std::io::Error::other("fixture refused"))?;
    let record = retain_capsule(&capsule, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(
        round_trip(record.encoded(), "archive::process::child_loads_capsule")?,
        record.encoded()
    );
    for typed in [InputKind::Unit, InputKind::Bound] {
        let encoded = Vector::declared(typed).encoded();
        assert_eq!(
            round_trip(&encoded, "archive::process::child_loads_capsule")?,
            encoded
        );
    }
    Ok(())
}

#[test]
#[ignore = "driven by the trial process-boundary claim"]
fn child_loads_trial() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(16385)
        .read_to_end(&mut encoded)?;
    let record = read_trial(&encoded, super::trials::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    publish(record.encoded())
}

#[test]
fn actual_and_independent_trials_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let report =
        fixture::report(&[1, 2, 3]).map_err(|()| std::io::Error::other("fixture refused"))?;
    let record = retain_trial(&report, super::trials::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(
        round_trip(record.encoded(), "archive::process::child_loads_trial")?,
        record.encoded()
    );
    let encoded = super::trial_vector::envelope(&super::trial_vector::body(&[3], &[1]));
    assert_eq!(
        round_trip(&encoded, "archive::process::child_loads_trial")?,
        encoded
    );
    Ok(())
}

#[test]
#[ignore = "driven by the run process-boundary claim"]
fn child_loads_run() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(32769)
        .read_to_end(&mut encoded)?;
    let record = macroonz_harness::report::archive::read_run(&encoded, super::runs::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    publish(record.encoded())
}

#[test]
fn actual_and_independent_runs_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let report =
        super::run_fixture::typed().map_err(|()| std::io::Error::other("fixture refused"))?;
    let record = macroonz_harness::report::archive::retain_run(&report, super::runs::LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(report);
    for encoded in [
        record.encoded().to_vec(),
        super::run_vector::RunVector::mixed().encoded(),
    ] {
        assert_eq!(
            round_trip(&encoded, "archive::process::child_loads_run")?,
            encoded
        );
    }
    Ok(())
}
