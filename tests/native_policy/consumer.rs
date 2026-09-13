//! Independent feature-isolated Cargo consumers of the root native entrances.

use super::check::cargo;
use super::types::StorageDependencies;
use std::path::Path;

#[path = "consumer_configuration.rs"]
mod configuration;

pub(super) fn manifest(root: &Path, features: &str) -> Result<String, String> {
    let path = root.to_str().ok_or("the dependency path is not UTF-8")?;
    if path.chars().any(char::is_control) {
        return Err("the dependency path contains a control character".to_owned());
    }
    let path = path.replace('\\', "/").replace('"', "\\\"");
    Ok(format!(
        "[package]\nname = \"native-clock-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\nbuild = false\n[[bin]]\nname = \"native-clock-consumer\"\npath = \"main.rs\"\n[lints.rust]\nwarnings = \"deny\"\nunsafe_code = \"forbid\"\n[dependencies]\nmacroonz = {{ path = \"{path}\", default-features = false, features = [{features}] }}\n[workspace]\n"
    ))
}

pub(super) fn observe(root: &Path, scratch: &Path, strict: &Path) -> Result<(), String> {
    let subject = scratch.join("consumer");
    std::fs::create_dir(&subject).map_err(|error| error.to_string())?;
    let root_lock = std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?;
    for (name, features) in [
        ("consumer-diet", ""),
        ("consumer-harness", "\"harness\""),
        ("consumer-full", "\"full\""),
        ("consumer-native", "\"native-tooling\""),
    ] {
        std::fs::write(subject.join("Cargo.toml"), manifest(root, features)?)
            .map_err(|error| error.to_string())?;
        std::fs::write(subject.join("Cargo.lock"), &root_lock)
            .map_err(|error| error.to_string())?;
        write_subject(
            &subject,
            scratch,
            name,
            "fn main() { let _clock = macroonz::native_clock::source(); let _name = macroonz::native_storage::StorageName::informed(\"run\"); }\n",
        )?;
        let lock = cargo(
            &subject,
            strict,
            scratch,
            &format!("{name}-lock"),
            &["update", "--workspace", "--offline"],
        )?;
        if !lock.status.success() {
            return Err(format!(
                "{name} lock: {}",
                String::from_utf8_lossy(&lock.stderr)
            ));
        }
        let storage = if name == "consumer-native" {
            StorageDependencies::Native
        } else {
            StorageDependencies::Absent
        };
        dependency_graph(&subject, scratch, strict, name, storage)?;
        super::dependency::observe(&subject, scratch, strict, name, storage)?;
        let output = cargo(
            &subject,
            strict,
            scratch,
            name,
            &["check", "-j1", "--locked", "--offline"],
        )?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if name == "consumer-native" {
            if !output.status.success() {
                return Err(format!("{name}: {stderr}"));
            }
        } else if output.status.code() != Some(101_i32)
            || !stderr.contains("E0433")
            || !stderr.contains("native_clock")
            || !stderr.contains("native_storage")
        {
            return Err(format!(
                "{name}: expected both native entrance feature refusals\n{stderr}"
            ));
        }
    }
    observe_native(&subject, scratch, strict)?;
    if std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())? != root_lock {
        return Err("the root lock changed during consumer qualification".to_owned());
    }
    Ok(())
}

