//! Bounded historical retention of both assessments without current execution or admission authority.

use super::types::{Sample, Settings};
use macroonz::harness::descriptor::{NamespacedName, archive::BindingArchiveLimits};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::muterprater::interpretation_archive::{
    ArchivedAssessment, AssessmentArchiveLimits, ValueEncoder, ValueEncodingRefusal,
    read_assessment, retain_assessment,
};
use macroonz::harness::muterprater::proposal_archive::{ArchivedProposal, read_proposal};
use macroonz::harness::muterprater::{MutationAssessment, discovery_archive::SurfaceArchiveLimits};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::native_storage::{
    StorageArtifact, StorageBatch, StorageLimits, StorageName, StorageRoot, StorageTransaction,
};
use std::io::Write;
use std::path::Path;

const BYTES: ArchiveLimits = ArchiveLimits::declared(131_072, 65_536);
const LIMITS: AssessmentArchiveLimits = AssessmentArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 1, 1),
    BindingArchiveLimits::declared(16_384, 512, 8),
    BYTES,
    16_384,
    4,
    1,
);
const STORAGE: StorageLimits = StorageLimits {
    artifacts: 3,
    bytes: 393_216,
};

pub(super) fn retain(
    settings: &Settings,
    weak: &MutationAssessment<'_, Sample<'_>, u32>,
    strong: &MutationAssessment<'_, Sample<'_>, u32>,
    proposal: &ArchivedProposal,
) -> Result<(), String> {
    let input = encoder("sample-u32-be", |sample: &Sample<'_>| {
        Ok(sample.value.to_be_bytes().to_vec())
    })?;
    let meaning = encoder("result-u32-be", |result: &u32| {
        Ok(result.to_be_bytes().to_vec())
    })?;
    let weak = retain_assessment(weak, &input, &meaning, LIMITS).map_err(super::debug)?;
    let strong = retain_assessment(strong, &input, &meaning, LIMITS).map_err(super::debug)?;
    let root = root(&settings.storage)?;
    let batch_name = name("assessment")?;
    let weak_name = name("weak")?;
    let strong_name = name("strong")?;
    let proposal_name = name("proposal")?;
    let artifacts = [
        StorageArtifact {
            name: &weak_name,
            bytes: weak.encoded(),
        },
        StorageArtifact {
            name: &strong_name,
            bytes: strong.encoded(),
        },
        StorageArtifact {
            name: &proposal_name,
            bytes: proposal.encoded(),
        },
    ];
    let batch = StorageBatch::informed(&artifacts, STORAGE).map_err(super::debug)?;
    let mut transaction = StorageTransaction::begin(&root, &batch_name, batch)
        .map_err(|error| format!("retain into a fresh assessment batch: {error:?}"))?;
    while transaction.write_next().map_err(super::debug)?.is_some() {}
    transaction.commit().map_err(super::debug)?;
    display(&weak)?;
    display(&strong)?;
    writeln!(
        std::io::stdout(),
        "{}",
        macroonz::presentation::archived_proposal(proposal).json()
    )
    .map_err(super::debug)
}

pub(super) fn inspect(directory: &Path) -> Result<(), String> {
    let (weak, strong, proposal) = load(directory)?;
    display(&weak)?;
    display(&strong)?;
    writeln!(
        std::io::stdout(),
        "{}",
        macroonz::presentation::archived_proposal(&proposal).json()
    )
    .map_err(super::debug)?;
    writeln!(
        std::io::stdout(),
        "loaded historical assessments and proposal; evaluations=0 compiled-executions=0 witnesses=0 admissions=0"
    )
    .map_err(super::debug)
}

pub(super) fn load(
    directory: &Path,
) -> Result<(ArchivedAssessment, ArchivedAssessment, ArchivedProposal), String> {
    let artifacts = root(directory)?
        .load(&name("assessment")?, STORAGE)
        .map_err(super::debug)?;
    let [proposal, strong, weak] = artifacts.as_slice() else {
        return Err("expected both historical assessments and their proposal".to_owned());
    };
    if proposal.name != name("proposal")?
        || strong.name != name("strong")?
        || weak.name != name("weak")?
    {
        return Err("historical assessment names differ".to_owned());
    }
    let weak = read_assessment(&weak.bytes, LIMITS).map_err(super::debug)?;
    let strong = read_assessment(&strong.bytes, LIMITS).map_err(super::debug)?;
    let proposal = read_proposal(&proposal.bytes, super::proposal::LIMITS).map_err(super::debug)?;
    Ok((weak, strong, proposal))
}

fn encoder<Value>(
    name: &'static str,
    encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
) -> Result<ValueEncoder<Value>, String> {
    Ok(ValueEncoder::declared(
        NamespacedName::named("nonzero", name).map_err(super::debug)?,
        1,
        ContentAddress::derived(
            DomainTag::declared("nonzero-encoding", IdentityProfileVersion::declared(1)),
            name.as_bytes(),
        ),
        super::checks::revision(name.as_bytes()),
        encode,
    ))
}

fn root(directory: &Path) -> Result<StorageRoot, String> {
    if !directory.is_absolute() {
        return Err("storage must be an absolute existing directory".to_owned());
    }
    StorageRoot::open(directory).map_err(super::debug)
}

fn name(value: &str) -> Result<StorageName, String> {
    StorageName::informed(value).map_err(super::debug)
}

fn display(record: &ArchivedAssessment) -> Result<(), String> {
    writeln!(
        std::io::stdout(),
        "{}",
        macroonz::presentation::archived_assessment(record).json()
    )
    .map_err(super::debug)
}
