#![doc = include_str!("README.md")]

mod declaration;
mod execute;
mod stamp;
mod timed;
mod type_contract;
mod types;
mod verdict;
mod work;

pub use execute::run_all;
pub use types::{
    BENCH_ROW_KEY_TAG, BenchAttachment, BenchAttachmentRefusal, BenchBinding, BenchBindingRefusal,
    BenchCall, BenchInvocation, BenchMeasurement, BenchOutcome, BenchReading, BenchReferences,
    BenchReport, BenchRow, BenchRowKey, BenchRowRefusal, BenchRunRefusal, BenchStage,
    BenchStampRefusal, BenchTable, BenchTableName, BenchTableRefusal, BenchTargetMismatch,
    BenchVerdictRefusal, ComplexityClaimRef, ContentionPosture, DeclaredBudgets,
    DeclaredBudgetsRefusal, ExactRatio, InputSizeAxis, InputSizeAxisRefusal, PlantedWorseRef,
    PreflightRef, PreflightTrial, PrimaryWorkPhase, SecondaryObservation,
    SecondaryObservationRefusal, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint, WorkFormula,
    WorkFormulaRefusal, WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment,
    WorkJudgmentInput, WorkObservationRef, WorkQualificationRefusal, WorkRecorder,
    WorkRecordingRefusal, WorkloadRef,
};
pub use verdict::bench_verdict;
