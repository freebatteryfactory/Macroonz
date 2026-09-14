use super::{CanonicalPublicationBytes, PublishedBytes};
use crate::compiler::identity::{Anchoring, Identity, Profile, Role, Subject, Transcript, Version};

const FILE_BYTES: Profile = Profile::declared(
    "macroonz/native-publication",
    "file-bytes",
    Version::declared(1),
);

fn derived<S: Subject>(bytes: &[u8]) -> Identity<S> {
    Identity::derived(Transcript::under_profile(
        FILE_BYTES,
        Role::Bundle,
        Anchoring::Rooted,
        bytes,
        0,
    ))
}

pub(super) fn canonical(bytes: &[u8]) -> Identity<CanonicalPublicationBytes> {
    derived(bytes)
}

/// Derives the physical-byte commitment specified by the publication inventory owner.
#[must_use]
pub fn published_digest(bytes: &[u8]) -> Identity<PublishedBytes> {
    derived(bytes)
}
