//! Retention of the existing real backend console and pinned source, without a new campaign.

use super::support::{
    BACKEND_CONSOLE, BACKEND_VERSION, COMPILED_MUTANT_FILE, CURRENT_BACKEND_SOURCE,
    campaign_source, compiled_artifact,
};
use macroonz_harness::muterprater::BackendVersion;
use macroonz_harness::muterprater::backend_archive::{
    BackendArchiveLimits, read_backend, retain_backend, retain_backend_with_material,
};
use macroonz_harness::muterprater::verdict_archive::{
    MutationRunArchiveLimits, retain_mutation_run,
};
use macroonz_harness::report::archive::ArchiveLimits;
use std::error::Error;
use std::io::Read as _;

const LIMITS: BackendArchiveLimits = BackendArchiveLimits::declared(
    MutationRunArchiveLimits::declared(ArchiveLimits::declared(65_536, 32_768), 8),
    32,
    8,
    8,
);

#[test]
fn retained_real_backend_material_is_complete_owned_and_read_in_a_fresh_process()
-> Result<(), Box<dyn Error>> {
    let source =
        campaign_source().map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let version = BackendVersion::stated(BACKEND_VERSION)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let manifest = compiled_artifact(BACKEND_CONSOLE, version, CURRENT_BACKEND_SOURCE)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let plain = retain_backend(&manifest, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    let retained = retain_backend_with_material(
        &manifest,
        BACKEND_CONSOLE,
        &[(COMPILED_MUTANT_FILE, CURRENT_BACKEND_SOURCE)],
        LIMITS,
    )
    .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    assert_eq!(plain.original_console(), None);
    assert_eq!(plain.invocation(), retained.invocation());
    assert_eq!(plain.profile(), retained.profile());
    assert_eq!(plain.output(), retained.output());
    assert_eq!(plain.run(), retained.run());
    assert_eq!(plain.announced(), retained.announced());
    assert_eq!(plain.unparsed(), retained.unparsed());
    assert_ne!(plain.address(), retained.address());
    assert_eq!(
        retained.invocation().backend(),
        manifest.invocation().backend()
    );
    assert_eq!(
        retained.invocation().version(),
        manifest.invocation().version().spelling()
    );
    assert_eq!(
        retained.invocation().executable(),
        manifest.invocation().command().executable()
    );
    assert_eq!(
        retained.invocation().arguments(),
        manifest.invocation().command().arguments()
    );
    assert_eq!(
        retained.invocation().target(),
        manifest.invocation().target()
    );
    assert_eq!(
        retained.profile().grammar(),
        manifest.reading().profile().grammar()
    );
    assert_eq!(
        retained.profile().source(),
        manifest.reading().profile().source()
    );
    assert_eq!(
        retained.output().as_bytes(),
        manifest.output().address().as_bytes()
    );
    let [saved] = retained.sources() else {
        return Err(std::io::Error::other("missing source").into());
    };
    assert_eq!(saved.file(), source.file());
    assert_eq!(
        saved.revision().as_bytes(),
        source.revision().address().as_bytes()
    );
    assert_eq!(saved.original(), Some(CURRENT_BACKEND_SOURCE));
    assert_eq!(
        retained.original_console(),
        Some(BACKEND_CONSOLE.as_bytes())
    );
    assert_eq!(retained.announced(), manifest.reading().announced());
    assert_eq!(
        retained.run(),
        &retain_mutation_run(manifest.reading().run(), LIMITS.run())
            .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?
    );
    assert_eq!(
        retained.unparsed().len(),
        manifest.reading().unparsed().len()
    );
    for (saved_line, original) in retained
        .unparsed()
        .iter()
        .zip(manifest.reading().unparsed())
    {
        assert_eq!(saved_line.ordinal(), u64::try_from(original.ordinal())?);
        assert_eq!(saved_line.text().bytes(), original.text().bytes());
        assert_eq!(saved_line.text().fidelity(), original.text().fidelity());
    }
    drop(manifest);
    for bytes in [plain.encoded(), retained.encoded()] {
        assert_eq!(
            crate::archive_process::round_trip(bytes, "backend_archive::retained_backend_child")?,
            bytes
        );
    }
    Ok(())
}

#[test]
#[ignore = "invoked by the parent with a bounded retained actual backend artifact"]
fn retained_backend_child() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65_537)
        .read_to_end(&mut encoded)?;
    let retained = read_backend(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    crate::archive_process::publish(retained.encoded())
}
