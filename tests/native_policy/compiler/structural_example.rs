//! Public structural generation and native compilation with unchanged independent checks.

use super::configure::{bounds, host, root, spelling, target, tool};
use super::types::Host;
use macroonz::native_process::{self, CaptureEnd, ProcessOutput, ProcessTool};
use serde_json::{Value, json};
use std::path::Path;

#[test]
fn public_structural_alternatives_compile_and_disagree_with_unchanged_checks() -> Result<(), String>
{
    let run = root()?;
    let host = host(&run)?;
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let build = crate::check::cargo(
        repository,
        repository,
        &run,
        "structural-examples",
        &[
            "build",
            "--example",
            "structural_mutation",
            "--example",
            "compiler_workflow",
            "--no-default-features",
            "--features",
            "native-tooling",
            "--locked",
            "--offline",
            "-j1",
        ],
    )?;
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let generator = caller(&host, &run, "structural_mutation")?;
    let compiler = caller(&host, &run, "compiler_workflow")?;
    for (family, alternatives) in [
        ("codec", 1usize),
        ("transition", 2usize),
        ("effect", 1usize),
    ] {
        let unchanged = generate(&generator, &run, family, "unchanged")?;
        let baseline = run.join(format!("{family}-unchanged"));
        let baseline_result = compile(&compiler, &host, &baseline, &unchanged)?;
        assert!(
            baseline_result.status().success(),
            "{}",
            String::from_utf8_lossy(baseline_result.stderr().bytes())
        );
        assert_eq!(
            baseline_result.stdout().bytes(),
            b"compiled count agrees with the independent expectation\n"
        );
        for index in 0usize..alternatives {
            let changed = generate(&generator, &run, family, &index.to_string())?;
            assert_ne!(changed, unchanged);
            let selected = run.join(format!("{family}-{index}"));
            let changed_result = compile(&compiler, &host, &selected, &changed)?;
            assert!(!changed_result.status().success());
            assert!(changed_result.stdout().bytes().is_empty());
            let diagnostic = String::from_utf8_lossy(changed_result.stderr().bytes());
            assert!(
                diagnostic.contains("read-back disagreed: Deviates(MemberValue"),
                "{diagnostic}"
            );
            assert!(diagnostic.contains("count"), "{diagnostic}");
            let artifact = selected.join(format!("subject{}", std::env::consts::EXE_SUFFIX));
            let request = tool(&host, &artifact, &selected, bounds()?)?
                .invocation(Vec::new())
                .map_err(debug)?;
            let observed =
                crate::process::completed(native_process::run(&request, None).map_err(debug)?)?;
            assert!(observed.status().success());
            assert_eq!(observed.stdout().bytes(), b"0\n");
        }
        let absent = invoke(
            &generator,
            &run,
            &json!({"family":family,"selection":alternatives.to_string()}),
        )?;
        assert!(!absent.status().success());
        assert!(absent.stdout().bytes().is_empty());
        assert!(String::from_utf8_lossy(absent.stderr().bytes()).contains("is absent"));
    }
    for configuration in [
        json!({"family":"invented","selection":"unchanged"}),
        json!({"family":"codec","selection":"-1"}),
        json!({"family":"codec"}),
    ] {
        let refused = invoke(&generator, &run, &configuration)?;
        assert!(!refused.status().success());
        assert!(refused.stdout().bytes().is_empty());
    }
    Ok(())
}

fn caller(host: &Host, run: &Path, example: &str) -> Result<ProcessTool, String> {
    let executable = target()?
        .join("debug/examples")
        .join(format!("{example}{}", std::env::consts::EXE_SUFFIX));
    tool(host, &executable, run, bounds()?)
}

fn generate(
    caller: &ProcessTool,
    run: &Path,
    family: &str,
    selection: &str,
) -> Result<Vec<u8>, String> {
    let output = invoke(caller, run, &json!({"family":family,"selection":selection}))?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert!(!output.stdout().bytes().is_empty());
    Ok(output.stdout().bytes().to_vec())
}

fn compile(
    caller: &ProcessTool,
    host: &Host,
    work: &Path,
    source: &[u8],
) -> Result<ProcessOutput, String> {
    std::fs::create_dir(work).map_err(debug)?;
    std::fs::write(work.join("subject.rs"), source).map_err(debug)?;
    invoke(
        caller,
        work,
        &json!({
            "rustc":spelling(&host.rustc)?, "directory":spelling(work)?, "source":"subject.rs",
            "artifact":spelling(&work.join(format!("subject{}", std::env::consts::EXE_SUFFIX)))?,
            "target":host.triple, "environment":host.environment, "expected_count":1u64,
        }),
    )
}

fn invoke(
    caller: &ProcessTool,
    run: &Path,
    configuration: &Value,
) -> Result<ProcessOutput, String> {
    let input = run.join("request.json");
    std::fs::write(&input, serde_json::to_vec(configuration).map_err(debug)?).map_err(debug)?;
    let request = caller.invocation(Vec::new()).map_err(debug)?;
    crate::process::completed(
        native_process::run(&request, Some(std::fs::File::open(input).map_err(debug)?))
            .map_err(debug)?,
    )
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
