//! Public contract observations from an outside crate target.

use macroonz::{
    AdmittedLimit, AdmittedPrefix, Bounded, BoundedConstruction, CauseId, CauseOrderDeclaration,
    CompletionPosture, ConstLimit, DeclaredCause, DeclaredCauseOrder, DeclaredMagnitude,
    FamilyAdmission, FamilyAdmissionCoverage, FamilyShape, FieldCardinality, Limit,
    LimitAdmissionProfile, LocalCauseKey, NonEmptyBounded, NonEmptyBoundedConstruction,
    PositiveLimit, RefusalFamily, RefusalFamilyId, StopBound, admit_order, admit_order_projection,
    admit_shape,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PairLimit {}

impl Limit for PairLimit {
    type Authority = DeclaredMagnitude;
}

impl ConstLimit for PairLimit {
    const MAX: usize = 2;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContractProfile {}

impl LimitAdmissionProfile for ContractProfile {
    const MAX_DECLARED_LIMIT: usize = 8;
}

struct OrderedFamily;

impl RefusalFamily for OrderedFamily {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
}

const GENERATED_ORDERED_SELECTION: &[&str] = &["First", "Second"];

impl CauseOrderDeclaration for OrderedFamily {
    const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("example.ordered"),
                LocalCauseKey::declared("first"),
            ),
            "First",
        ),
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("example.ordered"),
                LocalCauseKey::declared("second"),
            ),
            "Second",
        ),
    ]);
}

struct IncoherentCollection;

impl RefusalFamily for IncoherentCollection {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
}

impl CauseOrderDeclaration for IncoherentCollection {
    const DECLARED_ORDER: DeclaredCauseOrder = OrderedFamily::DECLARED_ORDER;
}

struct MismatchedProjection;

impl RefusalFamily for MismatchedProjection {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
}

const MISMATCHED_SELECTION: &[&str] = &["Second", "First"];

impl CauseOrderDeclaration for MismatchedProjection {
    const DECLARED_ORDER: DeclaredCauseOrder = OrderedFamily::DECLARED_ORDER;
}

macroonz::closed_register! {
    /// A roster used to observe the public stamp contract.
    enum ExampleRoster {
        /// The first row.
        First = "first", "the first row";
        /// The second row.
        Second = "second", "the second row";
    }
}

#[test]
fn compile_time_limits_govern_bounded_construction() {
    let admitted = AdmittedLimit::<PairLimit, ContractProfile>::under_profile();
    let positive = PositiveLimit::<PairLimit, ContractProfile>::inhabited_under_profile();

    let bounded = Bounded::admitted_const(vec![1_u8, 2_u8], &admitted);
    assert_eq!(bounded.map(|held| held.len()), Ok(2));
    assert_eq!(
        Bounded::admitted_const(vec![1_u8, 2_u8, 3_u8], &admitted),
        Err(BoundedConstruction::OverLimit)
    );

    let non_empty = NonEmptyBounded::admitted_const(1_u8, vec![2_u8], &positive);
    assert_eq!(non_empty.map(|held| held.len()), Ok(2));
    assert_eq!(
        NonEmptyBounded::admitted_const(1_u8, vec![2_u8, 3_u8], &positive),
        Err(NonEmptyBoundedConstruction::OverLimit)
    );
}

#[test]
fn admitted_prefix_derives_its_coverage_from_the_same_construction() {
    let positive = PositiveLimit::<PairLimit, ContractProfile>::inhabited_under_profile();
    let report = AdmittedPrefix::examined_completely(
        "first",
        vec!["second", "third"],
        &positive,
        StopBound::DeclaredIssueBound,
    );

    assert_eq!(report.carried().len(), 2);
    assert!(matches!(
        report.completion(),
        CompletionPosture::ReportTruncated(_)
    ));
    if let CompletionPosture::ReportTruncated(truncation) = report.completion() {
        assert_eq!(truncation.stopped_at(), StopBound::DeclaredIssueBound);
        assert_eq!(truncation.omitted().get(), 1);
    }
}

#[test]
fn typed_order_projects_to_the_declared_textual_order() {
    let shape = admit_shape::<OrderedFamily>().coverage();
    assert_eq!(shape, FamilyAdmissionCoverage::ShapeCoherence);
    assert!(OrderedFamily::DECLARED_ORDER.projects_to(GENERATED_ORDERED_SELECTION));

    let admitted =
        admit_order::<OrderedFamily>().map(|witness| (witness.cause_order(), witness.coverage()));
    assert_eq!(
        admitted,
        Ok((
            OrderedFamily::DECLARED_ORDER,
            FamilyAdmissionCoverage::ShapeCoherenceAndTypedOrder,
        ))
    );
    assert_eq!(
        admit_order::<IncoherentCollection>().map(|_| ()),
        Err(FamilyAdmission::NotShapeCoherent)
    );
    assert_eq!(
        admit_order_projection::<MismatchedProjection>(MISMATCHED_SELECTION).map(|_| ()),
        Err(FamilyAdmission::NotProjected)
    );
    assert_eq!(
        admit_order_projection::<OrderedFamily>(GENERATED_ORDERED_SELECTION)
            .map(|witness| witness.coverage()),
        Ok(FamilyAdmissionCoverage::ShapeCoherenceAndOrderProjection)
    );
}

#[test]
fn closed_register_and_field_cardinality_are_root_contracts() {
    assert_eq!(
        ExampleRoster::ALL,
        [ExampleRoster::First, ExampleRoster::Second]
    );
    assert_eq!(ExampleRoster::Second.slot(), 1);
    assert_eq!(ExampleRoster::First.stable_name(), "first");
    assert_eq!(ExampleRoster::Second.described(), "the second row");
    assert_eq!(FieldCardinality::Optional, FieldCardinality::Optional);
}
