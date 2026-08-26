//! Claim: A campaign selection can be minted only by the campaign that declares the borrowed schedule.
//!
//! Subject: The private campaign-selection seat at the public crate boundary.
//! Population: One lawful schedule offered through a direct selection literal.
//! Hostile control: The fixture supplies the schedule with its correct borrowed type while bypassing `FaultCampaign::select`.
//! Denominator: The only private field whose construction binds a selection to its campaign member.
//! Evidence ceiling: Compiler privacy proves outside unwritability under Rust 1.98 only.
//! Retained regression: The selection field becoming externally writable remains a permanent compile-refusal regression.

use macroonz_harness::fault::{CampaignSelection, FaultSchedule};

fn bypass_campaign(
    schedule: &FaultSchedule<(), ()>,
) -> CampaignSelection<'_, (), ()> {
    CampaignSelection { schedule }
}

fn main() {}
