use super::configure::{configuration, finish, publication, scratch, standin as body};
use super::fixture;
use super::types::Example;
use crate::compiler::configure::{bounds, host, root, tool};
use crate::compiler::refusal::standin;
use macroonz::native_publication::{
    FormatOutput, Formatter, PreparationError, PreparedPublication, Publication,
};
use std::path::Path;

#[test]
fn unformatted_preparation_preserves_the_complete_admitted_source() -> Result<(), String> {
    let publication = publication()?;
    let identity = publication.expansion().identity();
    let prepared = PreparedPublication::unformatted(publication);
    assert_eq!(prepared.publication().expansion().identity(), identity);
    assert_eq!(prepared.formatting().count(), 0);
    assert_eq!(prepared.files().count(), 4);
    for (file, source) in prepared.files().zip(prepared.publication().files()) {
        assert_eq!(file.path(), source.path());
        assert_eq!(file.bytes(), source.source().as_bytes());
        assert_eq!(file.canonical_digest(), source.canonical_digest());
    }
    Ok(())
}

#[test]
fn complete_preparation_refuses_missing_duplicate_foreign_old_and_oversized_outputs()
-> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = standin(
        &root,
        &host,
        "copy-formatter",
        &body("std::io::copy(&mut std::io::stdin(), &mut std::io::stdout())?;"),
    )?;
    let selected = Formatter::qualified(
        &tool(&host, &executable, &root, bounds()?)?,
        configuration(&root)?,
    )
    .map_err(|error| error.to_string())?;
    let original = publication()?;
    let mut reversed = outputs(&selected, &original, &root)?;
    reversed.reverse();
    let prepared = PreparedPublication::formatted(original, reversed, 65536)
        .map_err(|error| error.to_string())?;
    assert_eq!(
        prepared
            .files()
            .map(|file| file.path().spelling().to_owned())
            .collect::<Vec<_>>(),
        [
            "generated/definition.rs",
            "generated/alpha.rs",
            "generated/beta.rs",
            "generated/other.rs"
        ]
    );
    assert_eq!(prepared.formatting().count(), 4);
    let other = prepared.files().last().ok_or("no last file")?;
    assert_eq!(other.bytes(), b"pub const OTHER : u8 = 42 ; \n");

    let mut incomplete = outputs(&selected, prepared.publication(), &root)?;
    drop(incomplete.pop());
    assert_eq!(
        PreparedPublication::formatted(publication()?, incomplete, 65536).err(),
        Some(PreparationError::Inventory)
    );
    let mut duplicated = outputs(&selected, prepared.publication(), &root)?;
    drop(duplicated.pop());
    let first = prepared
        .publication()
        .files()
        .next()
        .ok_or("no first file")?;
    duplicated.push(finish(
        selected
            .format(&first, scratch(&root)?)
            .map_err(|error| error.to_string())?,
    )?);
    assert_eq!(
        PreparedPublication::formatted(publication()?, duplicated, 65536).err(),
        Some(PreparationError::Inventory)
    );

    let changed = fixture::expanded([7, 9, 43])?;
    let changed_landings = fixture::landings()?;
    let changed = Publication::declared(
        changed.0,
        &fixture::bindings("generated/definition.rs", "generated/other.rs")?,
        fixture::LIMITS,
    )
    .and_then(|value| value.with_stamp(changed.1, &changed_landings))
    .map_err(|error| error.to_string())?;
    assert_eq!(
        PreparedPublication::formatted(
            changed,
            outputs(&selected, prepared.publication(), &root)?,
            65536
        )
        .err(),
        Some(PreparationError::Source)
    );
    assert_eq!(
        PreparedPublication::formatted(
            publication()?,
            outputs(&selected, prepared.publication(), &root)?,
            1
        )
        .err(),
        Some(PreparationError::ByteBound)
    );
    let foreign = fixture::expanded([7, 9, 42])?;
    let foreign_landings = fixture::landings()?;
    let foreign = Publication::declared(
        foreign.0,
        &fixture::bindings("foreign.rs", "generated/other.rs")?,
        fixture::LIMITS,
    )
    .and_then(|value| value.with_stamp(foreign.1, &foreign_landings))
    .map_err(|error| error.to_string())?;
    assert_eq!(
        PreparedPublication::formatted(publication()?, outputs(&selected, &foreign, &root)?, 65536)
            .err(),
        Some(PreparationError::Inventory)
    );
    Ok(())
}

pub(super) fn outputs(
    selected: &Formatter,
    publication: &Publication<Example>,
    root: &Path,
) -> Result<Vec<FormatOutput>, String> {
    publication
        .files()
        .map(|file| {
            finish(
                selected
                    .format(&file, scratch(root)?)
                    .map_err(|error| error.to_string())?,
            )
        })
        .collect()
}

#[test]
fn failed_formatter_output_cannot_enter_a_complete_prepared_set() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let executable = standin(
        &root,
        &host,
        "failed-preparation",
        &body("std::fs::read(\"deliberately-absent-file\")?;"),
    )?;
    let selected = Formatter::qualified(
        &tool(&host, &executable, &root, bounds()?)?,
        configuration(&root)?,
    )
    .map_err(|error| error.to_string())?;
    let publication = publication()?;
    let actual = outputs(&selected, &publication, &root)?;
    assert_eq!(actual.len(), 4);
    assert_eq!(
        PreparedPublication::formatted(publication, actual, 65536).err(),
        Some(PreparationError::Formatter(
            macroonz::native_publication::FormatObservationError::Process
        ))
    );
    Ok(())
}

#[test]
fn an_inline_only_expansion_admits_an_empty_publication_without_formatter_output()
-> Result<(), String> {
    let limits = macroonz::native_publication::PublicationLimits { files: 0, bytes: 0 };
    let publication = Publication::declared(fixture::inline_only()?, &[], limits)
        .map_err(|error| error.to_string())?;
    let prepared = PreparedPublication::unformatted(publication);
    assert_eq!(prepared.files().count(), 0);
    assert_eq!(prepared.formatting().count(), 0);
    let formatted_inventory = Publication::declared(fixture::inline_only()?, &[], limits)
        .map_err(|error| error.to_string())?;
    let formatted = PreparedPublication::formatted(formatted_inventory, Vec::new(), 0)
        .map_err(|error| error.to_string())?;
    assert_eq!(formatted.files().count(), 0);
    assert_eq!(formatted.formatting().count(), 0);
    Ok(())
}
