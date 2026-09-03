//! Remaining recipe axes through the benchmark owner's planted-worse protocol.

use super::bench_driver::{Control, WorkFamily};
use super::breadth;
use macroonz_harness::bench::{BenchCall, BenchTable, WorkRecorder, WorkRecordingRefusal, run_all};
use macroonz_harness::identity::{DomainTag, IdentityProfileVersion};
use macroonz_harness::report::FindingCause;

const REVISION_TAG: DomainTag = DomainTag::declared(
    "recipe-economics-breadth-revision",
    IdentityProfileVersion::declared(1),
);
const PREFLIGHT_REFUSED: FindingCause =
    FindingCause::named(super::OWNER, "breadth-preflight-refused");

#[derive(Clone, Copy)]
enum BreadthFamily {
    StructuralQuestions,
    CodecFields,
    PayloadPath,
    ProjectionCatalog,
    TestCarriers,
}

#[test]
fn remaining_axes_use_typed_work_and_a_planted_worse_control() -> Result<(), String> {
    let report = run_all(&table(Control::Repeated)?, &super::invocation()).map_err(super::debug)?;
    super::bench_driver::assert_repeated(&report, 5, "unexpected breadth benchmark stage")
}

#[test]
fn an_identical_breadth_control_is_refused_before_secondary_observation() -> Result<(), String> {
    let report =
        run_all(&table(Control::Identical)?, &super::invocation()).map_err(super::debug)?;
    super::bench_driver::assert_identical(
        &report,
        "the identical breadth table did not retain one reading",
    )
}

fn table(control: Control) -> Result<BenchTable, String> {
    let first: (BreadthFamily, BenchCall, BenchCall) = (
        BreadthFamily::StructuralQuestions,
        structural_questions,
        structural_questions_worse,
    );
    let remaining: &[(BreadthFamily, BenchCall, BenchCall)] = &[
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
    ];
    super::bench_driver::table(
        control,
        "final-recipe-breadth",
        "identical-breadth-control",
        first,
        remaining,
    )
}

impl BreadthFamily {
    const REFUSED_COUNTS: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

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

impl WorkFamily for BreadthFamily {
    const CHECK: &'static str = "breadth-coordinates";
    const EXECUTION_SUITE: &'static str = "recipe-economics-breadth-preflight";
    const POPULATION: &'static str = "breadth-coordinate-table";
    const PREFLIGHT_REFUSED: FindingCause = PREFLIGHT_REFUSED;
    const REVISION_TAG: DomainTag = REVISION_TAG;
    const TAG: &'static str = "final-recipe-breadth";

    fn stem(self) -> &'static str {
        Self::stem(self)
    }

    fn axes(self) -> &'static [u64] {
        Self::axes(self)
    }

    fn preflight_stem(self) -> &'static str {
        Self::preflight_stem(self)
    }

    fn repeated_stem(self) -> &'static str {
        Self::repeated_stem(self)
    }

    fn complexity_stem(self) -> &'static str {
        Self::complexity_stem(self)
    }

    fn claim_stem(self) -> &'static str {
        Self::claim_stem(self)
    }

    fn counts(self, axis: u64) -> Vec<u64> {
        Self::counts(self, axis)
            .unwrap_or(Self::REFUSED_COUNTS)
            .to_vec()
    }

    fn observation_names() -> &'static [&'static str] {
        &[
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

    fn preflight() -> Result<(), String> {
        breadth::verify_breadth()
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
    super::bench_driver::record(family, axis, repetitions, recorder)
}
