use macroonz::harness::fuzz::RustcProfileRequest;
use macroonz::native_compiler::CompilerRequest;
use macroonz::native_process::ProcessTool;

pub(super) struct Settings {
    pub compiler: CompilerRequest,
    pub coverage: RustcProfileRequest,
    pub tool: ProcessTool,
    pub candidates: Vec<Vec<u8>>,
}
