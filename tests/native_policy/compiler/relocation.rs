use super::configure::{bounds, finish, host, locus, root, target, tool};
use super::real::package;
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation, RelativeSourcePath};
use macroonz::native_compiler::{
    self, CargoFixture, CargoTarget, CompilerError, CompilerRequest, DependencyLimits,
};

#[test]
fn relocated_cargo_manifest_compiles_at_its_new_nested_root_with_policy_preserved()
-> Result<(), String> {
    let original = root()?;
    let host = host(&original)?;
    let relocated = root()?;
    let nested = original.join("nested");
    let moved = relocated.join("nested");
    std::fs::create_dir(&nested).map_err(|error| error.to_string())?;
    std::fs::create_dir(&moved).map_err(|error| error.to_string())?;
    let (name, original_manifest) = package(&original, "[lib]\npath = \"fixture.rs\"\n")?;
    let manifest = nested.join("Cargo.toml");
    std::fs::copy(original_manifest, &manifest).map_err(|error| error.to_string())?;
    std::fs::copy(original.join("Cargo.lock"), nested.join("Cargo.lock"))
        .map_err(|error| error.to_string())?;
    for file in ["Cargo.toml", "Cargo.lock"] {
        std::fs::copy(nested.join(file), moved.join(file)).map_err(|error| error.to_string())?;
    }
    std::fs::write(nested.join("fixture.rs"), b"invalid original Rust")
        .map_err(|error| error.to_string())?;
    std::fs::write(
        moved.join("fixture.rs"),
        b"pub const ANSWER: u64 = 6 * 7;\n",
    )
    .map_err(|error| error.to_string())?;
    let dependency = target()?
        .join(&host.triple)
        .join("debug")
        .join(format!("lib{}.d", name.replace('-', "_")));
    let fixture = CargoFixture::informed(
        manifest,
        target()?,
        name,
        CargoTarget::Library,
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &original, bounds()?)?,
        fixture,
        RelativeSourcePath::informed("nested/fixture.rs").map_err(|error| format!("{error:?}"))?,
    )
    .and_then(CompilerRequest::instrumented)
    .and_then(|request| {
        request.with_dependencies(
            dependency.clone(),
            DependencyLimits {
                bytes: 65_536,
                files: 16,
            },
        )
    })
    .map_err(|error| error.to_string())?;
    let selected = request
        .clone()
        .relocated(relocated.clone())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        selected.manifest(),
        Some(moved.join("Cargo.toml").as_path())
    );
    assert_eq!(selected.process().directory(), relocated);
    assert_eq!(
        selected.process().executable(),
        request.process().executable()
    );
    assert_eq!(
        selected.process().environment(),
        request.process().environment()
    );
    assert_eq!(
        selected.process().limits().execution(),
        request.process().limits().execution()
    );
    assert_eq!(
        selected.process().limits().cleanup(),
        request.process().limits().cleanup()
    );
    assert_eq!(
        selected.process().limits().stdout(),
        request.process().limits().stdout()
    );
    assert_eq!(
        selected.process().limits().stderr(),
        request.process().limits().stderr()
    );
    assert_eq!(selected.output_root(), request.output_root());
    assert_eq!(selected.locus(), request.locus());
    assert_eq!(selected.dependency_file(), Some(dependency.as_path()));
    let output = finish(native_compiler::compile(&selected).map_err(|error| error.to_string())?)?;
    assert_eq!(
        output.compared(&DeclaredCompilation::compiles()),
        Ok(CompilationVerdict::Conforms)
    );
    assert_eq!(output.cargo_fresh(), Some(false));
    let sources = output
        .dependencies()
        .ok_or("dependencies absent")?
        .map_err(ToString::to_string)?;
    assert!(sources.files().contains(&moved.join("fixture.rs")));
    assert!(!sources.files().contains(&nested.join("fixture.rs")));
    Ok(())
}

#[test]
fn relocating_a_cargo_manifest_outside_its_declared_root_refuses() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let outside = root()?;
    let fixture = CargoFixture::informed(
        outside.join("Cargo.toml"),
        target()?,
        "outside".to_owned(),
        CargoTarget::Library,
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, &source, bounds()?)?,
        fixture,
        locus()?,
    )
    .map_err(|error| error.to_string())?;
    assert!(matches!(
        request.relocated(outside),
        Err(CompilerError::Configuration(_))
    ));
    Ok(())
}
