#![doc = include_str!("README.md")]

mod type_contract;
mod types;

use super::CapturedDelimiter;
pub use types::{
    CaptureCursor, CaptureExpectation, CaptureReadIssue, CaptureReadRefusal, CapturedSpacing,
};
