//! The Rust-language name rosters consumed by token capture and generation.

/// The strict and reserved Rust keywords through edition 2024.
const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// The spellings Rust does not admit behind the raw-identifier marker.
const RAW_IDENTIFIER_EXCLUSIONS: &[&str] = &["", "_", "crate", "self", "Self", "super"];

/// Whether one spelling is a Rust keyword no rendered item can be named by.
///
/// The language's own roster is written down once beside the token vocabulary, because capture and generation must not drift on which spellings Rust has already taken.
/// A grammar that let a keyword through would refuse nowhere and hand the collision to the adopter's build, inside an expansion whose lints rustc has silenced.
#[must_use]
pub fn rust_keyword(spelling: &str) -> bool {
    RUST_KEYWORDS.contains(&spelling)
}

/// Whether one lexer-admitted raw name is forbidden by Rust's raw-identifier grammar.
pub(crate) fn raw_identifier_is_reserved(name: &str) -> bool {
    RAW_IDENTIFIER_EXCLUSIONS.contains(&name)
}
