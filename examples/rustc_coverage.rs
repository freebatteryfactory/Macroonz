//! Runnable stable-rustc coverage composition through the Macroonz facade.

#[path = "support/rustc_coverage_replay.rs"]
mod replay;
#[path = "support/rustc_coverage_host.rs"]
mod host;

use macroonz::harness::corpus::{SeedInput, pack};
use macroonz::harness::descriptor::{
    DerivedRevision, NamespacedName, PopulationRef, RevisionBinding,
};
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageBudgets, CoverageCampaign, CoverageCorpus, CoverageProfile,
    CoverageSourceRoot, FuzzExecution, InstrumentedTarget, RustcProfileRequest,
    observe_rustc_profile, preflight_ready,
};
use macroonz::harness::report::{ByteBudget, CaseBudget};
use std::fmt;
use std::path::{Path, PathBuf};

struct ExampleFailure(String);

impl fmt::Debug for ExampleFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn main() -> Result<(), ExampleFailure> {
    let run = host::run_directory();
    let Some(parent) = run.parent() else {
        return Err(ExampleFailure(
            "qualification run had no parent directory".to_owned(),
        ));
    };
    std::fs::create_dir_all(parent).map_err(failure)?;
    std::fs::create_dir(&run).map_err(failure)?;
    let result = exercise(&run);
    let cleanup = std::fs::remove_dir_all(&run);
    match (result, cleanup) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(error), Ok(())) => Err(error),
        (Ok(()), Err(error)) => Err(failure(error)),
        (Err(original), Err(cleanup)) => Err(ExampleFailure(format!(
            "{}; qualification cleanup failed: {cleanup:?}",
            original.0
        ))),
    }
}

fn exercise(run: &Path) -> Result<(), ExampleFailure> {
    let rustc = host::declared_rustc().map_err(ExampleFailure)?;
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let subject = host::compile_subject(&rustc, &manifest, run).map_err(ExampleFailure)?;
    let target = InstrumentedTarget::declared(subject, Vec::new()).map_err(failure)?;
    let logical = NamespacedName::named("macroonz.example", "rustc-coverage").map_err(failure)?;
    let source_root = CoverageSourceRoot::declared(logical, manifest).map_err(failure)?;
    let campaign = coverage_campaign()?;
    let request =
        RustcProfileRequest::declared(rustc, target, source_root, run.join("cases"), campaign)
            .map_err(failure)?;
    let ready = preflight_ready(request).map_err(failure)?;
    let target_facts = ready.standing().target().clone();
    let mut frontier = CoverageCorpus::opening(&ready);

    let baseline =
        observe_rustc_profile(&ready, &mut frontier, &[0], host::wait_for_exit).map_err(failure)?;
    if baseline.execution() != FuzzExecution::Success {
        return Err(ExampleFailure("baseline target did not succeed".to_owned()));
    }
    let CoverageAdmission::Interesting(first) = frontier.admit(baseline).map_err(failure)? else {
        return Err(ExampleFailure(
            "the opening candidate added no coverage".to_owned(),
        ));
    };

    let expanded = observe_rustc_profile(&ready, &mut frontier, &[1, 2, 3], host::wait_for_exit)
        .map_err(failure)?;
    let CoverageAdmission::Interesting(second) = frontier.admit(expanded).map_err(failure)? else {
        return Err(ExampleFailure(
            "the second candidate added no coverage".to_owned(),
        ));
    };

    let repeated =
        observe_rustc_profile(&ready, &mut frontier, &[0], host::wait_for_exit).map_err(failure)?;
    if frontier.admit(repeated).map_err(failure)? != CoverageAdmission::Known {
        return Err(ExampleFailure(
            "repeated coverage was admitted as novel".to_owned(),
        ));
    }

    retain_seed_pack(campaign.population(), &first, &second)?;
    replay::reduce_and_replay(&second, target_facts, campaign.revision()).map_err(ExampleFailure)
}

fn coverage_campaign() -> Result<CoverageCampaign, ExampleFailure> {
    let population = PopulationRef::named("macroonz.example", "coverage-seeds").map_err(failure)?;
    let profile =
        NamespacedName::named("macroonz.example", "rustc-region-coverage").map_err(failure)?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "support/rustc_coverage_subject.rs"
    )));
    let budgets = CoverageBudgets::declared(
        CaseBudget::declared(3),
        ByteBudget::declared(5),
        4_194_304,
        4_096,
        CaseBudget::declared(2),
        ByteBudget::declared(4),
    )
    .map_err(failure)?;
    Ok(CoverageCampaign::declared(
        population,
        revision,
        CoverageProfile::declared(profile, 1),
        budgets,
    ))
}

fn retain_seed_pack(
    population: PopulationRef,
    first: &macroonz::harness::fuzz::InterestingBytes,
    second: &macroonz::harness::fuzz::InterestingBytes,
) -> Result<(), ExampleFailure> {
    let seeds = vec![
        SeedInput::declared(first.as_bytes().to_vec()).map_err(failure)?,
        SeedInput::declared(second.as_bytes().to_vec()).map_err(failure)?,
    ];
    let retained = pack(population, seeds).map_err(failure)?;
    if retained.seeds().len() == 2 {
        Ok(())
    } else {
        Err(ExampleFailure(
            "coverage corpus did not retain both novel seeds".to_owned(),
        ))
    }
}

fn failure(error: impl fmt::Debug) -> ExampleFailure {
    ExampleFailure(format!("{error:?}"))
}
