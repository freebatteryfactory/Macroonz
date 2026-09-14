//! Structural historical encoding preserves caller-stated positions and exact free text.

use crate::identity::{encode_bytes, encode_length};
use crate::oracle::{StructuralDisagreement, StructuralVerdict};

pub(super) fn write(verdict: &StructuralVerdict, body: &mut Vec<u8>) {
    match verdict {
        StructuralVerdict::Conforms => body.push(0),
        StructuralVerdict::Unparsable => body.push(2),
        StructuralVerdict::Deviates(found) => {
            body.push(1);
            disagreement(found, body);
        }
    }
}

fn disagreement(found: &StructuralDisagreement, body: &mut Vec<u8>) {
    match found {
        StructuralDisagreement::UnexpectedItem => body.push(0),
        StructuralDisagreement::OutputCardinality { declared, read } => {
            body.push(1);
            encode_length(*declared, body);
            encode_length(*read, body);
        }
        StructuralDisagreement::DuplicateImplementation { at } => position(2, *at, body),
        StructuralDisagreement::ImplementationTarget { at } => position(3, *at, body),
        StructuralDisagreement::TraitPath { at } => position(4, *at, body),
        StructuralDisagreement::ImplPosture { at } => position(5, *at, body),
        StructuralDisagreement::MeaningBearingAttribute { at, attribute } => {
            position(6, *at, body);
            encode_bytes(attribute.as_bytes(), body);
        }
        StructuralDisagreement::UnexpectedImplMember { at, member } => {
            position(7, *at, body);
            encode_bytes(member.as_bytes(), body);
        }
        StructuralDisagreement::DuplicateMember { at, member } => {
            position(8, *at, body);
            encode_bytes(member.as_bytes(), body);
        }
        StructuralDisagreement::MissingImplMember { at, member } => {
            position(9, *at, body);
            encode_bytes(member.as_bytes(), body);
        }
        StructuralDisagreement::MemberValueUnread { at, member } => {
            position(10, *at, body);
            encode_bytes(member.as_bytes(), body);
        }
        StructuralDisagreement::MemberValue { at, member } => {
            position(11, *at, body);
            encode_bytes(member.as_bytes(), body);
        }
    }
}

fn position(slot: u8, at: usize, body: &mut Vec<u8>) {
    body.push(slot);
    encode_length(at, body);
}
