//! Claim: the network transcript family and one complete simulated envelope retain exact external bytes.
//! Subject: the public family tag, format version, address, and encoded envelope.
//! Population: the duplicating two-node simulated specimen from the shared fixture.
//! Reversal: the custody module changes only an extra empty advance and observes a different address and envelope.
//! Denominator: every byte of this one format-two specimen.
//! Evidence ceiling: one exact receipt does not prove collision resistance or every lawful transcript.

use super::support::*;

#[test]
fn the_format_two_simulated_specimen_has_one_exact_receipt() -> Result<(), LaneFailure> {
    let (_rows, _schedule, pack, _standing) = packed_run(0usize)?;
    assert_eq!(TRANSCRIPT_TAG.spelling(), "network-transcript");
    assert_eq!(TRANSCRIPT_TAG.version().position(), 2u32);
    assert_eq!(TRANSCRIPT_FORMAT_VERSION, 2u32);
    assert_eq!(pack.encoded().len(), 590usize);
    assert_eq!(
        blake3::hash(pack.encoded()).to_hex().to_string(),
        "d3eff9186205c8e8309e9d71c9804e809a7a5bd3d5a6672b3368f5e3d459ed7c"
    );
    assert_eq!(
        pack.address().address().as_bytes(),
        &[
            76, 53, 253, 46, 224, 176, 211, 110, 161, 191, 174, 174, 138, 236, 213, 205, 217, 118,
            211, 33, 116, 218, 100, 52, 99, 47, 193, 132, 243, 95, 96, 37,
        ]
    );
    Ok(())
}
