#![forbid(unsafe_code)]
#![deny(warnings)]

use bakery::harness::clock::HarnessClock;
use bakery::harness::descriptor::{DerivedRevision, RevisionBinding};
use bakery::harness::muterprater::{
    DiscoveryDisposition, EvaluationBinding, EvaluationPair, ProductionBinding,
};
use bakery::harness::muterprater::interpret::observe_mutation;
use bakery::harness::properties::Agreement;
use bakery::harness::report::{
    ByteBudget, CaseBudget, InvocationProfile, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialSite,
};
use bakery::harness::runner::Invocation;

bakery::support! { macroonz_generated_support_observer::targets_support {} }
bakery::support! { macroonz_generated_support_observer::effects_support {} }
bakery::support! { macroonz_generated_support_observer::codec_support {} }
bakery::support! { macroonz_generated_support_observer::denied_support {} }

fn relation(left: &&str, right: &&str) -> Agreement {
    if left == right { Agreement::Agrees } else { Agreement::Differs }
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1), ByteBudget::declared(1_000_000), TimeBudget::declared(1),
        ),
        TargetBinding::bound(
            TargetTriple::declared("source-selection-control"),
            ToolchainIdentity::declared("declared-no-subject-compilation"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "source-selection"),
        HarnessClock::unavailable(),
    )
}

macro_rules! crossing {
    ($test:ident, $module:ident, $family:literal, $count:literal) => {
        #[test]
        fn $test() -> Result<(), String> {
            let lowered = $module::lowering().map_err(debug)?;
            let surface = lowered.surface();
            let [point] = surface.points() else { return Err("one admitted site required".into()); };
            assert_eq!(lowered.discovery().entries().len(), 1);
            assert_eq!(point.admitted_alternatives().len(), $count);
            assert!(point.admitted_alternatives().iter().all(|alternative| alternative.family().slug() == $family));
            let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!("crossing.rs")));
            let pair = EvaluationPair::paired(
                ProductionBinding::declared(surface.family(), revision, $module::production),
                EvaluationBinding::declared(surface, revision, $module::evaluation),
                relation,
            ).map_err(debug)?;
            let candidates = $module::candidate_orders();
            assert_eq!(candidates.len(), $count);
            let baseline = $module::production(&());
            let mut seen = Vec::new();
            for selected in surface.selections() {
                let reading = observe_mutation(surface, &pair, &(), selected, &invocation()).map_err(debug)?;
                let unchanged = reading.baseline().as_ref().map_err(debug)?;
                let changed = reading.selected().as_ref().map_err(debug)?;
                assert_eq!(*unchanged.meaning(), baseline);
                assert_eq!(unchanged.firings(), 0);
                assert_eq!(changed.firings(), 1);
                assert_ne!(*changed.meaning(), baseline);
                assert!(candidates.contains(changed.meaning()));
                assert!(!seen.contains(changed.meaning()));
                seen.push(*changed.meaning());
            }
            assert_eq!(seen.len(), candidates.len());
            Ok(())
        }
    };
}

crossing!(targets_are_selected_as_source, targets, "transition-target-substitution", 2);
crossing!(effects_are_selected_as_source, effects, "effect-binding-substitution", 1);
crossing!(codec_order_is_selected_as_source, codec, "declared-order-permutation", 1);

#[test]
fn owner_permission_withholds_executable_points_without_erasing_discovery() -> Result<(), String> {
    let lowering = denied::lowering().map_err(debug)?;
    assert!(lowering.surface().points().is_empty());
    assert!(lowering.surface().selections().is_empty());
    let [entry] = lowering.discovery().entries() else { return Err("discovery was lost".into()); };
    assert!(matches!(entry.disposition(), DiscoveryDisposition::MappedUnpermitted { .. }));
    assert_eq!(denied::candidate_orders().len(), 2);
    let unchanged = denied::evaluation(
        &(), bakery::harness::muterprater::EvaluationDirective::no_mutation(),
    ).map_err(debug)?;
    assert_eq!(*unchanged.meaning(), denied::production(&()));
    assert_eq!(unchanged.firings(), 0);
    Ok(())
}

fn debug(cause: impl core::fmt::Debug) -> String { format!("{cause:?}") }
