use super::input::{environment, text};
use super::types::Settings;
use macroonz::harness::descriptor::{
    DerivedRevision, NamespacedName, PopulationRef, RevisionBinding,
};
use macroonz::harness::fuzz::{
    CoverageBudgets, CoverageCampaign, CoverageProfile, CoverageSourceRoot, InstrumentedTarget,
    RustcProfileRequest,
};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::harness::report::{ByteBudget, CaseBudget, TargetTriple};
use macroonz::native_compiler::CompilerRequest;
use macroonz::native_process::{ProcessLimits, ProcessTool};
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let directory = PathBuf::from(text(&configuration, "directory")?);
    let artifact = PathBuf::from(text(&configuration, "artifact")?);
    let limits = ProcessLimits::informed(
        Duration::from_secs(60),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(debug)?;
    let tool = ProcessTool::informed(
        PathBuf::from(text(&configuration, "rustc")?),
        directory.clone(),
        environment(&configuration)?,
        limits,
        &[],
    )
    .map_err(debug)?;
    let source = RelativeSourcePath::informed(text(&configuration, "source")?).map_err(debug)?;
    let material = std::fs::read(directory.join(source.spelling())).map_err(debug)?;
    let selected_target = text(&configuration, "target")?;
    let compiler = CompilerRequest::rustc(&tool, source, artifact.clone(), selected_target)
        .map_err(debug)?
        .instrumented()
        .map_err(debug)?;
    let budgets = CoverageBudgets::declared(
        CaseBudget::declared(16),
        ByteBudget::declared(8192),
        1_048_576,
        10_000,
        CaseBudget::declared(8),
        ByteBudget::declared(4096),
    )
    .map_err(debug)?;
    let campaign = CoverageCampaign::declared(
        PopulationRef::named("example", "coverage-input").map_err(debug)?,
        RevisionBinding::derived(DerivedRevision::from_material(&material)),
        CoverageProfile::declared(
            NamespacedName::named("example", "source-lines").map_err(debug)?,
            1,
        ),
        budgets,
    );
    let coverage = RustcProfileRequest::declared(
        tool.executable().to_path_buf(),
        InstrumentedTarget::for_target(
            artifact,
            Vec::new(),
            TargetTriple::declared(selected_target),
        )
        .map_err(debug)?,
        CoverageSourceRoot::declared(
            NamespacedName::named("example", "subject").map_err(debug)?,
            directory,
        )
        .map_err(debug)?,
        PathBuf::from(text(&configuration, "scratch")?),
        campaign,
    )
    .map_err(debug)?;
    Ok(Settings {
        compiler,
        coverage,
        tool,
        candidates: candidates(&configuration)?,
    })
}

fn candidates(configuration: &Value) -> Result<Vec<Vec<u8>>, String> {
    let entries = configuration
        .get("candidates")
        .and_then(Value::as_array)
        .ok_or("candidates must be an array of byte arrays")?;
    if entries.is_empty() || entries.len() > 8 {
        return Err("supply between one and eight candidates".to_owned());
    }
    entries
        .iter()
        .map(|entry| {
            entry
                .as_array()
                .ok_or("each candidate must be a byte array")?
                .iter()
                .map(|value| {
                    u8::try_from(
                        value
                            .as_u64()
                            .ok_or("candidate member must be an unsigned byte")?,
                    )
                    .map_err(debug)
                })
                .collect()
        })
        .collect()
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
