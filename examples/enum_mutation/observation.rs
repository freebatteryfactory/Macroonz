//! Independently stated order expectations and permission controls.

use crate::{Priority, phases, priorities};
use macroonz::harness::clock::HarnessClock;
use macroonz::harness::descriptor::{DerivedRevision, RevisionBinding};
use macroonz::harness::muterprater::interpret::observe_mutation;
use macroonz::harness::muterprater::{
    DiscoveryDisposition, EvaluationBinding, EvaluationDirective, EvaluationPair, ProductionBinding,
};
use macroonz::harness::properties::Agreement;
use macroonz::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialSite,
};
use macroonz::harness::runner::Invocation;

pub(super) fn permitted() -> Result<(), String> {
    let expected = ["Urgent", "Normal", "Deferred"];
    assert_eq!(priorities::production(&()), expected);
    assert_eq!(
        priorities::candidate_orders(),
        [
            ["Normal", "Urgent", "Deferred"],
            ["Urgent", "Deferred", "Normal"]
        ]
    );
    let lowering = priorities::lowering().map_err(debug)?;
    let surface = lowering.surface();
    let [point] = surface.points() else {
        return Err("one permitted priority point required".to_owned());
    };
    assert_eq!(point.admitted_alternatives().len(), 2usize);
    let revision =
        RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("types.rs")));
    let pair = EvaluationPair::paired(
        ProductionBinding::declared(surface.family(), revision, priorities::production),
        EvaluationBinding::declared(surface, revision, priorities::evaluation),
        relation,
    )
    .map_err(debug)?;
    let mut observed = Vec::new();
    for selected in surface.selections() {
        let reading =
            observe_mutation(surface, &pair, &(), selected, &invocation()).map_err(debug)?;
        let unchanged = reading.baseline().as_ref().map_err(debug)?;
        let changed = reading.selected().as_ref().map_err(debug)?;
        assert_eq!(unchanged.meaning(), &expected);
        assert_eq!(unchanged.firings(), 0u32);
        assert_eq!(changed.firings(), 1u32);
        assert_ne!(changed.meaning(), &expected);
        observed.push(*changed.meaning());
    }
    assert_eq!(
        observed,
        [
            ["Normal", "Urgent", "Deferred"],
            ["Urgent", "Deferred", "Normal"]
        ]
    );
    assert_eq!(size_of::<Priority>(), 1usize);
    Ok(())
}

pub(super) fn withheld() -> Result<(), String> {
    let lowering = phases::lowering().map_err(debug)?;
    assert!(lowering.surface().points().is_empty());
    assert!(lowering.surface().selections().is_empty());
    let [discovered] = lowering.discovery().entries() else {
        return Err("unpermitted discovery was lost".to_owned());
    };
    assert!(matches!(
        discovered.disposition(),
        DiscoveryDisposition::MappedUnpermitted { .. }
    ));
    assert_eq!(phases::candidate_orders().len(), 2usize);
    let unchanged = phases::evaluation(&(), EvaluationDirective::no_mutation()).map_err(debug)?;
    assert_eq!(phases::production(&()), ["Received", "Checked", "Released"]);
    assert_eq!(unchanged.meaning(), &["Received", "Checked", "Released"]);
    assert_eq!(unchanged.firings(), 0u32);
    Ok(())
}

fn relation(left: &[&str; 3], right: &[&str; 3]) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(4_096u64),
            TimeBudget::declared(1u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("declared-order-observation"),
            ToolchainIdentity::declared("generated-values-without-subject-compilation"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "enum order"),
        HarnessClock::unavailable(),
    )
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
