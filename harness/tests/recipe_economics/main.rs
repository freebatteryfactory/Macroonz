//! Final recipe work curves, qualified through the existing benchmark owner.

mod breadth;
mod breadth_bench;

use macroonz_compiler::recipe::{HarnessPosture, RecipeBake, RecipeRole};
use macroonz_compiler::{CanonicalContent, CrateBinding, Door, Producer, TextCapture};
use macroonz_harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchInvocation, BenchMeasurement, BenchOutcome,
    BenchReferences, BenchRow, BenchStage, BenchTable, BenchTableName, ComplexityClaimRef,
    ContentionPosture, DeclaredBudgets, InputSizeAxis, PlantedWorseRef, PreflightRef,
    PreflightTrial, WorkConclusion, WorkFormula, WorkGapStanding, WorkJudgeBinding, WorkJudgment,
    WorkJudgmentInput, WorkObservationRef, WorkRecorder, WorkRecordingRefusal, WorkloadRef,
    bench_verdict, run_all,
};
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, InvocationProfile,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialFinding,
    TrialSite,
};
use macroonz_harness::runner::{Invocation, TrialBinding};
use std::sync::atomic::{AtomicU64, Ordering};

const OWNER: &str = "harness.recipe-economics";
const FORMULA: &[u8] =
    b"measured=one retained final-source coordinate; planted=two complete retained coordinates";
const DOOR: Door = Door::declared(
    "recipe-economics",
    "recipe-economics.grammar",
    "recipe-economics::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-economics",
        name: "recipe",
    },
);
const REVISION_TAG: DomainTag = DomainTag::declared(
    "recipe-economics-revision",
    IdentityProfileVersion::declared(1),
);
const MEASURED_REFUSED: FindingCause = FindingCause::named(OWNER, "measured-work-refused");
const WORSE_REFUSED: FindingCause = FindingCause::named(OWNER, "repeated-work-refused");
const GAP_REFUSED: FindingCause = FindingCause::named(OWNER, "repetition-not-distinguished");
const PREFLIGHT_REFUSED: FindingCause = FindingCause::named(OWNER, "golden-preflight-refused");

static CLOCK: AtomicU64 = AtomicU64::new(1u64);

#[derive(Clone, Copy)]
enum Family {
    Vocabulary,
    RelationDensity,
    Signature,
    ProjectionCount,
    InvocationCount,
}

#[derive(Clone, Copy)]
enum Shape {
    Sparse,
    Dense,
}

#[derive(Clone, Copy)]
enum Control {
    Repeated,
    Identical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Metrics {
    source_bytes: u64,
    vocabulary_members: u64,
    transition_rows: u64,
    selected_projections: u64,
    compiler_invocations: u64,
    generated_bytes: u64,
    canonical_bytes: u64,
    delivered_bytes: u64,
    axis_refusals: u64,
}

const VOCABULARY: &[(u64, Metrics)] = &[
    (2, Metrics::declared([271, 4, 2, 2, 1, 3_970, 1_290, 4_382])),
    (4, Metrics::declared([353, 8, 4, 2, 1, 5_742, 1_600, 6_242])),
    (
        8,
        Metrics::declared([517, 16, 8, 2, 1, 9_286, 2_220, 9_962]),
    ),
    (
        16,
        Metrics::declared([875, 32, 16, 2, 1, 16_422, 3_502, 17_462]),
    ),
];
const RELATION_DENSITY: &[(u64, Metrics)] = &[
    (2, Metrics::declared([341, 4, 4, 2, 1, 5_366, 1_472, 5_778])),
    (
        4,
        Metrics::declared([773, 8, 16, 2, 1, 14_118, 2_692, 14_618]),
    ),
    (
        8,
        Metrics::declared([2_477, 16, 64, 2, 1, 48_374, 7_316, 49_050]),
    ),
];
const SIGNATURE: &[(u64, Metrics)] = &[
    (1, Metrics::declared([345, 4, 2, 1, 1, 1_944, 1_600, 2_356])),
    (2, Metrics::declared([355, 4, 2, 1, 1, 1_991, 1_647, 2_403])),
    (4, Metrics::declared([375, 4, 2, 1, 1, 2_085, 1_741, 2_497])),
    (8, Metrics::declared([415, 4, 2, 1, 1, 2_273, 1_929, 2_685])),
];
const PROJECTION_COUNT: &[(u64, Metrics)] = &[
    (1, Metrics::declared([337, 8, 4, 1, 1, 2_878, 1_555, 3_378])),
    (2, Metrics::declared([353, 8, 4, 2, 1, 5_742, 1_600, 6_242])),
    (
        3,
        Metrics::declared([370, 8, 4, 3, 1, 9_603, 1_645, 10_103]),
    ),
];
const INVOCATION_COUNT: &[(u64, Metrics)] = &[
    (1, Metrics::declared([353, 8, 4, 2, 1, 5_742, 1_600, 6_242])),
    (
        2,
        Metrics::declared([706, 16, 8, 4, 2, 11_484, 3_200, 12_484]),
    ),
    (
        4,
        Metrics::declared([1_412, 32, 16, 8, 4, 22_968, 6_400, 24_968]),
    ),
    (
        8,
        Metrics::declared([2_824, 64, 32, 16, 8, 45_936, 12_800, 49_936]),
    ),
];

impl Metrics {
    const fn declared(counts: [u64; 8]) -> Self {
        let [
            source_bytes,
            vocabulary_members,
            transition_rows,
            selected_projections,
            compiler_invocations,
            generated_bytes,
            canonical_bytes,
            delivered_bytes,
        ] = counts;
        Self {
            source_bytes,
            vocabulary_members,
            transition_rows,
            selected_projections,
            compiler_invocations,
            generated_bytes,
            canonical_bytes,
            delivered_bytes,
            axis_refusals: 0,
        }
    }

