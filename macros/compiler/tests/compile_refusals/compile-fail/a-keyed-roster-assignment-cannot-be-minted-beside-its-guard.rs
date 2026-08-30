//! A keyed-roster assignment can be minted only by the constructor that proves exact denominator alignment and unique payload seats.

use macroonz_compiler::{KeyedRoster, KeyedRosterAssignment};

fn main() {
    let _assignment = KeyedRosterAssignment::<u8, &'static str, u8, &'static str, 1> {
        denominator: KeyedRoster::one(1, "one"),
        payloads: KeyedRoster::one(2, "seat"),
    };
}
