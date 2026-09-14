use super::types::{Example, InlineExample, InlineSeat, Seat, Values};
use macroonz::compiler::{CanonicalContent, Destination, Kind, NoQuestions, Role};

impl CanonicalContent for Values {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.0);
    }
}

impl Role for Seat {
    const ALL: &'static [Self] = &[Self::Definition, Self::Other];

    fn name(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Other => "other",
        }
    }

    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

impl Kind for Example {
    const NAME: &'static str = "publication-independent-values";
    type Content = Values;
    type Role = Seat;
    type Question = NoQuestions;
}

impl Role for InlineSeat {
    const ALL: &'static [Self] = &[Self];

    fn name(self) -> &'static str {
        "inline"
    }

    fn destination(self) -> Destination {
        Destination::DeclarationSite
    }
}

impl Kind for InlineExample {
    const NAME: &'static str = "publication-inline-only";
    type Content = Values;
    type Role = InlineSeat;
    type Question = NoQuestions;
}
