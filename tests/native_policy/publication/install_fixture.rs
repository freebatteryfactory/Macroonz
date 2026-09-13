use super::destination_fixture::LIMITS;
use super::fixture::{self, bindings, expanded, landings};
use super::stage_fixture::{authored, compiled, request};
use super::types::Example;
use crate::compiler::configure::{bounds, host};
use macroonz::native_publication::{
    CompiledPublication, PreparedPublication, Publication, PublicationDestination,
    PublicationInstallation, StagingPlan,
};
use macroonz::native_storage::StorageName;
use std::path::Path;

pub(super) fn subject(
    root: &Path,
    name: &str,
    values: [u8; 3],
    other: &str,
) -> Result<CompiledPublication<Example>, String> {
    let host = host(root)?;
    let (expansion, stamp) = expanded(values)?;
    let landings = landings()?;
    let publication = Publication::declared(
        expansion,
        &bindings("generated/definition.rs", other)?,
        fixture::LIMITS,
    )
    .and_then(|publication| publication.with_stamp(stamp, &landings))
    .map_err(|error| error.to_string())?;
    let source = format!(
        "#[path = \"generated/definition.rs\"] mod definition;\n#[path = \"generated/alpha.rs\"] mod alpha;\n#[path = \"generated/beta.rs\"] mod beta;\n#[path = {other:?}] mod other;\nfn main() {{ assert_eq!([alpha::VALUE, beta::VALUE, other::OTHER], {values:?}); }}\n"
    );
    let stage = StagingPlan::declared(
        PreparedPublication::unformatted(publication),
        vec![authored("fixture.rs", source.as_bytes())?],
        fixture::LIMITS,
    )
    .and_then(|plan| {
        plan.stage(
            root,
            &StorageName::informed(name)
                .map_err(macroonz::native_publication::StagingError::Storage)?,
        )
    })
    .map_err(|error| error.to_string())?;
    let request = request(
        &host,
        &host.rustc,
        stage.path(),
        &root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX)),
        bounds()?,
    )?;
    compiled(stage.compile(&request).map_err(|error| error.to_string())?)
}

pub(super) fn destination(path: &Path) -> Result<PublicationDestination, String> {
    PublicationDestination::open(path, LIMITS).map_err(|error| error.to_string())
}

pub(super) fn finish(mut installation: PublicationInstallation<'_>) -> Result<(), String> {
    while installation
        .write_next()
        .map_err(|error| error.to_string())?
        .is_some()
    {}
    installation.commit().map_err(|error| error.to_string())
}

pub(super) fn installed(
    path: &Path,
    expected: &PreparedPublication<Example>,
) -> Result<(), String> {
    assert!(
        destination(path)?
            .check(expected)
            .map_err(|error| error.to_string())?
            .is_current()
    );
    for file in expected.files() {
        assert_eq!(
            std::fs::read(path.join(file.path().spelling())).map_err(|error| error.to_string())?,
            file.bytes()
        );
    }
    assert!(!path.join(".macroonz-publication/item-pending").exists());
    Ok(())
}
