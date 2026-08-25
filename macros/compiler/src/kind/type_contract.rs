//! The two rosters this home owns, implemented: the one-role roster, and the roster of a kind that owes no questions.

use super::types::{Answer, CanonicalContent, Destination, NoQuestions, Question, Role, SoleRole};
use crate::identity::encode_bytes;

impl CanonicalContent for () {
    fn encode_content_into(&self, _into: &mut Vec<u8>) {}
}

impl CanonicalContent for &'static str {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.as_bytes(), into);
    }
}

impl Role for SoleRole {
    const ALL: &'static [Self] = &[Self::Sole];

    fn name(self) -> &'static str {
        "sole"
    }

    fn destination(self) -> Destination {
        Destination::DeclarationSite
    }
}

impl Question for NoQuestions {
    const ALL: &'static [Self] = &[];

    type Answer = Self;

    fn name(self) -> &'static str {
        match self {}
    }
}

impl Answer for NoQuestions {
    type Question = Self;

    fn question(&self) -> Self {
        match *self {}
    }

    fn encode_into(&self, _into: &mut Vec<u8>) {
        match *self {}
    }

    fn human(&self) -> String {
        match *self {}
    }
}
