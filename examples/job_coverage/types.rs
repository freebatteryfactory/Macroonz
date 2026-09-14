use macroonz::harness::fuzz::CoverageCampaign;
use macroonz::harness::report::TargetTriple;
use macroonz::native_compiler::CompilerRequest;
use macroonz::native_process::ProcessTool;
use std::path::PathBuf;

pub(super) struct Settings {
    pub compiler: CompilerRequest,
    pub tool: ProcessTool,
    pub scratch: PathBuf,
    pub target: TargetTriple,
    pub campaign: CoverageCampaign,
}