    const fn refused_axis() -> Self {
        Self {
            source_bytes: 0,
            vocabulary_members: 0,
            transition_rows: 0,
            selected_projections: 0,
            compiler_invocations: 0,
            generated_bytes: 0,
            canonical_bytes: 0,
            delivered_bytes: 0,
            axis_refusals: 1,
        }
    }

    const fn counts(self) -> [u64; 9] {
        [
            self.source_bytes,
            self.vocabulary_members,
            self.transition_rows,
            self.selected_projections,
            self.compiler_invocations,
            self.generated_bytes,
            self.canonical_bytes,
            self.delivered_bytes,
            self.axis_refusals,
        ]
    }
}

#[test]
fn final_recipe_work_is_typed_and_repeated_work_is_distinguished() -> Result<(), String> {
    let table = table(Control::Repeated)?;
    let report = run_all(&table, &invocation()).map_err(debug)?;
    bench_verdict(&report).map_err(debug)?;
    assert_eq!(report.readings().len(), 5);
    for reading in report.readings() {
        let BenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            ..
        } = reading.outcome()
        else {
            return Err(format!(
                "unexpected benchmark stage: {:?}",
                reading.outcome().stage()
            ));
        };
        assert!(judgment.qualifies());
        assert_eq!(measured.points().len(), planted_worse.points().len());
        for (measured_point, planted_point) in measured.points().iter().zip(planted_worse.points())
        {
            assert_eq!(measured_point.input_size(), planted_point.input_size());
            assert_eq!(measured_point.counts().len(), planted_point.counts().len());
            for (measured_count, planted_count) in
                measured_point.counts().iter().zip(planted_point.counts())
            {
                assert_eq!(measured_count.observation(), planted_count.observation());
                assert_eq!(
                    measured_count.count().checked_mul(2),
                    Some(planted_count.count())
                );
            }
        }
    }
    Ok(())
}

#[test]
fn retained_recipe_coordinates_match_the_final_source() -> Result<(), String> {
    preflight()
}

