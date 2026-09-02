//! Remaining recipe axes through the benchmark owner's planted-worse protocol.

use super::breadth;
use macroonz_harness::bench::{
    BenchAttachment, BenchBinding, BenchCall, BenchMeasurement, BenchOutcome, BenchReferences,
    BenchRow, BenchStage, BenchTable, BenchTableName, ComplexityClaimRef, ContentionPosture,
    DeclaredBudgets, InputSizeAxis, PlantedWorseRef, PreflightRef, PreflightTrial, WorkFormula,
    WorkJudgeBinding, WorkObservationRef, WorkRecorder, WorkRecordingRefusal, WorkloadRef,
    bench_verdict, run_all,
};
use macroonz_harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag,
};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding,
};
use macroonz_harness::runner::{Invocation, TrialBinding};

const REVISION_TAG: DomainTag = DomainTag::declared(
    "recipe-economics-breadth-revision",
    IdentityProfileVersion::declared(1),
);
const PREFLIGHT_REFUSED: FindingCause =
    FindingCause::named(super::OWNER, "breadth-preflight-refused");
const REFUSED_COUNTS: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

#[derive(Clone, Copy)]
enum BreadthFamily {
    StructuralQuestions,
    CodecFields,
    PayloadPath,
    ProjectionCatalog,
    TestCarriers,
}

#[derive(Clone, Copy)]
enum Control {
    Repeated,
    Identical,
}

#[test]
fn remaining_axes_use_typed_work_and_a_planted_worse_control() -> Result<(), String> {
    let report = run_all(&table(Control::Repeated)?, &super::invocation()).map_err(super::debug)?;
    bench_verdict(&report).map_err(super::debug)?;
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
                "unexpected breadth benchmark stage: {:?}",
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
fn an_identical_breadth_control_is_refused_before_secondary_observation() -> Result<(), String> {
    let report =
        run_all(&table(Control::Identical)?, &super::invocation()).map_err(super::debug)?;
    let [reading] = report.readings() else {
        return Err(String::from(
            "the identical breadth table did not retain one reading",
        ));
    };
    assert_eq!(
        reading.outcome().stage(),
        BenchStage::PlantedWorseNotDistinguished
    );
    Ok(())
}

fn table(control: Control) -> Result<BenchTable, String> {
    let specs: Vec<(BreadthFamily, BenchCall, BenchCall)> = match control {
        Control::Identical => vec![(
            BreadthFamily::StructuralQuestions,
            structural_questions,
            structural_questions,
        )],
        Control::Repeated => vec![
            (
                BreadthFamily::StructuralQuestions,
                structural_questions,
                structural_questions_worse,
            ),
            (BreadthFamily::CodecFields, codec_fields, codec_fields_worse),
            (BreadthFamily::PayloadPath, payload_path, payload_path_worse),
            (
                BreadthFamily::ProjectionCatalog,
                projection_catalog,
                projection_catalog_worse,
            ),
            (
                BreadthFamily::TestCarriers,
                test_carriers,
                test_carriers_worse,
            ),
        ],
    };
    let bindings = specs
        .into_iter()
        .map(|(family, measured, worse)| binding(family, measured, worse))
        .collect::<Result<Vec<_>, _>>()?;
    BenchTable::authored(
        BenchTableName::named(
            super::OWNER,
            match control {
                Control::Repeated => "final-recipe-breadth",
                Control::Identical => "identical-breadth-control",
            },
        )
        .map_err(super::debug)?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(super::debug)
}

fn binding(
    family: BreadthFamily,
    measured: BenchCall,
    worse: BenchCall,
) -> Result<BenchBinding, String> {
    let stem = family.stem();
    let workload = WorkloadRef::named(super::OWNER, stem).map_err(super::debug)?;
    let preflight =
        PreflightRef::named(super::OWNER, family.preflight_stem()).map_err(super::debug)?;
    let planted =
        PlantedWorseRef::named(super::OWNER, family.repeated_stem()).map_err(super::debug)?;
    let complexity =
        ComplexityClaimRef::named(super::OWNER, family.complexity_stem()).map_err(super::debug)?;
    let row = BenchRow::declared(
        BenchReferences::declared(workload, preflight, planted, complexity),
        BenchMeasurement::declared(
            InputSizeAxis::declared(family.axes().to_vec()).map_err(super::debug)?,
            DeclaredBudgets::declared(1, 0, 2, 1).map_err(super::debug)?,
            ContentionPosture::NoDeclaredContention,
            Some(WorkFormula::encoded(super::FORMULA.to_vec()).map_err(super::debug)?),
        ),
    )
    .map_err(super::debug)?;
    let attachment = BenchAttachment::attached(
        workload,
        measured,
        planted,
        worse,
        WorkJudgeBinding::bound(complexity, super::judge),
        observations()?,
    )
    .map_err(super::debug)?;
    let preflight = PreflightTrial::bound(
        preflight,
        trial_binding(family, preflight_call)?,
        super::preflight_invocation(),
    );
    BenchBinding::bound(row, attachment, preflight).map_err(super::debug)
}

impl BreadthFamily {
    const fn stem(self) -> &'static str {
        match self {
            Self::StructuralQuestions => "structural-questions",
            Self::CodecFields => "codec-fields",
            Self::PayloadPath => "payload-path",
            Self::ProjectionCatalog => "projection-catalog",
            Self::TestCarriers => "test-carriers",
        }
    }

    const fn axes(self) -> &'static [u64] {
        match self {
            Self::StructuralQuestions | Self::CodecFields | Self::PayloadPath => &[1, 2, 4, 8],
            Self::ProjectionCatalog => &[1, 2, 3, 4, 5],
            Self::TestCarriers => &[1, 2],
        }
    }

    fn source(self, axis: u64) -> Result<String, String> {
        let axis = usize::try_from(axis).map_err(super::debug)?;
        Ok(match self {
            Self::StructuralQuestions => breadth::posture_source(axis),
            Self::CodecFields => breadth::codec_source(axis),
            Self::PayloadPath => breadth::payload_path_source(axis),
            Self::ProjectionCatalog => breadth::projection_catalog_source(axis),
            Self::TestCarriers => breadth::carrier_source(axis),
        })
    }

    fn counts(self, axis: u64) -> Result<[u64; 16], String> {
        let source = self.source(axis)?;
        breadth::observe_counts(source.as_str())
    }

    const fn preflight_stem(self) -> &'static str {
        match self {
            Self::StructuralQuestions => "structural-questions-preflight",
            Self::CodecFields => "codec-fields-preflight",
            Self::PayloadPath => "payload-path-preflight",
            Self::ProjectionCatalog => "projection-catalog-preflight",
            Self::TestCarriers => "test-carriers-preflight",
        }
    }

    const fn repeated_stem(self) -> &'static str {
        match self {
            Self::StructuralQuestions => "structural-questions-repeated",
            Self::CodecFields => "codec-fields-repeated",
            Self::PayloadPath => "payload-path-repeated",
            Self::ProjectionCatalog => "projection-catalog-repeated",
            Self::TestCarriers => "test-carriers-repeated",
        }
    }

    const fn complexity_stem(self) -> &'static str {
        match self {
            Self::StructuralQuestions => "structural-questions-declared-work",
            Self::CodecFields => "codec-fields-declared-work",
            Self::PayloadPath => "payload-path-declared-work",
            Self::ProjectionCatalog => "projection-catalog-declared-work",
            Self::TestCarriers => "test-carriers-declared-work",
        }
    }

    const fn claim_stem(self) -> &'static str {
        match self {
            Self::StructuralQuestions => "structural-question-coordinates-match",
            Self::CodecFields => "codec-field-coordinates-match",
            Self::PayloadPath => "payload-path-coordinates-match",
            Self::ProjectionCatalog => "projection-catalog-coordinates-match",
            Self::TestCarriers => "test-carrier-coordinates-match",
        }
    }
}

