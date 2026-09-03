//! The name roads: how an owner and a spelling are parsed into one namespaced name, and the reference stamp every open reference is written with.

use crate::descriptor::types::{
    AuthoredTableName, CheckRef, ClaimRef, DoorRef, ExecutionSuite, MutationPointRef, NameRefusal,
    Namespace, NamespacedName, PopulationRef, ProducerName, ProjectionRef, Role, Stem,
    SubjectRoute, Tag,
};

impl Namespace {
    /// The owner one authored text declares.
    ///
    /// # Errors
    ///
    /// Returns [`NameRefusal::EmptyNamespace`] where the text is empty.
    pub const fn declared(text: &'static str) -> Result<Self, NameRefusal> {
        if text.is_empty() {
            return Err(NameRefusal::EmptyNamespace);
        }
        Ok(Self(text))
    }

    /// The owner's text.
    ///
    /// The one road out to characters, for the two places characters are what is wanted: an encoder writing a preimage, and a rendering writing a line for a person.
    #[must_use]
    pub const fn written(self) -> &'static str {
        self.0
    }
}

impl Stem {
    /// The spelling one authored text declares.
    ///
    /// # Errors
    ///
    /// Returns [`NameRefusal::EmptyStem`] where the text is empty.
    pub const fn declared(text: &'static str) -> Result<Self, NameRefusal> {
        if text.is_empty() {
            return Err(NameRefusal::EmptyStem);
        }
        Ok(Self(text))
    }

    /// The spelling's text, on the terms [`Namespace::written`] states.
    #[must_use]
    pub const fn written(self) -> &'static str {
        self.0
    }
}

impl NamespacedName {
    /// This name, parsed from the owner that declares it and the spelling it carries.
    ///
    /// # Errors
    ///
    /// Refuses an empty namespace, then an empty stem, so exactly one cause is true of any refused name.
    pub const fn named(namespace: &'static str, stem: &'static str) -> Result<Self, NameRefusal> {
        let namespace = match Namespace::declared(namespace) {
            Ok(namespace) => namespace,
            Err(refusal) => return Err(refusal),
        };
        let stem = match Stem::declared(stem) {
            Ok(stem) => stem,
            Err(refusal) => return Err(refusal),
        };
        Ok(Self { namespace, stem })
    }

    /// The owner that declares the spelling.
    #[must_use]
    pub const fn namespace(self) -> Namespace {
        self.namespace
    }

    /// The spelling itself.
    #[must_use]
    pub const fn stem(self) -> Stem {
        self.stem
    }
}

/// The two roads and the one reader every namespaced reference carries, written once and stamped over the roster.
///
/// Each reference is its own type so a claim cannot occupy a subject's seat.
/// What they share is how a name is parsed, and a hand-copied parser per newtype would be that one law standing in a dozen places.
macro_rules! namespaced_reference {
    (const $($reference:ident),+ $(,)?) => {
        $(
            namespaced_reference!(@implement $reference, pub const fn);
        )+
    };
    ($($reference:ident),+ $(,)?) => {
        $(
            namespaced_reference!(@implement $reference, pub fn);
        )+
    };
    (@implement $reference:ident, $($signature:tt)+) => {
        impl $reference {
            /// This reference, parsed from the owner that declares it and the spelling it carries.
            ///
            /// # Errors
            ///
            /// Refuses an empty namespace, then an empty stem.
            $($signature)+ named(
                namespace: &'static str,
                stem: &'static str,
            ) -> Result<Self, $crate::descriptor::NameRefusal> {
                match $crate::descriptor::NamespacedName::named(namespace, stem) {
                    Ok(name) => Ok(Self(name)),
                    Err(refusal) => Err(refusal),
                }
            }

            /// This reference, over a name already parsed.
            #[must_use]
            pub const fn over(name: $crate::descriptor::NamespacedName) -> Self {
                Self(name)
            }

            /// The namespaced name this reference carries.
            #[must_use]
            pub const fn name(self) -> $crate::descriptor::NamespacedName {
                self.0
            }
        }
    };
}

pub(crate) use namespaced_reference;

namespaced_reference!(
    AuthoredTableName,
    CheckRef,
    ClaimRef,
    DoorRef,
    ExecutionSuite,
    MutationPointRef,
    PopulationRef,
    ProducerName,
    ProjectionRef,
    Role,
    SubjectRoute,
    Tag,
);
