//! Projecting admitted corpus seeds into the generation home's exact caller-supplied origin.

use super::SeedPack;
use crate::generate::InputOrigin;

/// Yield one exact supplied-input origin per admitted seed, in pack order.
///
/// This operation chooses no generation budget, decoder, precondition, verdict, replay posture, or proposal ground. The caller combines each origin with its existing generation plan facts and judges the resulting behavior through the ordinary harness roads.
#[must_use]
pub fn warm_start(pack: &SeedPack) -> impl ExactSizeIterator<Item = InputOrigin> + '_ {
    pack.seeds()
        .iter()
        .map(|seed| InputOrigin::Supplied(seed.bytes().to_vec()))
}
