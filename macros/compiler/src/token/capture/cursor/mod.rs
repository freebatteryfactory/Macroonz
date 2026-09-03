//! The bounded mechanical reading layer over one normalized captured-token sequence.

mod read;
mod type_contract;
mod types;

use super::{CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle};
pub use types::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedSpacing,
};
