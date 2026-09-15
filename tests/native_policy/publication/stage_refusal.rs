use super::configure::publication;
use super::fixture::LIMITS;
use super::stage_fixture::{DEPENDENCIES, DRIVER, authored, plan, request, staged};
use crate::compiler::configure::{bounds, host, locus, root, standin, tool};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_compiler::{CargoFixture, CargoTarget, CompilerRequest};
use macroonz::native_publication::{
    InventoryError, PreparedPublication, PublicationLimits, StagingError, StagingPlan,
};
use macroonz::native_storage::StorageName;
use std::path::Path;

#[test]
fn staging_union_refuses_collisions_and_shared_bounds_before_filesystem_access()
-> Result<(), String> {
    for (name, limits, expected) in [
        ("generated/other.rs", LIMITS, InventoryError::PathCollision),
        ("GENERATED/OTHER.rs", LIMITS, InventoryError::PathCollision),
        ("generated", LIMITS, InventoryError::PathCollision),
        (
            "generated/other.rs/child",
            LIMITS,
            InventoryError::PathCollision,
        ),
        (
            "fixture.rs",
            PublicationLimits {
                files: 4,
                bytes: 65536,
            },
            InventoryError::FileBound,
        ),
        (
            "fixture.rs",
            PublicationLimits { files: 8, bytes: 1 },
            InventoryError::ByteBound,
        ),
    ] {
        let refusal = StagingPlan::declared(
            PreparedPublication::unformatted(publication()?),
            vec![authored(name, b"declared")?],
            limits,
        );
        assert!(matches!(refusal, Err(StagingError::Inventory(actual)) if actual == expected));
    }
    let root = root()?;
    let name = StorageName::informed("existing").map_err(|error| format!("{error:?}"))?;
    let first = plan(DRIVER)?
        .stage(&root, &name)
        .map_err(|error| error.to_string())?;
    assert!(matches!(
        plan(DRIVER)?.stage(&root, &name),
        Err(StagingError::Filesystem(_))
    ));
    assert_eq!(
        std::fs::read(first.path().join("fixture.rs")).map_err(|error| error.to_string())?,
        DRIVER.as_bytes()
    );
    assert!(matches!(
        plan(DRIVER)?.stage(Path::new("relative"), &name),
        Err(StagingError::Configuration(_))
    ));
    assert!(!root.join(".macroonz-storage-lock").exists());
    Ok(())
}

#[test]
fn changed_missing_extra_and_non_file_stage_entries_refuse_before_compiler_start()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    for change in [
        "changed",
        "missing",
        "extra-file",
        "extra-directory",
        "directory-file",
    ] {
        let stage = staged(&root, change, DRIVER)?;
        let owned = stage.path().join("generated/other.rs");
        match change {
            "changed" => std::fs::write(&owned, b"pub const OTHER: u8 = 43;\n"),
            "missing" => std::fs::remove_file(&owned),
            "extra-file" => std::fs::write(stage.path().join("extra.rs"), b"extra"),
            "extra-directory" => std::fs::create_dir(stage.path().join("extra")),
            _ => std::fs::remove_file(&owned).and_then(|()| std::fs::create_dir(&owned)),
        }
        .map_err(|error| error.to_string())?;
        let selected = request(
            &host,
            &root.join("nonexistent-tool"),
            stage.path(),
            &root.join("artifact"),
            bounds()?,
        )?;
        assert!(
            matches!(
                stage.compile(&selected),
                Err(StagingError::Source(_) | StagingError::Storage(_))
            ),
            "{change} must refuse source before attempting the nonexistent compiler"
        );
        assert!(!root.join("artifact").exists());
    }
    Ok(())
}

