//! Editable independent byte vectors for malformed-record and exact-encoding controls.

#[derive(Clone)]
pub(super) struct DeclarationVector {
    pub(super) sizes: Vec<u64>,
    pub(super) samples: u32,
    pub(super) warmups: u32,
    pub(super) ratio: (u64, u64),
    pub(super) contention: u8,
    pub(super) formula: Option<Vec<u8>>,
}

#[derive(Clone)]
pub(super) struct RowVector {
    pub(super) declaration: Vec<u8>,
    pub(super) target: Vec<u8>,
    pub(super) toolchain: Vec<u8>,
    pub(super) preflight: Vec<u8>,
    pub(super) stage: u8,
    pub(super) measured: Vec<u8>,
    pub(super) planted_worse: Vec<u8>,
    pub(super) judgment: Vec<u8>,
    pub(super) secondary: Vec<u8>,
}

#[derive(Clone)]
pub(super) struct ReportVector {
    pub(super) format: u32,
    pub(super) kind: u32,
    pub(super) custody: u32,
    pub(super) table: (Vec<u8>, Vec<u8>),
    pub(super) provenance: Vec<u8>,
    pub(super) rows: Vec<RowVector>,
}
