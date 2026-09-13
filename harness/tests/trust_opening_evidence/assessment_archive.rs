//! Complete faithful assessment retention, bounded failures and a separate historical reader.

use super::support::{CompiledRosterMeaning, REVISION_TAG};
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::descriptor::{NamespacedName, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::MutationAssessment;
use macroonz_harness::muterprater::discovery_archive::SurfaceArchiveLimits;
use macroonz_harness::muterprater::interpretation_archive::{
    ASSESSMENT_ARCHIVE_TAG, ArchivedAssessment, AssessmentArchiveLimits, AssessmentArchiveRefusal,
    ParityArchiveRefusal, ValueEncoder, ValueEncodingRefusal, ValueRole, read_assessment,
    retain_assessment,
};
use macroonz_harness::report::archive::ArchiveLimits;
use std::io::Read as _;
use std::sync::atomic::{AtomicU32, Ordering};

const BYTES: ArchiveLimits = ArchiveLimits::declared(131_072, 65536);
const LIMITS: AssessmentArchiveLimits = AssessmentArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 16, 16),
    BindingArchiveLimits::declared(16384, 512, 8),
    BYTES,
    16384,
    256,
    8,
);
static ENCODER_CALLS: AtomicU32 = AtomicU32::new(0);
static OVERSIZE_AT: AtomicU32 = AtomicU32::new(0);

fn with_bytes(bytes: ArchiveLimits) -> AssessmentArchiveLimits {
    AssessmentArchiveLimits::declared(
        bytes,
        LIMITS.surface(),
        LIMITS.binding(),
        LIMITS.reports(),
        LIMITS.source(),
        LIMITS.value(),
        LIMITS.substrates(),
    )
}

fn counted(bytes: Vec<u8>) -> Vec<u8> {
    let ordinal = ENCODER_CALLS
        .fetch_add(1, Ordering::SeqCst)
        .saturating_add(1);
    if ordinal == OVERSIZE_AT.load(Ordering::SeqCst) {
        vec![0; 257]
    } else {
        bytes
    }
}

const INPUT: fn(&[u32; 3]) -> Result<Vec<u8>, ValueEncodingRefusal> = |value| {
    Ok(counted(
        value.iter().flat_map(|word| word.to_be_bytes()).collect(),
    ))
};
const MEANING: fn(&CompiledRosterMeaning) -> Result<Vec<u8>, ValueEncodingRefusal> =
    |value| Ok(counted(format!("{value:?}").into_bytes()));

fn encoder<Value>(
    name: &'static str,
    encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
) -> Result<ValueEncoder<Value>, String> {
    Ok(ValueEncoder::declared(
        NamespacedName::named("assessment-value", name).map_err(|cause| format!("{cause:?}"))?,
        1,
        ContentAddress::derived(REVISION_TAG, name.as_bytes()),
        RevisionBinding::declared(ContentAddress::derived(
            REVISION_TAG,
            b"assessment-encoding",
        )),
        encode,
    ))
}

pub(super) fn controls(
    assessment: &MutationAssessment<'_, [u32; 3], CompiledRosterMeaning>,
    stronger: &MutationAssessment<'_, [u32; 3], CompiledRosterMeaning>,
) -> Result<(), String> {
    let input = encoder("input", INPUT)?;
    let meaning = encoder("meaning", MEANING)?;
    ENCODER_CALLS.store(0, Ordering::SeqCst);
    OVERSIZE_AT.store(0, Ordering::SeqCst);
    for limit in refused_limits() {
        assert!(retain_assessment(assessment, &input, &meaning, limit).is_err());
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 0);
    }
    for (at, role) in [
        (1, ValueRole::Input),
        (2, ValueRole::Production),
        (3, ValueRole::BaselineEvaluation),
        (4, ValueRole::CompiledBaseline),
        (5, ValueRole::CompiledSelected),
        (6, ValueRole::SelectedEvaluation),
    ] {
        ENCODER_CALLS.store(0, Ordering::SeqCst);
        OVERSIZE_AT.store(at, Ordering::SeqCst);
        assert_eq!(
            retain_assessment(assessment, &input, &meaning, LIMITS),
            Err(AssessmentArchiveRefusal::Parity(
                ParityArchiveRefusal::ValueTooLarge { role }
            ))
        );
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), at);
    }
    ENCODER_CALLS.store(0, Ordering::SeqCst);
    OVERSIZE_AT.store(0, Ordering::SeqCst);
    let record = retain_assessment(assessment, &input, &meaning, LIMITS)
        .map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 6);
    let [
        production,
        baseline,
        compiled_baseline,
        compiled_selected,
        selected,
    ] = record.meanings();
    assert_eq!(production.bytes(), b"Stated(1)");
    assert_eq!(baseline.bytes(), production.bytes());
    assert_eq!(compiled_baseline.bytes(), production.bytes());
    assert_eq!(selected.bytes(), b"Unstated");
    assert_eq!(compiled_selected.bytes(), selected.bytes());
    assert_eq!(
        record.input().bytes(),
        &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    let exact = with_bytes(ArchiveLimits::declared(
        record.encoded().len(),
        BYTES.field(),
    ));
    assert_eq!(
        retain_assessment(assessment, &input, &meaning, exact)
            .map_err(|cause| format!("{cause:?}"))?,
        record
    );
    let short = with_bytes(ArchiveLimits::declared(
        record.encoded().len().saturating_sub(1),
        BYTES.field(),
    ));
    assert!(retain_assessment(assessment, &input, &meaning, short).is_err());
    assert!(read_assessment(record.encoded(), short).is_err());
    let returned = super::archive_process::round_trip(
        record.encoded(),
        "assessment_archive::child_reads_assessment",
    )
    .map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(
        read_assessment(&returned, LIMITS).map_err(|cause| format!("{cause:?}"))?,
        record
    );
    let stronger_record = retain_assessment(stronger, &input, &meaning, LIMITS)
        .map_err(|cause| format!("{cause:?}"))?;
    reject_transplants(&record, &stronger_record)?;
    Ok(())
}

