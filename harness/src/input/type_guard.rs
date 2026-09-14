//! The constructors that keep profiles, bytes, typed values and decoder revisions together.

use super::{
    BoundInput, InputBinding, InputCaseId, InputEnvelope, InputLimits, InputProfile, InputRefusal,
};
use crate::descriptor::{NamespacedName, RevisionBinding};
use crate::identity::ContentAddress;

impl InputProfile {
    /// The caller's input convention under its declared schema commitment.
    #[must_use]
    pub const fn declared(name: NamespacedName, version: u32, schema: ContentAddress) -> Self {
        Self {
            name,
            version,
            schema,
        }
    }

    /// The convention's admitted name.
    #[must_use]
    pub const fn name(self) -> NamespacedName {
        self.name
    }

    /// The convention's declared version.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }

    /// The caller's schema commitment.
    #[must_use]
    pub const fn schema(self) -> ContentAddress {
        self.schema
    }
}

impl InputLimits {
    /// The byte ceilings enforced before envelope and specimen allocation.
    #[must_use]
    pub const fn declared(envelope: usize, payload: usize) -> Self {
        Self { envelope, payload }
    }

    /// The complete encoded-envelope ceiling.
    #[must_use]
    pub const fn envelope(self) -> usize {
        self.envelope
    }

    /// The decoded specimen-byte ceiling.
    #[must_use]
    pub const fn payload(self) -> usize {
        self.payload
    }
}

impl InputCaseId {
    /// The address derived by this home's writer or verified reader.
    pub(in crate::input) const fn derived(address: ContentAddress) -> Self {
        Self(address)
    }
}

crate::identity::content_address_reference! {
    /// The case's content address.
    value InputCaseId;
}

impl InputEnvelope {
    /// The complete envelope admitted by the format owner.
    pub(in crate::input) fn admitted(
        profile: InputProfile,
        case: InputCaseId,
        payload: Vec<u8>,
        encoded: Vec<u8>,
    ) -> Self {
        Self {
            profile,
            case,
            payload,
            encoded,
        }
    }

    /// The independently admitted input convention.
    #[must_use]
    pub const fn profile(&self) -> InputProfile {
        self.profile
    }

    /// The identity derived over this specimen and its convention.
    #[must_use]
    pub const fn case(&self) -> InputCaseId {
        self.case
    }

    /// The exact specimen bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// The exact content-addressed envelope, ready for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }
}

impl<Input> InputBinding<Input> {
    /// The decoder for one profile, bound to its declared executable revision.
    #[must_use]
    pub const fn declared(
        profile: InputProfile,
        revision: RevisionBinding,
        decode: fn(&mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Input>,
    ) -> Self {
        Self {
            profile,
            revision,
            decode,
        }
    }

    /// The profile this decoder admits.
    #[must_use]
    pub const fn profile(&self) -> InputProfile {
        self.profile
    }

    /// The revision and posture supplied for this decoder.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }

    /// One typed value decoded from the complete specimen.
    ///
    /// # Errors
    ///
    /// Refuses profile or schema mismatch before calling the decoder, then preserves a decoder refusal or rejects unread specimen bytes.
    pub fn decode(&self, envelope: InputEnvelope) -> Result<BoundInput<Input>, InputRefusal> {
        if self.profile.name != envelope.profile.name
            || self.profile.version != envelope.profile.version
        {
            return Err(InputRefusal::ProfileMismatch);
        }
        if self.profile.schema != envelope.profile.schema {
            return Err(InputRefusal::SchemaMismatch);
        }
        let mut source = arbitrary::Unstructured::new(envelope.payload());
        let value = (self.decode)(&mut source).map_err(InputRefusal::DecoderRefused)?;
        if !source.is_empty() {
            return Err(InputRefusal::TrailingInputBytes {
                count: source.len(),
            });
        }
        Ok(BoundInput {
            value,
            envelope,
            revision: self.revision,
        })
    }
}

impl<Input> BoundInput<Input> {
    /// The caller-owned typed value the decoder returned.
    #[must_use]
    pub const fn value(&self) -> &Input {
        &self.value
    }

    /// The exact admitted bytes and their input profile.
    #[must_use]
    pub const fn envelope(&self) -> &InputEnvelope {
        &self.envelope
    }

    /// The decoder revision that produced this value.
    #[must_use]
    pub const fn revision(&self) -> RevisionBinding {
        self.revision
    }
}
