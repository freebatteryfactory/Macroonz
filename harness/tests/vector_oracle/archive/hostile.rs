//! Recomputed integrity cannot admit malformed method data or evade independent byte ceilings.

use super::{compiled, vector, wire};
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedOracle, OracleArchiveRefusal, read_verdict, retain_compilation,
    retain_compiled, retain_structural, retain_transcript, retain_vector,
};
use macroonz_harness::oracle::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict,
    DiagnosticAnchor, PrimarySourceSpanRefusal, RelativeSourcePathRefusal, RustcErrorCode,
    RustcErrorCodeRefusal, SourcePositionRefusal, SpecifiedContext, StructuralDisagreement,
    StructuralVerdict, TranscriptDerivation,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};

const LIMITS: ArchiveLimits = ArchiveLimits::declared(2048, 256);

fn records() -> Vec<(ArchivedMethod, u32, Vec<u8>)> {
    let mut vector = Vec::new();
    wire::frame(&[0, 255], &mut vector);
    wire::frame(&[0, 254], &mut vector);
    let mut transcript = Vec::new();
    wire::frame(&[1; 32], &mut transcript);
    wire::frame(&[2; 32], &mut transcript);
    let mut compilation = vec![1];
    wire::frame(b"E0308", &mut compilation);
    compilation.extend_from_slice(&compiled::span_bytes("src/main.rs", [1, 2, 3, 4]));
    vec![
        (ArchivedMethod::Vector, 1, wire::body(1, 1, &vector)),
        (ArchivedMethod::Transcript, 2, wire::body(2, 1, &transcript)),
        (
            ArchivedMethod::Structural,
            3,
            wire::body(3, 1, &wire::positioned_text(7, 4, "N")),
        ),
        (
            ArchivedMethod::Compiled,
            4,
            wire::body(4, 1, &wire::named(2, "N")),
        ),
        (
            ArchivedMethod::Compilation,
            5,
            wire::body(5, 1, &compilation),
        ),
    ]
}

fn refuses(body: &[u8], method: ArchivedMethod, reason: OracleArchiveRefusal) {
    assert_eq!(
        read_verdict(&wire::address(body), method, LIMITS),
        Err(reason)
    );
}

