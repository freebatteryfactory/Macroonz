//! Band 17 — pakvm: the executor's value machine, memory model, live
//! handles, the continuation record, and the step machine.

pub mod types;

pub use types::{
    ArenaIndex, CLOSURE_OBLIGATIONS, CapabilityHandle, CaptureRecord, ContinuationRecord,
    INVALID_CAPTURES, LambdaBoundaryPosture, PROHIBITED_INHABITANTS, PortHandle, ReplyHandle,
    STEP_PRODUCTIONS, ValueCategory, ValueResidence, VmTerminal,
};
