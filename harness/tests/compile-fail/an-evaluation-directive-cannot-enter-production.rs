//! A production binding has no evaluation-directive seat.
//!
//! Production and evaluation remain separate call paths. The no-mutation posture and active selections exist only on the evaluation callable, so adding a directive to ordinary production is not expressible through this API.

use threadpak_testpak::muterprater::{EvaluationDirective, ProductionBinding};

fn production_cannot_receive_directive(
    binding: &ProductionBinding<u8, u8>,
    directive: EvaluationDirective<'_>,
) {
    let _ = binding.evaluate(&0, directive);
}

fn main() {}
