//! The executable roads: how a revision is derived and held under a posture, and what makes one row executable.

use crate::descriptor::types::{
    CheckRef, DERIVED_REVISION_DOMAIN, DerivedRevision, ExecutableAttachment, RevisionBinding,
    RevisionPosture, SubjectRoute,
};
use crate::identity::ContentAddress;

impl RevisionPosture {
    /// The weaker of two postures — what both halves of a pair can honestly claim.
    ///
    /// Every combination is stated rather than folded, because the order is a declaration.
    #[must_use]
    pub const fn meet(self, other: Self) -> Self {
        match (self, other) {
            (Self::Derived, Self::Derived) => Self::Derived,
            (Self::Derived | Self::Declared, Self::Declared) | (Self::Declared, Self::Derived) => {
                Self::Declared
            }
            (Self::Untracked, Self::Derived | Self::Declared | Self::Untracked)
            | (Self::Derived | Self::Declared, Self::Untracked) => Self::Untracked,
        }
    }
}

impl DerivedRevision {
    /// Derive one executable revision from the canonical material the owning operation produced.
    ///
    /// The address is minted here under [`DERIVED_REVISION_DOMAIN`] and cannot arrive already made.
    ///
    /// # Authority
    ///
    /// This evidence proves the derivation over the supplied bytes.
    /// The operation supplying them remains responsible for making them the complete canonical material of the subject or check it binds.
    #[must_use]
    pub fn from_material(material: &[u8]) -> Self {
        Self {
            revision: ContentAddress::derived(DERIVED_REVISION_DOMAIN, material),
        }
    }

    /// The address this derivation minted.
    #[must_use]
    pub const fn revision(self) -> ContentAddress {
        self.revision
    }
}

impl RevisionBinding {
    /// A revision generated from canonical material by this home's derivation operation.
    #[must_use]
    pub const fn derived(evidence: DerivedRevision) -> Self {
        Self {
            revision: evidence.revision(),
            posture: RevisionPosture::Derived,
        }
    }

    /// A revision a hand author committed to explicitly.
    #[must_use]
    pub const fn declared(revision: ContentAddress) -> Self {
        Self {
            revision,
            posture: RevisionPosture::Declared,
        }
    }

    /// A revision under no stable commitment.
    #[must_use]
    pub const fn untracked(revision: ContentAddress) -> Self {
        Self {
            revision,
            posture: RevisionPosture::Untracked,
        }
    }

    /// The revision identity.
    #[must_use]
    pub const fn revision(self) -> ContentAddress {
        self.revision
    }

    /// The posture the identity is held under.
    #[must_use]
    pub const fn posture(self) -> RevisionPosture {
        self.posture
    }
}

impl<Invocation, Conclusion> ExecutableAttachment<Invocation, Conclusion> {
    /// What makes one row executable: the references it is over, a posture-bearing revision binding for each, and the callable.
    #[must_use]
    pub const fn attached(
        subject: SubjectRoute,
        check: CheckRef,
        subject_revision: RevisionBinding,
        check_revision: RevisionBinding,
        call: fn(&Invocation) -> Conclusion,
    ) -> Self {
        Self {
            subject,
            check,
            subject_revision,
            check_revision,
            call,
        }
    }

    /// The subject route this attachment executes.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The check reference this attachment judges under.
    #[must_use]
    pub const fn check(&self) -> CheckRef {
        self.check
    }

    /// The subject's revision binding.
    #[must_use]
    pub const fn subject_revision(&self) -> RevisionBinding {
        self.subject_revision
    }

    /// The check's revision binding.
    #[must_use]
    pub const fn check_revision(&self) -> RevisionBinding {
        self.check_revision
    }

    /// The weaker of the two revision postures — the one every per-posture reading of this attachment is stated over.
    #[must_use]
    pub const fn posture(&self) -> RevisionPosture {
        self.subject_revision
            .posture()
            .meet(self.check_revision.posture())
    }

    /// The capture-free callable.
    #[must_use]
    pub const fn call(&self) -> fn(&Invocation) -> Conclusion {
        self.call
    }

    /// The conclusion this attachment reaches over one set of invocation facts.
    #[must_use]
    pub fn conclude(&self, invocation: &Invocation) -> Conclusion {
        (self.call)(invocation)
    }
}

/// Cloning copies the five seats.
///
/// The derive is not used because it would demand `Clone` of both parameters, which they do not owe: they appear only behind a function pointer, and a function pointer is `Copy` whatever its ends are.
impl<Invocation, Conclusion> Clone for ExecutableAttachment<Invocation, Conclusion> {
    fn clone(&self) -> Self {
        Self {
            subject: self.subject,
            check: self.check,
            subject_revision: self.subject_revision,
            check_revision: self.check_revision,
            call: self.call,
        }
    }
}
