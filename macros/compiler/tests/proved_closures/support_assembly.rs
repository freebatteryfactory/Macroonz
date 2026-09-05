//! The support assembly observed through the closure lane's expansions: cargo proved under another declaration refuses the join, sources follow axis order, declared parentage leaves the canonical axis bytes unchanged, and promoted cargo cannot be reseated.

#[path = "assembly_diagnostics.rs"]
mod diagnostics;

use super::{DECLARATION, DOOR, OTHER_DECLARATION, expansion, expansion_rendered, spelled};
use macroonz_compiler::support::{
    AssemblyIssue, AxisCargo, CargoAxis, DeclaredCargo, DeferredCargo, EXPECTED_SCHEMA_ID,
    ProvedCargo, SupportAssembly, SupportAxes,
};
use macroonz_compiler::{
    CanonicalContent, Destination, Disposition, Expansion, Kind, NoQuestions, Observed, OwnerFact,
    RefusalClass, Refused, Request, Role, TextCapture, encode_bytes,
};

/// The outside observer's reason for leaving an unrelated support axis empty.
const SUPPORT_AXIS_ABSENT: OwnerFact = OwnerFact {
    home: "lane",
    name: "support-axis-not-part-of-this-reversal",
};

/// A kind whose one seat reaches only the benchmark carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BenchCargo;

impl Kind for BenchCargo {
    const NAME: &'static str = "lane.bench-cargo";
    type Content = &'static str;
    type Role = BenchSeat;
    type Question = NoQuestions;
}

/// The one seat the benchmark-cargo fixture fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BenchSeat {
    /// Cargo proved for a benchmark target.
    Cargo,
}

impl Role for BenchSeat {
    const ALL: &'static [Self] = &[Self::Cargo];

    fn name(self) -> &'static str {
        "cargo"
    }

    fn destination(self) -> Destination {
        Destination::BenchCarrier
    }
}

/// The expansion whose one proved delivery belongs to the benchmark carrier.
fn bench_expansion(source: &str) -> Option<Expansion<BenchCargo>> {
    let read = TextCapture::read(source).ok()?;
    Request::<BenchCargo>::over(read.input().clone(), "bench-cargo", &DOOR)
        .render(|_plan, out| out.unit(BenchSeat::Cargo, spelled("bench")?))
        .ok()
}

/// One axis whose absence is explicit and irrelevant to the active reversal.
fn absent_axis<Material>() -> AxisCargo<Material> {
    AxisCargo::Absent {
        because: Disposition::NotApplicable {
            because: SUPPORT_AXIS_ABSENT,
        },
    }
}

/// Declaration-site cargo proved under another declaration cannot enter this assembly's declared axis.
#[test]
fn declared_cargo_from_another_declaration_refuses_the_checked_join() -> Result<(), ()> {
    let stated = expansion(DECLARATION).ok_or(())?;
    let foreign = expansion(OTHER_DECLARATION).ok_or(())?;
    let declared = DeclaredCargo::stamped_from(&foreign, spelled("matcher").map_err(|_| ())?)
        .map_err(|_| ())?;
    let stated_root = stated.plan().account().commitment();
    let foreign_root = foreign.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        stated_root,
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: absent_axis(),
            bench: absent_axis(),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::RootsDisagree {
            axis: CargoAxis::Declared,
            stated: stated_root,
            carried: foreign_root,
        }
    );
    Ok(())
}

/// The proving-terminal roster includes declared cargo first, followed by the one occupied deferred form.
///
/// Two terminals stand over the same declaration but render different bytes, so their closed identities make the axis order independently observable.
#[test]
fn assembly_sources_include_declared_parentage_in_axis_order() -> Result<(), ()> {
    let declaring = expansion(DECLARATION).ok_or(())?;
    let testing = expansion_rendered(DECLARATION, "other_head", "other_tail").ok_or(())?;
    assert_ne!(declaring.identity(), testing.identity());
    let declared = DeclaredCargo::stamped_from(&declaring, spelled("matcher").map_err(|_| ())?)
        .map_err(|_| ())?;
    let deferred = DeferredCargo::deferred(testing.test_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(
        &testing,
        CargoAxis::Deferred,
        Destination::TestCarrier,
        deferred,
    )
    .map_err(|_| ())?;
    let assembly = SupportAssembly::assembled(
        declaring.plan().account().commitment(),
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: AxisCargo::Carried(proved),
            bench: absent_axis(),
        },
    )
    .map_err(|_| ())?;
    assert_eq!(
        assembly.sources().collect::<Vec<_>>(),
        [declaring.identity(), testing.identity()]
    );
    Ok(())
}

