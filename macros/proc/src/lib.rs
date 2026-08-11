//! `threadpak-macros`: the Rust-facing expansion shell.
//!
//! The shell decides nothing. Every entry point here parses its declared
//! input and hands the work to `threadpak-macroc`, which owns the meaning.
//! The shell depends on the services and on nothing else — it reaches the
//! machine's vocabulary only through them.
//!
//! # What "semantically empty" means here
//!
//! Read the derives below and there is no grammar, no roster, no shape
//! decision, and no message. Each one captures tokens, hands their text to the
//! services, and turns what comes back into tokens: rendered source through
//! `str::parse`, or a refusal through `compile_error!` at the input's own span.
//! Every sentence a user reads was composed inside the services, where the
//! typed value it projects lives.
//!
//! The services never depend on this crate, not even for tests, so the question
//! "does a consumer wearing this derive actually compile?" is answered from
//! outside both — by the consumer fixture at `xtask/fixtures/macro-consumer`.

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use threadpak_macroc::{FrontendRole, RefusalDeriveDisposition, describe_frontend_role, disposed};

/// A no-op derive that expands to nothing.
///
/// The expansion is empty by construction: the skeleton exists to price the
/// crate topology, not to author anything. What it does prove is that the
/// shell can call the services during expansion and can see the machine's
/// public types through them.
///
/// ```
/// use threadpak_macros::ThreadpakSkeleton;
///
/// #[derive(ThreadpakSkeleton)]
/// struct Local;
/// ```
#[proc_macro_derive(ThreadpakSkeleton)]
pub fn threadpak_skeleton(item: TokenStream) -> TokenStream {
    let described = describe_frontend_role(FrontendRole::RustDeclaration);
    if described.is_empty() {
        return item;
    }
    TokenStream::new()
}

/// Derives a refusal family's declared facts from its declaration.
///
/// The name says what it is rather than what it is about: this is the derive
/// FOR `RefusalFamily`, and it also writes the typed cause order that band 00's
/// `CauseOrderDeclaration` carries. Naming it `RefusalFamily` outright would
/// have put a derive and a trait under one word in one namespace, which is
/// exactly the collision the machine's vocabulary rules forbid.
///
/// The declaration is stated with the `#[refusal(...)]` helper attribute; the
/// grammar, the shapes, the coverage rules, and every refusal live in
/// `threadpak_macroc::derive_refusal`. A malformed declaration expands to
/// `compile_error!` carrying the services' own rendering of the established
/// cause and the byte it was established at.
///
/// The expansion names the machine by absolute path, so a consumer needs the
/// machine on its own dependency list and needs nothing of the services:
///
/// ```text
/// #[derive(RefusalFamilyDerive)]
/// #[refusal(shape = single_cause, order(NotCanonical = "demo.not-canonical"))]
/// enum DemoFamily {
///     NotCanonical,
/// }
/// ```
///
/// This crate carries no dependency on the machine, so the worked example —
/// the derive applied, and its output proven equal to a hand-written twin —
/// lives where a real consumer lives: `xtask/fixtures/macro-consumer`.
#[proc_macro_derive(RefusalFamilyDerive, attributes(refusal))]
pub fn refusal_family_derive(item: TokenStream) -> TokenStream {
    let span = declared_span(&item);
    match disposed(&item.to_string()) {
        RefusalDeriveDisposition::Generated(rendered) => match rendered.source().parse() {
            Ok(expanded) => expanded,
            Err(_) => refused(
                "threadpak refusal-family derive: the rendered implementation is not \
                 well-formed Rust tokens",
                span,
            ),
        },
        RefusalDeriveDisposition::Refused(refusal) => refused(&refusal.compiler_message(), span),
    }
}

/// The span of the declared input's first token, or the call site where the
/// input is empty. The shell reports at the best position it has and invents
/// none.
fn declared_span(item: &TokenStream) -> Span {
    item.clone()
        .into_iter()
        .next()
        .map_or_else(Span::call_site, |tree| tree.span())
}

/// One `compile_error!` at the declared span, carrying a message the services
/// composed.
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
