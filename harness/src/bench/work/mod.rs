#![doc = include_str!("README.md")]

mod measure;
mod types;

pub(in crate::bench) use measure::{curve, judge};
pub use types::{
    BenchAttachment, BenchAttachmentRefusal, BenchCall, SecondaryObservation,
    SecondaryObservationRefusal, WorkConclusion, WorkCount, WorkCurve, WorkCurvePoint,
    WorkGapStanding, WorkJudge, WorkJudgeBinding, WorkJudgment, WorkJudgmentInput,
    WorkQualificationRefusal, WorkRecorder, WorkRecordingRefusal,
};
