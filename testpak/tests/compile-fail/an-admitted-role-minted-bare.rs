//! The reversal for band 02's identity-role admission: the witness cannot be
//! written down, only earned.
//!
//! The role declared below is the exact declaration admission refuses — Class
//! B's own creation law under Class A's question — so a caller who could fill
//! the witness's seat directly would be carrying evidence for a join that would
//! have failed. The seat is private, so the value does not exist.

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