#[test]
fn an_identical_control_is_refused_before_secondary_observation() -> Result<(), String> {
    let table = table(Control::Identical)?;
    let report = run_all(&table, &invocation()).map_err(debug)?;
    let [reading] = report.readings() else {
        return Err(String::from(
            "the identical-control table did not retain one reading",
        ));
    };
    assert_eq!(
        reading.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    Ok(())
}

fn table(control: Control) -> Result<BenchTable, String> {
    let specs: Vec<(Family, BenchCall, BenchCall)> = match control {
        Control::Identical => vec![(Family::Vocabulary, vocabulary, vocabulary)],
        Control::Repeated => vec![
            (Family::Vocabulary, vocabulary, vocabulary_worse),
            (
                Family::RelationDensity,
                relation_density,
                relation_density_worse,
            ),
            (Family::Signature, signature, signature_worse),
            (
                Family::ProjectionCount,
                projection_count,
                projection_count_worse,
            ),
            (
                Family::InvocationCount,
                invocation_count,
                invocation_count_worse,
            ),
        ],
    };
    let bindings = specs
        .into_iter()
        .map(|(family, measured, worse)| binding(family, measured, worse))
        .collect::<Result<Vec<_>, _>>()?;
    BenchTable::authored(
        BenchTableName::named(
            OWNER,
            match control {
                Control::Repeated => "final-recipe-work",
                Control::Identical => "identical-control",
            },
        )
        .map_err(debug)?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(debug)
}

fn binding(family: Family, measured: BenchCall, worse: BenchCall) -> Result<BenchBinding, String> {
    let stem = family.stem();
    let workload = WorkloadRef::named(OWNER, stem).map_err(debug)?;
    let preflight = PreflightRef::named(OWNER, family.preflight_stem()).map_err(debug)?;
    let planted = PlantedWorseRef::named(OWNER, family.repeated_stem()).map_err(debug)?;
    let complexity = ComplexityClaimRef::named(OWNER, family.complexity_stem()).map_err(debug)?;
    let row = BenchRow::declared(
        BenchReferences::declared(workload, preflight, planted, complexity),
        BenchMeasurement::declared(
            InputSizeAxis::declared(family.axes().to_vec()).map_err(debug)?,
            DeclaredBudgets::declared(1, 0, 2, 1).map_err(debug)?,
            ContentionPosture::NoDeclaredContention,
            Some(WorkFormula::encoded(FORMULA.to_vec()).map_err(debug)?),
        ),
    )
    .map_err(debug)?;
    let attachment = BenchAttachment::attached(
        workload,
        measured,
        planted,
        worse,
        WorkJudgeBinding::bound(complexity, judge),
        observations()?,
    )
    .map_err(debug)?;
    let preflight = PreflightTrial::bound(
        preflight,
        trial_binding(family, preflight_call)?,
        preflight_invocation(),
    );
    BenchBinding::bound(row, attachment, preflight).map_err(debug)
}

impl Family {
    const fn stem(self) -> &'static str {
        match self {
            Self::Vocabulary => "vocabulary",
            Self::RelationDensity => "relation-density",
            Self::Signature => "signature",
            Self::ProjectionCount => "projection-count",
            Self::InvocationCount => "invocation-count",
        }
    }

    const fn axes(self) -> &'static [u64] {
        match self {
            Self::Vocabulary => &[2, 4, 8, 16],
            Self::RelationDensity => &[2, 4, 8],
            Self::Signature | Self::InvocationCount => &[1, 2, 4, 8],
            Self::ProjectionCount => &[1, 2, 3],
        }
    }

    const fn coordinates(self) -> &'static [(u64, Metrics)] {
        match self {
            Self::Vocabulary => VOCABULARY,
            Self::RelationDensity => RELATION_DENSITY,
            Self::Signature => SIGNATURE,
            Self::ProjectionCount => PROJECTION_COUNT,
            Self::InvocationCount => INVOCATION_COUNT,
        }
    }

    const fn preflight_stem(self) -> &'static str {
        match self {
            Self::Vocabulary => "vocabulary-preflight",
            Self::RelationDensity => "relation-density-preflight",
            Self::Signature => "signature-preflight",
            Self::ProjectionCount => "projection-count-preflight",
            Self::InvocationCount => "invocation-count-preflight",
        }
    }

    const fn repeated_stem(self) -> &'static str {
        match self {
            Self::Vocabulary => "vocabulary-repeated",
            Self::RelationDensity => "relation-density-repeated",
            Self::Signature => "signature-repeated",
            Self::ProjectionCount => "projection-count-repeated",
            Self::InvocationCount => "invocation-count-repeated",
        }
    }

    const fn complexity_stem(self) -> &'static str {
        match self {
            Self::Vocabulary => "vocabulary-declared-work",
            Self::RelationDensity => "relation-density-declared-work",
            Self::Signature => "signature-declared-work",
            Self::ProjectionCount => "projection-count-declared-work",
            Self::InvocationCount => "invocation-count-declared-work",
        }
    }

    const fn claim_stem(self) -> &'static str {
        match self {
            Self::Vocabulary => "vocabulary-coordinates-match",
            Self::RelationDensity => "relation-density-coordinates-match",
            Self::Signature => "signature-coordinates-match",
            Self::ProjectionCount => "projection-count-coordinates-match",
            Self::InvocationCount => "invocation-count-coordinates-match",
        }
    }
}