fn dependency_graph(
    subject: &Path,
    scratch: &Path,
    strict: &Path,
    name: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    for target in ["all", "wasm32-unknown-unknown"] {
        let output = cargo(
            subject,
            strict,
            scratch,
            &format!("{name}-graph-{target}"),
            &[
                "tree",
                "--locked",
                "--offline",
                "--edges",
                "normal",
                "--prefix",
                "none",
                "--format",
                "{p}",
                "--target",
                target,
            ],
        )?;
        if !output.status.success() {
            return Err(format!(
                "{name} graph: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let graph = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
        for package in [
            "cap-std",
            "cap-primitives",
            "io-extras",
            "io-lifetimes",
            "fs-set-times",
            "winx",
        ] {
            let prefix = format!("{package} v");
            let present = graph.lines().any(|line| line.starts_with(&prefix));
            if present != (storage == StorageDependencies::Native && target == "all") {
                return Err(format!(
                    "{name}/{target}: unexpected {package} presence={present}"
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn qualify_graphs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = super::check::scratch()?;
    let subject = scratch.join("graph-consumer");
    std::fs::create_dir(&subject).map_err(|error| error.to_string())?;
    std::fs::write(subject.join("main.rs"), "fn main() {}\n").map_err(|error| error.to_string())?;
    let root_lock = std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?;
    for (name, features, storage) in [
        ("diet", "", StorageDependencies::Absent),
        ("harness", "\"harness\"", StorageDependencies::Absent),
        ("full", "\"full\"", StorageDependencies::Absent),
        ("native", "\"native-tooling\"", StorageDependencies::Native),
        (
            "full-native",
            "\"full\", \"native-tooling\"",
            StorageDependencies::Native,
        ),
    ] {
        std::fs::write(subject.join("Cargo.toml"), manifest(root, features)?)
            .map_err(|error| error.to_string())?;
        std::fs::write(subject.join("Cargo.lock"), &root_lock)
            .map_err(|error| error.to_string())?;
        let output = cargo(
            &subject,
            root,
            &scratch,
            name,
            &["update", "--workspace", "--offline"],
        )?;
        if !output.status.success() {
            return Err(format!(
                "{name} lock: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        dependency_graph(&subject, &scratch, root, name, storage)?;
        super::dependency::observe(&subject, &scratch, root, name, storage)?;
        workflow_surface(&subject, &scratch, root, name, storage)?;
        configuration::observe(&subject, &scratch, root, name, storage)?;
        presentation_surface(&subject, &scratch, root, name)?;
        publication_invariants(&subject, &scratch, root, name, storage)?;
    }
    super::dependency::refuse_injected(root, &subject, &scratch)?;
    if std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())? != root_lock {
        return Err("the root lock changed during graph qualification".to_owned());
    }
    Ok(())
}

fn presentation_surface(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
) -> Result<(), String> {
    for (name, source, expected) in [
        (
            "readers",
            "fn main() { let _run = macroonz::presentation::run; let _input = macroonz::presentation::input_run; let _historical = macroonz::presentation::archived_run; let _diagnostic = macroonz::presentation::compiler_diagnostic; }",
            (posture == "diet").then_some("E0433"),
        ),
        (
            "private",
            "fn main() { let _forge = |value: &mut macroonz::presentation::Presentation| { let _record = &mut value.value; }; }",
            Some(if posture == "diet" { "E0433" } else { "E0616" }),
        ),
        (
            "native-readers",
            "fn main() { let _compiler = macroonz::presentation::native_compilation; let _storage = macroonz::presentation::storage_error; let _publication = macroonz::presentation::publication_format_error; }",
            match posture {
                "native" | "full-native" => None,
                "diet" => Some("E0433"),
                _ => Some("E0425"),
            },
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-presentation-{name}"),
            source,
            expected,
        )?;
        let name = format!("{posture}-presentation-{name}-wasm");
        let output = cargo(
            subject,
            profile,
            scratch,
            &name,
            &[
                "check",
                "-j1",
                "--locked",
                "--offline",
                "--target",
                "wasm32-unknown-unknown",
            ],
        )?;
        surface_result(&name, &output, expected)?;
    }
    Ok(())
}

fn workflow_surface(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let run_refusal = if posture == "diet" {
        Some("E0433")
    } else {
        None
    };
    let retained_refusal = if storage == StorageDependencies::Native {
        None
    } else if posture == "diet" {
        Some("E0433")
    } else {
        Some("E0425")
    };
    let native_refusal = if storage == StorageDependencies::Native {
        None
    } else {
        Some("E0433")
    };
    let private_field = Some(if storage == StorageDependencies::Native {
        "E0616"
    } else {
        "E0433"
    });
    for (name, source, expected) in [
        (
            "execute",
            "fn main() { let _run = macroonz::workflow::run::<Vec<u8>>; }\n",
            run_refusal,
        ),
        (
            "retained",
            "fn main() { let _record = Option::<macroonz::workflow::StoredRun>::None; }\n",
            retained_refusal,
        ),
        (
            "benchmark",
            "fn main() { let _load = macroonz::workflow::benchmark::load; }\n",
            native_refusal,
        ),
        (
            "process",
            "fn main() { let _run = macroonz::native_process::run; }\n",
            native_refusal,
        ),
        (
            "compiler",
            "fn main() { let _compile = macroonz::native_compiler::compile; }\n",
            native_refusal,
        ),
        (
            "coverage",
            "fn main() { let _preflight = macroonz::native_coverage::preflight; }\n",
            native_refusal,
        ),
        (
            "process-tool-invariant",
            "fn main() { let _forge = |tool: &mut macroonz::native_process::ProcessTool| { tool.directory = \"relative\".into(); }; }\n",
            private_field,
        ),
        (
            "compiler-invariant",
            "fn main() { let _forge = |request: &macroonz::native_compiler::CompilerRequest| { let _locus = &request.locus; }; }\n",
            private_field,
        ),
        (
            "compiler-observation-invariant",
            "fn main() { let _forge = |output: &macroonz::native_compiler::CompilerOutput| { let _raw = &output.observation; }; }\n",
            private_field,
        ),
        (
            "process-invariant",
            "fn main() { let _limits = macroonz::native_process::ProcessLimits { execution: std::time::Duration::ZERO, cleanup: std::time::Duration::ZERO, stdout: 0, stderr: 0 }; }\n",
            Some(if storage == StorageDependencies::Native {
                "E0451"
            } else {
                "E0433"
            }),
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-workflow-{name}"),
            source,
            expected,
        )?;
    }
    coverage_invariants(subject, scratch, profile, posture, private_field)?;
    mutation_invariants(
        subject,
        scratch,
        profile,
        posture,
        native_refusal,
        private_field,
    )
}

fn publication_invariants(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let native_refusal = (storage != StorageDependencies::Native).then_some("E0433");
    let private_field = Some(if storage == StorageDependencies::Native {
        "E0616"
    } else {
        "E0433"
    });
    for (name, source, expected) in [
        (
            "publication",
            "fn main() { let _qualify = macroonz::native_publication::Formatter::qualified; let _path = macroonz::native_publication::PublicationPath::informed(\"generated.rs\"); }\n",
            native_refusal,
        ),
        (
            "publication-path",
            "fn main() { let _forge = |path: &macroonz::native_publication::PublicationPath| { let _raw = &path.0; }; }\n",
            private_field,
        ),
        (
            "publication-inventory",
            "fn _forge<K: macroonz::compiler::Kind>(value: &mut macroonz::native_publication::Publication<K>) { value.destinations.clear(); } fn main() {}\n",
            private_field,
        ),
        (
            "formatter-profile",
            "fn main() { let _forge = |formatter: &macroonz::native_publication::Formatter| { let _raw = &formatter.process; }; }\n",
            private_field,
        ),
        (
            "formatter-source",
            "fn main() { let _forge = |output: &macroonz::native_publication::FormatOutput| { let _raw = &output.context; }; }\n",
            private_field,
        ),
        (
            "formatter-observation",
            "fn main() { let _forge = |output: &mut macroonz::native_publication::FormatOutput| { output.observation = Ok(()); }; }\n",
            private_field,
        ),
        (
            "prepared-inventory",
            "fn _forge<K: macroonz::compiler::Kind>(value: &mut macroonz::native_publication::PreparedPublication<K>) { value.files.clear(); } fn main() {}\n",
            private_field,
        ),
        (
            "staging-plan",
            "fn _forge<K: macroonz::compiler::Kind>(value: &mut macroonz::native_publication::StagingPlan<K>) { value.authored.clear(); } fn main() {}\n",
            private_field,
        ),
        (
            "staged-source",
            "fn _forge<K: macroonz::compiler::Kind>(value: &macroonz::native_publication::StagedPublication<K>) { let _plan = &value.plan; } fn main() {}\n",
            private_field,
        ),
        (
            "compiled-publication",
            "fn _forge<K: macroonz::compiler::Kind>(value: &macroonz::native_publication::CompiledPublication<K>) { let _compiler = &value.compiler; } fn main() {}\n",
            private_field,
        ),
        (
            "compiler-dependencies",
            "fn main() { let _forge = |value: &mut macroonz::native_compiler::DependencyInfo| { value.files.clear(); }; }\n",
            private_field,
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-{name}"),
            source,
            expected,
        )?;
    }
    destination_invariants(subject, scratch, profile, posture, storage)?;
    command_invariants(subject, scratch, profile, posture, storage)
}

fn command_invariants(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let native_refusal = (storage != StorageDependencies::Native).then_some("E0433");
    let private_field = Some(if storage == StorageDependencies::Native {
        "E0616"
    } else {
        "E0433"
    });
    for (name, source, expected) in [
        (
            "publication-command",
            "fn _call<K: macroonz::compiler::Kind>(publication: macroonz::native_publication::Publication<K>) { let _result = macroonz::native_publication::bake(macroonz::native_publication::BakeCommand::Prepare, || Ok::<_, ()>(publication)); } fn main() { let _relocate = macroonz::native_compiler::CompilerRequest::relocated; }\n",
            native_refusal,
        ),
        (
            "publication-cache",
            "fn _cache<K: macroonz::compiler::Kind>(plan: macroonz::native_publication::StagingPlan<K>) { drop(plan.cached(std::path::Path::new(\"explicit\"))); } fn main() {}\n",
            native_refusal,
        ),
        (
            "publication-command-custody",
            "fn _forge<K: macroonz::compiler::Kind>(error: &mut macroonz::native_publication::BakeError<K, ()>) { error.lease = None; } fn main() {}\n",
            private_field,
        ),
        (
            "publication-command-cause",
            "fn _forge<K: macroonz::compiler::Kind>(error: &mut macroonz::native_publication::BakeError<K, ()>) { let _cause = &mut error.cause; } fn main() {}\n",
            private_field,
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-{name}"),
            source,
            expected,
        )?;
    }
    Ok(())
}

fn destination_invariants(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let native_refusal = (storage != StorageDependencies::Native).then_some("E0433");
    let private_field = Some(if storage == StorageDependencies::Native {
        "E0616"
    } else {
        "E0433"
    });
    for (name, source, expected) in [
        (
            "destination",
            "fn main() { let _open = macroonz::native_publication::PublicationDestination::open; let _recover = macroonz::native_publication::PublicationDestination::recover; }\n",
            native_refusal,
        ),
        (
            "destination-limits",
            "fn main() { let _forge = |value: &mut macroonz::native_publication::PublicationDestination| { value.limits.files = usize::MAX; }; }\n",
            private_field,
        ),
        (
            "destination-check",
            "fn main() { let _forge = |value: &mut macroonz::native_publication::DestinationCheck| { value.state = macroonz::native_publication::DestinationState::Installed; }; }\n",
            private_field,
        ),
        (
            "installation-intent",
            "fn main() { let _forge = |value: &macroonz::native_publication::PublicationInstallation<'_>| { let _intent = &value.intent; }; }\n",
            private_field,
        ),
        (
            "installation-progress",
            "fn main() { let _forge = |value: &mut macroonz::native_publication::PublicationInstallation<'_>| { value.cursor = usize::MAX; }; }\n",
            private_field,
        ),
        (
            "installation-consumed",
            "fn _reuse(value: macroonz::native_publication::PublicationInstallation<'_>) { drop(value.commit()); drop(value.commit()); } fn main() {}\n",
            Some(if storage == StorageDependencies::Native {
                "E0382"
            } else {
                "E0433"
            }),
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-{name}"),
            source,
            expected,
        )?;
    }
    Ok(())
}

fn mutation_invariants(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    native_refusal: Option<&str>,
    private_field: Option<&str>,
) -> Result<(), String> {
    for (name, source, expected) in [
        (
            "mutation",
            "fn main() { let _run = macroonz::native_mutation::run; }\n",
            native_refusal,
        ),
        (
            "mutation-request",
            "fn main() { let _forge = |request: &mut macroonz::native_mutation::MutationRequest| { request.target = \"relabel\".to_owned(); }; }\n",
            private_field,
        ),
        (
            "mutation-output",
            "fn main() { let _forge = |output: &mut macroonz::native_mutation::MutationOutput| { let _context = &mut output.context; }; }\n",
            private_field,
        ),
        (
            "mutation-observation",
            "fn main() { let _forge = |output: &mut macroonz::native_mutation::MutationOutput| { let _observation = &mut output.observation; }; }\n",
            private_field,
        ),
        (
            "historical-comparison",
            "fn main() { let _forge = |value: &macroonz::harness::muterprater::backend_archive::BackendSourceComparison<'_>| { let _current = &value.current; }; }\n",
            Some(if posture == "diet" { "E0433" } else { "E0616" }),
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-{name}"),
            source,
            expected,
        )?;
    }
    Ok(())
}

fn coverage_invariants(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    native_field: Option<&str>,
) -> Result<(), String> {
    let harness_field = Some(if posture == "diet" { "E0433" } else { "E0616" });
    for (name, source, expected) in [
        (
            "coverage-invariant",
            "fn main() { let _forge = |coverage: &macroonz::native_coverage::NativeCoverage| { let _ready = &coverage.ready; }; }\n",
            native_field,
        ),
        (
            "coverage-cleanup-invariant",
            "fn main() { let _forge = |cleanup: &macroonz::harness::fuzz::CoverageCaseCleanup| { let _path = &cleanup.directory; }; }\n",
            harness_field,
        ),
        (
            "coverage-roots-invariant",
            "fn main() { let _forge = |roots: &macroonz::harness::fuzz::CoverageSourceRoots| { let _root = &roots.first; }; }\n",
            harness_field,
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-workflow-{name}"),
            source,
            expected,
        )?;
    }
    Ok(())
}

fn surface(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    name: &str,
    source: &str,
    expected: Option<&str>,
) -> Result<(), String> {
    write_subject(subject, scratch, name, source)?;
    let output = cargo(
        subject,
        profile,
        scratch,
        name,
        &["check", "-j1", "--locked", "--offline"],
    )?;
    surface_result(name, &output, expected)
}

fn surface_result(
    name: &str,
    output: &std::process::Output,
    expected: Option<&str>,
) -> Result<(), String> {
    let stderr = String::from_utf8_lossy(&output.stderr);
    match expected {
        None if output.status.success() => Ok(()),
        Some(code) if output.status.code() == Some(101_i32) && stderr.contains(code) => Ok(()),
        _ => Err(format!(
            "{name}: unexpected workflow feature posture\n{stderr}"
        )),
    }
}

fn observe_native(subject: &Path, scratch: &Path, strict: &Path) -> Result<(), String> {
    write_subject(
        subject,
        scratch,
        "native-execution",
        include_str!("subject.rs"),
    )?;
    for (name, arguments) in [
        ("consumer-run", vec!["run", "-j1", "--locked", "--offline"]),
        (
            "consumer-wasm",
            vec![
                "check",
                "-j1",
                "--locked",
                "--offline",
                "--target",
                "wasm32-unknown-unknown",
            ],
        ),
    ] {
        let output = cargo(subject, strict, scratch, name, &arguments)?;
        if !output.status.success() {
            return Err(format!(
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(())
}

fn write_subject(subject: &Path, scratch: &Path, name: &str, source: &str) -> Result<(), String> {
    let invocation = scratch
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or("consumer invocation name absent")?;
    let filename = format!("{invocation}-{name}.rs");
    let manifest_path = subject.join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path).map_err(|error| error.to_string())?;
    let paths = manifest
        .lines()
        .filter(|line| line.starts_with("path = "))
        .collect::<Vec<_>>();
    let [previous] = paths.as_slice() else {
        return Err("consumer manifest must declare one source path".to_owned());
    };
    std::fs::write(subject.join(&filename), source).map_err(|error| error.to_string())?;
    std::fs::write(
        manifest_path,
        manifest.replace(*previous, &format!("path = \"{filename}\"")),
    )
    .map_err(|error| error.to_string())
}
