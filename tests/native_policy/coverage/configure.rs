use super::types::Instrumentation;
use crate::compiler::configure::{bounds, finish, locus, tool};
use crate::compiler::types::Host;
use macroonz::harness::descriptor::{
    DerivedRevision, NamespacedName, PopulationRef, RevisionBinding,
};
use macroonz::harness::fuzz::{
    CoverageBudgets, CoverageCampaign, CoverageProfile, CoverageSourceRoot, InstrumentedTarget,
    RustcProfileRequest,
};
use macroonz::harness::report::{ByteBudget, CaseBudget};
use macroonz::native_compiler::{self, CompilerRequest};
use macroonz::native_coverage::{self, NativeCoverage};
use macroonz::native_process::ProcessLimits;
use std::path::Path;

pub(super) fn compiled(
    root: &Path,
    host: &Host,
    instrumentation: Instrumentation,
) -> Result<std::path::PathBuf, String> {
    let source = include_str!("subject.rs");
    std::fs::write(root.join("fixture.rs"), source).map_err(|error| error.to_string())?;
    let artifact = root.join(format!("coverage-target{}", std::env::consts::EXE_SUFFIX));
    let request = CompilerRequest::rustc(
        &tool(host, &host.rustc, root, bounds()?)?,
        locus()?,
        artifact,
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let request = match instrumentation {
        Instrumentation::Selected => request.instrumented().map_err(|error| error.to_string())?,
        Instrumentation::Absent => request,
    };
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    output
        .executable()
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("no target: {output:?}"))
}

pub(super) fn request(
    root: &Path,
    rustc: &Path,
    target: &Path,
    cases: &str,
) -> Result<RustcProfileRequest, String> {
    request_with_campaign(
        root,
        rustc,
        target,
        cases,
        campaign(include_bytes!("subject.rs"))?,
    )
}

pub(super) fn request_with_campaign(
    root: &Path,
    rustc: &Path,
    target: &Path,
    cases: &str,
    campaign: CoverageCampaign,
) -> Result<RustcProfileRequest, String> {
    let source = CoverageSourceRoot::declared(
        NamespacedName::named("native-test", "source").map_err(debug)?,
        root.to_path_buf(),
    )
    .map_err(debug)?;
    RustcProfileRequest::declared(
        rustc.to_path_buf(),
        InstrumentedTarget::declared(target.to_path_buf(), Vec::new()).map_err(debug)?,
        source,
        root.join(cases),
        campaign,
    )
    .map_err(debug)
}

pub(super) fn campaign(material: &[u8]) -> Result<CoverageCampaign, String> {
    campaign_with_export(material, 1_048_576)
}

pub(super) fn campaign_with_export(
    material: &[u8],
    export_bytes: u64,
) -> Result<CoverageCampaign, String> {
    let budgets = CoverageBudgets::declared(
        CaseBudget::declared(8),
        ByteBudget::declared(64),
        export_bytes,
        10_000,
        CaseBudget::declared(4),
        ByteBudget::declared(32),
    )
    .map_err(debug)?;
    Ok(CoverageCampaign::declared(
        PopulationRef::named("native-test", "coverage").map_err(debug)?,
        RevisionBinding::derived(DerivedRevision::from_material(material)),
        CoverageProfile::declared(
            NamespacedName::named("native-test", "lines").map_err(debug)?,
            1,
        ),
        budgets,
    ))
}

pub(super) fn ready(
    root: &Path,
    host: &Host,
    target: &Path,
    cases: &str,
    target_limits: ProcessLimits,
) -> Result<NativeCoverage, String> {
    native_coverage::preflight(
        request(root, &host.rustc, target, cases)?,
        tool(host, &host.rustc, root, bounds()?)?,
        target_limits,
    )
    .map_err(debug)
}

pub(super) fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
