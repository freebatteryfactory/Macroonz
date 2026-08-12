//! `threadpak-macros`: the Rust-facing expansion shell.
//!
//! The shell decides nothing. It does exactly three things, and none of them is
//! a decision:
//!
//! 1. **Convert.** `proc_macro::TokenStream` becomes
//!    [`threadpak_macroc::CapturedInput`], walking the compiler's own token trees
//!    natively and issuing one [`SpanHandle`] per token into a table the shell
//!    keeps.
//! 2. **Call.** [`threadpak_macroc::compile_refusal`] does the work and returns
//!    either a closed expansion or a diagnostic.
//! 3. **Emit.** The closed expansion's token tree becomes a `TokenStream`, or the
//!    diagnostic becomes a `compile_error!` at the exact token its span handle
//!    names.
//!
//! # What "semantically empty" means here
//!
//! Read the derive below and there is no grammar, no roster, no shape decision,
//! no identity, no plan, and no message. Every sentence a user reads was
//! composed inside the services, where the typed value it projects lives. The
//! shell does not even build the string it emits: the services hand it tokens.
//!
//! # The offending token, not the first one
//!
//! The shell keeps a span table while converting, so a diagnostic's
//! [`SpanHandle`] resolves to the exact `proc_macro::Span` of the token the
//! services refused at. `compile_error!` is emitted at that span. A shell that
//! reported at `token[0]` would send every reader of every refusal to the same
//! wrong place.
//!
//! The services never resolve a handle themselves — they cannot, because
//! `proc_macro` is a proc-macro-crate-only API and the services are ordinary
//! callable Rust. That split is the reason the seam is a handle rather than a
//! span.
//!
//! # Dependency-minimized, and it stays that way
//!
//! Nothing here depends on `proc-macro2`, on `syn`, or on `quote`. The
//! conversion below walks `TokenTree` values with the standard library alone.
//!
//! The services never depend on this crate, not even for tests, so the question
//! "does a consumer wearing this derive actually compile?" is answered from
//! outside both — by the consumer fixture at `xtask/fixtures/macro-consumer`.

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use threadpak_macroc::{
    CapturedDelimiter, CapturedInput, CapturedPayload, CapturedTokenTree, ClosedExpansion,
    GeneratedDelimiter, GeneratedSpacing, GeneratedToken, GeneratedTree, LocalCoordinate,
    MacrocDiagnostic, RefusalCompileContext, SpanHandle, compile_refusal,
};

/// Derives a refusal family's declared facts from its declaration.
///
/// The macro namespace and the type namespace are different namespaces, so a
/// derive named `RefusalFamily` and a trait named `RefusalFamily` do not
/// collide: `#[derive(RefusalFamily)]` resolves in the macro namespace and
/// `impl RefusalFamily for …` resolves in the type namespace. Naming the derive
/// for the contract it derives is the honest name, and a consumer that imports
/// both may always spell either one path-qualified.
///
/// The declaration is stated with the `#[refusal(...)]` helper attribute; the
/// grammar, the shapes, the coverage rules, the identities, the plan, the
/// rendering, the closure, and every refusal live in
/// `threadpak_macroc::derive_refusal`. A malformed declaration expands to
/// `compile_error!` at the offending token, carrying the services' own rendering
/// of the established cause.
///
/// ```text
/// #[derive(RefusalFamily)]
/// #[refusal(
///     family = "demo.example",
///     shape = single_cause,
///     order(NotCanonical = "not-canonical"),
/// )]
/// enum DemoFamily {
///     NotCanonical,
/// }
/// ```
///
/// This crate carries no dependency on the machine, so the worked example —
/// the derive applied, and its output proven equal to a hand-written twin —
/// lives where a real consumer lives: `xtask/fixtures/macro-consumer`.
#[proc_macro_derive(RefusalFamily, attributes(refusal))]
pub fn refusal_family(item: TokenStream) -> TokenStream {
    let mut spans: Vec<Span> = Vec::new();
    let Ok(trees) = capture_stream(item, 0, &mut spans) else {
        return refused(
            "threadpak refusal-family derive: the declared input exceeds a declared magnitude",
            call_site(&spans),
        );
    };
    let issued = u32::try_from(spans.len()).unwrap_or(u32::MAX);
    let Ok(input) = CapturedInput::taken(trees, issued) else {
        return refused(
            "threadpak refusal-family derive: the declared input exceeds a declared magnitude",
            call_site(&spans),
        );
    };
    match compile_refusal(&input, &RefusalCompileContext::expanding()) {
        Ok(closed) => emit(&closed),
        Err(diagnostic) => refused(&message(&diagnostic), site(&diagnostic, &spans)),
    }
}

/// The closed expansion's token tree, as the compiler's tokens. The shell's only
/// act.
fn emit(closed: &ClosedExpansion) -> TokenStream {
    emit_tree(closed.emitted())
}

/// One generated tree as the compiler's tokens.
fn emit_tree(tree: &GeneratedTree) -> TokenStream {
    tree.tokens().map(emit_token).collect()
}

