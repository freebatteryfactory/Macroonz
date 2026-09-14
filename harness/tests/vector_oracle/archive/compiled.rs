//! Compiled records retain each exact caller-stated payload through existing diagnostic guards.

use super::wire;
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedVerdict, read_verdict, retain_compilation, retain_compiled,
};
use macroonz_harness::oracle::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict,
    DiagnosticAnchor, PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use macroonz_harness::report::archive::ArchiveLimits;

const LIMITS: ArchiveLimits = ArchiveLimits::declared(2048, 256);

pub(super) fn span(path: &str, coordinates: [u64; 4]) -> Result<PrimarySourceSpan, ()> {
    let [line, column, end_line, end_column] = coordinates;
    PrimarySourceSpan::informed(
        RelativeSourcePath::informed(path).map_err(|_| ())?,
        SourcePosition::informed(line, column).map_err(|_| ())?,
        SourcePosition::informed(end_line, end_column).map_err(|_| ())?,
    )
    .map_err(|_| ())
}

pub(super) fn span_bytes(path: &str, coordinates: [u64; 4]) -> Vec<u8> {
    let mut bytes = Vec::new();
    wire::frame(path.as_bytes(), &mut bytes);
    for coordinate in coordinates {
        bytes.extend_from_slice(&coordinate.to_be_bytes());
    }
    bytes
}

#[test]
fn every_compiled_readback_arm_and_free_member_string_is_retained() -> Result<(), ()> {
    for (found, payload) in [
        (CompiledDisagreement::AcceptedWhereRefusalDeclared, vec![0]),
        (
            CompiledDisagreement::RefusedWhereAcceptanceDeclared,
            vec![1],
        ),
        (
            CompiledDisagreement::UnexpectedMember {
                member: "extra".to_owned(),
            },
            wire::named(2, "extra"),
        ),
        (
            CompiledDisagreement::DuplicateMember {
                member: "é\0".to_owned(),
            },
            wire::named(3, "é\0"),
        ),
        (
            CompiledDisagreement::MissingMember {
                member: "missing".to_owned(),
            },
            wire::named(4, "missing"),
        ),
        (
            CompiledDisagreement::MemberValue {
                member: String::new(),
            },
            wire::named(5, ""),
        ),
    ] {
        let written =
            retain_compiled(&CompiledVerdict::Deviates(found.clone()), LIMITS).map_err(|_| ())?;
        let independent = wire::envelope(4, 1, &payload);
        assert_eq!(written.encoded(), independent);
        let loaded =
            read_verdict(&independent, ArchivedMethod::Compiled, LIMITS).map_err(|_| ())?;
        assert_eq!(loaded.verdict(), &ArchivedVerdict::CompiledDeviates(found));
    }
    let written = retain_compiled(&CompiledVerdict::Conforms, LIMITS).map_err(|_| ())?;
    assert_eq!(written.encoded(), wire::envelope(4, 0, &[]));
    assert_eq!(written.verdict(), &ArchivedVerdict::CompiledConforms);
    Ok(())
}

#[test]
fn exact_compilation_agreement_acceptance_and_refusal_preserve_their_distinct_data()
-> Result<(), ()> {
    let source = span("src/main.rs", [2, 3, 4, 5])?;
    let code = RustcErrorCode::informed("E0308").map_err(|_| ())?;
    let mut refused = vec![1];
    wire::frame(b"E0308", &mut refused);
    refused.extend_from_slice(&span_bytes("src/main.rs", [2, 3, 4, 5]));
    for (verdict, expected, slot, payload) in [
        (
            CompilationVerdict::Conforms,
            ArchivedVerdict::CompilationConforms,
            0,
            vec![],
        ),
        (
            CompilationVerdict::Deviates(CompilationDisagreement::AcceptedWhereRefusalDeclared),
            ArchivedVerdict::CompilationDeviates(
                CompilationDisagreement::AcceptedWhereRefusalDeclared,
            ),
            1,
            vec![0],
        ),
        (
            CompilationVerdict::Deviates(CompilationDisagreement::RefusedWhereAcceptanceDeclared {
                observed: DiagnosticAnchor::at(code.clone(), source.clone()),
            }),
            ArchivedVerdict::CompilationDeviates(
                CompilationDisagreement::RefusedWhereAcceptanceDeclared {
                    observed: DiagnosticAnchor::at(code, source),
                },
            ),
            1,
            refused,
        ),
    ] {
        let written = retain_compilation(&verdict, LIMITS).map_err(|_| ())?;
        let independent = wire::envelope(5, slot, &payload);
        assert_eq!(written.encoded(), independent);
        assert_eq!(
            read_verdict(&independent, ArchivedMethod::Compilation, LIMITS)
                .map_err(|_| ())?
                .verdict(),
            &expected
        );
    }
    Ok(())
}

#[test]
fn exact_code_and_span_disagreements_preserve_all_fields_including_caller_stated_equal_pairs()
-> Result<(), ()> {
    for other in ["E0308", "E0277"] {
        let found = CompilationDisagreement::ErrorCode {
            expected: RustcErrorCode::informed("E0308").map_err(|_| ())?,
            observed: RustcErrorCode::informed(other).map_err(|_| ())?,
        };
        let mut payload = vec![2];
        wire::frame(b"E0308", &mut payload);
        wire::frame(other.as_bytes(), &mut payload);
        assert_exact(found, &payload)?;
    }
    for (other_path, other_coordinates) in [
        ("src/a.rs", [7, 8, 7, 8]),
        ("src/é.rs", [u64::MAX, 11, u64::MAX, 12]),
    ] {
        let found = CompilationDisagreement::PrimarySpan {
            expected: span("src/a.rs", [7, 8, 7, 8])?,
            observed: span(other_path, other_coordinates)?,
        };
        let mut payload = vec![3];
        payload.extend_from_slice(&span_bytes("src/a.rs", [7, 8, 7, 8]));
        payload.extend_from_slice(&span_bytes(other_path, other_coordinates));
        assert_exact(found, &payload)?;
    }
    Ok(())
}

fn assert_exact(found: CompilationDisagreement, payload: &[u8]) -> Result<(), ()> {
    let written =
        retain_compilation(&CompilationVerdict::Deviates(found.clone()), LIMITS).map_err(|_| ())?;
    let independent = wire::envelope(5, 1, payload);
    assert_eq!(written.encoded(), independent);
    let loaded = read_verdict(&independent, ArchivedMethod::Compilation, LIMITS).map_err(|_| ())?;
    assert_eq!(
        loaded.verdict(),
        &ArchivedVerdict::CompilationDeviates(found)
    );
    Ok(())
}
