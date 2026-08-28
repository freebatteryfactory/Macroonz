//! Identity derivation is observed across fresh harness test processes.

use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use std::error::Error;
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Output};

const OUTPUT_PREFIX: &str = "MACROONZ_IDENTITY_PROCESS_ADDRESS=";
const PROCESS_TAG: DomainTag =
    DomainTag::declared("process-determinism", IdentityProfileVersion::declared(1));
const PREIMAGE: &[u8] = b"declared-process-preimage";

fn address_hex() -> Result<String, std::fmt::Error> {
    let mut encoded = String::with_capacity(64);
    for byte in ContentAddress::derived(PROCESS_TAG, PREIMAGE).as_bytes() {
        write!(&mut encoded, "{byte:02x}")?;
    }
    Ok(encoded)
}

fn address_from(output: Output) -> Result<String, Box<dyn Error>> {
    if !output.status.success() {
        return Err(
            std::io::Error::other(format!("identity child exited with {}", output.status)).into(),
        );
    }
    let stdout = String::from_utf8(output.stdout)?;
    let Some(address) = stdout
        .lines()
        .find_map(|line| line.split_once(OUTPUT_PREFIX).map(|(_, address)| address))
    else {
        return Err(std::io::Error::other("identity child emitted no address").into());
    };
    Ok(address.to_owned())
}

fn fresh_process_address() -> Result<String, Box<dyn Error>> {
    let executable = std::env::current_exe()?;
    let output = Command::new(executable)
        .args([
            "--ignored",
            "--exact",
            "child_process_reports_address",
            "--nocapture",
        ])
        .output()?;
    address_from(output)
}

#[test]
#[ignore = "driven by the parent process-identity claim"]
fn child_process_reports_address() -> Result<(), Box<dyn Error>> {
    let address = address_hex()?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(OUTPUT_PREFIX.as_bytes())?;
    stdout.write_all(address.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

/// Claim: identical declared identity inputs derive the same address in fresh processes.
/// Subject: the public harness content-address derivation mechanism.
/// Population: two separately spawned test processes and the parent process.
/// Hostile control: neither child receives address bytes from the parent or its sibling.
/// Denominator: the fixed domain spelling, profile version, and preimage used by all three processes.
/// Evidence ceiling: this establishes current-host process determinism only, not cross-target portability or collision resistance.
/// Retained regression: process-local or entropy-backed identity derivation remains a permanent owner regression.
#[test]
fn fresh_processes_repeat_the_declared_identity() -> Result<(), Box<dyn Error>> {
    let first = fresh_process_address()?;
    let second = fresh_process_address()?;
    assert_eq!(first, second);
    assert_eq!(first, address_hex()?);
    Ok(())
}
