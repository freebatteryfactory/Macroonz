//! Independent presentation observations over the existing real compiler controls.

use super::configure::{anchor, bounds, host, root, standin, tool};
use crate::presentation_formats::field;
pub(super) use crate::presentation_formats::parsed;
use macroonz::harness::oracle::{
    self, CompiledObservation, CompiledVerdict, DeclaredBehavior, DeclaredCompilation,
    DeclaredReadBack, DeclaredReadBackRoster, ObservedMember, ObservedValue,
};
use macroonz::native_compiler::CompilerOutput;
use macroonz::presentation;
use serde_json::{Value, json};

pub(super) fn accepted(output: &CompilerOutput, fresh: Option<bool>) -> Result<(), String> {
    let value = parsed(&presentation::native_compilation(output))?;
    let observed = output.observed().map_err(|error| format!("{error:?}"))?;
    let pure = parsed(&presentation::compilation_observation(observed))?;
    assert_eq!(
        field(&pure, "/record")?,
        field(&value, "/record/observation/value")?
    );
    assert_eq!(field(&value, "/record/observation/kind")?, "observed");
    assert_eq!(field(&value, "/record/observation/value/kind")?, "compiled");
    assert_eq!(field(&value, "/record/process/state")?, "finished");
    assert_eq!(field(&value, "/record/process/status/code")?, &json!(0i32));
    assert_eq!(field(&value, "/record/cargo_fresh")?, &json!(fresh));
    assert_eq!(field(&value, "/record/request/locus")?, "fixture.rs");
    assert_eq!(
        field(&value, "/record/request/process/limits/stdout")?,
        &json!(1_048_576usize)
    );
    let arguments = field(&value, "/record/request/process/arguments")?;
    assert_eq!(arguments, &json!(output.request().process().arguments()));
    let compared = output
        .compared(&DeclaredCompilation::compiles())
        .map_err(|error| format!("{error:?}"))?;
    let compared = parsed(&presentation::compilation_comparison(&compared))?;
    assert_eq!(field(&compared, "/record/kind")?, "conforms");
    Ok(())
}

pub(super) fn refused(output: &CompilerOutput) -> Result<(), String> {
    let value = parsed(&presentation::native_compilation(output))?;
    assert_eq!(
        field(&value, "/record/observation/value/kind")?,
        "refused-by-compiler"
    );
    assert_eq!(
        field(&value, "/record/observation/value/value/code")?,
        "E0308"
    );
    assert_eq!(
        field(&value, "/record/observation/value/value/primary")?,
        &json!({"source":"fixture.rs", "start":{"line":2u64,"column":21u64}, "end":{"line":2u64,"column":28u64}})
    );
    assert_eq!(field(&value, "/record/executable")?, &Value::Null);
    assert_eq!(field(&value, "/record/cargo_fresh")?, &Value::Null);
    for (code, column, kind) in [
        ("E0308", 28, "conforms"),
        ("E0277", 28, "error-code"),
        ("E0308", 29, "primary-span"),
    ] {
        let expected = DeclaredCompilation::refuses(anchor(code, column)?);
        let verdict = output
            .compared(&expected)
            .map_err(|error| format!("{error:?}"))?;
        let shown = parsed(&presentation::compilation_comparison(&verdict))?;
        let archived = oracle::archive::retain_compilation(
            &verdict,
            macroonz::harness::report::archive::ArchiveLimits::declared(4096, 1024),
        )
        .map_err(|error| format!("{error:?}"))?;
        let history = parsed(&presentation::archived_oracle(&archived))?;
        assert_eq!(field(&history, "/standing")?, "historical-unauthenticated");
        assert_eq!(
            field(&history, "/record/verdict")?,
            field(&shown, "/record")?
        );
        if kind == "conforms" {
            assert_eq!(field(&shown, "/record/kind")?, "conforms");
        } else {
            assert_eq!(field(&shown, "/record/kind")?, "deviates");
            assert_eq!(field(&shown, "/record/value/kind")?, kind);
        }
        if kind == "error-code" {
            assert_eq!(
                field(&shown, "/record/value/value")?,
                &json!({"expected":"E0277", "observed":"E0308"})
            );
        }
        if kind == "primary-span" {
            assert_eq!(
                field(&shown, "/record/value/value/expected/end/column")?,
                &json!(29u64)
            );
            assert_eq!(
                field(&shown, "/record/value/value/observed/end/column")?,
                &json!(28u64)
            );
        }
    }
    Ok(())
}