fn vocabulary(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::Vocabulary, axis, 1, recorder)
}

fn vocabulary_worse(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::Vocabulary, axis, 2, recorder)
}

fn relation_density(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::RelationDensity, axis, 1, recorder)
}

fn relation_density_worse(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(Family::RelationDensity, axis, 2, recorder)
}

fn signature(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::Signature, axis, 1, recorder)
}

fn signature_worse(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::Signature, axis, 2, recorder)
}

fn projection_count(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::ProjectionCount, axis, 1, recorder)
}

fn projection_count_worse(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(Family::ProjectionCount, axis, 2, recorder)
}

fn invocation_count(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(Family::InvocationCount, axis, 1, recorder)
}

fn invocation_count_worse(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(Family::InvocationCount, axis, 2, recorder)
}

fn record(
    family: Family,
    axis: u64,
    repetitions: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let metrics = family
        .coordinates()
        .iter()
        .find_map(|(candidate, metrics)| (*candidate == axis).then_some(*metrics))
        .unwrap_or_else(Metrics::refused_axis);
    for _ in 0..repetitions {
        for (name, count) in observation_names().into_iter().zip(metrics.counts()) {
            let observation = WorkObservationRef::named(OWNER, name)
                .map_err(WorkRecordingRefusal::ObservationName)?;
            recorder.record(observation, count)?;
        }
    }
    Ok(())
}

fn judge(input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    let formula_matches = input
        .formula()
        .is_some_and(|formula| formula.bytes() == FORMULA);
    let measured_holds = formula_matches
        && input.measured().points().iter().all(|point| {
            point
                .counts()
                .last()
                .is_some_and(|count| count.count() == 0)
                && point.counts().iter().any(|count| count.count() > 0)
        });
    let repeated_holds = formula_matches
        && input.measured().points().len() == input.planted_worse().points().len()
        && input
            .measured()
            .points()
            .iter()
            .zip(input.planted_worse().points())
            .all(|(measured, planted)| {
                measured.input_size() == planted.input_size()
                    && measured.counts().len() == planted.counts().len()
                    && measured.counts().iter().zip(planted.counts()).all(
                        |(measured_count, planted_count)| {
                            measured_count.observation() == planted_count.observation()
                                && measured_count.count().checked_mul(2)
                                    == Some(planted_count.count())
                        },
                    )
            });
    WorkJudgment::stated(
        if measured_holds {
            WorkConclusion::Satisfied
        } else {
            WorkConclusion::Refused(MEASURED_REFUSED)
        },
        if repeated_holds {
            WorkConclusion::Refused(WORSE_REFUSED)
        } else {
            WorkConclusion::Satisfied
        },
        if repeated_holds {
            WorkGapStanding::Distinguished
        } else {
            WorkGapStanding::NotDistinguished(GAP_REFUSED)
        },
    )
}

fn preflight_call(_: &Invocation) -> TrialConclusion {
    if preflight().is_ok() {
        TrialConclusion::Passed
    } else {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::RefusedByCheck,
            PREFLIGHT_REFUSED,
            FindingLocation::at(file!(), line!()),
            None,
        ))
    }
}

fn preflight() -> Result<(), String> {
    let mut mismatches = Vec::new();
    for family in [
        Family::Vocabulary,
        Family::RelationDensity,
        Family::Signature,
        Family::ProjectionCount,
        Family::InvocationCount,
    ] {
        for (axis, expected) in family.coordinates() {
            let observed = observe_family(family, *axis)?;
            if observed != *expected {
                mismatches.push(format!(
                    "{} axis {axis} moved from {expected:?} to {observed:?}",
                    family.stem()
                ));
            }
        }
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(mismatches.join("\n"))
    }
}

