//! Recipe behavior observed through callable, wrapper, projector, and refusal claims.

mod caller_rust;
mod dispatch_contract;
mod evidence_contract;
mod host_parity;
mod projector_authority;
mod refusal_contract;
mod structural_refusals;
mod support;

use support::{
    CALLER_OWNED_TRIAL_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, CallerOwnedTrials, DOOR,
    EVIDENCE_RECIPE, EXACT_DISPATCH_RECIPE, TARGET_UNAVAILABLE_RECIPE, bake, cargo_bytes,
    emitted_bytes, refusal_summary,
};
