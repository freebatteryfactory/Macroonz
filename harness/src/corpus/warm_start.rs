//! Handing admitted seeds to the generation home as exact supplied material.

use super::SeedPack;
use crate::generate::InputOrigin;

/// Yield one exact supplied-input origin per admitted seed, in pack order.
///
/// This road chooses no budget, decoder, precondition, verdict, replay posture, or proposal ground.
/// The caller combines each origin with its own generation plan and judges what happens through the ordinary roads.
#[must_use]
pub fn warm_start(pack: &SeedPack) -> impl ExactSizeIterator<Item = InputOrigin> + '_ {
    pack.seeds()
        .iter()
        .map(|seed| InputOrigin::Supplied(seed.bytes().to_vec()))
}