fn refused_limits() -> [AssessmentArchiveLimits; 7] {
    [
        with_bytes(ArchiveLimits::declared(1, 65536)),
        with_bytes(ArchiveLimits::declared(131_072, 31)),
        AssessmentArchiveLimits::declared(
            BYTES,
            SurfaceArchiveLimits::declared(BYTES, 0, 0),
            LIMITS.binding(),
            BYTES,
            16384,
            256,
            8,
        ),
        AssessmentArchiveLimits::declared(
            BYTES,
            LIMITS.surface(),
            BindingArchiveLimits::declared(1, 512, 8),
            BYTES,
            16384,
            256,
            8,
        ),
        AssessmentArchiveLimits::declared(
            BYTES,
            LIMITS.surface(),
            LIMITS.binding(),
            ArchiveLimits::declared(1, 65536),
            16384,
            256,
            8,
        ),
        AssessmentArchiveLimits::declared(
            BYTES,
            LIMITS.surface(),
            LIMITS.binding(),
            BYTES,
            0,
            256,
            8,
        ),
        AssessmentArchiveLimits::declared(
            BYTES,
            LIMITS.surface(),
            LIMITS.binding(),
            BYTES,
            16384,
            256,
            0,
        ),
    ]
}

fn reseal(encoded: &[u8]) -> Result<Vec<u8>, String> {
    let body = encoded.get(32..).ok_or("no assessment body")?;
    let mut result = ContentAddress::derived(ASSESSMENT_ARCHIVE_TAG, body)
        .as_bytes()
        .to_vec();
    result.extend_from_slice(body);
    Ok(result)
}

fn transplant(encoded: &[u8], before: &[u8], after: &[u8]) -> Result<Vec<u8>, String> {
    let mut old = u64::try_from(before.len())
        .map_err(|cause| format!("{cause:?}"))?
        .to_be_bytes()
        .to_vec();
    old.extend_from_slice(before);
    let at = encoded
        .windows(old.len())
        .rposition(|window| window == old)
        .ok_or("nested member absent")?;
    let mut replacement = u64::try_from(after.len())
        .map_err(|cause| format!("{cause:?}"))?
        .to_be_bytes()
        .to_vec();
    replacement.extend_from_slice(after);
    let mut changed = encoded.to_vec();
    let end = at
        .checked_add(old.len())
        .ok_or("nested member offset overflow")?;
    changed.splice(at..end, replacement);
    reseal(&changed)
}

fn reject_transplants(
    weak: &ArchivedAssessment,
    strong: &ArchivedAssessment,
) -> Result<(), String> {
    let [_, _, _, _, weak_report] = weak.reports();
    let [_, _, _, _, strong_report] = strong.reports();
    let foreign_judgment = transplant(
        weak.encoded(),
        weak_report.encoded(),
        strong_report.encoded(),
    )?;
    assert!(read_assessment(&foreign_judgment, LIMITS).is_err());
    let false_kill = transplant(
        weak.encoded(),
        weak.mutation().encoded(),
        strong.mutation().encoded(),
    )?;
    assert!(read_assessment(&false_kill, LIMITS).is_err());
    for (offset, word) in [(32usize, 2u32), (36, 2), (40, 1)] {
        let mut changed = weak.encoded().to_vec();
        let end = offset.checked_add(4).ok_or("header offset overflow")?;
        changed
            .get_mut(offset..end)
            .ok_or("short header")?
            .copy_from_slice(&word.to_be_bytes());
        assert!(read_assessment(&reseal(&changed)?, LIMITS).is_err());
    }
    let mut trailing = weak.encoded().to_vec();
    trailing.push(0);
    assert!(read_assessment(&reseal(&trailing)?, LIMITS).is_err());
    assert!(
        read_assessment(
            weak.encoded()
                .get(..weak.encoded().len().saturating_sub(1))
                .ok_or("short record")?,
            LIMITS
        )
        .is_err()
    );
    Ok(())
}

#[test]
#[ignore = "driven by the complete faithful assessment crossing"]
fn child_reads_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(131_073)
        .read_to_end(&mut bytes)?;
    let record = read_assessment(&bytes, LIMITS)
        .map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    super::archive_process::publish(record.encoded())
}