#[test]
fn every_method_refuses_unknown_headers_dispositions_and_trailing_material() -> Result<(), ()> {
    for (method, kind, body) in records() {
        assert!(read_verdict(&wire::address(&body), method, LIMITS).is_ok());
        for (offset, value, expected) in [
            (0usize, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
            (4, 99u32, ArchiveRefusal::WrongKind { found: 99 }),
            (8, 1u32, ArchiveRefusal::UnsupportedCustody { found: 1 }),
        ] {
            let mut changed = body.clone();
            changed
                .get_mut(offset..offset.saturating_add(4))
                .ok_or(())?
                .copy_from_slice(&value.to_be_bytes());
            refuses(&changed, method, expected.into());
        }
        let other = if method == ArchivedMethod::Vector {
            ArchivedMethod::Transcript
        } else {
            ArchivedMethod::Vector
        };
        refuses(
            &body,
            other,
            ArchiveRefusal::WrongKind { found: kind }.into(),
        );
        let mut unknown = body.clone();
        *unknown.get_mut(12).ok_or(())? = 255;
        refuses(&unknown, method, ArchiveRefusal::InvalidSlot.into());
        let mut trailing = body;
        trailing.push(0);
        refuses(&trailing, method, ArchiveRefusal::TrailingBytes.into());
    }
    Ok(())
}

#[test]
fn all_raw_and_readdressed_truncated_prefixes_refuse_without_allocating_claimed_lengths()
-> Result<(), ()> {
    for (method, _, body) in records() {
        let encoded = wire::address(&body);
        for end in 0..encoded.len() {
            assert!(read_verdict(encoded.get(..end).ok_or(())?, method, LIMITS).is_err());
        }
        for end in 0..body.len() {
            assert!(
                read_verdict(&wire::address(body.get(..end).ok_or(())?), method, LIMITS).is_err()
            );
        }
        let mut corrupt = encoded;
        *corrupt.last_mut().ok_or(())? ^= 1;
        assert_eq!(
            read_verdict(&corrupt, method, LIMITS),
            Err(ArchiveRefusal::AddressMismatch.into())
        );
        let frame_offset = match method {
            ArchivedMethod::Vector | ArchivedMethod::Transcript => 13usize,
            ArchivedMethod::Structural => 22,
            ArchivedMethod::Compiled | ArchivedMethod::Compilation => 14,
        };
        let mut enormous = body;
        enormous
            .get_mut(frame_offset..frame_offset.saturating_add(8))
            .ok_or(())?
            .fill(255);
        assert!(matches!(
            read_verdict(&wire::address(&enormous), method, LIMITS),
            Err(OracleArchiveRefusal::Archive(
                ArchiveRefusal::Truncated | ArchiveRefusal::LengthOutsidePlatform { .. }
            )),
        ));
    }
    Ok(())
}

#[test]
fn vector_and_transcript_private_relationships_survive_untrusted_records() {
    let mut vector = Vec::new();
    wire::frame(b"same", &mut vector);
    wire::frame(b"same", &mut vector);
    refuses(
        &wire::body(1, 1, &vector),
        ArchivedMethod::Vector,
        OracleArchiveRefusal::EqualVectorBuffers,
    );
    let mut transcript = Vec::new();
    wire::frame(&[7; 32], &mut transcript);
    wire::frame(&[7; 32], &mut transcript);
    refuses(
        &wire::body(2, 1, &transcript),
        ArchivedMethod::Transcript,
        OracleArchiveRefusal::EqualTranscriptClaims,
    );
    for width in [31, 33] {
        let mut wrong_width = Vec::new();
        wire::frame(&vec![1; width], &mut wrong_width);
        wire::frame(&[2; 32], &mut wrong_width);
        refuses(
            &wire::body(2, 1, &wrong_width),
            ArchivedMethod::Transcript,
            ArchiveRefusal::InvalidAddressWidth.into(),
        );
    }
}

#[test]
fn free_text_requires_utf8_and_disagreement_rosters_are_closed() {
    for (method, kind, payload) in [
        (ArchivedMethod::Structural, 3, wire::position(7, 0)),
        (ArchivedMethod::Compiled, 4, vec![2]),
    ] {
        let mut invalid_text = payload;
        wire::frame(&[255], &mut invalid_text);
        refuses(
            &wire::body(kind, 1, &invalid_text),
            method,
            ArchiveRefusal::InvalidText.into(),
        );
    }
    for (method, kind) in [
        (ArchivedMethod::Structural, 3),
        (ArchivedMethod::Compiled, 4),
        (ArchivedMethod::Compilation, 5),
    ] {
        refuses(
            &wire::body(kind, 1, &[255]),
            method,
            ArchiveRefusal::InvalidSlot.into(),
        );
    }
}

#[test]
fn historical_compiler_anchors_reuse_code_path_position_and_span_refusals() {
    for (code, path, positions, expected) in [
        (
            "E308",
            "src/main.rs",
            [1, 1, 1, 2],
            OracleArchiveRefusal::ErrorCode(RustcErrorCodeRefusal::Grammar),
        ),
        (
            "E0308",
            "../main.rs",
            [1, 1, 1, 2],
            OracleArchiveRefusal::SourcePath(RelativeSourcePathRefusal::NonNormalSegment { at: 0 }),
        ),
        (
            "E0308",
            "src/main.rs",
            [0, 1, 1, 2],
            OracleArchiveRefusal::SourcePosition(SourcePositionRefusal::ZeroLine),
        ),
        (
            "E0308",
            "src/main.rs",
            [1, 0, 1, 2],
            OracleArchiveRefusal::SourcePosition(SourcePositionRefusal::ZeroColumn),
        ),
        (
            "E0308",
            "src/main.rs",
            [2, 1, 1, 2],
            OracleArchiveRefusal::PrimarySpan(PrimarySourceSpanRefusal::Reversed),
        ),
    ] {
        let mut payload = vec![1];
        wire::frame(code.as_bytes(), &mut payload);
        payload.extend_from_slice(&compiled::span_bytes(path, positions));
        refuses(
            &wire::body(5, 1, &payload),
            ArchivedMethod::Compilation,
            expected,
        );
    }
}

fn limits(
    method: ArchivedMethod,
    field: usize,
    retain: impl Fn(ArchiveLimits) -> Result<ArchivedOracle, OracleArchiveRefusal>,
) -> Result<(), ()> {
    let written = retain(LIMITS).map_err(|_| ())?;
    let width = written.encoded().len();
    let exact = ArchiveLimits::declared(width, field);
    assert_eq!(retain(exact), Ok(written.clone()));
    assert_eq!(
        read_verdict(written.encoded(), method, exact),
        Ok(written.clone())
    );
    for (offered, expected) in [
        (
            ArchiveLimits::declared(width.saturating_sub(1), field),
            ArchiveRefusal::EnvelopeTooLarge,
        ),
        (
            ArchiveLimits::declared(width, field.saturating_sub(1)),
            ArchiveRefusal::FieldTooLarge,
        ),
    ] {
        assert_eq!(retain(offered), Err(expected.into()));
        assert_eq!(
            read_verdict(written.encoded(), method, offered),
            Err(expected.into())
        );
    }
    Ok(())
}

#[test]
fn vector_and_transcript_writer_and_reader_ceilings_are_independent() -> Result<(), ()> {
    let vector = vector::verdict(&[0, 255], &[0, 254])?;
    limits(ArchivedMethod::Vector, 2, |limit| {
        retain_vector(&vector, limit)
    })?;
    let context = SpecifiedContext::spelled(&["outside", "limits"]).map_err(|_| ())?;
    let transcript = TranscriptDerivation::opened()
        .derived(&context)
        .compared(&[17; 32]);
    limits(ArchivedMethod::Transcript, 32, |limit| {
        retain_transcript(transcript, limit)
    })
}

#[test]
fn structural_and_compiler_writer_and_reader_ceilings_are_independent() -> Result<(), ()> {
    let structural = StructuralVerdict::Deviates(StructuralDisagreement::MemberValue {
        at: 0,
        member: "N".to_owned(),
    });
    limits(ArchivedMethod::Structural, 1, |limit| {
        retain_structural(&structural, limit)
    })?;
    let compiled = CompiledVerdict::Deviates(CompiledDisagreement::UnexpectedMember {
        member: "N".to_owned(),
    });
    limits(ArchivedMethod::Compiled, 1, |limit| {
        retain_compiled(&compiled, limit)
    })?;
    let compilation =
        CompilationVerdict::Deviates(CompilationDisagreement::RefusedWhereAcceptanceDeclared {
            observed: DiagnosticAnchor::at(
                RustcErrorCode::informed("E0308").map_err(|_| ())?,
                compiled::span("src/main.rs", [1, 2, 3, 4])?,
            ),
        });
    limits(ArchivedMethod::Compilation, 11, |limit| {
        retain_compilation(&compilation, limit)
    })
}
