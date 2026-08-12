//! The subject roster is closed, and the seal is what closes it.
//!
//! `IdentitySubject::SUBJECT_NAME` is a segment of the derive-key context the
//! plane derives every projection identity under. An open trait would let a type
//! declared anywhere choose that separation context — reuse a name the plane
//! already spells, or mint one it never admitted — which is a law change wearing
//! the shape of an extension point.
//!
//! Two roads out, and both are closed. The first implementation below tries to
//! furnish the seal and cannot: `SubjectSeal`'s only mint is crate-internal, so
//! the constructor is not reachable from here. The second tries to skip the seat
//! and cannot: the constant is a required trait item with no default, so an
//! implementation that omits it is not an implementation.
//!
//! No value is constructed below. The two implementations alone are the proof.

use threadpak_macroc::plane::{IdentitySubject, SubjectSeal};

/// A subject the services never declared, reaching for the seal.
struct ASubjectFromOutside;

impl IdentitySubject for ASubjectFromOutside {
    const SEAL: SubjectSeal = SubjectSeal(());
    const SUBJECT_NAME: &'static str = "a-subject-from-outside";
}

/// A subject the services never declared, skipping the seal.
struct ASubjectThatSkipsTheSeal;

impl IdentitySubject for ASubjectThatSkipsTheSeal {
    const SUBJECT_NAME: &'static str = "a-subject-that-skips-the-seal";
}

fn main() {}
