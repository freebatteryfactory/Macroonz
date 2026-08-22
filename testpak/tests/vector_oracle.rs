//! A nonempty independently authored golden-vector pack exercises the public parser, byte comparison, and conclusion road.

use std::fmt;
use threadpak_testpak::oracle::{
    ByteDifference, ORACLE_CAUSE_FAMILY, VectorPack, VectorPackRefusal, VectorVerdict,
};
use threadpak_testpak::report::{FailureClass, FindingCause, FindingLocation, TrialConclusion};

const PACK: &[u8] = b"tpak-vec\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x07neutral\x00\x00\x00\x00\x00\x00\x00\x08xor-mask\x00\x00\x00\x00\x00\x00\x00\x03\x10\x20\x30\x00\x00\x00\x00\x00\x00\x00\x03\xba\x8a\x9a";

enum VectorRoadFailure {
    Pack(VectorPackRefusal),
    MissingEntry,
}

impl fmt::Debug for VectorRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pack(refusal) => formatter.debug_tuple("Pack").field(refusal).finish(),
            Self::MissingEntry => formatter.write_str("MissingEntry"),
        }
    }
}

impl From<VectorPackRefusal> for VectorRoadFailure {
    fn from(refusal: VectorPackRefusal) -> Self {
        Self::Pack(refusal)
    }
}

fn rendered(input: &[u8]) -> Vec<u8> {
    input.iter().map(|byte| byte ^ 0xaau8).collect()
}

fn refusal_signature(conclusion: &TrialConclusion) -> Option<(FailureClass, FindingCause)> {
    match conclusion {
        TrialConclusion::Passed => None,
        TrialConclusion::Refused(finding) => Some((finding.class(), finding.cause())),
    }
}

#[test]
fn nonempty_vector_pack_distinguishes_bytes_lengths_and_trailing_material()
-> Result<(), VectorRoadFailure> {
    let pack = VectorPack::read(PACK)?;
    let Some(entry) = pack.entries().first().copied() else {
        return Err(VectorRoadFailure::MissingEntry);
    };
    assert_eq!(pack.entries().len(), 1usize);
    assert_eq!(entry.subject().namespace(), "neutral");
    assert_eq!(entry.subject().stem(), "xor-mask");
    assert_eq!(entry.input(), &[0x10u8, 0x20u8, 0x30u8]);
    assert_eq!(entry.expected(), &[0xbau8, 0x8au8, 0x9au8]);

    let exact = entry.compared(&rendered(entry.input()));
    assert_eq!(exact, VectorVerdict::Agrees);
    assert_eq!(
        exact.concluded(FindingLocation::at(file!(), line!())),
        TrialConclusion::Passed
    );

    let middle = entry.compared(&[0xbau8, 0x00u8, 0x9au8]);
    let VectorVerdict::Disagrees(middle_difference) = &middle else {
        return Err(VectorRoadFailure::MissingEntry);
    };
    assert_eq!(
        middle_difference.difference(),
        ByteDifference::AtByte { at: 1usize }
    );
    assert_eq!(middle_difference.expected(), entry.expected());
    assert_eq!(middle_difference.produced(), &[0xbau8, 0x00u8, 0x9au8]);
    assert_eq!(
        refusal_signature(&middle.concluded(FindingLocation::at(file!(), line!()))),
        Some((
            FailureClass::OracleDisagreement,
            FindingCause::named(ORACLE_CAUSE_FAMILY, "golden-vector-disagreement"),
        ))
    );

    let prefix = entry.compared(&[0xbau8, 0x8au8]);
    let VectorVerdict::Disagrees(prefix_difference) = prefix else {
        return Err(VectorRoadFailure::MissingEntry);
    };
    assert_eq!(
        prefix_difference.difference(),
        ByteDifference::Length {
            expected: 3usize,
            produced: 2usize,
        }
    );

    let mut trailing = PACK.to_vec();
    trailing.push(0u8);
    assert_eq!(
        VectorPack::read(&trailing),
        Err(VectorPackRefusal::TrailingBytes { at: PACK.len() })
    );
    Ok(())
}
