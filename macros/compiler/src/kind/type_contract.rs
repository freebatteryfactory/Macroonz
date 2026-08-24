//! The two rosters this home owns, implemented: the one-role roster, and the roster of a kind that owes no questions.

use super::types::{Answer, Destination, NoQuestions, Question, Role, SoleRole};

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
