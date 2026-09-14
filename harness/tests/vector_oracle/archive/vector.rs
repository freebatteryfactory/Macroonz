//! Independent vector records preserve complete bytes and the normalized-report boundary.

use super::wire;
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedVerdict, read_verdict, retain_vector,
};
use macroonz_harness::oracle::{ByteDifference, VectorPack, VectorVerdict};
use macroonz_harness::report::FindingLocation;
use macroonz_harness::report::archive::ArchiveLimits;

const LIMITS: ArchiveLimits = ArchiveLimits::declared(2048, 256);

pub(super) fn verdict(expected: &[u8], produced: &[u8]) -> Result<VectorVerdict, ()> {
    let mut pack = b"macroonz".to_vec();
    pack.extend_from_slice(&1u64.to_be_bytes());
    pack.extend_from_slice(&1u64.to_be_bytes());
    for material in [b"outside".as_slice(), b"vector".as_slice(), b"", expected] {
        wire::frame(material, &mut pack);
    }
    let pack = VectorPack::read(&pack).map_err(|_| ())?;
    Ok(pack.entries().first().ok_or(())?.compared(produced))
}

#[test]
fn complete_vector_buffers_match_independent_wire_bytes() -> Result<(), ()> {
    for (expected, produced, difference) in [
        (
            b"abc".as_slice(),
            b"ax".as_slice(),
            ByteDifference::AtByte { at: 1 },
        ),
        (
            b"abc".as_slice(),
            b"ab".as_slice(),
            ByteDifference::Length {
                expected: 3,
                produced: 2,
            },
        ),
        (
            b"".as_slice(),
            &[255][..],
            ByteDifference::Length {
                expected: 0,
                produced: 1,
            },
        ),
        (
            &[0, 255][..],
            &[0, 254][..],
            ByteDifference::AtByte { at: 1 },
        ),
    ] {
        let mut payload = Vec::new();
        wire::frame(expected, &mut payload);
        wire::frame(produced, &mut payload);
        let independent = wire::envelope(1, 1, &payload);
        let actual = retain_vector(&verdict(expected, produced)?, LIMITS).map_err(|_| ())?;
        assert_eq!(actual.encoded(), independent);
        let loaded = read_verdict(&independent, ArchivedMethod::Vector, LIMITS).map_err(|_| ())?;
        assert_eq!(loaded, actual);
        let ArchivedVerdict::VectorDisagrees(found) = loaded.verdict() else {
            return Err(());
        };
        assert_eq!(found.expected(), expected);
        assert_eq!(found.produced(), produced);
        assert_eq!(found.difference(), difference);
    }
    let agreement = retain_vector(&verdict(b"abc", b"abc")?, LIMITS).map_err(|_| ())?;
    assert_eq!(agreement.encoded(), wire::envelope(1, 0, &[]));
    assert_eq!(agreement.verdict(), &ArchivedVerdict::VectorAgrees);
    Ok(())
}

#[test]
fn distinct_details_with_one_normalized_conclusion_remain_distinct() -> Result<(), ()> {
    let first = verdict(b"abc", b"axc")?;
    let second = verdict(b"abc", b"ab")?;
    let site = FindingLocation::at("declared-test.rs", 7);
    assert_eq!(first.concluded(site), second.concluded(site));
    let first = retain_vector(&first, LIMITS).map_err(|_| ())?;
    let second = retain_vector(&second, LIMITS).map_err(|_| ())?;
    assert_ne!(first.verdict(), second.verdict());
    assert_ne!(first.address(), second.address());
    assert_ne!(first.encoded(), second.encoded());
    Ok(())
}
