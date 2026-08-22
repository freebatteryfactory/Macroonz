//! A production binding has no mutation-control seat.
//!
//! Production and evaluation remain separate call paths. `NoMutation` and active selections exist only on the evaluation copy, so adding a selector to ordinary production is not expressible through this API.

use threadpak_testpak::muterprater::{EvaluationControl, ProductionBinding};

fn production_cannot_receive_control(
    binding: &ProductionBinding<u8, u8>,
    control: EvaluationControl,
) {
    let _ = binding.evaluate(&0, control);
}

fn main() {}
