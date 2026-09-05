//! Caller-side held-output protocol for an external live-process memory observation.
//! The supervising parent owns the deadline and the OS measurement; this module claims neither.

use std::hint::black_box;
use std::io::{Read, Write};

pub(super) fn checksum(material: &[u8]) -> Result<u64, String> {
    black_box(material).iter().try_fold(0_u64, |sum, byte| {
        sum.checked_add(u64::from(*byte))
            .ok_or_else(|| "held checksum overflow".to_owned())
    })
}

pub(super) fn wait_for_release(material: &[u8], expected: u64) -> Result<(), String> {
    std::io::stdout()
        .flush()
        .map_err(|error| error.to_string())?;
    let mut release = Vec::new();
    std::io::stdin()
        .take(9)
        .read_to_end(&mut release)
        .map_err(|error| error.to_string())?;
    if release != b"release\n" {
        return Err("observer did not supply the exact release token and EOF".to_owned());
    }
    if checksum(material)? != expected {
        return Err("held allocation changed before release".to_owned());
    }
    Ok(())
}
