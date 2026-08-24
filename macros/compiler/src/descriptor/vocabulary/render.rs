//! Spelling one path, one call, and one clause at the harness's address.
//!
//! Every path a descriptor emission writes begins with the harness binding's own metavariable, so a consumer that renamed the dependency gets its own name back and nothing here learns what the crate is called.

use super::{HarnessName, HarnessWord};
use crate::bounded::Overflow;
use crate::descriptor::Binding;
use crate::token::{GeneratedToken, call, twin_path};

/// One path at the harness's address, rooted at the harness binding's metavariable.
#[must_use]
pub fn path(segments: &[HarnessName]) -> Vec<GeneratedToken> {
    let spelled: Vec<&str> = segments.iter().map(|segment| segment.spelling()).collect();
    twin_path(Binding::Harness.name(), &spelled)
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
