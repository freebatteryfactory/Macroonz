//! Compilation custody for the grammar campaign, including the instrumented compiler rather than only its calling executable.

use super::support::{FuzzRoadFailure, RunScratch, external, rustc_path, successful_output};
use macroonz_harness::descriptor::{DerivedRevision, RevisionBinding};
use macroonz_harness::fuzz::RUSTC_COVERAGE_TOOLCHAIN;
use macroonz_harness::identity::encode_bytes;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy)]
pub(super) enum Instrumentation {
    WrapperOnly,
    CompilerAndWrapper,
}

fn revision(repository: &Path) -> Result<RevisionBinding, FuzzRoadFailure> {
    successful_output(
        Command::new("git").current_dir(repository).args([
            "diff",
            "--exit-code",
            "HEAD",
            "--",
            "macros/compiler",
            "Cargo.toml",
            "Cargo.lock",
        ]),
        "clean compiler source before grammar campaign",
    )?;
    let compiler_tree = successful_output(
        Command::new("git")
            .current_dir(repository)
            .args(["rev-parse", "HEAD:macros/compiler/src"]),
        "compiler source tree",
    )?;
    let mut material = Vec::new();
    for source in [
        compiler_tree.as_slice(),
        include_bytes!("recipe_subject.rs").as_slice(),
        include_bytes!("recipe_observation.rs").as_slice(),
        include_bytes!("recipe_compilation.rs").as_slice(),
    ] {
        encode_bytes(source, &mut material);
    }
    let lock = std::fs::read(repository.join("Cargo.lock")).map_err(external)?;
    encode_bytes(&lock, &mut material);
    for manifest in ["Cargo.toml", "macros/compiler/Cargo.toml"] {
        encode_bytes(
            &std::fs::read(repository.join(manifest)).map_err(external)?,
            &mut material,
        );
    }
    writeln!(
        std::io::stdout().lock(),
        "Compiler source tree: {}",
        String::from_utf8_lossy(&compiler_tree).trim()
    )
    .map_err(external)?;
    Ok(RevisionBinding::derived(DerivedRevision::from_material(
        &material,
    )))
}

pub(super) fn compile(
    instrumentation: Instrumentation,
) -> Result<(PathBuf, PathBuf, RunScratch, RevisionBinding), FuzzRoadFailure> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = manifest.parent().ok_or(FuzzRoadFailure::Fixture)?;
    let revision = revision(repository)?;
    let directory = repository
        .join("target/qualification")
        .join(format!("recipe-grammar-{}", std::process::id()));
    if directory.exists() {
        return Err(FuzzRoadFailure::External(
            "grammar campaign scratch already exists".to_owned(),
        ));
    }
    let run = RunScratch::created(directory)?;
    let build = run.join("build");
    let mut cargo = Command::new("cargo");
    cargo
        .current_dir(repository)
        .arg(format!("+{RUSTC_COVERAGE_TOOLCHAIN}"))
        .args([
            "rustc",
            "-p",
            "macroonz-compiler",
            "--lib",
            "--features",
            "host",
            "--locked",
            "--offline",
        ])
        .env("CARGO_TARGET_DIR", &build)
        .env("CARGO_BUILD_JOBS", "1")
        .env("CARGO_INCREMENTAL", "0")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC_WRAPPER");
    if matches!(instrumentation, Instrumentation::CompilerAndWrapper) {
        cargo.args(["--", "-C", "instrument-coverage"]);
    }
    writeln!(
        std::io::stdout().lock(),
        "Instrumented compiler command: {cargo:?}"
    )
    .map_err(external)?;
    let compiled = cargo.status().map_err(external)?;
    if !compiled.success() {
        return Err(external(compiled));
    }
    let rustc = rustc_path()?;
    let subject = run.join(format!("recipe-subject{}", std::env::consts::EXE_SUFFIX));
    let library = build.join("debug/libmacroonz_compiler.rlib");
    let mut command = Command::new(&rustc);
    command
        .args([
            "--edition=2024",
            "-Dwarnings",
            "-C",
            "instrument-coverage",
            "-C",
            "opt-level=0",
        ])
        .arg(manifest.join("tests/fuzz_compose/recipe_subject.rs"))
        .arg("--extern")
        .arg(format!("macroonz_compiler={}", library.display()))
        .arg("-L")
        .arg(format!("dependency={}", build.join("debug/deps").display()))
        .arg("-o")
        .arg(&subject);
    writeln!(
        std::io::stdout().lock(),
        "Instrumented subject command: {command:?}"
    )
    .map_err(external)?;
    let linked = command.status().map_err(external)?;
    if !linked.success() {
        return Err(external(linked));
    }
    Ok((rustc, subject, run, revision))
}