/// One generated token as the compiler's token.
fn emit_token(token: &GeneratedToken) -> TokenTree {
    match token {
        GeneratedToken::Word(word) => TokenTree::Ident(Ident::new(word, Span::call_site())),
        GeneratedToken::Punct { mark, spacing } => TokenTree::Punct(Punct::new(
            *mark,
            match spacing {
                GeneratedSpacing::Joint => Spacing::Joint,
                GeneratedSpacing::Alone => Spacing::Alone,
            },
        )),
        GeneratedToken::Text(text) => TokenTree::Literal(Literal::string(text)),
        GeneratedToken::Group { delimiter, tokens } => TokenTree::Group(Group::new(
            match delimiter {
                GeneratedDelimiter::Parenthesis => Delimiter::Parenthesis,
                GeneratedDelimiter::Brace => Delimiter::Brace,
                GeneratedDelimiter::Bracket => Delimiter::Bracket,
            },
            tokens.iter().map(emit_token).collect(),
        )),
    }
}

/// Convert one token stream into captured trees, issuing a span handle per
/// token into the shell's own table.
fn capture_stream(
    stream: TokenStream,
    depth: u32,
    spans: &mut Vec<Span>,
) -> Result<Vec<CapturedTokenTree>, Unbounded> {
    let mut captured = Vec::new();
    for (index, tree) in stream.into_iter().enumerate() {
        let coordinate = LocalCoordinate {
            depth,
            index: u32::try_from(index).unwrap_or(u32::MAX),
        };
        captured.push(capture_tree(&tree, coordinate, depth, spans)?);
    }
    Ok(captured)
}

/// The one way converting refuses: a group larger than the declared magnitude.
/// The shell reports it rather than capturing a truncated group, because a
/// truncated group is a different declaration.
struct Unbounded;

/// Convert one token tree, issuing its handle first so the handle order matches
/// the reading order.
fn capture_tree(
    tree: &TokenTree,
    coordinate: LocalCoordinate,
    depth: u32,
    spans: &mut Vec<Span>,
) -> Result<CapturedTokenTree, Unbounded> {
    let handle = issue(tree.span(), spans);
    let payload = match tree {
        TokenTree::Ident(ident) => CapturedPayload::Word(ident.to_string()),
        TokenTree::Punct(punct) => CapturedPayload::Punct(punct.as_char()),
        TokenTree::Literal(literal) => literal_payload(&literal.to_string()),
        TokenTree::Group(group) => {
            let inner = capture_stream(group.stream(), depth.saturating_add(1), spans)?;
            let delimiter = match group.delimiter() {
                Delimiter::Parenthesis => CapturedDelimiter::Parenthesis,
                Delimiter::Brace => CapturedDelimiter::Brace,
                Delimiter::Bracket => CapturedDelimiter::Bracket,
                Delimiter::None => CapturedDelimiter::Bare,
            };
            return CapturedTokenTree::group_of(delimiter, inner, coordinate, handle)
                .map_err(|_| Unbounded);
        }
    };
    Ok(CapturedTokenTree::captured(payload, coordinate, handle))
}

/// The payload one literal token carries. A quoted text becomes a text payload
/// with its quotes removed; anything else is a number as written.
fn literal_payload(spelling: &str) -> CapturedPayload {
    match spelling
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
    {
        Some(inner) => CapturedPayload::Text(inner.to_owned()),
        None => CapturedPayload::Number(spelling.to_owned()),
    }
}

/// Issue the next handle for one span.
fn issue(span: Span, spans: &mut Vec<Span>) -> SpanHandle {
    let index = u32::try_from(spans.len()).unwrap_or(u32::MAX);
    spans.push(span);
    SpanHandle::at(index)
}

/// The compiler span one diagnostic's handle names, or the call site where the
/// table does not reach it.
fn site(diagnostic: &MacrocDiagnostic, spans: &[Span]) -> Span {
    let index = usize::try_from(diagnostic.site.token.index()).unwrap_or(usize::MAX);
    spans
        .get(index)
        .copied()
        .unwrap_or_else(|| call_site(spans))
}

/// The declaration's own first span, or the call site where nothing was
/// captured.
fn call_site(spans: &[Span]) -> Span {
    spans.first().copied().unwrap_or_else(Span::call_site)
}

/// The one line a `compile_error!` carries. Composed inside the services and
/// only read here.
fn message(diagnostic: &MacrocDiagnostic) -> String {
    diagnostic.summary.shown()
}

/// One `compile_error!` at one span, carrying a message the services composed.
fn refused(message: &str, span: Span) -> TokenStream {
    let mut bang = Punct::new('!', Spacing::Alone);
    bang.set_span(span);
    let mut terminator = Punct::new(';', Spacing::Alone);
    terminator.set_span(span);
    let mut literal = Literal::string(message);
    literal.set_span(span);
    let mut argument = Group::new(
        Delimiter::Parenthesis,
        TokenStream::from(TokenTree::Literal(literal)),
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
