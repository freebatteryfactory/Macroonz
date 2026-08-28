//! A composition refusal can be read outside its owner but cannot be minted beside the guard that establishes it.
//!
//! The issue vocabulary is public so a caller can match an answer, while the error body is private so no caller can make that answer appear to have come from the composition declaration pass.

use macroonz_compiler::descriptor::{CompositionError, CompositionIssue, DeclarationError, Seat};

fn main() {
    let _minted = CompositionError::established(vec![CompositionIssue::Declaration {
        refusal: DeclarationError::Absent {
            seat: Seat::Provider,
        },
    }]);
}
