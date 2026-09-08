//! Compiler historical encoding reads the existing informed diagnostic payloads.

use crate::identity::encode_bytes;
use crate::oracle::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict,
    PrimarySourceSpan,
};

pub(super) fn write(verdict: &CompiledVerdict, body: &mut Vec<u8>) {
    match verdict {
        CompiledVerdict::Conforms => body.push(0),
        CompiledVerdict::Deviates(found) => {
            body.push(1);
            match found {
                CompiledDisagreement::AcceptedWhereRefusalDeclared => body.push(0),
                CompiledDisagreement::RefusedWhereAcceptanceDeclared => body.push(1),
                CompiledDisagreement::UnexpectedMember { member } => member_text(2, member, body),
                CompiledDisagreement::DuplicateMember { member } => member_text(3, member, body),
                CompiledDisagreement::MissingMember { member } => member_text(4, member, body),
                CompiledDisagreement::MemberValue { member } => member_text(5, member, body),
            }
        }
    }
}

fn member_text(slot: u8, member: &str, body: &mut Vec<u8>) {
    body.push(slot);
    encode_bytes(member.as_bytes(), body);
}

pub(super) fn compilation(verdict: &CompilationVerdict, body: &mut Vec<u8>) {
    match verdict {
        CompilationVerdict::Conforms => body.push(0),
        CompilationVerdict::Deviates(found) => {
            body.push(1);
            match found {
                CompilationDisagreement::AcceptedWhereRefusalDeclared => body.push(0),
                CompilationDisagreement::RefusedWhereAcceptanceDeclared { observed } => {
                    body.push(1);
                    encode_bytes(observed.code().spelling().as_bytes(), body);
                    span(observed.primary(), body);
                }
                CompilationDisagreement::ErrorCode { expected, observed } => {
                    body.push(2);
                    encode_bytes(expected.spelling().as_bytes(), body);
                    encode_bytes(observed.spelling().as_bytes(), body);
                }
                CompilationDisagreement::PrimarySpan { expected, observed } => {
                    body.push(3);
                    span(expected, body);
                    span(observed, body);
                }
            }
        }
    }
}

fn span(primary: &PrimarySourceSpan, body: &mut Vec<u8>) {
    encode_bytes(primary.source().spelling().as_bytes(), body);
    for value in [
        primary.start().line(),
        primary.start().column(),
        primary.end().line(),
        primary.end().column(),
    ] {
        body.extend_from_slice(&value.to_be_bytes());
    }
}
