//! Judging the identifier spellings a renderer may write.

use crate::token::bank::rust_keyword;

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

/// Whether one spelling can NAME a rendered item: a single identifier the language has not already taken.
///
/// The identifier alphabet here and the keyword bank are the two halves of one law: the alphabet says which spellings can be a name, the bank says which of those the language took, and an item name must clear both.
/// The direct grammars keep reading the two halves separately, because an authored declaration deserves a refusal naming which half disagreed at which token; every constructor that mints a name or a path segment programmatically reads this one.
#[must_use]
pub fn rendered_name(spelling: &str) -> bool {
    rendered_identifier(spelling) && !rust_keyword(spelling)
}
