//! The receipt-rich road's first unwritable road: a receipt nobody proved.
//!
//! `ClosedExpansion` is the only public value in the services that carries a
//! token tree an expansion may emit, and it has exactly one constructor. That
//! constructor is crate-internal, so the whole road — capture, plan, origin
//! graph, trace, rendering, closure, explanation — is not a sequence a caller is
//! trusted to follow. It is the only way to hold the value at all.
//!
//! **One fixture covers every seat that dies here.** Deleting the plan, the
//! origin graph, the trace, the invalidation set, or the explanation is the same
//! unwritable move as deleting the closure: each of them is a parameter of this
//! one constructor, and a caller who cannot reach the constructor cannot omit
//! one of its arguments. A second fixture per seat would prove the same
//! privacy three more times.
//!
//! No value is constructed below. Naming the constructor is the proof.

use threadpak_macroc::ClosedExpansion;

fn main() {
    let _ = ClosedExpansion::bound;
}
