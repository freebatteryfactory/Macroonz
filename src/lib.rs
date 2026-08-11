//! `ThreadPak` is a host-neutral semantic machine written in safe Rust. Programs are
//! typed data, not text: a builder constructs typed declarations, and the machine
//! validates, seals, executes, and remembers them. Any frontend enters through the
//! same public declaration path.
//!
//! The spine:
//!
//! ```text
//! typed declarations → Semantic Form → Execution Form → ProgramImage (.tpk)
//! → PakVM → runtime (the Stitch) → Bvisor → accepted history (.tlog)
//! ```
//!
//! Hosts live in other repositories and pin an exact `ThreadPak` revision. The machine
//! never knows which host is running it.
//!
//! The crate is a numbered waterfall of semantic homes (see the repository README's
//! band map); each home is declared here as it materializes, in dependency order.
//! The repository is in architecture closure: declaration surfaces and compile-time
//! laws are real code; machine runtime algorithms remain unopened until authorized.

pub mod types;

#[path = "00_refusal/mod.rs"]
pub mod refusal;

#[path = "01_logic/mod.rs"]
pub mod logic;

#[path = "02_identity/mod.rs"]
pub mod identity;

#[path = "03_value/mod.rs"]
pub mod value;

#[path = "04_numeric/mod.rs"]
pub mod numeric;

#[path = "05_bounds/mod.rs"]
pub mod bounds;

#[path = "06_authority/mod.rs"]
pub mod authority;

#[path = "07_bytes/mod.rs"]
pub mod bytes;

#[path = "08_schema/mod.rs"]
pub mod schema;

#[path = "09_time/mod.rs"]
pub mod time;

#[path = "10_history/mod.rs"]
pub mod history;

#[path = "11_navigation/mod.rs"]
pub mod navigation;

#[path = "12_port/mod.rs"]
pub mod port;

#[path = "13_declaration/mod.rs"]
pub mod declaration;

#[path = "14_semantic/mod.rs"]
pub mod semantic;

#[path = "15_execution/mod.rs"]
pub mod execution;

#[path = "16_image/mod.rs"]
pub mod image;

#[path = "17_pakvm/mod.rs"]
pub mod pakvm;

#[path = "18_bvisor/mod.rs"]
pub mod bvisor;

#[path = "19_runtime/mod.rs"]
pub mod runtime;

#[path = "20_derived/mod.rs"]
pub mod derived;

#[path = "21_application/mod.rs"]
pub mod application;

#[path = "22_security/mod.rs"]
pub mod security;

#[path = "23_evidence/mod.rs"]
pub mod evidence;

#[cfg(test)]
mod laws;
