//! Canonical assembly-issue encoding.
use super::AssemblyIssue;
use crate::identity::encode_bytes;
impl AssemblyIssue {
    /// Returns this issue's canonical bytes.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_into(&mut bytes);
        bytes
    }
    /// Appends this issue's canonical bytes.
    pub fn encode_into(&self, into: &mut Vec<u8>) {
        into.push(self.slot());
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
