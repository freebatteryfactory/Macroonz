use super::configure::{bounds, finish, host, locus, root, spelling, standin, target, tool};
use super::real::package;
use macroonz::native_compiler::{
    self, CargoFixture, CargoTarget, CompilerRequest, DependencyError, DependencyLimits,
};

const LIMITS: DependencyLimits = DependencyLimits {
    bytes: 65536,
    files: 16,
};

#[test]
fn rustc_dependencies_report_included_sources_and_keep_execution_time_bytes() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    std::fs::write(
        root.join("fixture.rs"),
        "include!(\"owned file.rs\");\nfn main() { let _value = OWNED; }\n",
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(root.join("owned file.rs"), "const OWNED: u8 = 42;\n")
        .map_err(|error| error.to_string())?;
    std::fs::write(root.join("unused.rs"), "this is deliberately invalid Rust")
        .map_err(|error| error.to_string())?;
    let artifact = root.join(format!("dependency result{}", std::env::consts::EXE_SUFFIX));
    let dependency = root.join("result dependencies.d");
    let request = CompilerRequest::rustc(
        &tool(&host, &host.rustc, &root, bounds()?)?,
        locus()?,
        artifact.clone(),
        &host.triple,
    )
    .and_then(|request| request.with_dependencies(dependency.clone(), LIMITS))
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let info = output
        .dependencies()
        .ok_or("dependency observation absent")?
        .map_err(|error| format!("{error}: {}", root.display()))?;
    assert_eq!(info.artifact(), artifact);
    assert_eq!(
        info.files(),
        [root.join("fixture.rs"), root.join("owned file.rs")]
    );
    assert!(!info.files().contains(&root.join("unused.rs")));
    let original = info.bytes().to_vec();
    let before = macroonz::presentation::native_compilation(&output);
    std::fs::write(dependency, b"changed after execution\n").map_err(|error| error.to_string())?;
    assert_eq!(info.bytes(), original);
    let after = macroonz::presentation::native_compilation(&output);
    assert_eq!(
        before, after,
        "presentation must not read changed disk bytes"
    );
    let shown = super::presentation::parsed(&after)?;
    assert_eq!(
        crate::presentation_formats::field(&shown, "/record/dependencies/kind")?,
        "observed"
    );
    let files = crate::presentation_formats::field(&shown, "/record/dependencies/value/files")?
        .as_array()
        .ok_or("missing dependency files")?;
    assert_eq!(files.len(), 2);
    assert_eq!(
        files
            .iter()
            .filter_map(|file| file.get("shown").and_then(serde_json::Value::as_str))
            .collect::<Vec<_>>(),
        vec![
            spelling(&root.join("fixture.rs"))?,
            spelling(&root.join("owned file.rs"))?
        ]
    );
    Ok(())
}

#[test]
fn cargo_dependencies_join_the_selected_library_artifact_without_inventing_source_membership()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let (name, manifest) = package(&root, "[lib]\npath = \"fixture.rs\"\n")?;
    std::fs::write(root.join("fixture.rs"), "include!(\"owned file.rs\");\n")
        .map_err(|error| error.to_string())?;
    std::fs::write(root.join("owned file.rs"), "pub const OWNED: u8 = 42;\n")
        .map_err(|error| error.to_string())?;
    let dependency = target()?
        .join(&host.triple)
        .join("debug")
        .join(format!("lib{}.d", name.replace('-', "_")));
    let selected = CargoFixture::informed(
        manifest,
        target()?,
        name,
        CargoTarget::Library,
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &root, bounds()?)?,
        selected,
        locus()?,
    )
    .and_then(|request| request.with_dependencies(dependency, LIMITS))
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let info = output
        .dependencies()
        .ok_or("dependency observation absent")?
        .map_err(|error| format!("{error}: {}", root.display()))?;
    assert!(info.files().contains(&root.join("fixture.rs")));
    assert!(info.files().contains(&root.join("owned file.rs")));
    assert_eq!(output.cargo_fresh(), Some(false));
    assert_eq!(output.executable(), None);
    Ok(())
}

#[test]
fn dependency_capture_refuses_foreign_ambiguous_incomplete_and_oversized_rules()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let artifact = root.join("reported-artifact");
    let record = serde_json::json!({"$message_type":"artifact", "emit":"link", "artifact": spelling(&artifact)?}).to_string() + "\n";
    let executable = standin(
        &root,
        &host,
        "dependency-emitter",
        &format!(
            "use std::io::Write; fn main() -> std::io::Result<()> {{ std::io::stderr().write_all({record:?}.as_bytes()) }}"
        ),
    )?;
    let path = root.join("declared.d");
    let selected = tool(&host, &executable, &root, bounds()?)?;
    let raw_artifact = spelling(&artifact)?.replace(' ', "\\ ");
    for (payload, limits, expected) in [
        (
            "foreign: source.rs\n".to_owned(),
            LIMITS,
            DependencyError::Artifact,
        ),
        (
            format!("{raw_artifact}: first.rs\n{raw_artifact}: second.rs\n"),
            LIMITS,
            DependencyError::Artifact,
        ),
        (
            format!("{raw_artifact}: first.rs"),
            LIMITS,
            DependencyError::Representation("incomplete dependency text".to_owned()),
        ),
        (
            format!("{raw_artifact}: first.rs first.rs\n"),
            LIMITS,
            DependencyError::Representation("duplicate source path".to_owned()),
        ),
        (
            format!("{raw_artifact}: first.rs second.rs\n"),
            DependencyLimits {
                bytes: 65536,
                files: 1,
            },
            DependencyError::FileBound,
        ),
        (
            format!("{raw_artifact}: first.rs\n"),
            DependencyLimits {
                bytes: 1,
                files: 16,
            },
            DependencyError::ByteBound,
        ),
    ] {
        std::fs::write(&path, payload).map_err(|error| error.to_string())?;
        let request = CompilerRequest::rustc(&selected, locus()?, artifact.clone(), &host.triple)
            .and_then(|request| request.with_dependencies(path.clone(), limits))
            .map_err(|error| error.to_string())?;
        let output =
            finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
        assert_eq!(
            output
                .dependencies()
                .ok_or("dependency observation absent")?
                .err(),
            Some(&expected)
        );
        assert!(
            output
                .observed()
                .is_ok_and(|observed| observed.refusal().is_none())
        );
        let shown =
            super::presentation::parsed(&macroonz::presentation::native_compilation(&output))?;
        assert_eq!(
            crate::presentation_formats::field(&shown, "/record/observation/value/kind")?,
            "compiled"
        );
        assert_eq!(
            crate::presentation_formats::field(&shown, "/record/dependencies/kind")?,
            "refused"
        );
    }
    Ok(())
}
