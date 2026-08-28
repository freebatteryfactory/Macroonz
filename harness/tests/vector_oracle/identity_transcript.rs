//! The transcript method re-encodes a published preimage grammar without importing the identity producer's encoder.
//!
//! The positive control mints a real harness content address over independently authored bytes, while the oracle composes the same bytes from typed transcript members.

use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::oracle::{
    ContextRefusal, ORACLE_CAUSE_FAMILY, SpecifiedContext, TranscriptDerivation, TranscriptMember,
    TranscriptVerdict,
};
use macroonz_harness::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};
use std::fmt;

const TRANSCRIPT_TAG: DomainTag =
    DomainTag::declared("oracle-transcript", IdentityProfileVersion::declared(1));
const PUBLISHED_PREIMAGE: &[u8] =
    b"\x00\x00\x00\x00\x00\x00\x00\x03law\x07\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c";

enum TranscriptRoadFailure {
    Context(ContextRefusal),
    ExpectedDisagreement,
}

impl fmt::Debug for TranscriptRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Context(refusal) => formatter.debug_tuple("Context").field(refusal).finish(),
            Self::ExpectedDisagreement => formatter.write_str("ExpectedDisagreement"),
        }
    }
}

impl From<ContextRefusal> for TranscriptRoadFailure {
    fn from(refusal: ContextRefusal) -> Self {
        Self::Context(refusal)
    }
}

#[test]
fn typed_transcript_members_rederive_an_independently_published_address()
-> Result<(), TranscriptRoadFailure> {
    let context =
        SpecifiedContext::spelled(&["macroonz", "harness-identity", "oracle-transcript", "v1"])?;
    let transcript = TranscriptDerivation::opened()
        .framed_text("law")
        .discriminant(7u8)
        .fixed32(0x0102_0304u32)
        .fixed64(0x0506_0708_090a_0b0cu64);
    let published = ContentAddress::derived(TRANSCRIPT_TAG, PUBLISHED_PREIMAGE);

    assert_eq!(
        context.spelling(),
        "macroonz/harness-identity/oracle-transcript/v1"
    );
    assert_eq!(
        transcript.members(),
        &[
            TranscriptMember::Framed(b"law".to_vec()),
            TranscriptMember::Discriminant(7u8),
            TranscriptMember::Fixed32(0x0102_0304u32),
            TranscriptMember::Fixed64(0x0506_0708_090a_0b0cu64),
        ]
    );
    assert_eq!(transcript.preimage().as_slice(), PUBLISHED_PREIMAGE);

    let derived = transcript.derived(&context);
    assert_eq!(derived.as_bytes(), published.as_bytes());
    assert_eq!(
        derived.compared(published.as_bytes()),
        TranscriptVerdict::Agrees
    );

    let mut hostile = *published.as_bytes();
    let Some(first) = hostile.first_mut() else {
        return Err(TranscriptRoadFailure::ExpectedDisagreement);
    };
    *first ^= 1u8;
    let verdict = derived.compared(&hostile);
    let TranscriptVerdict::Disagrees(disagreement) = &verdict else {
        return Err(TranscriptRoadFailure::ExpectedDisagreement);
    };
    assert_eq!(disagreement.rederived(), derived.as_bytes());
    assert_eq!(disagreement.published(), &hostile);

    let conclusion = verdict.concluded(FindingLocation::at(file!(), line!()));
    let TrialConclusion::Refused(finding) = conclusion else {
        return Err(TranscriptRoadFailure::ExpectedDisagreement);
    };
    assert_eq!(finding.class(), FailureClass::OracleDisagreement);
    assert_eq!(
        finding.cause(),
        FindingCause::named(ORACLE_CAUSE_FAMILY, "transcript-derivation-disagreement")
    );
    Ok(())
}

#[test]
fn a_context_segment_cannot_smuggle_its_own_separator() {
    assert_eq!(
        SpecifiedContext::spelled(&["macroonz/harness-identity", "oracle-transcript", "v1"]),
        Err(ContextRefusal::EmbeddedSeparator { at: 0usize })
    );
}