#[test]
fn staged_directory_links_refuse_the_visible_compiler_source() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let outside = crate::compiler::configure::root()?;
    std::fs::write(outside.join("sentinel"), b"untouched").map_err(|error| error.to_string())?;
    let stage = staged(&root, "linked", DRIVER)?;
    let selected = request(
        &host,
        &root.join("nonexistent-tool"),
        stage.path(),
        &root.join("artifact"),
        bounds()?,
    )?;
    let visible = stage.path().join("generated");
    std::fs::rename(&visible, root.join("old-generated")).map_err(|error| error.to_string())?;
    link(&outside, &visible)?;
    assert!(matches!(
        stage.compile(&selected),
        Err(StagingError::Source(_))
    ));
    unlink(&visible)?;
    assert_eq!(
        std::fs::read(outside.join("sentinel")).map_err(|error| error.to_string())?,
        b"untouched"
    );
    Ok(())
}

#[test]
fn staged_root_replacement_cannot_redirect_compilation() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let stage = staged(&root, "root-replacement", DRIVER)?;
    let selected = request(
        &host,
        &host.rustc,
        stage.path(),
        &root.join(format!("root-control{}", std::env::consts::EXE_SUFFIX)),
        bounds()?,
    )?;
    let visible = stage.path().to_path_buf();
    let replacement = std::fs::rename(&visible, root.join("old-root"));
    #[cfg(windows)]
    {
        assert_eq!(
            replacement.err().and_then(|error| error.raw_os_error()),
            Some(32_i32)
        );
        drop(super::stage_fixture::compiled(
            stage
                .compile(&selected)
                .map_err(|error| error.to_string())?,
        )?);
    }
    #[cfg(unix)]
    {
        replacement.map_err(|error| error.to_string())?;
        std::fs::create_dir(&visible).map_err(|error| error.to_string())?;
        assert!(matches!(
            stage.compile(&selected),
            Err(StagingError::Source(_))
        ));
    }
    Ok(())
}

#[test]
fn compilation_request_crossings_refuse_before_running_a_tool() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let marker = root.join("started");
    let executable = standin(
        &root,
        &host,
        "start-marker",
        &format!(
            "fn main() -> std::io::Result<()> {{ std::fs::write({:?}, b\"started\") }}",
            marker.to_str().ok_or("non-Unicode marker")?
        ),
    )?;
    for name in [
        "different-root",
        "missing-locus",
        "missing-dependencies",
        "inside-output",
        "inside-dependencies",
        "foreign-manifest",
    ] {
        let stage = staged(&root, name, DRIVER)?;
        let directory = if name == "different-root" {
            &root
        } else {
            stage.path()
        };
        let selected = tool(&host, &executable, directory, bounds()?)?;
        let locus = if name == "missing-locus" {
            RelativeSourcePath::informed("unstaged.rs").map_err(|error| format!("{error:?}"))?
        } else {
            locus()?
        };
        let artifact = if name == "inside-output" {
            stage.path().join("output")
        } else {
            root.join("output")
        };
        let compiler = if name == "foreign-manifest" {
            let fixture = CargoFixture::informed(
                root.join("Cargo.toml"),
                root.join("output"),
                "unselected".to_owned(),
                CargoTarget::Library,
                host.triple.clone(),
            )
            .map_err(|error| error.to_string())?;
            CompilerRequest::cargo(&selected, fixture, locus)
        } else {
            CompilerRequest::rustc(&selected, locus, artifact, &host.triple)
        }
        .map_err(|error| error.to_string())?;
        let compiler = if name == "missing-dependencies" {
            compiler
        } else {
            let dependencies = if name == "inside-dependencies" {
                stage.path().join("observed.d")
            } else {
                root.join("observed.d")
            };
            compiler
                .with_dependencies(dependencies, DEPENDENCIES)
                .map_err(|error| error.to_string())?
        };
        assert!(
            matches!(
                stage.compile(&compiler),
                Err(StagingError::Configuration(_))
            ),
            "{name}"
        );
        assert!(!marker.exists(), "{name} started a refused compiler");
    }
    Ok(())
}

pub(super) fn link(destination: &Path, source: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(destination, source).map_err(|error| error.to_string())
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(destination, source).map_err(|error| error.to_string())
    }
}

pub(super) fn unlink(path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::fs::remove_dir(path).map_err(|error| error.to_string())
    }
    #[cfg(unix)]
    {
        std::fs::remove_file(path).map_err(|error| error.to_string())
    }
}
