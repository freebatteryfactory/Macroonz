//! Sparse history keeps absence, exact source and unauthenticated claim comparisons.

use super::fixture::{self, mapped};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::report::{
    legacy::{LegacyLimits, LegacyProfile, LegacyRecord, read_record},
    replay::compare_legacy,
};
use macroonz::presentation;
use serde_json::{Value, json};
use std::fmt::Write as _;

fn read(source: &[u8]) -> Result<LegacyRecord, String> {
    mapped(read_record(
        source,
        LegacyProfile::declared("witness", 1),
        LegacyLimits::declared(4096, 1024, 128, 32, 2, 8192),
    ))
}

#[test]
fn presentation_legacy_preserves_missing_null_empty_and_zero_without_defaults() -> Result<(), String>
{
    for (source, expected, profile) in [
        ("{}", "missing", json!({"kind":"missing","value":null})),
        (
            r#"{"witness":null,"subject_revision":null,"trial_name":null,"input_profile":null}"#,
            "null",
            json!({"kind":"null","value":null}),
        ),
        (
            r#"{"witness":[],"subject_revision":0,"trial_name":"","input_profile":{}}"#,
            "present",
            json!({"kind":"present","value":{
                "name":{"kind":"missing","value":null}, "revision":{"kind":"missing","value":null},
            }}),
        ),
        (
            r#"{"witness":[],"subject_revision":0,"trial_name":"","input_profile":{"name":null,"revision":null}}"#,
            "present",
            json!({"kind":"present","value":{
                "name":{"kind":"null","value":null}, "revision":{"kind":"null","value":null},
            }}),
        ),
    ] {
        let shown = parsed(&presentation::legacy_record(&read(source.as_bytes())?))?;
        assert_eq!(field(&shown, "/standing")?, "historical-unauthenticated");
        for name in ["kind", "schema"] {
            assert_eq!(field(&shown, &format!("/record/{name}/kind"))?, "missing");
        }
        for name in ["witness", "subject_revision", "trial_name"] {
            assert_eq!(field(&shown, &format!("/record/{name}/kind"))?, expected);
        }
        assert_eq!(field(&shown, "/record/input_profile")?, &profile);
        if expected == "present" {
            assert_eq!(field(&shown, "/record/witness/value")?, "");
            assert_eq!(
                field(&shown, "/record/subject_revision/value")?,
                &json!(0u64)
            );
            assert_eq!(field(&shown, "/record/trial_name/value")?, "");
        }
    }
    Ok(())
}

#[test]
fn presentation_legacy_preserves_every_supplied_field_and_exact_source() -> Result<(), String> {
    let source = json!({
        "kind":"witness", "schema":1u64, "witness":[0u8,255u8,13u8],
        "input_profile":{"name":"<profile>|\n","revision":0u64},
        "trial_name":" unsplit/trial ", "subject_name":"<subject>", "check_name":"|check|",
        "subject_revision":0u64, "check_revision":u64::MAX,
        "target":"other-target", "toolchain":"other-tool", "reported_outcome":"Passed <script>|\n",
        "execution_digest":"ab".repeat(32), "fingerprint_digest":"cd".repeat(32),
    });
    let compact = source.to_string();
    let spaced = format!(" \n{compact}\n");
    let first = parsed(&presentation::legacy_record(&read(compact.as_bytes())?))?;
    let second = parsed(&presentation::legacy_record(&read(spaced.as_bytes())?))?;
    assert_ne!(
        field(&first, "/record/source_address")?,
        field(&second, "/record/source_address")?
    );
    for (name, expected) in source.as_object().ok_or("source missing")? {
        let current = field(&first, &format!("/record/{name}"))?;
        assert_eq!(current, field(&second, &format!("/record/{name}"))?);
        assert_eq!(field(current, "/kind")?, "present");
        if name != "witness" && name != "input_profile" {
            assert_eq!(field(current, "/value")?, expected);
        }
    }
    assert_eq!(field(&first, "/record/witness/value")?, "00ff0d");
    assert_eq!(
        field(&first, "/record/input_profile/value/name/value")?,
        "<profile>|\n"
    );
    assert_eq!(
        field(&first, "/record/input_profile/value/revision/value")?,
        &json!(0u64)
    );
    assert_eq!(
        decoded_hex(field(&second, "/record/source")?)?,
        spaced.as_bytes()
    );
    assert!(field(&first, "/record")?.get("outcome").is_none());
    Ok(())
}

