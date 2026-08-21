//! The reversal for band 02's identity-role admission: the witness's seats
//! cannot be written down.
//!
//! The role declared below is the exact declaration admission refuses — Class
//! B's own creation law under Class A's question — so a caller who could fill
//! the witness's seats directly would be carrying evidence for a join that would
//! have failed. Both seats are private, so the literal below is not a value a
//! caller can write.
//!
//! # What this file establishes, exactly
//!
//! REPRESENTATION PRIVACY, and that is a narrower claim than *only earned*. A
//! second public road returning this witness — beside `admitted`, which returns
//! one inside a `Result` — would leave this error exactly where it is while
//! handing out the token for a join nobody ran.
//!
//! That absence is not derived, for the reason band 00's family-admission
//! reversal states: a road IN is not distinguishable from the lawful mint beside
//! it without a declaration of which mint is the one.

use core::marker::PhantomData;
use threadpak::identity::{AdmittedIdentityRole, CreationLaw, IdentityClass, IdentityRole};

struct IncoherentRole;

impl IdentityRole for IncoherentRole {
    const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
    const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
}

fn main() {
    let _forged: AdmittedIdentityRole<IncoherentRole> =
        AdmittedIdentityRole { _role: PhantomData };
}
