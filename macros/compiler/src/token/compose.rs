//! Composing the Rust a renderer writes, out of the tokens that spell it.
//!
//! A renderer states what it means — a path, a call, a binding, an attribute — and never assembles punctuation by hand.
//! A path stated as segments cannot be mis-spaced and cannot lose a colon; a call stated as a path and its arguments cannot lose a parenthesis; and neither can be built out of a string somebody supplied.
//!
//! Every helper here composes and never bounds.
//! The declared magnitude bites only where a group closes, so exactly the helpers that write a group return [`Overflow`] and the rest are total.

use super::{GeneratedDelimiter, GeneratedToken};
use crate::bounded::Overflow;

/// Whether one spelling is a single Rust identifier a rendering is willing to write.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern rather than a name.
/// ONE alphabet, seated with the token home, for every spelling any home renders in identifier position — a path segment, an exported address, a stamped item's own name, a declared grammar's word.
/// A second copy would agree with this one until one of them was edited, and the failure would surface in a consumer's build with no idea where the name came from; the homes that once each carried a copy now all read this seat.
#[must_use]
pub fn rendered_identifier(spelling: &str) -> bool {
    let mut characters = spelling.chars();
    let Some(head) = characters.next() else {
        return false;
    };
    if !head.is_ascii_alphabetic() && head != '_' {
        return false;
    }
    if spelling == "_" {
        return false;
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// Whether one spelling is a Rust keyword no rendered item can be named by.
///
/// The language's own roster — the strict and reserved keywords through edition 2024 — written down once beside the identifier alphabet, because it is the same law from the other side: an alphabet says which spellings CAN be a name, and this roster says which of those the language already took.
/// A grammar that let a keyword through would refuse nowhere and hand the collision to the adopter's build, inside an expansion whose lints rustc has silenced.
#[must_use]
pub fn rust_keyword(spelling: &str) -> bool {
    matches!(
        spelling,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "gen"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}

/// One delimited group.
///
/// # Errors
///
/// Returns [`Overflow`] where the group carries more tokens than the declared magnitude admits.
pub fn group(
    delimiter: GeneratedDelimiter,
    tokens: Vec<GeneratedToken>,
) -> Result<GeneratedToken, Overflow> {
    GeneratedToken::group(delimiter, tokens)
}

/// One macro metavariable, as the two tokens that spell it.
///
/// The `$` is written joint, so the projection a person reads is `$name` rather than `$ name`.
#[must_use]
pub fn metavariable(name: &str) -> Vec<GeneratedToken> {
    vec![GeneratedToken::joint('$'), GeneratedToken::word(name)]
}

/// The absolute path `::a::b::c`.
#[must_use]
pub fn absolute_path(segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    extend_path(&mut tokens, segments);
    tokens
}

/// The path `root::a::b::c`, rooted at a crate the caller named.
///
/// The root is written as a plain word, so a caller that renamed its dependency is named the way it named itself.
#[must_use]
pub fn bound_path(root: &str, segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(root)];
    extend_path(&mut tokens, segments);
    tokens
}

/// The path `$binding::a::b::c`, rooted at a metavariable a macro shell will bind.
#[must_use]
pub fn twin_path(binding: &str, segments: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = metavariable(binding);
    extend_path(&mut tokens, segments);
    tokens
}

/// Write `::segment` for each segment onto a path being built.
fn extend_path(tokens: &mut Vec<GeneratedToken>, segments: &[&str]) {
    for segment in segments {
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(GeneratedToken::word(segment));
    }
}

/// One call `path(arguments)`.
///
/// # Errors
///
/// Returns [`Overflow`] where the argument list outgrows the declared magnitude.
pub fn call(
    mut path: Vec<GeneratedToken>,
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    path.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    Ok(path)
}

/// One call `receiver.method(arguments)`.
///
/// # Errors
///
/// Returns [`Overflow`] where the argument list outgrows the declared magnitude.
pub fn method_call(
    mut receiver: Vec<GeneratedToken>,
    method: &str,
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    receiver.push(GeneratedToken::alone('.'));
    receiver.push(GeneratedToken::word(method));
    call(receiver, arguments)
}

/// One chain `receiver.first().second().third()`, every method taking no argument.
///
/// # Errors
///
/// Returns [`Overflow`] where a call in the chain outgrows the declared magnitude.
pub fn method_chain(
    mut receiver: Vec<GeneratedToken>,
    methods: &[&str],
) -> Result<Vec<GeneratedToken>, Overflow> {
    for method in methods {
        receiver = method_call(receiver, method, Vec::new())?;
    }
    Ok(receiver)
}

/// One statement `let name = expression;`.
///
/// A value a rendered block needs twice is bound once, which makes the agreement between its two readers structural rather than a comparison of two separately built values.
#[must_use]
pub fn bound_local(name: &str, expression: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(name),
        GeneratedToken::alone('='),
    ];
    tokens.extend(expression);
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// The type `::core::result::Result<ok, error>`.
#[must_use]
pub fn result_type(ok: Vec<GeneratedToken>, error: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = absolute_path(&["core", "result", "Result"]);
    tokens.push(GeneratedToken::alone('<'));
    tokens.extend(ok);
    tokens.push(GeneratedToken::alone(','));
    tokens.extend(error);
    tokens.push(GeneratedToken::alone('>'));
    tokens
}

/// One item `const name: kind = value;`.
///
/// The visibility is the caller's and is written before this.
#[must_use]
pub fn constant(
    name: &str,
    kind: Vec<GeneratedToken>,
    value: Vec<GeneratedToken>,
) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word("const"), GeneratedToken::word(name)];
    tokens.push(GeneratedToken::alone(':'));
    tokens.extend(kind);
    tokens.push(GeneratedToken::alone('='));
    tokens.extend(value);
    tokens.push(GeneratedToken::alone(';'));
    tokens
}

