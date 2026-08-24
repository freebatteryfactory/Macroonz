//! The canonical bytes one refusal of this home is named by.
//!
//! Every row's position rides ahead of the material it governs, and every variable-length member is framed through the identity home's one framing, so no two values can be cut at another boundary and produce one byte string.
//! A roster row enters as its DECLARED NAME rather than as a second numbering of somebody else's table: a name is as stable as a slot and readable in a preimage.

use super::{AssemblyIssue, DeclarationError, ShellError};
use crate::identity::{encode_bytes, encode_length};

impl AssemblyIssue {
    /// The bytes this issue is, on their own.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this issue's canonical bytes: the row's position, then the typed material that row carries.
    ///
    /// Exhaustive over the roster on purpose: an issue added to [`AssemblyIssue`] stops compiling HERE until somebody says what of it a related identity stands over.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        self.material_into(into);
    }

    /// Appends the material one row carries, and nothing for the row that carries none.
    fn material_into(&self, into: &mut Vec<u8>) {
        match self {
            Self::RootsDisagree {
                axis,
                stated,
                carried,
            } => {
                encode_bytes(axis.name().as_bytes(), into);
                encode_bytes(stated.as_bytes(), into);
                encode_bytes(carried.as_bytes(), into);
            }
            Self::CargoConsumedTwice {
                source,
                destination,
            }
            | Self::CargoNotTheSourcesOwn {
                source,
                destination,
            } => {
                encode_bytes(source.as_bytes(), into);
                encode_bytes(destination.name().as_bytes(), into);
            }
            Self::CargoReachesASecondDestination { axis, destination } => {
                encode_bytes(axis.name().as_bytes(), into);
                encode_bytes(destination.name().as_bytes(), into);
            }
            Self::TwoFormsCarried => {}
            Self::StampedCargoAbsent { form } => encode_bytes(form.name().as_bytes(), into),
        }
    }
}

impl ShellError {
    /// The bytes this refusal is, on their own.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this refusal's canonical bytes: the row's position, then its own material.
    ///
    /// The two declarations of a disagreement travel in the order the row holds them — the assembly's first, the plan's second — so a reader of the pair knows which is which without the encoding saying so twice.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
        match self {
            Self::NotOneDeclaration { stated, planned } => {
                encode_bytes(stated.as_bytes(), into);
                encode_bytes(planned.as_bytes(), into);
            }
            Self::TreeUnbounded { bound, observed } => {
                encode_length(*bound, into);
                encode_length(*observed, into);
            }
        }
    }
}

impl DeclarationError {
    /// The bytes this refusal is, on their own.
    #[must_use]
    pub fn canonical_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }

    /// Appends this refusal's canonical bytes: the row's position, and nothing beside it.
    ///
    /// Every row here is payload-free because every one is a fact about the seat the declaration did not fill, and the seat is the row.
    pub fn encode_into(self, into: &mut Vec<u8>) {
        into.push(self.slot());
    }
}
