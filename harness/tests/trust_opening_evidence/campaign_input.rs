//! Explicit current-run material crosses the public backend reader without historical fixture substitution.

use super::campaign_accounting::{expected, reconcile};
use macroonz_harness::muterprater::wrap::read_artifact;
use macroonz_harness::muterprater::{
    AdapterQualification, BackendCommand, BackendVersion, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactStanding, CompiledSuitePressure, GrammarStanding,
    MutationBackendInvocation, MutationSourceRevision, WrappedBackend,
};
use macroonz_harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use std::io::{Read, Write};
use std::path::{Component, Path};

const MATERIAL_LIMIT: usize = 16 * 1024 * 1024;

fn bytes(path: &Path) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    bounded(file).map_err(|error| format!("{}: {error}", path.display()))
}

fn bounded(reader: impl Read) -> Result<Vec<u8>, String> {
    let maximum = u64::try_from(MATERIAL_LIMIT).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    reader
        .take(maximum.checked_add(1).ok_or("material limit overflow")?)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.is_empty() || bytes.len() > MATERIAL_LIMIT {
        return Err("missing or oversized material".to_owned());
    }
    Ok(bytes)
}

fn text(root: &Path, name: &str) -> Result<String, String> {
    String::from_utf8(bytes(&root.join(name))?).map_err(|error| error.to_string())
}

fn label(root: &Path, name: &str) -> Result<String, String> {
    let material = text(root, name)?;
    let label = material.trim();
    if label.is_empty() || label.lines().count() != 1 {
        return Err(format!("missing or multiline campaign label: {name}"));
    }
    Ok(label.to_owned())
}

fn source(root: &Path, file: &str) -> Result<MutationSourceRevision, String> {
    if !Path::new(file)
        .components()
        .all(|part| matches!(part, Component::Normal(_)))
    {
        return Err("source path is not a relative declared file".to_owned());
    }
    let root = root.canonicalize().map_err(|error| error.to_string())?;
    let path = root
        .join(file)
        .canonicalize()
        .map_err(|error| error.to_string())?;
    if !path.starts_with(&root) {
        return Err("declared source escapes its supplied root".to_owned());
    }
    MutationSourceRevision::from_content(file, &bytes(&path)?).map_err(|error| format!("{error:?}"))
}

fn observe(root: &Path) -> Result<(), String> {
    let console = text(root, "console.txt")?;
    let declared = expected(&text(root, "expected.tsv")?)?;
    let command = text(root, "command.txt")?;
    let command = command.lines().collect::<Vec<_>>();
    let [executable, arguments @ ..] = command.as_slice() else {
        return Err("missing backend command".to_owned());
    };
    let version = BackendVersion::stated(&label(root, "version.txt")?)
        .map_err(|error| format!("{error:?}"))?;
    let invocation = MutationBackendInvocation::declared(
        WrappedBackend::CargoMutants,
        version.clone(),
        BackendCommand::declared(executable, arguments).map_err(|error| format!("{error:?}"))?,
        TargetBinding::bound(
            TargetTriple::declared(&label(root, "target.txt")?),
            ToolchainIdentity::declared(&label(root, "toolchain.txt")?),
        ),
    );
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("harness must have a workspace parent")?;
    let files = text(root, "sources.txt")?;
    let mut recorded = Vec::new();
    let mut current = Vec::new();
    for file in files.lines() {
        recorded.push(source(&root.join("recorded"), file)?);
        current.push(source(repository, file)?);
    }
    let manifest = read_artifact(&console, invocation, recorded, |_| None, |_, _| None)
        .map_err(|error| format!("{error:?}"))?;
    reconcile(manifest.reading(), &declared)?;
    let mut output = std::io::stdout().lock();
    writeln!(
        output,
        "compiled campaign census: {:?}",
        manifest.reading().run().census()
    )
    .map_err(|error| error.to_string())?;
    for line in manifest.reading().unparsed() {
        writeln!(
            output,
            "unparsed ordinal={} material={:?}",
            line.ordinal(),
            line.text()
        )
        .map_err(|error| error.to_string())?;
    }
    let qualification =
        AdapterQualification::of(manifest.reading(), GrammarStanding::Checked(version))
            .map_err(|error| format!("{error:?}"))?;
    let custody = CompiledSuiteArtifactCustody::current(manifest, current)
        .map_err(|error| format!("{error:?}"))?;
    let pressure = CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    )
    .map_err(|error| format!("{error:?}"))?;
    writeln!(
        output,
        "compiled campaign: {} exact identities, rejection={:?}; activation unobserved, owner unmapped, family outside bank",
        declared.len(),
        pressure.kill().target().identity()
    ).map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
#[ignore = "explicit completed mutation campaign material; run by exact selection"]
fn a_completed_backend_campaign_matches_its_declared_roster_and_current_sources()
-> Result<(), String> {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("mutation-campaign");
    if !root.is_absolute() {
        return Err("campaign material root must be absolute".to_owned());
    }
    observe(&root)
}

#[test]
fn campaign_material_is_nonempty_and_refuses_the_first_byte_past_its_bound() -> Result<(), String> {
    assert_eq!(
        bounded(std::io::empty()),
        Err("missing or oversized material".to_owned())
    );
    for size in [
        MATERIAL_LIMIT.checked_sub(1).ok_or("invalid lower bound")?,
        MATERIAL_LIMIT,
    ] {
        let material = vec![b'x'; size];
        assert_eq!(bounded(std::io::Cursor::new(&material))?, material);
    }
    let too_many = MATERIAL_LIMIT.checked_add(1).ok_or("invalid upper bound")?;
    assert_eq!(
        bounded(std::io::Cursor::new(vec![b'x'; too_many])),
        Err("missing or oversized material".to_owned())
    );
    Ok(())
}