/// Declared-cargo provenance guards the join without changing the accepted canonical assembly bytes.
///
/// The independent expected encoding writes the outer declaration root and the exact declared matcher/stamped payload, but no second source or root inside the declared axis.
#[test]
fn declared_cargo_parentage_does_not_change_its_canonical_axis_encoding() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let declared =
        DeclaredCargo::stamped_from(&bound, spelled("matcher").map_err(|_| ())?).map_err(|_| ())?;
    let root = bound.plan().account().commitment();

    let mut expected = Vec::new();
    encode_bytes(root.as_bytes(), &mut expected);
    encode_bytes(EXPECTED_SCHEMA_ID.as_bytes(), &mut expected);
    expected.push(0);

    expected.push(1);
    let mut declared_axis = Vec::new();
    encode_bytes(&declared.matched().canonical_bytes(), &mut declared_axis);
    encode_bytes(&declared.stamped().canonical_bytes(), &mut declared_axis);
    encode_bytes(&declared_axis, &mut expected);

    for _axis in [CargoAxis::Deferred, CargoAxis::Bench] {
        expected.push(0);
        expected.push(1);
        encode_bytes(&SUPPORT_AXIS_ABSENT.citation_bytes(), &mut expected);
    }

    let assembly = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: AxisCargo::Carried(declared),
            deferred: absent_axis(),
            bench: absent_axis(),
        },
    )
    .map_err(|_| ())?;
    assert_eq!(assembly.canonical_content_bytes(), expected);
    Ok(())
}

/// Opaque deferred cargo cannot be promoted for the stamped declaration axis, even from that axis's own delivery.
#[test]
fn the_declared_axis_accepts_only_stamped_cargo() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.emit().tokens().ok_or(())?.clone());
    let refusal = ProvedCargo::carried(
        &bound,
        CargoAxis::Declared,
        Destination::DeclarationSite,
        cargo,
    )
    .err()
    .ok_or(())?;
    let issue = AssemblyIssue::DeclaredAxisRequiresStampedCargo;
    assert_eq!(refusal.first_issue(), &issue);
    assert_eq!(issue.slot(), 7);
    assert_eq!(issue.canonical_bytes(), [7]);
    assert_eq!(issue.axis(), Some(CargoAxis::Declared));
    assert_eq!(issue.observed(), Observed::ContractDisagreement);
    assert_eq!(refusal.class(), RefusalClass::CarrierNotAssembled);
    Ok(())
}

/// Cargo proved for a test target cannot be reseated in the public benchmark field after promotion.
#[test]
fn proved_test_cargo_cannot_be_reseated_as_benchmark_cargo() -> Result<(), ()> {
    let bound = expansion(DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.test_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(&bound, CargoAxis::Deferred, Destination::TestCarrier, cargo)
        .map_err(|_| ())?;
    let root = bound.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: absent_axis(),
            deferred: absent_axis(),
            bench: AxisCargo::Carried(proved),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::CargoReachesASecondDestination {
            axis: CargoAxis::Bench,
            destination: Destination::TestCarrier,
        }
    );
    Ok(())
}

/// Cargo proved for a benchmark target cannot be reseated in the public deferred field after promotion.
#[test]
fn proved_benchmark_cargo_cannot_be_reseated_as_test_cargo() -> Result<(), ()> {
    let bound = bench_expansion(OTHER_DECLARATION).ok_or(())?;
    let cargo = DeferredCargo::deferred(bound.bench_carrier().tokens().ok_or(())?.clone());
    let proved = ProvedCargo::carried(&bound, CargoAxis::Bench, Destination::BenchCarrier, cargo)
        .map_err(|_| ())?;
    let root = bound.plan().account().commitment();
    let refusal = SupportAssembly::assembled(
        root,
        None,
        SupportAxes {
            declared: absent_axis(),
            deferred: AxisCargo::Carried(proved),
            bench: absent_axis(),
        },
    )
    .err()
    .ok_or(())?;
    assert_eq!(
        refusal.first_issue(),
        &AssemblyIssue::CargoReachesASecondDestination {
            axis: CargoAxis::Deferred,
            destination: Destination::BenchCarrier,
        }
    );
    Ok(())
}
