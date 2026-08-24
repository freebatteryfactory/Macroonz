//! A stamped plan is read off a plan, never written beside one.
//!
//! The reading road checks that the seat exists in the plan and that its delivery is a publication artifact, and then writes down what it proved.
//! A caller-written literal would state a decision no plan made — and the publication record minted over it would say those were the plan's decisions — so the seats are not the caller's to write.
//!
//! Minting the identities is lawful and stays lawful; the unwritable thing is the pair that claims a plan decided them.

use macroonz::identity::{GeneratedUnit, Identity, Role, Transcript};
use macroonz::plan::DigestContract;
use macroonz::stamp::StampedPlan;

fn main() {
    let unit = Identity::<GeneratedUnit>::derived(Transcript::rooted(
        Role::GeneratedUnit,
        b"a unit nobody planned",
        0,
    ));
    let _forged = StampedPlan {
        unit,
        staged: DigestContract { anchored_to: unit },
    };
}
