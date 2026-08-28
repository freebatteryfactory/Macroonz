//! Generated compiler challenges over every authored type-separation row.

mod diagnostic;
mod render;
mod scratch;

use macroonz_harness::depot::swap_pairs::SWAP_PAIRS;

/// Every authored directional challenge compiles with its seat and refuses its substitute at that exact call.
#[test]
fn every_swap_pair_is_observed_in_its_declared_direction() -> Result<(), String> {
    let scratch = scratch::Scratch::claimed()?;
    let mut challenges = Vec::with_capacity(SWAP_PAIRS.len());

    for (ordinal, pair) in SWAP_PAIRS.iter().enumerate() {
        let challenge = render::Challenge::for_pair(ordinal, *pair)?;
        scratch.write(&challenge.lawful)?;
        scratch.write(&challenge.hostile)?;
        challenges.push(challenge);
    }

    scratch.generate_lockfile()?;

    for challenge in &challenges {
        let lawful = scratch.check(&challenge.lawful.bin_name)?;
        if !lawful.status.success() {
            return Err(scratch::failed_command("lawful control", &lawful));
        }

        let hostile = scratch.check(&challenge.hostile.bin_name)?;
        if hostile.status.success() {
            return Err(format!(
                "swap-pair row {} accepted {} where {} was required",
                challenge.ordinal, challenge.substitute, challenge.seat
            ));
        }
        diagnostic::require_one_mismatch(&hostile.stdout, &challenge.hostile.primary)?;
    }

    Ok(())
}
