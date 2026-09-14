use macroonz::native_process::ProcessTool;
use std::cell::Cell;
use std::path::PathBuf;

pub(super) struct Settings {
    pub compiler: ProcessTool,
    pub target: String,
    pub toolchain: String,
    pub storage: PathBuf,
}

pub(super) struct Sample<'settings> {
    pub value: u32,
    pub settings: &'settings Settings,
    pub evaluations: Cell<u32>,
    pub executions: Cell<u32>,
}

pub(super) struct CapturedResult {
    pub source: [u8; 32],
    pub input: u32,
    pub meaning: u32,
}
