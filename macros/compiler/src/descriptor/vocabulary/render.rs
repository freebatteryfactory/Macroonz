//! Spelling one path, one call, and one clause at the harness's address.
//!
//! Every path a descriptor emission writes begins with the harness binding's root and repeated segment metavariables, so a consumer that reaches the harness through a facade or renamed dependency gets its own path back and nothing here learns what the crate is called.

use super::{HarnessName, HarnessWord};
use crate::bounded::Overflow;
use crate::token::{GeneratedToken, call};

/// One path at the harness's address, rooted at the harness binding's segmented path.
#[must_use]
pub fn path(segments: &[HarnessName]) -> Vec<GeneratedToken> {
    let spelled: Vec<&str> = segments.iter().map(|segment| segment.spelling()).collect();
    crate::support::rooted_path(crate::support::CrateFacing::Harness, &spelled)
}

/// One call to a road at the harness's address.
///
/// # Errors
///
/// Returns [`Overflow`] where the argument list outgrows the declared magnitude.
pub fn road(
    segments: &[HarnessName],
    arguments: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, Overflow> {
    call(path(segments), arguments)
}

/// One `<word>:` clause key, as the two tokens that spell it.
#[must_use]
pub fn key(word: HarnessWord) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word(word.spelling()),
        GeneratedToken::alone(':'),
    ]
}
