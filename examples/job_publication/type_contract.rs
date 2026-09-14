use super::types::{PublicationSeat, PublishedValues, Values};
use macroonz::compiler::{CanonicalContent, Destination, Kind, NoQuestions, Role};

impl CanonicalContent for Values {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.0);
    }
}

impl Role for PublicationSeat {
    const ALL: &'static [Self] = &[Self::Definition];
    fn name(self) -> &'static str {
        "definition"
    }
    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

impl Kind for PublishedValues {
    const NAME: &'static str = "example.published-values";
    type Content = Values;
    type Role = PublicationSeat;
    type Question = NoQuestions;
}
