//! The typed token seam: what the services read, and what they write.
//!
//! # Why the services carry their own token vocabulary
//!
//! `proc_macro` is a proc-macro-crate-only API. A crate that is not compiled as
//! a proc-macro cannot name its types at all, so the services — which are
//! ordinary callable Rust and must stay so — cannot take a `TokenStream` and
//! cannot hand one back. The answer is not to fall back to strings: a string is
//! a token stream with its structure thrown away, and everything the capture
//! then has to do is re-derive structure that the compiler already had.
//!
//! So the seam is typed on both sides.
//!
//! **Reading.** [`CapturedTokenTree`] is what one token of a declared input is:
//! a payload, a **stable [`TokenPath`]** naming exactly where it sits in the
//! tree, and an opaque [`SpanHandle`] indexing the producer's own span table.
//! Delimited groups stay groups; nothing is re-lexed and no balance is
//! re-discovered.
//!
//! **Every producer walks under the same declared magnitudes.** Depth, level,
//! whole-tree token count, and a capture-work budget are declared here once and
//! spent by every producer — the compiler shell and the text reader alike — so
//! "how big may a declared input be" has one answer rather than one per road.
//!
//! **Writing.** [`GeneratedTree`] is what a renderer produces. The human Rust
//! text is [`GeneratedTree::inspected`] — a PROJECTION of the tree, produced for
//! a person to read, never the artifact itself. The artifact is the tree.
//!
//! # The span handle is opaque, and deliberately so
//!
//! A [`SpanHandle`] means "the token at this index of the table the producer
//! built while capturing". The services never resolve one: they carry it into a
//! diagnostic so that whoever produced the input can map it back to the exact
//! compiler span. That is what puts a `compile_error!` on the offending token
//! rather than on the first token of the declaration.
//!
//! # The seats
//!
//! `types.rs` declares; its own child `type_guard.rs` holds every road that
//! reaches a private field, which is where all four magnitudes are settled.
//! `text.rs` is the callable text route end to end, `resolve.rs` answers a span
//! handle, `encode.rs` writes the canonical bytes, and `inspect.rs` renders what
//! a person is shown.

mod encode;
mod inspect;
mod resolve;
mod text;
mod types;

pub use types::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree,
    SpanHandle, SpanResolutionRefusal, SpanTable, TextCapture, TextReadCause, TextReadRefusal,
    TokenPath,
};
