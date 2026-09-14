//! Literal controls for the Cargo diagnostic observation profile.

use super::{
    DiagnosticReadFailure, E0277_AT_LOCUS, E0308_AT_LOCUS, InfrastructureFault, ObservationRefusal,
    SourceField, cargo_diagnostic, observed_refusal, test_locus, test_root,
};
use macroonz_harness::oracle::{
    CompilationDisagreement, CompilationVerdict, DeclaredCompilation, DiagnosticAnchor,
    PrimarySourceSpan, RustcErrorCode, SourcePosition,
};

fn capture_refuses(bytes: &[u8]) -> Result<(), String> {
    let result = observed_refusal(bytes, &test_root("diagnostic-protocol"), &test_locus()?);
    if matches!(
        result,
        Err(DiagnosticReadFailure::Infrastructure {
            fault: InfrastructureFault::CaptureFailed,
            detail: _,
        })
    ) {
        Ok(())
    } else {
        Err(format!(
            "expected unreadable capture for {bytes:?}, got {result:?}"
        ))
    }
}

#[test]
fn envelope_kind_is_order_independent_and_fields_are_schema_local() -> Result<(), String> {
    for line in [
        r#"{"reason":"future","message":"opaque"}"#,
        r#"{"message":"opaque","reason":"future"}"#,
        r#"{"message":null}"#,
        r#"{"message":{"reason":null,"level":false,"code":[]},"reason":"future"}"#,
        r#"{"unknown":1,"unknown":2}"#,
        r#"{"Reason":false,"reason":"future"}"#,
    ] {
        assert!(cargo_diagnostic(line)?.is_none(), "{line}");
    }
    for line in [
        r#"{"reason":"compiler-message","message":{"level":"warning","message":"human text","code":null,"spans":[]}}"#,
        r#"{"message":{"message":"human text","spans":[],"code":null,"level":"warning"},"reason":"compiler-message"}"#,
    ] {
        let diagnostic = cargo_diagnostic(line)?.ok_or("missing diagnostic")?;
        assert_eq!(diagnostic.level.as_deref(), Some("warning"));
        assert_eq!(diagnostic.code, None);
        assert!(diagnostic.spans.is_empty());
    }
    for line in [
        r#"{"reason":null}"#,
        r#"{"reason":7}"#,
        r#"{"reason":"compiler-message"}"#,
        r#"{"reason":"compiler-message","message":null}"#,
        r#"{"message":"opaque","reason":"compiler-message"}"#,
        r#"{"reason":"compiler-message","message":{"level":"warning","spans":false}}"#,
        r#"{"reason":"compiler-message","message":{"level":"warning","code":{"code":7}}}"#,
    ] {
        capture_refuses(line.as_bytes())?;
    }
    Ok(())
}

#[test]
fn decoded_recognized_duplicates_refuse_even_after_null_or_equal_values() -> Result<(), String> {
    for line in [
        r#"{"reason":"future","reason":"future"}"#,
        r#"{"reason":"future","re\u0061son":"future"}"#,
        r#"{"reason":"future","message":null,"message":null}"#,
        r#"{"message":{},"mess\u0061ge":{},"reason":"compiler-message"}"#,
        r#"{"reason":"compiler-message","message":{"code":null,"code":null}}"#,
        r#"{"reason":"compiler-message","message":{"code":null,"c\u006fde":{"code":"E0308"}}}"#,
        r#"{"reason":"compiler-message","message":{"code":{"code":"E0308","code":"E0308"}}}"#,
        r#"{"reason":"compiler-message","message":{"level":"error","lev\u0065l":"error"}}"#,
        r#"{"reason":"compiler-message","message":{"spans":[],"spans":[]}}"#,
    ] {
        capture_refuses(line.as_bytes())?;
    }
    for members in [
        r#""file_name":"a","file_name":"a""#,
        r#""line_start":1,"line_start":1"#,
        r#""line_end":1,"line_end":1"#,
        r#""column_start":1,"column_start":1"#,
        r#""column_end":1,"column_end":1"#,
        r#""is_primary":false,"is_prim\u0061ry":false"#,
    ] {
        capture_refuses(span_row(members).as_bytes())?;
    }
    Ok(())
}

