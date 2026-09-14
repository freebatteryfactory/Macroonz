//! Historical oracle inspection retains method-specific data and authority limits.

use super::presentation_formats::{agree, decoded_hex, field};
use macroonz::harness::oracle::{
    CompilationVerdict, CompiledVerdict, SpecifiedContext, StructuralDisagreement,
    StructuralVerdict, TranscriptDerivation, TranscriptVerdict, VectorPack, VectorVerdict,
    archive::{self, ArchivedOracle},
};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::presentation;
use serde_json::{Value, json};

const LIMITS: ArchiveLimits = ArchiveLimits::declared(4096, 1024);

fn parsed(record: &ArchivedOracle) -> Result<Value, String> {
    let shown = presentation::archived_oracle(record);
    agree(&shown)?;
    let value: Value = serde_json::from_str(&shown.json()).map_err(|error| error.to_string())?;
    assert_eq!(field(&value, "/standing")?, "historical-unauthenticated");
    assert_eq!(
        field(&value, "/record/archive_address")?
            .as_str()
            .ok_or("missing address")?
            .len(),
        64
    );
    Ok(value)
}

#[test]
fn historical_transcript_disagreement_preserves_both_independent_digests() -> Result<(), String> {
    let context = SpecifiedContext::spelled(&["outside", "presentation"])
        .map_err(|error| format!("{error:?}"))?;
    let derived = TranscriptDerivation::opened()
        .framed(b"independent bytes")
        .derived(&context);
    let mut published = *derived.as_bytes();
    let byte = published.last_mut().ok_or("empty digest")?;
    *byte = byte.wrapping_add(1);
    let record = archive::retain_transcript(derived.compared(&published), LIMITS)
        .map_err(|error| format!("{error:?}"))?;
    let loaded = archive::read_verdict(
        record.encoded(),
        archive::ArchivedMethod::Transcript,
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?;
    let value = parsed(&loaded)?;
    assert_eq!(field(&value, "/record/method")?, "transcript");
    assert_eq!(field(&value, "/record/verdict/kind")?, "disagrees");
    for (name, bytes) in [("rederived", derived.as_bytes()), ("published", &published)] {
        assert_eq!(
            decoded_hex(field(&value, &format!("/record/verdict/value/{name}"))?)?,
            bytes.as_slice()
        );
    }
    assert!(field(&value, "/record")?.get("preimage").is_none());
    Ok(())
}

#[test]
fn successful_historical_methods_do_not_invent_payloads_or_current_observations()
-> Result<(), String> {
    let cases = [
        (
            archive::retain_vector(&VectorVerdict::Agrees, LIMITS),
            "vector",
            "agrees",
        ),
        (
            archive::retain_transcript(TranscriptVerdict::Agrees, LIMITS),
            "transcript",
            "agrees",
        ),
        (
            archive::retain_structural(&StructuralVerdict::Conforms, LIMITS),
            "structural",
            "conforms",
        ),
        (
            archive::retain_compiled(&CompiledVerdict::Conforms, LIMITS),
            "compiled",
            "conforms",
        ),
        (
            archive::retain_compilation(&CompilationVerdict::Conforms, LIMITS),
            "compilation",
            "conforms",
        ),
        (
            archive::retain_structural(&StructuralVerdict::Unparsable, LIMITS),
            "structural",
            "unparsable",
        ),
    ];
    for (record, method, kind) in cases {
        let record = record.map_err(|error| format!("{error:?}"))?;
        let value = parsed(&record)?;
        assert_eq!(field(&value, "/record/method")?, method);
        assert_eq!(
            field(&value, "/record/verdict")?,
            &json!({"kind":kind, "value":null})
        );
        assert_eq!(
            field(&value, "/record")?
                .as_object()
                .ok_or("missing record")?
                .len(),
            3
        );
    }
    Ok(())
}

#[test]
fn vector_inspection_keeps_buffers_and_distinguishes_byte_from_length_disagreement()
-> Result<(), String> {
    let mut pack = b"macroonz".to_vec();
    pack.extend_from_slice(&1u64.to_be_bytes());
    pack.extend_from_slice(&1u64.to_be_bytes());
    for bytes in [b"outside".as_slice(), b"presentation", b"", b"abc"] {
        pack.extend_from_slice(
            &u64::try_from(bytes.len())
                .map_err(|error| error.to_string())?
                .to_be_bytes(),
        );
        pack.extend_from_slice(bytes);
    }
    let pack = VectorPack::read(&pack).map_err(|error| format!("{error:?}"))?;
    let entry = pack.entries().first().ok_or("missing vector")?;
    for (produced, bytes, difference) in [
        (
            b"axc".as_slice(),
            "617863",
            json!({"kind":"at-byte", "value":{"at":1usize}}),
        ),
        (
            b"ab".as_slice(),
            "6162",
            json!({"kind":"length", "value":{"expected":3usize, "produced":2usize}}),
        ),
    ] {
        let record = archive::retain_vector(&entry.compared(produced), LIMITS)
            .map_err(|error| format!("{error:?}"))?;
        let value = parsed(&record)?;
        assert_eq!(field(&value, "/record/verdict/value/expected")?, "616263");
        assert_eq!(field(&value, "/record/verdict/value/produced")?, bytes);
        assert_eq!(
            field(&value, "/record/verdict/value/difference")?,
            &difference
        );
    }
    Ok(())
}

#[test]
fn historical_structural_free_text_and_coordinates_do_not_become_current_indexes()
-> Result<(), String> {
    let record = archive::retain_structural(
        &StructuralVerdict::Deviates(StructuralDisagreement::MemberValueUnread {
            at: usize::MAX,
            member: "<free>|\0é".to_owned(),
        }),
        LIMITS,
    )
    .map_err(|error| format!("{error:?}"))?;
    let shown = presentation::archived_oracle(&record);
    let value = parsed(&record)?;
    assert_eq!(
        field(&value, "/record/verdict/value/value/at")?,
        &json!(usize::MAX)
    );
    assert_eq!(
        field(&value, "/record/verdict/value/value/member")?,
        "<free>|\0é"
    );
    assert_eq!(
        field(&value, "/record/verdict/value/kind")?,
        "member-value-unread"
    );
    assert!(!shown.html().contains("<free>"));
    Ok(())
}