fn claim<'value>(shown: &'value Value, name: &str) -> Result<&'value Value, String> {
    let claims = field(shown, "/record/claims")?
        .as_array()
        .ok_or("claims missing")?;
    assert_eq!(claims.len(), 16);
    let found = claims
        .iter()
        .find(|value| value.get("field").and_then(Value::as_str) == Some(name))
        .ok_or_else(|| format!("claim missing: {name}"))?;
    field(found, "/relation")
}

#[test]
fn presentation_legacy_claims_never_upgrade_matching_labels_or_digest_claims() -> Result<(), String>
{
    for payload in [vec![1u8], vec![2u8]] {
        let run = fixture::run(&payload)?;
        let current = fixture::selected(&run)?;
        let mut key = String::new();
        for byte in current.standing().key().address().as_bytes() {
            write!(key, "{byte:02x}").map_err(|error| error.to_string())?;
        }
        for (profile, profile_expected) in [
            (None, "parent-missing"),
            (Some(Value::Null), "parent-null"),
            (Some(json!({})), "missing"),
            (Some(json!({"name":null})), "null"),
            (
                Some(json!({"name":"free name","revision":0u64})),
                "uninterpreted",
            ),
        ] {
            let mut source = json!({
                "witness":payload, "target":"declared-workflow-host", "toolchain":"different",
                "execution_digest":key, "fingerprint_digest":"00".repeat(32),
                "reported_outcome":"passed", "subject_revision":0u64, "check_name":null,
            });
            if let Some(profile) = profile {
                source
                    .as_object_mut()
                    .ok_or("source missing")?
                    .insert("input_profile".into(), profile);
            }
            let historical = read(source.to_string().as_bytes())?;
            let comparison = mapped(compare_legacy(&historical, current, run.input()))?;
            fixture::reset();
            let shown = parsed(&presentation::legacy_comparison(&comparison))?;
            assert_eq!(fixture::observations(), (0, 0, 0));
            assert_eq!(
                field(&shown, "/record/historical")?,
                &json!({
                    "kind":"unverifiable", "value":"incomplete-legacy-record",
                })
            );
            for (name, expected) in [
                ("input_profile.name", profile_expected),
                ("target", "same-claim"),
                ("toolchain", "moved-claim"),
                ("execution_digest", "same-claim"),
                ("reported_outcome", "uninterpreted"),
                ("subject_revision", "uninterpreted"),
                ("trial_name", "missing"),
                ("check_name", "null"),
                (
                    "fingerprint_digest",
                    if payload == [1u8] {
                        "moved-claim"
                    } else {
                        "current-unavailable"
                    },
                ),
            ] {
                assert_eq!(claim(&shown, name)?, expected);
            }
            assert!(field(&shown, "/record")?.get("outcome").is_none());
        }
    }
    Ok(())
}

#[test]
fn presentation_legacy_refused_witness_does_not_invent_a_comparison() -> Result<(), String> {
    let run = fixture::run(&[1])?;
    for (source, expected) in [
        ("{}", json!({"kind":"missing-witness","value":null})),
        (
            r#"{"witness":null}"#,
            json!({"kind":"null-witness","value":null}),
        ),
        (
            r#"{"witness":[2]}"#,
            json!({"kind":"current","value":"witness-bytes-differ"}),
        ),
    ] {
        let refusal = compare_legacy(
            &read(source.as_bytes())?,
            fixture::selected(&run)?,
            run.input(),
        )
        .err()
        .ok_or("invalid witness joined")?;
        let shown = parsed(&presentation::legacy_join_refusal(refusal))?;
        assert_eq!(field(&shown, "/record/cause")?, &expected);
        assert!(field(&shown, "/record")?.get("claims").is_none());
    }
    Ok(())
}
