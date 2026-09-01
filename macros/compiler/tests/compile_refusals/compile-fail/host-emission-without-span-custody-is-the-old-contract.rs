//! Host emission cannot reproduce caller-authored source placement without the capture's span custody.

use macroonz_compiler::PartitionCargo;
use macroonz_compiler::host::{Emittable, emit};

struct EmptyEmission;

impl Emittable for EmptyEmission {
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo> {
        core::iter::empty()
    }
}

fn main() {
    let _emitted = emit(&EmptyEmission);
}
