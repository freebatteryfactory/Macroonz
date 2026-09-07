//! Typed encoder declarations and independent retention limits.

use super::{ParityArchiveLimits, ValueEncoder, ValueEncodingRefusal};
use crate::descriptor::archive::BindingArchiveLimits;
use crate::descriptor::{NamespacedName, RevisionBinding};
use crate::identity::ContentAddress;
use crate::report::ForeignText;
use crate::report::archive::ArchiveLimits;

#[path = "read.rs"]
mod read;
#[path = "read_members.rs"]
mod members;
#[path = "guard_parity.rs"]
mod readings;

pub use read::read_parity;

impl<Value> ValueEncoder<Value> {
    /// Bind a caller encoder to its explicit convention, schema and executable revision.
    #[must_use]
    pub const fn declared(
        convention: NamespacedName,
        version: u32,
        schema: ContentAddress,
        revision: RevisionBinding,
        encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
    ) -> Self {
        Self {
            convention,
            version,
            schema,
            revision,
            encode,
        }
    }

    /// Invoke this declared callback once with its ordinary Rust effect and unwind ceiling.
    pub(crate) fn encode(&self, value: &Value) -> Result<Vec<u8>, ValueEncodingRefusal> {
        (self.encode)(value)
    }

    /// The declared encoding convention.
    #[must_use]
    pub const fn convention(&self) -> NamespacedName {
        self.convention
    }

    /// The declared convention version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// The declared encoding schema commitment.
    #[must_use]
    pub const fn schema(&self) -> ContentAddress {
        self.schema
    }

    /// The encoder's declared executable revision.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }
}

impl ValueEncodingRefusal {
    /// Retain a caller-owned storage refusal without creating a trial finding.
    #[must_use]
    pub const fn refused(cause: NamespacedName, foreign: Option<ForeignText>) -> Self {
        Self { cause, foreign }
    }
    /// The caller-owned cause.
    #[must_use]
    pub const fn cause(&self) -> NamespacedName {
        self.cause
    }
    /// The optional bounded caller material.
    #[must_use]
    pub const fn foreign(&self) -> Option<&ForeignText> {
        self.foreign.as_ref()
    }
}

impl ParityArchiveLimits {
    /// Declare independent outer bytes, witness, trial, value and substrate ceilings.
    #[must_use]
    pub const fn declared(
        bytes: ArchiveLimits,
        binding: BindingArchiveLimits,
        trial: ArchiveLimits,
        value: usize,
        substrates: usize,
    ) -> Self {
        Self {
            bytes,
            binding,
            trial,
            value,
            substrates,
        }
    }
    /// The complete-envelope and outer framed-field limits.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }
    /// The nested witness limits.
    #[must_use]
    pub const fn binding(self) -> BindingArchiveLimits {
        self.binding
    }
    /// The nested trial limits.
    #[must_use]
    pub const fn trial(self) -> ArchiveLimits {
        self.trial
    }
    /// The independent byte limit for each encoded value.
    #[must_use]
    pub const fn value(self) -> usize {
        self.value
    }
    /// The independent substrate population limit.
    #[must_use]
    pub const fn substrates(self) -> usize {
        self.substrates
    }
}
