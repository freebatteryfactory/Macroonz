//! Canonical generated-token receipts for the descriptor renderers whose authored facts are moving.

use super::{CONCURRENCY_BODY, NETWORK_BODY, concurrent, networked};

#[test]
fn network_and_concurrency_generated_bytes_remain_exact() -> Result<(), ()> {
    let network = networked(NETWORK_BODY).ok_or(())?.map_err(|_| ())?;
    let concurrency = concurrent(CONCURRENCY_BODY).ok_or(())?.map_err(|_| ())?;
    let network_bytes = network.emit().tokens().ok_or(())?.canonical_bytes();
    let concurrency_bytes = concurrency.emit().tokens().ok_or(())?.canonical_bytes();

    assert_eq!(
        blake3::hash(&network_bytes).to_hex().as_str(),
        "b887a7fd69a1f9e0240e45defbda22c6e04bd1cc7c165110fa0e2592bdbbbdb0"
    );
    assert_eq!(
        blake3::hash(&concurrency_bytes).to_hex().as_str(),
        "8bc89422a0c546d37b48748fc8594ad54bd45319a88bee14c4bf3fea3f7cca46"
    );
    Ok(())
}
