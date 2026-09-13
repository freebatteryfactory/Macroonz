use super::types::{Definition, PublishedValue, Value};
use macroonz::compiler::{CanonicalContent, Destination, Kind, NoQuestions, Role};

impl CanonicalContent for Value {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.0.to_le_bytes());
    }
}

impl Role for Definition {
    const ALL: &'static [Self] = &[Self];

    fn name(self) -> &'static str {
        "definition"
    }

    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

impl Kind for PublishedValue {
    const NAME: &'static str = "example.published-value";
    type Content = Value;
    type Role = Definition;
    type Question = NoQuestions;
}
