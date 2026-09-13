use super::fixture::LIMITS;
use super::stage_fixture::{DEPENDENCIES, DRIVER, authored, compiled, formatted, request, staged};
use crate::compiler::configure::{bounds, host, locus, root, target, tool};
use crate::compiler::real::{package, read_count};
use macroonz::native_compiler::{CargoFixture, CargoTarget, CompilerRequest};
use macroonz::native_process::ProcessLimits;
use macroonz::native_publication::{StagingObservationError, StagingPlan, StagingRun};
use macroonz::native_storage::StorageName;
use std::time::Duration;

#[test]
fn formatted_staging_compiles_every_published_file_and_executes_independent_values()
-> Result<(), String> {
    let original = root()?;
    let host = host(&original)?;
    let mut expected = None;
    for root in [&original, &root()?] {
        for name in ["first", "second"] {
            let prepared = formatted(&host, root)?;
            let actual = prepared
                .files()
                .map(|file| {
                    (
                        file.bytes().to_vec(),
                        file.canonical_digest(),
                        file.published_digest(),
                    )
                })
                .collect::<Vec<_>>();
            if let Some(expected) = &expected {
                assert_eq!(&actual, expected);
            } else {
                expected = Some(actual);
            }
            let stage = StagingPlan::declared(
                prepared,
                vec![authored("fixture.rs", DRIVER.as_bytes())?],
                LIMITS,
            )
            .and_then(|plan| {
                plan.stage(
                    root,
                    &StorageName::informed(name)
                        .map_err(macroonz::native_publication::StagingError::Storage)?,
                )
            })
            .map_err(|error| error.to_string())?;
            let selected = request(
                &host,
                &host.rustc,
                stage.path(),
                &root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
                bounds()?,
            )?;
            let result = compiled(
                stage
                    .compile(&selected)
                    .map_err(|error| error.to_string())?,
            )?;
            assert_eq!(result.prepared().formatting().count(), 4);
            assert_eq!(
                result
                    .compiler()
                    .dependencies()
                    .ok_or("absent dependencies")?
                    .map_err(ToString::to_string)?
                    .files()
                    .len(),
                5
            );
            read_count(result.compiler(), &host, root)?;
            let installed = root.join(format!("installed-{name}"));
            std::fs::create_dir(&installed).map_err(|error| error.to_string())?;
            let destination = super::install_fixture::destination(&installed)?;
            super::install_fixture::finish(
                destination
                    .begin(&result)
                    .map_err(|error| error.to_string())?,
            )?;
            super::install_fixture::installed(&installed, result.prepared())?;
            assert!(!installed.join("fixture.rs").exists());
        }
    }
    Ok(())
}

#[test]
fn staged_cargo_manifest_and_lock_compile_the_actual_formatted_tree() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let (name, manifest) = package(
        &root,
        "[[bin]]\nname = \"staged-reader\"\npath = \"fixture.rs\"\n",
    )?;
    let inputs = vec![
        authored("fixture.rs", DRIVER.as_bytes())?,
        authored(
            "Cargo.toml",
            &std::fs::read(manifest).map_err(|error| error.to_string())?,
        )?,
        authored(
            "Cargo.lock",
            &std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?,
        )?,
    ];
    let stage = StagingPlan::declared(formatted(&host, &root)?, inputs, LIMITS)
        .and_then(|plan| {
            plan.stage(
                &root,
                &StorageName::informed("cargo")
                    .map_err(macroonz::native_publication::StagingError::Storage)?,
            )
        })
        .map_err(|error| error.to_string())?;
    let fixture = CargoFixture::informed(
        stage.path().join("Cargo.toml"),
        target()?,
        name,
        CargoTarget::Binary("staged-reader".to_owned()),
        host.triple.clone(),
    )
    .map_err(|error| error.to_string())?;
    let dependency = target()?.join(&host.triple).join("debug/staged-reader.d");
    let request = CompilerRequest::cargo(
        &tool(&host, &host.cargo, stage.path(), bounds()?)?,
        fixture,
        locus()?,
    )
    .and_then(|request| request.with_dependencies(dependency, DEPENDENCIES))
    .map_err(|error| error.to_string())?;
    let output = compiled(stage.compile(&request).map_err(|error| error.to_string())?)?;
    assert_eq!(output.compiler().cargo_fresh(), Some(false));
    read_count(output.compiler(), &host, &root)?;
    Ok(())
}

#[test]
fn staged_compilation_refuses_unused_generated_material_and_actual_compile_failure()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let missing = "#[path = \"generated/definition.rs\"] mod definition;\n#[path = \"generated/alpha.rs\"] mod alpha;\n#[path = \"generated/other.rs\"] mod other;\nfn main() { let _value = (alpha::VALUE, other::OTHER); }\n";
    for (name, source, expected) in [
        (
            "unused",
            missing,
            StagingObservationError::Unused("generated/beta.rs".to_owned()),
        ),
        (
            "refused",
            "fn main() { let _value: u8 = \"invalid\"; }\n",
            StagingObservationError::Compilation,
        ),
    ] {
        let stage = staged(&root, name, source)?;
        let selected = request(
            &host,
            &host.rustc,
            stage.path(),
            &root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
            bounds()?,
        )?;
        let StagingRun::Refused(output) = stage
            .compile(&selected)
            .map_err(|error| error.to_string())?
        else {
            return Err("bad subject qualified".to_owned());
        };
        assert_eq!(output.reason(), &expected);
        assert_eq!(
            output.compiler().process().status().success(),
            name == "unused"
        );
        assert_eq!(output.staged().prepared().files().count(), 4);
    }
    Ok(())
}

#[test]
fn staged_pending_cleanup_retains_context_and_compares_source_after_retry() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    for name in ["unchanged", "changed"] {
        let stage = staged(&root, name, DRIVER)?;
        let path = stage.path().to_path_buf();
        let limits = ProcessLimits::informed(Duration::from_secs(30), Duration::ZERO, 65536, 65536)
            .map_err(|error| error.to_string())?;
        let selected = request(
            &host,
            &host.rustc,
            &path,
            &root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
            limits,
        )?;
        let StagingRun::Pending(pending) = stage
            .compile(&selected)
            .map_err(|error| error.to_string())?
        else {
            return Err("zero cleanup lost pending custody".to_owned());
        };
        let _owned = pending.compiler().process();
        if name == "changed" {
            std::fs::write(
                path.join("generated/other.rs"),
                b"pub const OTHER: u8 = 43;\n",
            )
            .map_err(|error| error.to_string())?;
        }
        let completed = pending.finish(Duration::from_secs(5));
        if name == "unchanged" {
            drop(compiled(completed)?);
        } else {
            let StagingRun::Refused(output) = completed else {
                return Err("changed pending source qualified".to_owned());
            };
            assert!(matches!(
                output.reason(),
                StagingObservationError::Source(_)
            ));
            assert!(output.compiler().process().status().success());
        }
    }
    Ok(())
}
