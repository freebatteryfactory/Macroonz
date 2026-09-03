#![doc = include_str!("README.md")]

mod read;
mod type_contract;
mod types;

use super::{CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle};
pub use types::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedSpacing,
};
