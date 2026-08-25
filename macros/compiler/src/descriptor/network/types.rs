//! Every public type of the network declaration home, declared and nothing else.
//!
//! Construction and reading live in this module's own child `type_guard.rs`.

use crate::descriptor::{DirectBinding, HelperRefusal};

#[path = "type_guard.rs"]
mod guard;

/// Where this helper's family sits among the declaration helpers.
pub const NETWORK_HELPER_POSITION: u32 = 4;

/// One declared link: its spelling, and the two node spellings it joins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkRow {
    name: String,
    from: String,
    to: String,
}

/// One declared fault phrase, in the sim's own vocabulary — every number at exactly the width its harness seat declares.
///
/// A send ordinal and a tick span are thirty-two bits wide, a tick is sixty-four; a number past its seat refuses at capture, because generated code cannot outsource the range to rustc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaultRow {
    /// `drop <link> at <n>`.
    Drop {
        /// The send ordinal the fault fires on.
        at: u32,
    },
    /// `delay <link> at <n> by <n>`.
    Delay {
        /// The send ordinal the fault fires on.
        at: u32,
        /// How many ticks later the delivery comes due.
        by: u32,
    },
    /// `duplicate <link> at <n>`.
    Duplicate {
        /// The send ordinal the fault fires on.
        at: u32,
    },
    /// `partition <link> from <n> until <n>`.
    Partition {
        /// The first tick the interval covers.
        from: u64,
        /// The first tick past the interval.
        until: u64,
    },
}

/// One link's gathered phrases, in authored order.
///
/// The row carries the resolved link rather than its spelling, so the rendering never looks a spelling up again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisciplineRow {
    link: LinkRow,
    faults: Vec<FaultRow>,
}

/// One declared schedule: its spelling, and its disciplines in first-mention link order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleRow {
    name: String,
    disciplines: Vec<DisciplineRow>,
}

/// The complete payload one network declaration reads to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkDeclaration {
    harness: DirectBinding,
    module: String,
    namespace: String,
    nodes: Vec<String>,
    links: Vec<LinkRow>,
    schedules: Vec<ScheduleRow>,
}

/// What a network request produces: one direct declaration-site unit carrying the builder module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkModule;

/// How one network declaration was not read.
#[must_use = "a network capture refusal names the cause and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkCaptureError(HelperRefusal);