fn structural_questions(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::StructuralQuestions, axis, 1, recorder)
}

fn structural_questions_worse(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::StructuralQuestions, axis, 2, recorder)
}

fn codec_fields(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::CodecFields, axis, 1, recorder)
}

fn codec_fields_worse(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::CodecFields, axis, 2, recorder)
}

fn payload_path(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::PayloadPath, axis, 1, recorder)
}

fn payload_path_worse(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::PayloadPath, axis, 2, recorder)
}

fn projection_catalog(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::ProjectionCatalog, axis, 1, recorder)
}

fn projection_catalog_worse(
    axis: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::ProjectionCatalog, axis, 2, recorder)
}

fn test_carriers(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::TestCarriers, axis, 1, recorder)
}

fn test_carriers_worse(axis: u64, recorder: &mut WorkRecorder) -> Result<(), WorkRecordingRefusal> {
    record(BreadthFamily::TestCarriers, axis, 2, recorder)
}

fn record(
    family: BreadthFamily,
    axis: u64,
    repetitions: u64,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let counts = family.counts(axis).unwrap_or(REFUSED_COUNTS);
    for _ in 0..repetitions {
        for (name, count) in observation_names().into_iter().zip(counts) {
            let observation = WorkObservationRef::named(super::OWNER, name)
                .map_err(WorkRecordingRefusal::ObservationName)?;
            recorder.record(observation, count)?;
        }
    }
    Ok(())
}

fn observation_names() -> [&'static str; 16] {
    [
        "breadth-source-bytes",
        "breadth-vocabularies",
        "breadth-variants",
        "breadth-relations",
        "breadth-relation-rows",
        "breadth-codecs",
        "breadth-codec-members",
        "breadth-selected-roles",
        "breadth-planned-units",
        "breadth-rendered-units",
        "breadth-explanation-answers",
        "breadth-generated-bytes",
        "breadth-recipe-bytes",
        "breadth-delivered-bytes",
        "breadth-test-carrier-bytes",
        "breadth-axis-refusals",
    ]
}

fn observations() -> Result<Vec<WorkObservationRef>, String> {
    observation_names()
        .into_iter()
        .map(|name| WorkObservationRef::named(super::OWNER, name).map_err(super::debug))
        .collect()
}

fn preflight_call(_: &Invocation) -> TrialConclusion {
    if breadth::verify_breadth().is_ok() {
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

fn trial_binding(
    family: BreadthFamily,
    call: fn(&Invocation) -> TrialConclusion,
) -> Result<TrialBinding, String> {
    let stem = family.stem();
    let subject = SubjectRoute::named(super::OWNER, stem).map_err(super::debug)?;
    let check = CheckRef::named(super::OWNER, "breadth-coordinates").map_err(super::debug)?;
    let row = Row::declared(
        ClaimRef::named(super::OWNER, family.claim_stem()).map_err(super::debug)?,
        ExecutionSuite::named(super::OWNER, "recipe-economics-breadth-preflight")
            .map_err(super::debug)?,
        Classification::authored(
            vec![Role::named(super::OWNER, "benchmark").map_err(super::debug)?],
            vec![Tag::named(super::OWNER, "final-recipe-breadth").map_err(super::debug)?],
        )
        .map_err(super::debug)?,
        subject,
        check,
        PopulationRef::named(super::OWNER, "breadth-coordinate-table").map_err(super::debug)?,
        Origin::HandWritten,
    )
    .map_err(super::debug)?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        family.stem().as_bytes(),
    ));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, call),
        Provenance::Unproduced,
    )
    .map_err(super::debug)
}
