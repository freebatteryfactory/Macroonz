//! The compiler-owned roster contracts and how a disposition-set completion refusal reads.

use super::types::{
    Answer, CanonicalContent, Destination, DispositionSetError, NoQuestions, Question, Role,
    SoleRole,
};
use crate::identity::encode_bytes;
use core::fmt;

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

impl fmt::Display for DispositionSetError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CountMismatch { expected, observed } => write!(
                into,
                "the kind set declares {expected} names but its disposition record surrendered {observed} rows"
            ),
            Self::KindMismatch { expected, observed } => write!(
                into,
                "the kind set declares `{expected}` at this position but its disposition record surrendered `{observed}`"
            ),
        }
    }
}

impl core::error::Error for DispositionSetError {}