fn span_row(members: &str) -> String {
    format!(r#"{{"reason":"compiler-message","message":{{"spans":[{{{members}}}]}}}}"#)
}

#[test]
fn absence_remains_absence_but_present_invalid_values_never_default() -> Result<(), String> {
    for payload in ["{}", r#"{"code":null}"#, r#"{"code":{}}"#] {
        let row = format!(r#"{{"reason":"compiler-message","message":{payload}}}"#);
        let diagnostic = cargo_diagnostic(&row)?.ok_or("missing diagnostic")?;
        assert_eq!(diagnostic.code, None);
        assert_eq!(diagnostic.level, None);
        assert!(diagnostic.spans.is_empty());
    }
    let empty_span_row = span_row("");
    let diagnostic = cargo_diagnostic(&empty_span_row)?.ok_or("missing diagnostic")?;
    let [span] = diagnostic.spans.as_slice() else {
        return Err("empty span object did not remain one observation".to_owned());
    };
    assert_eq!(span.file_name, None);
    assert_eq!(span.line_start, None);
    assert_eq!(span.line_end, None);
    assert_eq!(span.column_start, None);
    assert_eq!(span.column_end, None);
    assert_eq!(span.is_primary, None);
    for payload in [
        r#"{"level":null}"#,
        r#"{"level":true}"#,
        r#"{"spans":null}"#,
        r#"{"spans":{}}"#,
        r#"{"spans":[null]}"#,
        r#"{"code":[]}"#,
        r#"{"code":{"code":null}}"#,
    ] {
        let row = format!(r#"{{"reason":"compiler-message","message":{payload}}}"#);
        capture_refuses(row.as_bytes())?;
    }
    for name in [
        "file_name",
        "line_start",
        "line_end",
        "column_start",
        "column_end",
        "is_primary",
    ] {
        for invalid in ["null", "[]", "{}"] {
            capture_refuses(span_row(&format!(r#""{name}":{invalid}"#)).as_bytes())?;
        }
    }
    for members in [
        r#""file_name":7"#,
        r#""is_primary":0"#,
        r#""is_primary":"true""#,
    ] {
        capture_refuses(span_row(members).as_bytes())?;
    }
    let missing_location = br#"{"reason":"compiler-message","message":{"level":"error","spans":[{"is_primary":true}]}}"#;
    assert_eq!(
        observed_refusal(
            missing_location,
            &test_root("missing-location"),
            &test_locus()?
        ),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MissingSourceField(SourceField::FileName)
        ))
    );
    Ok(())
}

#[test]
fn coordinates_are_exact_unsigned_integers_and_unknown_numbers_use_parser_capabilities()
-> Result<(), String> {
    for (literal, expected) in [
        ("0", 0u64),
        ("9007199254740993", 9_007_199_254_740_993u64),
        ("18446744073709551615", u64::MAX),
    ] {
        let diagnostic = cargo_diagnostic(&span_row(&format!(r#""line_start":{literal}"#)))?
            .ok_or("missing diagnostic")?;
        let [span] = diagnostic.spans.as_slice() else {
            return Err("missing numeric span".to_owned());
        };
        assert_eq!(span.line_start, Some(expected));
    }
    for literal in [
        "-1",
        "-0",
        "1.0",
        "1e0",
        "18446744073709551616",
        "\"1\"",
        "true",
        "01",
        "+1",
    ] {
        capture_refuses(span_row(&format!(r#""line_start":{literal}"#)).as_bytes())?;
    }
    for number in ["-1", "-0", "1.5", "1e3", "18446744073709551616", "1e308"] {
        assert!(cargo_diagnostic(&format!(r#"{{"unknown":{number}}}"#))?.is_none());
    }
    for number in ["1e309", "-1e309", "1e", "1+2", "--1", "NaN", "Infinity"] {
        capture_refuses(format!(r#"{{"unknown":{number}}}"#).as_bytes())?;
    }
    Ok(())
}

#[test]
fn unicode_is_validated_in_retained_and_discarded_keys_and_values() -> Result<(), String> {
    let escaped = r#"{"re\u0061son":"compiler-message","message":{"level":"err\u006fr","spans":[{"file_name":"src/\uD83E\uDD80.rs"}]}}"#;
    let raw = r#"{"reason":"compiler-message","message":{"level":"error","spans":[{"file_name":"src/🦀.rs"}]}}"#;
    let first = cargo_diagnostic(escaped)?.ok_or("missing escaped diagnostic")?;
    let second = cargo_diagnostic(raw)?.ok_or("missing raw diagnostic")?;
    assert_eq!(first.level, second.level);
    let ([first_span], [second_span]) = (first.spans.as_slice(), second.spans.as_slice()) else {
        return Err("missing Unicode spans".to_owned());
    };
    assert_eq!(first_span.file_name, second_span.file_name);
    for line in [
        r#"{"\uD83E\uDD80":[{"key":"\uD83E\uDD80"}],"🦀":"raw"}"#,
        r#"{"é":1,"e\u0301":2}"#,
        r#"{"reason":"future","message":[null,true,false,1,-1,1.5,"text",{"nested":[] }]}"#,
    ] {
        assert!(cargo_diagnostic(line)?.is_none());
    }
    for line in [
        r#"{"\uD800":0}"#,
        r#"{"unknown":"\uDC00"}"#,
        r#"{"unknown":[{"nested":"\uD800"}]}"#,
        r#"{"unknown":[{"\uDC00":0}]}"#,
        r#"{"reason":"future","message":"\uD800x"}"#,
        r#"{"reason":"compiler-message","message":{"level":"\uD800"}}"#,
        r#"{"reason":"compiler-message","message":{"message":"\uDC00"}}"#,
        r#"{"reason":"compiler-message","message":{"spans":[{"file_name":"\uD800\u0041"}]}}"#,
    ] {
        capture_refuses(line.as_bytes())?;
    }
    capture_refuses(b"{\"unknown\":\"raw\x01control\"}")?;
    capture_refuses(b"{\"unknown\":\"\xff\"}")?;
    Ok(())
}

#[test]
fn framing_consumes_each_complete_physical_record_and_all_later_records() -> Result<(), String> {
    let root = test_root("diagnostic-framing");
    let locus = test_locus()?;
    let expected = observed_refusal(E0308_AT_LOCUS, &root, &locus);
    assert!(expected.is_ok());
    for (prefix, suffix) in [
        (b" \t\r\n".as_slice(), b" \t\r\n".as_slice()),
        (b"\n\n".as_slice(), b"\n{}\n".as_slice()),
        (b"".as_slice(), b"".as_slice()),
    ] {
        assert_eq!(
            observed_refusal(&[prefix, E0308_AT_LOCUS, suffix].concat(), &root, &locus),
            expected
        );
    }
    for line in [
        "[]",
        "null",
        "7",
        "\"text\"",
        "{}{}",
        "{}tail",
        "{",
        "{\"x\":1,}",
        "{\"x\":[1,]}",
        "{/*comment*/}",
        "{\"x\":",
        "\u{000b}",
        "\u{000c}",
        "\u{00a0}",
        "\u{2028}",
        "\u{feff}{}",
    ] {
        capture_refuses(line.as_bytes())?;
        capture_refuses(&[E0308_AT_LOCUS, b"\n", line.as_bytes()].concat())?;
    }
    capture_refuses(&[E0308_AT_LOCUS, b" {}"].concat())?;
    capture_refuses(
        &[
            E0308_AT_LOCUS,
            b"\n{\"reason\":\"future\",\"message\":\"truncated",
        ]
        .concat(),
    )?;
    Ok(())
}

#[test]
fn retained_and_discarded_containers_share_the_pinned_depth_boundary() -> Result<(), String> {
    // The outer object consumes one level; the pinned default admits 127 open containers.
    for depth in [126usize, 127usize] {
        let arrays = format!("{}0{}", "[".repeat(depth), "]".repeat(depth));
        let objects = format!("{}0{}", r#"{"x":"#.repeat(depth), "}".repeat(depth));
        for value in [arrays, objects] {
            let row = format!(r#"{{"unknown":{value}}}"#);
            if depth == 126usize {
                assert!(cargo_diagnostic(&row)?.is_none());
            } else {
                capture_refuses(row.as_bytes())?;
            }
        }
    }
    // Envelope, diagnostic, spans array and span consume four levels before unknown nesting.
    for depth in [123usize, 124usize] {
        let row = span_row(&format!(
            r#""unknown":{}0{}"#,
            "[".repeat(depth),
            "]".repeat(depth)
        ));
        if depth == 123usize {
            assert!(cargo_diagnostic(&row)?.is_some());
        } else {
            capture_refuses(row.as_bytes())?;
        }
    }
    Ok(())
}

#[test]
fn an_independently_expected_code_does_not_replace_the_observed_code() -> Result<(), String> {
    let root = test_root("diagnostic-independent-code");
    let locus = test_locus()?;
    let code = RustcErrorCode::informed("E0308")
        .map_err(|failure| format!("expected code refused: {failure:?}"))?;
    let start = SourcePosition::informed(1u64, 1u64)
        .map_err(|failure| format!("expected start refused: {failure:?}"))?;
    let end = SourcePosition::informed(1u64, 2u64)
        .map_err(|failure| format!("expected end refused: {failure:?}"))?;
    let primary = PrimarySourceSpan::informed(locus.clone(), start, end)
        .map_err(|failure| format!("expected span refused: {failure:?}"))?;
    let expected_anchor = DiagnosticAnchor::at(code, primary);
    let observed = observed_refusal(E0277_AT_LOCUS, &root, &locus)
        .map_err(|failure| format!("observation refused: {failure:?}"))?;
    let observed_anchor = observed.refusal().ok_or("missing observed anchor")?;
    assert_eq!(observed_anchor.code().spelling(), "E0277");
    let verdict = macroonz_harness::oracle::compiled::compared_compilation(
        &observed,
        &DeclaredCompilation::refuses(expected_anchor.clone()),
    );
    assert_eq!(
        verdict,
        CompilationVerdict::Deviates(CompilationDisagreement::ErrorCode {
            expected: expected_anchor.code().clone(),
            observed: observed_anchor.code().clone(),
        })
    );
    Ok(())
}

#[test]
fn primary_mapping_precedes_locus_selection_and_code_admission() -> Result<(), String> {
    let root = test_root("diagnostic-mapping-order");
    let locus = test_locus()?;
    let unrelated_incomplete = br#"{"reason":"compiler-message","message":{"level":"error","code":null,"spans":[{"file_name":"src/other.rs","is_primary":true}]}}"#;
    assert_eq!(
        observed_refusal(unrelated_incomplete, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::MissingSourceField(SourceField::LineStart),
        ))
    );
    let zero_position = br#"{"reason":"compiler-message","message":{"level":"error","code":null,"spans":[{"file_name":"src/main.rs","line_start":0,"line_end":1,"column_start":1,"column_end":2,"is_primary":true}]}}"#;
    assert!(matches!(
        observed_refusal(zero_position, &root, &locus),
        Err(DiagnosticReadFailure::Observation(
            ObservationRefusal::Position(_)
        ))
    ));
    Ok(())
}
