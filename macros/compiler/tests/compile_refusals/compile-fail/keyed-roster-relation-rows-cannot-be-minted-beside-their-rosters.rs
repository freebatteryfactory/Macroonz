//! Referenced roster rows can be minted only by the constructor that resolves every offered key against both declared rosters.

use macroonz_compiler::{KeyedRoster, KeyedRosterRows};

fn inaccessible_base<'rosters>() -> KeyedRosterRows<'rosters, u8, u8, u8, u8, (), 1, 1, 0> {
    panic!("the private fields prevent any caller-provided base")
}

fn main() {
    let roster = KeyedRoster::one(1_u8, 1_u8);
    let _forged = KeyedRosterRows::<u8, u8, u8, u8, (), 1, 1, 0> {
        left: &roster,
        right: &roster,
        ..inaccessible_base()
    };
}
