//! Recipe behavior observed through callable, wrapper, projector, and refusal claims.

mod caller_rust;
mod codec_projection;
mod diagnostic_custody;
mod diagnostic_matrix;
mod dispatch_contract;
mod evidence_contract;
mod generic_account;
mod host_parity;
mod limit_contract;
mod maximal_recipe;
mod projector_authority;
mod readback_contract;
mod refusal_contract;
mod structural_refusals;
mod support;
#[path = "../support/captured_tokens.rs"]
mod captured_tokens;

use support::{
    CALLER_OWNED_TRIAL_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, CallerOwnedTrials, DOOR,
    EVIDENCE_RECIPE, EXACT_DISPATCH_RECIPE, EXACT_EFFECT_RECIPE, TARGET_UNAVAILABLE_RECIPE, bake,
    cargo_bytes, emitted_bytes, refusal_summary,
};
