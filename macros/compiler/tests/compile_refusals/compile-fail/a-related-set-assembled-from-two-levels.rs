//! A related set is built from issue MATERIAL, never from identities somebody already derived.
//!
//! A road taking a whole-body commitment and a set of per-issue identities takes two halves that do not check each other: each derives honestly on its own, so the pair can name one refusal's body over another refusal's issues and still read exactly like a set that belongs together.
//! The one road in is handed the material and derives BOTH levels itself, so neither half is ever a caller's to hold.
//!
//! Minting an identity of a public subject is lawful and stays lawful — that is what makes this the honest test.
//! Holding both levels is allowed; seating them is not expressible, because the only entry point takes bytes that were established as issues rather than names somebody chose.

use macroonz::RelatedSet;
use macroonz::identity::{Identity, RelatedIssue, Role, Transcript};

fn main() {
    let issues = vec![Identity::<RelatedIssue>::derived(Transcript::rooted(
        Role::DiagnosticRelation,
        b"an issue",
        1,
    ))];
    let _set = RelatedSet::derived_over(1_u8, &issues);
}
