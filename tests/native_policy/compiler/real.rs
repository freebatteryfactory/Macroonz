use super::configure::{anchor, bounds, finish, host, locus, root, spelling, target, tool};
use super::types::Host;
use macroonz::harness::oracle::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict,
    DeclaredBehavior, DeclaredCompilation, DeclaredReadBack, DeclaredReadBackRoster,
    ObservedMember, ObservedValue,
};
use macroonz::native_compiler::{self, CargoFixture, CargoTarget, CompilerRequest, CompilerRun};
use macroonz::native_process::{ProcessLimits, ProcessRequest};
use std::path::{Path, PathBuf};
use std::time::Duration;

const LAWFUL: &str = "use std::io::Write;\nfn main() -> std::io::Result<()> {\n    let count: u64 = 6 * 7;\n    writeln!(std::io::stdout(), \"{count}\")\n}\n";
const HOSTILE: &str = "fn main() {\n    let value: u8 = \"wrong\";\n    let _copy = value;\n}\n";

#[test]
fn rustc_compilation_read_back_and_exact_refusal_keep_independent_expectations()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    std::fs::write(root.join("fixture.rs"), LAWFUL).map_err(|error| error.to_string())?;
    let artifact = root.join(format!("lawful{}", std::env::consts::EXE_SUFFIX));
    let request = CompilerRequest::rustc(
        &tool(&host, &host.rustc, &root, bounds()?)?,
        locus()?,
        artifact.clone(),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        output.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    assert_eq!(output.executable(), Some(artifact.as_path()));
    assert_eq!(output.cargo_fresh(), None);
    super::presentation::accepted(&output, None)?;
    read_count(&output, &host, &root)?;

    std::fs::write(root.join("fixture.rs"), HOSTILE).map_err(|error| error.to_string())?;
    let refused = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let expected = anchor("E0308", 28)?;
    super::presentation::refused(&refused)?;
    assert_eq!(
        refused.observed().map(|value| value.refusal()),
        Ok(Some(&expected))
    );
    assert_eq!(
        refused.compared(&DeclaredCompilation::refuses(expected)),
        Ok(CompilationVerdict::Conforms)
    );
    assert!(matches!(
        refused.compared(&DeclaredCompilation::refuses(anchor("E0277", 28)?)),
        Ok(CompilationVerdict::Deviates(
            CompilationDisagreement::ErrorCode { .. }
        ))
    ));
    assert!(matches!(
        refused.compared(&DeclaredCompilation::refuses(anchor("E0308", 29)?)),
        Ok(CompilationVerdict::Deviates(
            CompilationDisagreement::PrimarySpan { .. }
        ))
    ));
    assert_eq!(refused.executable(), None);
    let reader = tool(&host, &artifact, &root, bounds()?)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    assert!(
        refused.read_back(&reader, None).is_err(),
        "old lawful executable must not confer read-back authority on a refused build"
    );
    Ok(())
}

#[test]
fn relocated_source_retains_primary_coordinates_and_pending_compilation_is_retryable()
-> Result<(), String> {
    let first = root()?;
    let host = host(&first)?;
    for root in [&first, &root()?] {
        std::fs::write(root.join("fixture.rs"), HOSTILE).map_err(|error| error.to_string())?;
        let bounds =
            ProcessLimits::informed(Duration::from_secs(30), Duration::ZERO, 65_536, 65_536)
                .map_err(|error| error.to_string())?;
        let request = CompilerRequest::rustc(
            &tool(&host, &host.rustc, root, bounds)?,
            locus()?,
            root.join("refused-output"),
            &host.triple,
        )
        .map_err(|error| error.to_string())?;
        let CompilerRun::Pending(pending) =
            native_compiler::compile(&request).map_err(|error| error.to_string())?
        else {
            return Err("zero cleanup did not retain compiler custody".to_owned());
        };
        let shown =
            super::presentation::parsed(&macroonz::presentation::pending_compilation(&pending))?;
        assert_eq!(
            crate::presentation_formats::field(&shown, "/record/state")?,
            "pending-cleanup"
        );
        assert!(shown.pointer("/record/observation").is_none());
        let output = finish(pending.finish(Duration::from_secs(3)))?;
        let expected = anchor("E0308", 28)?;
        assert_eq!(
            output.observed().map(|value| value.refusal()),
            Ok(Some(&expected))
        );
    }
    Ok(())
}

#[test]
fn cargo_reports_selected_artifact_freshness_and_actual_refusals() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let (package, manifest) = package(
        &root,
        "[[bin]]\nname = \"read-count\"\npath = \"fixture.rs\"\n",
    )?;
    std::fs::write(root.join("fixture.rs"), LAWFUL).map_err(|error| error.to_string())?;
    let fixture = CargoFixture::informed(
        manifest,
        target()?,
        package,
        CargoTarget::Binary("read-count".to_owned()),
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &root, bounds()?)?,
        fixture,
        locus()?,
    )
    .map_err(|error| error.to_string())?;
    let first = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        first.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    assert_eq!(first.cargo_fresh(), Some(false));
    super::presentation::accepted(&first, Some(false))?;
    read_count(&first, &host, &root)?;
    let cached = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        cached.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    assert_eq!(cached.cargo_fresh(), Some(true));
    super::presentation::accepted(&cached, Some(true))?;
    std::fs::write(root.join("fixture.rs"), HOSTILE).map_err(|error| error.to_string())?;
    let refused = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let expected = anchor("E0308", 28)?;
    assert_eq!(
        refused.observed().map(|value| value.refusal()),
        Ok(Some(&expected))
    );
    assert_eq!(refused.cargo_fresh(), None);
    super::presentation::refused(&refused)?;
    assert_eq!(refused.executable(), None);
    Ok(())
}

