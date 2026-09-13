use super::{StagingError, StagingPlan};
use crate::compiler::Kind;
use crate::compiler::identity::{
    Anchoring, Identity, Profile, Role, Transcript, Version, encode_bytes, encode_length,
};
use crate::native_storage::StorageName;
use std::collections::BTreeMap;
use std::fmt::Write;

pub(super) fn name<K: Kind>(plan: &StagingPlan<K>) -> Result<StorageName, StagingError> {
    let files = plan
        .files()
        .map(|(path, bytes)| (path.spelling().to_owned(), bytes))
        .collect::<BTreeMap<_, _>>();
    let mut bytes = Vec::new();
    encode_length(files.len(), &mut bytes);
    for (path, content) in files {
        encode_bytes(path.as_bytes(), &mut bytes);
        encode_bytes(content, &mut bytes);
    }
    let identity = Identity::<super::types::StagingSource>::derived(Transcript::under_profile(
        Profile::declared(
            "macroonz/native-publication",
            "staging-cache",
            Version::declared(1),
        ),
        Role::Bundle,
        Anchoring::Rooted,
        &bytes,
        0,
    ));
    let mut spelling = "cache-".to_owned();
    for byte in identity.as_bytes() {
        write!(spelling, "{byte:02x}")
            .map_err(|error| StagingError::Configuration(error.to_string()))?;
    }
    StorageName::informed(&spelling).map_err(StagingError::Storage)
}