pub(super) fn observation_failure(output: &CompilerOutput, cause: &str) -> Result<(), String> {
    let value = parsed(&presentation::native_compilation(output))?;
    assert_eq!(
        field(&value, "/record/observation/kind")?,
        "observation-failed"
    );
    assert_eq!(
        field(&value, "/record/observation/value/phase")?,
        "compiler-observation"
    );
    assert_eq!(
        field(&value, "/record/observation/value/cause/kind")?,
        cause
    );
    assert!(value.pointer("/record/observation/value/code").is_none());
    Ok(())
}

pub(super) fn read_back(
    read: &macroonz::native_process::ProcessOutput,
    verdict: &CompiledVerdict,
) -> Result<(), String> {
    let process = parsed(&presentation::native_process(read))?;
    assert_eq!(field(&process, "/record/stdout/bytes")?, "34320a");
    assert_eq!(field(&process, "/record/stdout/end/kind")?, "eof");
    let verdict = parsed(&presentation::compiled_comparison(verdict))?;
    assert_eq!(field(&verdict, "/record/kind")?, "conforms");
    Ok(())
}

#[test]
fn presentation_keeps_value_kinds_duplicate_members_and_independent_disagreement()
-> Result<(), String> {
    let values = vec![
        ObservedValue::Word("<word>|".to_owned()),
        ObservedValue::Text("<word>|".to_owned()),
        ObservedValue::Count(u64::MAX),
        ObservedValue::Truth(false),
        ObservedValue::Series(Vec::new()),
    ];
    let observation = CompiledObservation::ReadBack(vec![
        ObservedMember {
            name: "<member>|".to_owned(),
            value: ObservedValue::Series(values),
        },
        ObservedMember {
            name: "<member>|".to_owned(),
            value: ObservedValue::Text("again".to_owned()),
        },
    ]);
    let shown = presentation::compiled_observation(&observation);
    let value = parsed(&shown)?;
    assert_eq!(
        field(&value, "/record/value")?
            .as_array()
            .ok_or("missing member array")?
            .len(),
        2
    );
    assert_eq!(
        field(&value, "/record/value/0/value/value")?,
        &json!([
            {"kind":"word","value":"<word>|"}, {"kind":"text","value":"<word>|"}, {"kind":"count","value":u64::MAX}, {"kind":"truth","value":false}, {"kind":"series","value":[]}
        ])
    );
    assert!(!shown.html().contains("<member>"));
    let declared = [DeclaredReadBack {
        name: "<member>|",
        value: ObservedValue::Text("separate expectation".to_owned()),
    }];
    let declared = DeclaredBehavior::ReadsBack(
        DeclaredReadBackRoster::declared(&declared).map_err(|error| format!("{error:?}"))?,
    );
    let verdict = oracle::compiled::compared(&observation, &declared);
    let verdict = parsed(&presentation::compiled_comparison(&verdict))?;
    assert_eq!(field(&verdict, "/record/value/kind")?, "duplicate-member");
    assert_eq!(field(&verdict, "/record/value/value")?, "<member>|");
    Ok(())
}

#[test]
fn presentation_retains_native_prefix_beyond_foreign_text_bound_and_invalid_utf8()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = standin(
        &root,
        &host,
        "long-native-prefix",
        "use std::io::Write; fn main() -> std::io::Result<()> { let mut out = std::io::stdout(); out.write_all(&[b'a'; 8192])?; out.write_all(&[0xff]) }",
    )?;
    let request = tool(&host, &executable, &root, bounds()?)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    let output = crate::process::completed(
        macroonz::native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    let shown = parsed(&presentation::native_process(&output))?;
    assert_eq!(
        field(&shown, "/record/stdout/retained_bytes")?,
        &json!(8193usize)
    );
    assert_eq!(
        field(&shown, "/record/stdout/bytes")?.as_str(),
        Some(format!("{}ff", "61".repeat(8192)).as_str())
    );
    assert_eq!(field(&shown, "/record/stdout/fidelity")?, "lossy");
    assert_eq!(field(&shown, "/record/stdout/end/kind")?, "eof");
    Ok(())
}
