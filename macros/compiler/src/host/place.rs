//! Placing one refusal as a `compile_error!` at the position it is a fact about.
//!
//! Both roads end in one composition, so a refusal the compiler established and one this host raised point at their token the same way and neither can drift into answering with the declaration's first span.

use super::types::{CaptureError, Spans};
use crate::diagnostic::Diagnostic;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// The `compile_error!` one diagnostic expands to, at the token its site names.
///
/// The line is the one the compiler composed and is only read here; this host builds no sentence.
/// A diagnostic that names no token was established before any capture issued one, and is reported at the invocation — which is the only thing about the expansion such an observation is a fact about.
#[must_use]
pub fn place(diagnostic: &Diagnostic, spans: &Spans) -> TokenStream {
    refused(diagnostic.summary(), sited(diagnostic, spans))
}

impl CaptureError {
    /// The `compile_error!` this refusal expands to, at the position it is a fact about.
    ///
    /// A magnitude is a fact about the whole declaration and no one token overran it, so it is reported at the invocation.
    /// A literal this crate could not read is a fact about exactly one token, and that token's handle was issued before its payload was read — so the span is held and the report goes there.
    ///
    /// # Nonclaims
    ///
    /// It is not a [`Diagnostic`]: composing one needs the door whose prefix, grammar, and callable entry the line carries, and a capture runs before any door is named.
    #[must_use]
    pub fn placed(self, spans: &Spans) -> TokenStream {
        let at = match &self {
            Self::Unbounded { .. } => Span::call_site(),
            Self::Unread { at, .. } => spans.at(*at),
        };
        refused(&self.to_string(), at)
    }
}

/// The compiler span one diagnostic points at.
fn sited(diagnostic: &Diagnostic, spans: &Spans) -> Span {
    diagnostic
        .site()
        .token()
        .map_or_else(Span::call_site, |handle| spans.at(handle))
}

/// One `compile_error!` at one span, carrying a line composed elsewhere.
fn refused(message: &str, span: Span) -> TokenStream {
    let mut bang = Punct::new('!', Spacing::Alone);
    bang.set_span(span);
    let mut terminator = Punct::new(';', Spacing::Alone);
    terminator.set_span(span);
    let mut line = Literal::string(message);
    line.set_span(span);
    let mut argument = Group::new(
        Delimiter::Parenthesis,
        TokenStream::from(TokenTree::Literal(line)),
    );
    argument.set_span(span);
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(bang),
        TokenTree::Group(argument),
        TokenTree::Punct(terminator),
    ]
    .into_iter()
    .collect()
}
