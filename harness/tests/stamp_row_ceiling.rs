//! The closed-register stamp's row ceiling, driven from OUTSIDE the crate that
//! exports the stamp.
//!
//! # Positions
//!
//! The stamp admits a declaration that spends its declared supply of positions
//! to the last one, and answers an exact position for every row of it. The
//! positions come from the supply itself rather than from a running total, so
//! there is no arithmetic in the expansion to saturate: the first and last rows
//! pin both ends of the supply, and every row between them answers its own
//! place in `ALL`.
//!
//! # The outside seat
//!
//! This plane is a separate crate that depends on `macroonz` by path, so it is an ordinary outside consumer. It reaches `closed_register!` through the public export, with no crate-internal path available to it.
//!
//! The reversal for this claim lives beside this file as a compile-fail fixture,
//! so both roads of one obligation are read in one crate.
//!
//! # The ceiling
//!
//! Nothing below writes the ceiling down. The roster spends the supply, and the assertions compare against `macroonz::CLOSED_REGISTER_ROW_CEILING`, which is that supply's own length. Extending the supply therefore fails this file because the roster stops spending it.
//!
//! # Nonclaims
//!
//! This says the ceiling stands exactly where the stamp says it stands.
//! It says nothing about whether any vocabulary should approach it.
//! The outside compile refusal beside this lane is the reversal for the same boundary.

macroonz::closed_register! {
    /// A roster that spends the stamp's declared supply of positions to its
    /// last one.
    ///
    /// Synthetic and deliberately meaningless. It stands for no vocabulary —
    /// the rows are numbered because the only things under judgement here are
    /// how many rows the stamp admits and what position each of them answers.
    /// The rows carry no documentation of their own for the same reason: a
    /// sentence written once per row to satisfy a habit is a sentence nobody
    /// wrote.
    enum SpentSupply {
        Row00 = "row-00", "row 00";
        Row01 = "row-01", "row 01";
        Row02 = "row-02", "row 02";
        Row03 = "row-03", "row 03";
        Row04 = "row-04", "row 04";
        Row05 = "row-05", "row 05";
        Row06 = "row-06", "row 06";
        Row07 = "row-07", "row 07";
        Row08 = "row-08", "row 08";
        Row09 = "row-09", "row 09";
        Row10 = "row-10", "row 10";
        Row11 = "row-11", "row 11";
        Row12 = "row-12", "row 12";
        Row13 = "row-13", "row 13";
        Row14 = "row-14", "row 14";
        Row15 = "row-15", "row 15";
        Row16 = "row-16", "row 16";
        Row17 = "row-17", "row 17";
        Row18 = "row-18", "row 18";
        Row19 = "row-19", "row 19";
        Row20 = "row-20", "row 20";
        Row21 = "row-21", "row 21";
        Row22 = "row-22", "row 22";
        Row23 = "row-23", "row 23";
        Row24 = "row-24", "row 24";
        Row25 = "row-25", "row 25";
        Row26 = "row-26", "row 26";
        Row27 = "row-27", "row 27";
        Row28 = "row-28", "row 28";
        Row29 = "row-29", "row 29";
        Row30 = "row-30", "row 30";
        Row31 = "row-31", "row 31";
        Row32 = "row-32", "row 32";
        Row33 = "row-33", "row 33";
        Row34 = "row-34", "row 34";
        Row35 = "row-35", "row 35";
        Row36 = "row-36", "row 36";
        Row37 = "row-37", "row 37";
        Row38 = "row-38", "row 38";
        Row39 = "row-39", "row 39";
        Row40 = "row-40", "row 40";
        Row41 = "row-41", "row 41";
        Row42 = "row-42", "row 42";
        Row43 = "row-43", "row 43";
        Row44 = "row-44", "row 44";
        Row45 = "row-45", "row 45";
        Row46 = "row-46", "row 46";
        Row47 = "row-47", "row 47";
        Row48 = "row-48", "row 48";
        Row49 = "row-49", "row 49";
        Row50 = "row-50", "row 50";
        Row51 = "row-51", "row 51";
        Row52 = "row-52", "row 52";
        Row53 = "row-53", "row 53";
        Row54 = "row-54", "row 54";
        Row55 = "row-55", "row 55";
        Row56 = "row-56", "row 56";
        Row57 = "row-57", "row 57";
        Row58 = "row-58", "row 58";
        Row59 = "row-59", "row 59";
        Row60 = "row-60", "row 60";
        Row61 = "row-61", "row 61";
        Row62 = "row-62", "row 62";
        Row63 = "row-63", "row 63";
    }
}

/// The highest position the declared supply can pair a row with, derived from
/// the supply's own length rather than written down.
fn highest_position() -> u8 {
    u8::try_from(macroonz::CLOSED_REGISTER_ROW_CEILING.saturating_sub(1)).unwrap_or(u8::MAX)
}

/// The stamp admits a declaration that spends its declared supply of positions
/// and answers an exact position for every row of it.
///
/// The assertions are relational: the roster's length is held against the
/// supply's own length, and the last row's position against that length rather
/// than against a number. A roster shorter or longer than the supply fails the
/// first assertion, which is what makes an extended supply visible here instead
/// of silently accepted.
///
/// The declared name and the prose are read back on the last row as well: a
/// consumer that exercised only the roster and its positions would leave both
/// unreached at the one seat that reaches them from outside. What the stamp
/// generates in full is stated at the stamp's own documentation, and is neither
/// restated nor counted here.
#[test]
fn a_stamped_roster_declares_its_own_ceiling() {
    assert_eq!(
        SpentSupply::ALL.len(),
        macroonz::CLOSED_REGISTER_ROW_CEILING
    );

    let first = SpentSupply::ALL.first().copied();
    let last = SpentSupply::ALL.last().copied();
    assert_eq!(first.map(SpentSupply::slot), Some(0u8));
    assert_eq!(last.map(SpentSupply::slot), Some(highest_position()));

    assert!(
        SpentSupply::ALL
            .iter()
            .enumerate()
            .all(|(position, row)| usize::from(row.slot()) == position)
    );

    assert_eq!(last.map(SpentSupply::stable_name), Some("row-63"));
    assert_eq!(last.map(SpentSupply::described), Some("row 63"));
}
