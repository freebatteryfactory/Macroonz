#![doc = include_str!("README.md")]

mod measure;
mod classify;
mod types;

pub(in crate::bench) use classify::stage as qualification_stage;
pub(in crate::bench) use measure::{curve, judge};
pub(in crate::bench) use types::WorkStage;
pub use types::{
    BenchAttachment, BenchAttachmentRefusal, BenchCall, SecondaryObservation,
    SecondaryObservationRefusal, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint,
    WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal,
};
