//! Ordinary consuming adoption and Rust ownership refusals through a renamed facade.

use super::support::{observe_consuming, observed_in_scratch};

#[test]
fn consuming_adoption_preserves_move_privacy_payload_and_borrow_boundaries() -> Result<(), String> {
    observed_in_scratch(observe_consuming)
}
