//! The whole road one proc-macro entry point is.

use super::capture::capture;
use super::emit::emit;
use super::place::place;
use super::types::{Emittable, Spans};
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
    expand_emittable(input, road)
}

/// Capture one declared input, hand it to a road returning any compiler-owned emittable, and emit that value's declaration-site cargos.
///
/// This is the same host operation as [`expand`], generalized for a composed result whose emitted cargos come from more than one sealed expansion.
#[must_use]
pub fn expand_emittable<E: Emittable>(
    input: TokenStream,
    road: impl FnOnce(CapturedInput) -> Result<E, Diagnostic>,
) -> TokenStream {
    let mut spans = Spans::empty();
    match capture(input, &mut spans) {
        Ok(captured) => match road(captured) {
            Ok(expansion) => match emit(&expansion) {
                Ok(tokens) => tokens,
                Err(refusal) => super::place::emission_refused(&refusal),
            },
            Err(diagnostic) => place(&diagnostic, &spans),
        },
        Err(refusal) => refusal.placed(&spans),
    }
}

/// Capture two declared inputs into one span table, hand both to the road, and expand to what came back.
///
/// The shape an attribute road takes: the first stream is the attribute's own body and the second is the item it sits on, and a road that completes its reading from the item receives both captures whole.
/// One span table holds both streams' handles in capture order, so a refusal about either lands on its own token; each stream stands under the declared magnitudes on its own.
///
/// What expands is the road's answer alone.
/// The item is the author's and this host neither rewrites nor re-emits it — the caller appends the original stream after this call, exactly as it arrived.
#[must_use]
pub fn expand_on<K: Kind>(
    body: TokenStream,
    item: TokenStream,
    road: impl FnOnce(CapturedInput, CapturedInput) -> Result<Expansion<K>, Diagnostic>,
) -> TokenStream {
    let mut spans = Spans::empty();
    let captured_body = match capture(body, &mut spans) {
        Ok(captured) => captured,
        Err(refusal) => return refusal.placed(&spans),
    };
    let captured_item = match capture(item, &mut spans) {
        Ok(captured) => captured,
        Err(refusal) => return refusal.placed(&spans),
    };
    match road(captured_body, captured_item) {
        Ok(expansion) => match emit(&expansion) {
            Ok(tokens) => tokens,
            Err(refusal) => super::place::emission_refused(&refusal),
        },
        Err(diagnostic) => place(&diagnostic, &spans),
    }
}