fn observe_family(family: Family, axis: u64) -> Result<Metrics, String> {
    let axis = usize::try_from(axis).map_err(debug)?;
    match family {
        Family::Vocabulary => observe_source(&recipe_source(axis, Shape::Sparse, 0, 2)),
        Family::RelationDensity => observe_source(&recipe_source(axis, Shape::Dense, 0, 2)),
        Family::Signature => observe_source(&recipe_source(2, Shape::Sparse, axis, 1)),
        Family::ProjectionCount => observe_source(&recipe_source(4, Shape::Sparse, 0, axis)),
        Family::InvocationCount => {
            let mut total = Metrics::declared([0; 8]);
            for _ in 0..axis {
                total = add(
                    total,
                    observe_source(&recipe_source(4, Shape::Sparse, 0, 2))?,
                )?;
            }
            Ok(total)
        }
    }
}

fn add(left: Metrics, right: Metrics) -> Result<Metrics, String> {
    let sums = left
        .counts()
        .into_iter()
        .zip(right.counts())
        .map(|(left, right)| {
            left.checked_add(right)
                .ok_or_else(|| String::from("metric overflow"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [
        source,
        vocabulary,
        transitions,
        projections,
        invocations,
        generated,
        canonical,
        delivered,
        refused,
    ] = sums.as_slice()
    else {
        return Err(String::from("metric roster changed"));
    };
    Ok(Metrics {
        source_bytes: *source,
        vocabulary_members: *vocabulary,
        transition_rows: *transitions,
        selected_projections: *projections,
        compiler_invocations: *invocations,
        generated_bytes: *generated,
        canonical_bytes: *canonical,
        delivered_bytes: *delivered,
        axis_refusals: *refused,
    })
}

fn observe_source(source: &str) -> Result<Metrics, String> {
    let capture = TextCapture::read(source).map_err(debug)?;
    let bake = macroonz_compiler::recipe::bake(capture.input(), HarnessPosture::Available, &DOOR)
        .map_err(|refusal| refusal.summary().to_owned())?;
    let recipe = bake.projection().plan().content();
    let selected = [
        RecipeRole::Companions,
        RecipeRole::Dispatch,
        RecipeRole::Typestate,
    ]
    .into_iter()
    .filter(|role| recipe.effective(*role).is_some())
    .count();
    Metrics::from_bake(source, &bake, selected)
}

impl Metrics {
    fn from_bake(source: &str, bake: &RecipeBake, selected: usize) -> Result<Self, String> {
        let recipe = bake.projection().plan().content();
        let generated_bytes = bake
            .projection()
            .closure()
            .rendered()
            .units()
            .iter()
            .map(|unit| unit.tree().canonical_bytes().len())
            .sum::<usize>();
        let delivered_bytes = bake
            .emit()
            .tokens()
            .map_or(0, |tree| tree.canonical_bytes().len());
        let transition = recipe
            .transition_relation()
            .ok_or_else(|| String::from("the economics recipe has no transition relation"))?;
        let states = recipe
            .vocabulary(transition.left_vocabulary())
            .ok_or_else(|| String::from("the economics recipe has no state vocabulary"))?;
        let events = recipe
            .vocabulary(transition.right_vocabulary())
            .ok_or_else(|| String::from("the economics recipe has no event vocabulary"))?;
        let vocabulary_members = states
            .members()
            .count()
            .checked_add(events.members().count())
            .ok_or_else(|| String::from("vocabulary count overflow"))?;
        Ok(Self::declared([
            u64::try_from(source.len()).map_err(debug)?,
            u64::try_from(vocabulary_members).map_err(debug)?,
            u64::try_from(transition.row_count()).map_err(debug)?,
            u64::try_from(selected).map_err(debug)?,
            1,
            u64::try_from(generated_bytes).map_err(debug)?,
            u64::try_from(recipe.canonical_content_bytes().len()).map_err(debug)?,
            u64::try_from(delivered_bytes).map_err(debug)?,
        ]))
    }
}

fn recipe_source(vocabulary: usize, shape: Shape, generics: usize, projections: usize) -> String {
    let mut source = String::from("pub mod subject {\npub enum State {");
    for member in 0..vocabulary {
        push_indexed(&mut source, 'S', member, ",");
    }
    source.push_str("}\npub enum Event {");
    for member in 0..vocabulary {
        push_indexed(&mut source, 'E', member, ",");
    }
    source.push_str("}\nbake! {\nvocabularies { State; Event; };\ntransitions(State, Event) {");
    match shape {
        Shape::Sparse => sparse_rows(&mut source, vocabulary),
        Shape::Dense => dense_rows(&mut source, vocabulary),
    }
    source.push_str("};\nabsence(refused);\nprojections {");
    if projections == 1 && generics > 0 {
        exact_dispatch(&mut source, generics);
    } else {
        source.push_str("companions;");
        if projections >= 2 {
            source.push_str("dispatch(apply);");
        }
        if projections >= 3 {
            source.push_str("typestate(State);");
        }
    }
    source.push_str("};\n}\n}\n");
    source
}

fn sparse_rows(source: &mut String, vocabulary: usize) {
    for member in 0..vocabulary {
        let target = member
            .checked_add(1)
            .and_then(|next| next.checked_rem(vocabulary))
            .unwrap_or(0);
        source.push_str("(S");
        source.push_str(&member.to_string());
        source.push_str(", E");
        source.push_str(&member.to_string());
        source.push_str(") => S");
        source.push_str(&target.to_string());
        source.push_str(" with(crate::effect);");
    }
}

fn dense_rows(source: &mut String, vocabulary: usize) {
    for state in 0..vocabulary {
        for event in 0..vocabulary {
            let target = state
                .checked_add(event)
                .and_then(|sum| sum.checked_add(1))
                .and_then(|sum| sum.checked_rem(vocabulary))
                .unwrap_or(0);
            source.push_str("(S");
            source.push_str(&state.to_string());
            source.push_str(", E");
            source.push_str(&event.to_string());
            source.push_str(") => S");
            source.push_str(&target.to_string());
            source.push_str(" with(crate::effect);");
        }
    }
}

fn exact_dispatch(source: &mut String, generics: usize) {
    source.push_str("dispatch { pub fn apply<");
    for generic in 0..generics {
        if generic > 0 {
            source.push(',');
        }
        push_indexed(source, 'T', generic, ": Clone");
    }
    source.push_str(">(state: State, event: Event) -> Result<State, TransitionRefusal>;");
    source.push_str("};");
}

fn push_indexed(source: &mut String, prefix: char, index: usize, suffix: &str) {
    source.push(prefix);
    source.push_str(&index.to_string());
    source.push_str(suffix);
}

fn observation_names() -> [&'static str; 9] {
    [
        "source-bytes",
        "vocabulary-members",
        "transition-rows",
        "selected-projections",
        "compiler-invocations",
        "generated-bytes",
        "canonical-bytes",
        "delivered-bytes",
        "axis-refusals",
    ]
}

fn observations() -> Result<Vec<WorkObservationRef>, String> {
    observation_names()
        .into_iter()
        .map(|name| WorkObservationRef::named(OWNER, name).map_err(debug))
        .collect()
}

fn target() -> TargetBinding {
    TargetBinding::bound(
        TargetTriple::declared("x86_64-pc-windows-msvc"),
        ToolchainIdentity::declared("1.98.0"),
    )
}

fn invocation() -> BenchInvocation {
    BenchInvocation::declared(
        target(),
        HarnessClock::reading(|| CLOCK.fetch_add(10, Ordering::SeqCst)),
        ContentionPosture::NoDeclaredContention,
    )
}

fn preflight_invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(4_096),
            TimeBudget::declared(1),
        ),
        target(),
        TrialSite::located(
            module_path!(),
            file!(),
            line!(),
            "recipe-economics-preflight",
        ),
        HarnessClock::unavailable(),
    )
}

fn trial_binding(
    family: Family,
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<TrialBinding, String> {
    let stem = family.stem();
    let subject = SubjectRoute::named(OWNER, stem).map_err(debug)?;
    let check = CheckRef::named(OWNER, "golden-coordinates").map_err(debug)?;
    let row = Row::declared(
        ClaimRef::named(OWNER, family.claim_stem()).map_err(debug)?,
        ExecutionSuite::named(OWNER, "recipe-economics-preflight").map_err(debug)?,
        Classification::authored(
            vec![Role::named(OWNER, "benchmark").map_err(debug)?],
            vec![Tag::named(OWNER, "final-recipe").map_err(debug)?],
        )
        .map_err(debug)?,
        subject,
        check,
        PopulationRef::named(OWNER, "retained-coordinate-table").map_err(debug)?,
        Origin::HandWritten,
    )
    .map_err(debug)?;
    let revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, stem.as_bytes()));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, call),
        Provenance::Unproduced,
    )
    .map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
