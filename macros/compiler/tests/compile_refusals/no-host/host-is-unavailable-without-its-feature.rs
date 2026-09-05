//! The compiler core does not expose the optional proc-host adapter.

use macroonz_compiler::host::Spans;

fn main() {
    let _ = core::mem::size_of::<Spans>();
}
