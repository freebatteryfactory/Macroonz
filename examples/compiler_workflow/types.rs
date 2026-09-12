use macroonz::native_compiler::CompilerRequest;
use macroonz::native_process::ProcessRequest;

pub(super) struct Settings {
    pub compiler: CompilerRequest,
    pub reader: ProcessRequest,
    pub expected_count: u64,
}
