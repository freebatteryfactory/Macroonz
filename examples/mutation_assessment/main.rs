//! Observe an actual comparison mutation and assess independent witnesses without rerunning it.

mod capture;
mod checks;
mod declaration;
mod native;
mod proposal;
mod record;
mod replay;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::harness::clock::HarnessClock;
use macroonz::harness::muterprater::{
    EquivalenceAxis, EvaluationBinding, EvaluationPair, MutationVerdict, ProductionBinding,
    SpecimenMaterializerBinding, interpret, specimen,
};
use macroonz::harness::properties::{Agreement, SharedSubstrate, SubstrateRef, SubstrateRoster};
use macroonz::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialSite,
};
use macroonz::harness::runner::Invocation;
use std::cell::Cell;
use std::io::Write;
use std::path::PathBuf;
use types::{Sample, Settings};

fn main() -> Result<(), String> {
    let configuration = input::read()?;
    match input::text(&configuration, "action")? {
        "inspect" => {
            return record::inspect(std::path::Path::new(input::text(
                &configuration,
                "storage",
            )?));
        }
        "replay" | "replay-weakened" => {
            let (_, _, saved) = record::load(std::path::Path::new(input::text(
                &configuration,
                "storage",
            )?))?;
            let current = invocation(
                input::text(&configuration, "target")?,
                input::text(&configuration, "toolchain")?,
            );
            return match input::text(&configuration, "action")? {
                "replay" => replay::original(&saved, current),
                _ => replay::weakened(&saved, current),
            };
        }
        "assess" => {}
        _ => return Err("action must be assess, inspect, replay or replay-weakened".to_owned()),
    }
    let settings = settings(&configuration)?;
    let value = configuration
        .get("sample")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or("sample must be a u32")?;
    if value == 0 {
        return Err("choose a positive sample for the positive-input witness".to_owned());
    }
    let sample = Sample {
        value,
        settings: &settings,
        evaluations: Cell::new(0),
        executions: Cell::new(0),
    };
    assess(&sample)
}

fn settings(configuration: &serde_json::Value) -> Result<Settings, String> {
    let compiler = macroonz::configuration::v1::process_tool(
        PathBuf::from(input::text(configuration, "rustc")?),
        PathBuf::from(input::text(configuration, "directory")?),
        input::environment(configuration)?,
        &[],
    )
    .map_err(debug)?;
    Ok(Settings {
        compiler,
        target: input::text(configuration, "target")?.to_owned(),
        toolchain: input::text(configuration, "toolchain")?.to_owned(),
        storage: PathBuf::from(input::text(configuration, "storage")?),
    })
}

fn assess(sample: &Sample<'_>) -> Result<(), String> {
    let lowering = declaration::surface()?;
    let surface = lowering.surface();
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(
            surface.family(),
            checks::revision(b"nonzero-v1"),
            declaration::production,
        ),
        EvaluationBinding::declared(
            surface,
            checks::revision(b"comparison-selection-v1"),
            declaration::evaluation,
        ),
        |left, right| declaration::same(*left, *right),
    )
    .map_err(debug)?;
    let selections = surface.selections();
    let [selection] = selections.as_slice() else {
        return Err("expected the one declared comparison mutation".to_owned());
    };
    let invocation = invocation(&sample.settings.target, &sample.settings.toolchain);
    let observed = interpret::observe_mutation(surface, &pair, sample, *selection, &invocation)
        .map_err(debug)?;
    let materializer = SpecimenMaterializerBinding::bound(&pair, declaration::materialize);
    let compiled = specimen::observe_mutation(&observed, &materializer, |request| {
        native::observe(&request)
    })
    .map_err(debug)?;
    let substrate = SharedSubstrate::Standing(
        SubstrateRoster::declared(&[SubstrateRef::named(
            "nonzero",
            "declared-predicate-and-u32-encoding",
        )
        .map_err(debug)?])
        .map_err(debug)?,
    );
    let qualified = interpret::qualify_execution(&compiled, substrate).map_err(debug)?;
    assert_eq!(qualified.difference(), Agreement::Differs);
    let weak =
        interpret::observe_witness(&qualified, checks::weak()?, &invocation).map_err(debug)?;
    let weak = interpret::qualify_witness(weak).map_err(|refused| debug(refused.cause()))?;
    let strong =
        interpret::observe_witness(&qualified, checks::strong()?, &invocation).map_err(debug)?;
    let strong = interpret::qualify_witness(strong).map_err(|refused| debug(refused.cause()))?;
    assert_eq!(weak.mutation().verdict(), MutationVerdict::Survived);
    assert_eq!(weak.mutation().equivalence(), EquivalenceAxis::Refuted);
    assert_eq!(strong.mutation().verdict(), MutationVerdict::Killed);
    let proposed = proposal::offer(&weak, &strong, &invocation)?;
    record::retain(sample.settings, &weak, &strong, &proposed)?;
    assert_eq!(sample.evaluations.get(), 2);
    assert_eq!(sample.executions.get(), 2);
    writeln!(
        std::io::stdout(),
        "weak=Survived strong=Killed equivalence=Refuted; evaluations=2 compiled-executions=2 admissions=0"
    )
    .map_err(debug)
}

fn invocation(target: &str, toolchain: &str) -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1000),
        ),
        TargetBinding::bound(
            TargetTriple::declared(target),
            ToolchainIdentity::declared(toolchain),
        ),
        TrialSite::located(
            module_path!(),
            file!(),
            line!(),
            "independent nonzero witnesses",
        ),
        HarnessClock::unavailable(),
    )
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
