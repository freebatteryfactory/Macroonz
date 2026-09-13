use super::configure::{configuration, finish, publication, scratch};
use crate::compiler::configure::{bounds, host, root, tool};
use macroonz::native_publication::{Formatter, published_digest};

#[test]
fn actual_formatter_preserves_canonical_material_across_regeneration_and_relocation()
-> Result<(), String> {
    let original = root()?;
    let host = host(&original)?;
    let executable = host
        .rustc
        .parent()
        .ok_or("missing toolchain directory")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let publication = publication()?;
    let mut expected = None;
    for directory in [&original, &root()?] {
        std::fs::write(directory.join(".rustfmt.toml"), b"edition = \"1900\"\n")
            .map_err(|error| error.to_string())?;
        let selected = Formatter::qualified(
            &tool(&host, &executable, directory, bounds()?)?,
            configuration(directory)?,
        )
        .map_err(|error| format!("{error:?}"))?;
        assert_eq!(selected.version().0.arguments(), ["--version"]);
        assert!(selected.version().1.status().success());
        assert_eq!(selected.request().executable(), executable);
        for _regeneration in 0u8..2u8 {
            let actual = rendered(&selected, &publication, directory)?;
            if let Some(expected) = &expected {
                assert_eq!(&actual, expected);
            } else {
                expected = Some(actual);
            }
        }
    }
    Ok(())
}

#[test]
fn formatter_configuration_is_explicit_bounded_and_compared_before_input_write()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = host
        .rustc
        .parent()
        .ok_or("toolchain absent")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let selected_tool = tool(&host, &executable, &root, bounds()?)?;
    for config in [
        root.clone(),
        root.join("missing.toml"),
        std::path::PathBuf::from("relative.toml"),
    ] {
        assert!(Formatter::qualified(&selected_tool, config).is_err());
    }
    let config = configuration(&root)?;
    std::fs::write(&config, vec![b' '; 65537]).map_err(|error| error.to_string())?;
    assert!(Formatter::qualified(&selected_tool, config.clone()).is_err());
    let selected = Formatter::qualified(&selected_tool, configuration(&root)?)
        .map_err(|error| error.to_string())?;
    std::fs::write(config, b"max_width = 80\n").map_err(|error| error.to_string())?;
    let input = scratch(&root)?;
    std::fs::write(root.join("formatter-input"), b"must remain unchanged")
        .map_err(|error| error.to_string())?;
    let publication = publication()?;
    let file = publication.files().next().ok_or("no file")?;
    assert!(matches!(
        selected.format(&file, input),
        Err(macroonz::native_publication::FormatError::Configuration(_))
    ));
    assert_eq!(
        std::fs::read(root.join("formatter-input")).map_err(|error| error.to_string())?,
        b"must remain unchanged"
    );
    Ok(())
}

fn rendered(
    selected: &Formatter,
    publication: &macroonz::native_publication::Publication<super::types::Example>,
    directory: &std::path::Path,
) -> Result<Vec<String>, String> {
    let mut actual = Vec::new();
    for file in publication.files() {
        let output = finish(
            selected
                .format(&file, scratch(directory)?)
                .map_err(|error| format!("{error:?}"))?,
        )?;
        let text = output
            .source()
            .map_err(|error| format!("{error:?}: {output:?}"))?;
        assert_eq!(output.canonical_digest(), file.canonical_digest());
        assert_eq!(output.original(), file.source());
        assert_eq!(output.path(), file.path());
        assert_eq!(
            output.profile().1,
            b"max_width = 100\nnewline_style = \"Windows\"\n"
        );
        assert_eq!(
            output.published_digest(),
            Ok(published_digest(text.as_bytes()))
        );
        assert!(!text.contains('\r'));
        if file.path().spelling() == "generated/other.rs" {
            assert_eq!(text, "pub const OTHER: u8 = 42;\n");
            assert_ne!(
                published_digest(output.original().as_bytes()),
                published_digest(text.as_bytes())
            );
        }
        if file.path().spelling() == "generated/definition.rs" {
            assert!(text.contains("Declares one independently supplied value."));
        }
        actual.push(text.to_owned());
    }
    Ok(actual)
}

#[test]
fn actual_rustfmt_failure_retains_diagnostics_without_published_bytes() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = host
        .rustc
        .parent()
        .ok_or("toolchain absent")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let config = configuration(&root)?;
    std::fs::write(&config, b"max_width = \"invalid\"\n").map_err(|error| error.to_string())?;
    let selected = Formatter::qualified(&tool(&host, &executable, &root, bounds()?)?, config)
        .map_err(|error| format!("{error:?}"))?;
    let publication = publication()?;
    let file = publication.files().last().ok_or("no file")?;
    let output = finish(
        selected
            .format(&file, scratch(&root)?)
            .map_err(|error| error.to_string())?,
    )?;
    assert!(!output.process().status().success());
    assert!(!output.process().stderr().bytes().is_empty());
    assert_eq!(
        output.source(),
        Err(macroonz::native_publication::FormatObservationError::Process)
    );
    assert!(output.published_digest().is_err());
    Ok(())
}
