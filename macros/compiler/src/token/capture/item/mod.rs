//! The shallow structural lens over one supported complete caller-authored Rust item.

mod lens;
mod type_contract;
mod types;

use super::{CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle};
pub use types::{AuthoredItem, AuthoredItemKind, AuthoredItemReadIssue, AuthoredItemReadRefusal};