#[test]
fn cargo_library_compiles_without_inventing_an_executable() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let (package, manifest) = package(&root, "[lib]\npath = \"fixture.rs\"\n")?;
    std::fs::write(root.join("fixture.rs"), "pub const COUNT: u64 = 42;\n")
        .map_err(|error| error.to_string())?;
    let fixture = CargoFixture::informed(
        manifest,
        target()?,
        package,
        CargoTarget::Library,
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &root, bounds()?)?,
        fixture,
        locus()?,
    )
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        output.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    assert_eq!(output.cargo_fresh(), Some(false));
    assert_eq!(output.executable(), None);
    Ok(())
}

pub(crate) fn package(root: &Path, selection: &str) -> Result<(String, PathBuf), String> {
    let suffix = root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("missing fixture suffix")?;
    let name = format!("compiler-{suffix}");
    let manifest = root.join("Cargo.toml");
    std::fs::write(&manifest, format!("[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nbuild = false\n{selection}[workspace]\n")).map_err(|error| error.to_string())?;
    std::fs::write(
        root.join("Cargo.lock"),
        format!("version = 4\n[[package]]\nname = \"{name}\"\nversion = \"0.0.0\"\n"),
    )
    .map_err(|error| error.to_string())?;
    Ok((name, manifest))
}

pub(crate) fn read_count(
    output: &native_compiler::CompilerOutput,
    host: &Host,
    root: &Path,
) -> Result<(), String> {
    let executable = output
        .executable()
        .ok_or_else(|| format!("no compiled executable: {output:?}"))?;
    let request = ProcessRequest::informed(
        executable.to_path_buf(),
        root.to_path_buf(),
        Vec::new(),
        host.environment.clone(),
        bounds()?,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let read = super::super::process::completed(
        output
            .read_back(&request, None)
            .map_err(|error| error.to_string())?,
    )?;
    let expected = [DeclaredReadBack {
        name: "count",
        value: ObservedValue::Count(42),
    }];
    let declared = DeclaredBehavior::ReadsBack(
        DeclaredReadBackRoster::declared(&expected).map_err(|error| format!("{error:?}"))?,
    );
    let verdict = native_compiler::compared_read_back(&read, decode_count, &declared)
        .map_err(|error| format!("{error:?}"))?;
    assert_eq!(verdict, CompiledVerdict::Conforms);
    super::presentation::read_back(&read, &verdict)?;
    let wrong = [DeclaredReadBack {
        name: "count",
        value: ObservedValue::Count(43),
    }];
    let wrong = DeclaredBehavior::ReadsBack(
        DeclaredReadBackRoster::declared(&wrong).map_err(|error| format!("{error:?}"))?,
    );
    assert_eq!(
        native_compiler::compared_read_back(&read, decode_count, &wrong),
        Ok(CompiledVerdict::Deviates(
            CompiledDisagreement::MemberValue {
                member: "count".to_owned()
            }
        ))
    );
    let duplicate = |bytes: &[u8]| {
        let mut members = decode_count(bytes)?;
        members.extend(decode_count(bytes)?);
        Ok(members)
    };
    assert_eq!(
        native_compiler::compared_read_back(&read, duplicate, &declared),
        Ok(CompiledVerdict::Deviates(
            CompiledDisagreement::DuplicateMember {
                member: "count".to_owned()
            }
        ))
    );
    let incompatible_reader = |bytes: &[u8]| {
        std::str::from_utf8(bytes)
            .map_err(|error| error.to_string())?
            .trim()
            .parse::<bool>()
            .map_err(|error| error.to_string())?;
        Ok(Vec::new())
    };
    assert!(matches!(
        native_compiler::compared_read_back(&read, incompatible_reader, &declared),
        Err(native_compiler::ReadBackError::Decode(_))
    ));
    let other = tool(host, &root.join("different-executable"), root, bounds()?)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    assert!(
        output.read_back(&other, None).is_err(),
        "{}",
        spelling(other.executable())?
    );
    Ok(())
}

fn decode_count(bytes: &[u8]) -> Result<Vec<ObservedMember>, String> {
    let value = std::str::from_utf8(bytes)
        .map_err(|error| error.to_string())?
        .trim()
        .parse::<u64>()
        .map_err(|error| error.to_string())?;
    Ok(vec![ObservedMember {
        name: "count".to_owned(),
        value: ObservedValue::Count(value),
    }])
}
