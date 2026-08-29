//! Generated compiler challenges over every authored type-separation row.

mod diagnostic;
mod render;
mod scratch;

use macroonz_harness::depot::swap_pairs::SWAP_PAIRS;

/// Every authored directional challenge compiles with its seat and refuses its substitute at that exact call.
#[test]
fn every_swap_pair_is_observed_in_its_declared_direction() -> Result<(), String> {
    let scratch = scratch::Scratch::claimed()?;
    let outcome = (|| -> Result<(), String> {
        let mut challenges = Vec::with_capacity(SWAP_PAIRS.len());

        for (ordinal, pair) in SWAP_PAIRS.iter().enumerate() {
            let challenge = render::Challenge::for_pair(ordinal, *pair)?;
            scratch.write(&challenge.lawful)?;
            scratch.write(&challenge.hostile)?;
            challenges.push(challenge);
        }

        scratch
            .generate_lockfile()
            .map_err(|failure| format!("scratch lock generation was not runnable: {failure:?}"))?;

        for challenge in &challenges {
            let lawful = scratch
                .check(&challenge.lawful.bin_name)
                .map_err(|failure| format!("lawful control was not runnable: {failure:?}"))?;
            diagnostic::require_compilation(
                &lawful,
                scratch.root(),
                challenge.lawful.expected.primary().source(),
                &macroonz_harness::oracle::DeclaredCompilation::compiles(),
            )?;

            let hostile = scratch
                .check(&challenge.hostile.bin_name)
                .map_err(|failure| format!("hostile control was not runnable: {failure:?}"))?;
            diagnostic::require_compilation(
                &hostile,
                scratch.root(),
                challenge.hostile.expected.primary().source(),
                &macroonz_harness::oracle::DeclaredCompilation::refuses(
                    challenge.hostile.expected.clone(),
                ),
            )
            .map_err(|failure| {
                format!(
                    "swap-pair row {} offered {} where {} was required: {failure}",
                    challenge.ordinal, challenge.substitute, challenge.seat
                )
            })?;
        }

        Ok(())
    })();
    scratch.finish(outcome)
}