/// One item `fn name(parameters) -> result { body }`.
///
/// The visibility is the caller's and is written before this, so one helper spells a private function, a `pub` one, and a `pub(crate)` one.
///
/// # Errors
///
/// Returns [`Overflow`] where the parameter list or the body outgrows the declared magnitude.
pub fn function(
    name: &str,
    parameters: Vec<GeneratedToken>,
    result: Vec<GeneratedToken>,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = vec![
        GeneratedToken::word("fn"),
        GeneratedToken::word(name),
        group(GeneratedDelimiter::Parenthesis, parameters)?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
    ];
    tokens.extend(result);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// One comparison `left == right`.
#[must_use]
pub fn equality(mut left: Vec<GeneratedToken>, right: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    left.push(GeneratedToken::joint('='));
    left.push(GeneratedToken::alone('='));
    left.extend(right);
    left
}

/// Every comparison joined by `&&`.
#[must_use]
pub fn and_all(comparisons: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, comparison) in comparisons.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::joint('&'));
            tokens.push(GeneratedToken::alone('&'));
        }
        tokens.extend(comparison);
    }
    tokens
}

/// Two token runs separated by a comma.
#[must_use]
pub fn comma(mut left: Vec<GeneratedToken>, right: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    left.push(GeneratedToken::alone(','));
    left.extend(right);
    left
}

/// Every token run separated by a comma, with no trailing one.
#[must_use]
pub fn comma_many(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone(','));
        }
        tokens.extend(part);
    }
    tokens
}

/// Two spellings as the two comma-separated text literals a two-argument parser takes.
#[must_use]
pub fn text_pair(first: &str, second: &str) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::text(first),
        GeneratedToken::alone(','),
        GeneratedToken::text(second),
    ]
}

/// One attribute `#[body]`.
///
/// # Errors
///
/// Returns [`Overflow`] where the body outgrows the declared magnitude.
pub fn attribute(body: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    Ok(vec![
        GeneratedToken::alone('#'),
        group(GeneratedDelimiter::Bracket, body)?,
    ])
}

/// One documentation attribute over one sentence.
///
/// # Errors
///
/// Returns [`Overflow`] where the attribute outgrows the declared magnitude.
pub fn documentation(sentence: &str) -> Result<Vec<GeneratedToken>, Overflow> {
    attribute(vec![
        GeneratedToken::word("doc"),
        GeneratedToken::alone('='),
        GeneratedToken::text(sentence),
    ])
}

/// The `::std::vec![…]` a roster-taking constructor is handed.
///
/// # Errors
///
/// Returns [`Overflow`] where the roster outgrows the declared magnitude.
pub fn roster(items: Vec<GeneratedToken>) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = absolute_path(&["std", "vec"]);
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Bracket, items)?);
    Ok(tokens)
}
