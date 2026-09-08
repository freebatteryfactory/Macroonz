//! Sparse-field presence and hostile JSON are observed independently of the parser's visitors.

use super::legacy;
use macroonz_harness::report::legacy::{
    LegacyField, LegacyLimits, LegacyPresence, LegacyRefusal, read_record,
};

#[test]
fn every_nullable_top_level_field_preserves_explicit_null() -> Result<(), ()> {
    let record=read_record(br#"{"witness":null,"input_profile":null,"trial_name":null,"subject_name":null,"check_name":null,"subject_revision":null,"check_revision":null,"target":null,"toolchain":null,"execution_digest":null,"fingerprint_digest":null,"reported_outcome":null}"#,legacy::PROFILE,legacy::LIMITS).map_err(|_| ())?;
    assert_eq!(record.witness(), &LegacyPresence::Null);
    assert_eq!(record.input_profile(), &LegacyPresence::Null);
    assert_eq!(record.trial_name(), &LegacyPresence::Null);
    assert_eq!(record.subject_name(), &LegacyPresence::Null);
    assert_eq!(record.check_name(), &LegacyPresence::Null);
    assert_eq!(record.subject_revision(), &LegacyPresence::Null);
    assert_eq!(record.check_revision(), &LegacyPresence::Null);
    assert_eq!(record.target(), &LegacyPresence::Null);
    assert_eq!(record.toolchain(), &LegacyPresence::Null);
    assert_eq!(record.execution_digest(), &LegacyPresence::Null);
    assert_eq!(record.fingerprint_digest(), &LegacyPresence::Null);
    assert_eq!(record.reported_outcome(), &LegacyPresence::Null);
    let missing = read_record(b"{}", legacy::PROFILE, legacy::LIMITS).map_err(|_| ())?;
    assert_eq!(missing.trial_name(), &LegacyPresence::Missing);
    assert_eq!(missing.subject_name(), &LegacyPresence::Missing);
    assert_eq!(missing.check_name(), &LegacyPresence::Missing);
    assert_eq!(missing.subject_revision(), &LegacyPresence::Missing);
    assert_eq!(missing.check_revision(), &LegacyPresence::Missing);
    assert_eq!(missing.target(), &LegacyPresence::Missing);
    assert_eq!(missing.toolchain(), &LegacyPresence::Missing);
    assert_eq!(missing.execution_digest(), &LegacyPresence::Missing);
    assert_eq!(missing.fingerprint_digest(), &LegacyPresence::Missing);
    assert_eq!(missing.reported_outcome(), &LegacyPresence::Missing);
    Ok(())
}

#[test]
fn every_top_level_field_and_both_profile_leaves_refuse_duplicates() {
    for (member, field) in [
        (r#""kind":"neutral""#, LegacyField::Kind),
        (r#""schema":1"#, LegacyField::Schema),
        (r#""witness":null"#, LegacyField::Witness),
        (r#""input_profile":{}"#, LegacyField::InputProfile),
        (r#""trial_name":null"#, LegacyField::TrialName),
        (r#""subject_name":null"#, LegacyField::SubjectName),
        (r#""check_name":null"#, LegacyField::CheckName),
        (r#""subject_revision":0"#, LegacyField::SubjectRevision),
        (r#""check_revision":0"#, LegacyField::CheckRevision),
        (r#""target":null"#, LegacyField::Target),
        (r#""toolchain":null"#, LegacyField::Toolchain),
        (r#""execution_digest":null"#, LegacyField::ExecutionDigest),
        (
            r#""fingerprint_digest":null"#,
            LegacyField::FingerprintDigest,
        ),
        (r#""reported_outcome":null"#, LegacyField::ReportedOutcome),
    ] {
        let source = format!("{{{member},{member}}}");
        assert_eq!(
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS),
            Err(LegacyRefusal::DuplicateField(field))
        );
    }
    for (member, field) in [
        (r#""name":null"#, LegacyField::ProfileName),
        (r#""revision":0"#, LegacyField::ProfileRevision),
    ] {
        let source = format!(r#"{{"input_profile":{{{member},{member}}}}}"#);
        assert_eq!(
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS),
            Err(LegacyRefusal::DuplicateField(field))
        );
    }
}

#[test]
fn complete_digest_grammar_and_its_two_byte_budgets_are_observed() -> Result<(), ()> {
    for value in [
        "F".repeat(64),
        "g".repeat(64),
        "0".repeat(63),
        "0".repeat(65),
        "é".repeat(32),
    ] {
        let source = format!(r#"{{"execution_digest":"{value}"}}"#);
        assert_eq!(
            read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS),
            Err(LegacyRefusal::InvalidDigest)
        );
    }
    let source=br#"{"fingerprint_digest":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}"#;
    let retained = source.len().checked_add(32).ok_or(())?;
    assert!(
        read_record(
            source,
            legacy::PROFILE,
            LegacyLimits::declared(source.len(), 64, 0, 1, 1, retained)
        )
        .is_ok()
    );
    assert_eq!(
        read_record(
            source,
            legacy::PROFILE,
            LegacyLimits::declared(source.len(), 63, 0, 1, 1, retained)
        ),
        Err(LegacyRefusal::TextTooLarge)
    );
    assert_eq!(
        read_record(
            source,
            legacy::PROFILE,
            LegacyLimits::declared(source.len(), 64, 0, 1, 1, retained.saturating_sub(1))
        ),
        Err(LegacyRefusal::RetainedTooLarge)
    );
    Ok(())
}

#[test]
fn escaped_strings_use_decoded_limits_and_large_or_deep_sources_refuse() {
    let escaped = br#"{"target":"\u00e9"}"#;
    assert!(
        read_record(
            escaped,
            legacy::PROFILE,
            LegacyLimits::declared(64, 2, 0, 1, 1, 66)
        )
        .is_ok()
    );
    assert_eq!(
        read_record(
            escaped,
            legacy::PROFILE,
            LegacyLimits::declared(64, 1, 0, 1, 1, 66)
        ),
        Err(LegacyRefusal::TextTooLarge)
    );
    let source = format!(r#"{{"target":"{}"}}"#, "\\u0061".repeat(4096));
    let large = LegacyLimits::declared(65_536, 256, 64, 16, 2, 131_072);
    assert_eq!(
        read_record(source.as_bytes(), legacy::PROFILE, large),
        Err(LegacyRefusal::TextTooLarge)
    );
    assert_eq!(
        read_record(source.as_bytes(), legacy::PROFILE, legacy::LIMITS),
        Err(LegacyRefusal::SourceTooLarge)
    );
    let witness = format!(r#"{{"witness":[{}0]}}"#, "0,".repeat(1024));
    assert_eq!(
        read_record(witness.as_bytes(), legacy::PROFILE, legacy::LIMITS),
        Err(LegacyRefusal::WitnessTooLarge)
    );
    let nested = format!(r#"{{"witness":{}0{}}}"#, "[".repeat(1024), "]".repeat(1024));
    assert!(matches!(
        read_record(nested.as_bytes(), legacy::PROFILE, legacy::LIMITS),
        Err(LegacyRefusal::InvalidJson { .. })
    ));
    assert!(
        read_record(
            br#"{"witness":null,"input_profile":null}"#,
            legacy::PROFILE,
            LegacyLimits::declared(64, 0, 0, 2, 1, 64)
        )
        .is_ok()
    );
}

#[test]
fn scalar_and_profile_type_errors_never_become_absent_data() {
    for source in [
        br#"{"trial_name":0}"#.as_slice(),
        br#"{"subject_name":[]}"#,
        br#"{"check_name":false}"#,
        br#"{"subject_revision":"1"}"#,
        br#"{"check_revision":true}"#,
        br#"{"target":{}}"#,
        br#"{"toolchain":[]}"#,
        br#"{"execution_digest":[]}"#,
        br#"{"fingerprint_digest":1}"#,
        br#"{"reported_outcome":false}"#,
        br#"{"input_profile":{"name":[]}}"#,
        br#"{"input_profile":{"revision":1.0}}"#,
        br#"{"input_profile":{"revision":-1}}"#,
        br#"{"input_profile":{"revision":18446744073709551616}}"#,
    ] {
        assert!(
            matches!(
                read_record(source, legacy::PROFILE, legacy::LIMITS),
                Err(LegacyRefusal::InvalidJson { .. })
            ),
            "{source:?}"
        );
    }
}
