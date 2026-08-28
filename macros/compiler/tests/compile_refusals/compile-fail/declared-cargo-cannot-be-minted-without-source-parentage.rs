//! Declared cargo cannot be minted without a proving terminal and declaration parentage.
//!
//! Matcher and stamped trees alone do not establish which closed expansion proved the declaration-site delivery.

use macroonz_compiler::GeneratedTree;
use macroonz_compiler::support::DeclaredCargo;

fn bypass(matched: GeneratedTree, stamped: GeneratedTree) -> DeclaredCargo {
    DeclaredCargo::declared(matched, stamped)
}

fn main() {}
