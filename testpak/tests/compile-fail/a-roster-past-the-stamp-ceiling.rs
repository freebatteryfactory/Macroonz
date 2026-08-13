//! The reversal for the closed-register stamp's row ceiling: a declaration past
//! the stamp's declared supply of positions refuses with the STAMP'S OWN
//! sentence.
//!
//! The stamp carries sixty-four positions written out as literals and pairs each
//! row of a declaration with exactly one of them. That supply is what makes the
//! ceiling a statement rather than an accident: the walk over the rows stops at
//! the sixty-fifth step whatever the declaration's length, so the refusal below
//! is reached before any recursion limit is, and it is reached identically by a
//! sixty-five-row roster and by a two-hundred-row one.
//!
//! What this file rules out is the older failure it replaces. A muncher that
//! counted its way along the rows ran until the compiler's recursion limit
//! stopped it, and what an author saw was a diagnostic about the stamp's own
//! internals at a boundary nobody had declared. The sixty-fifth row below must
//! produce the stamp's sentence, naming the ceiling and the mechanism the
//! ceiling is the length of.
//!
//! Sixty-four is this stamp implementation's authoring profile, not a semantic
//! cap on any vocabulary.

threadpak::closed_register! {
    /// Sixty-five rows: the declared supply, and one row past it.
    pub enum PastTheCeiling {
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
        Row64 = "row-64", "row 64";
    }
}

fn main() {
    let _ = PastTheCeiling::Row00.slot();
}
