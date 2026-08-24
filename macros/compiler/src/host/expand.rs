//! The whole road one proc-macro entry point is.

use super::capture::capture;
use super::emit::emit;
use super::place::place;
use super::types::Spans;
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::Kind;
use crate::token::CapturedInput;
use proc_macro::TokenStream;

/// Capture one declared input, hand it to the road that compiles it, and expand to what came back.
///
/// The road is the caller's: it reads its own grammar off the capture, states its own door, and answers with one sealed expansion or one diagnostic.
/// This host decides nothing about either — it converts what it is handed, and a refusal reaches a person as a `compile_error!` at the token the refusal itself names.
#[must_use]
pub fn expand<K: Kind>(
    input: TokenStream,
    road: impl FnOnce(CapturedInput) -> Result<Expansion<K>, Diagnostic>,
) -> TokenStream {
    let mut spans = Spans::empty();
    match capture(input, &mut spans) {
        Ok(captured) => match road(captured) {
            Ok(expansion) => emit(&expansion),
            Err(diagnostic) => place(&diagnostic, &spans),
        },
        Err(refusal) => refusal.placed(&spans),
    }
}
