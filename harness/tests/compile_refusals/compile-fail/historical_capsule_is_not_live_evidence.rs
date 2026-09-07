//! Loaded historical claims cannot be used as an earned replay capsule.

use macroonz_harness::report::ReplayCapsule;
use macroonz_harness::report::archive::ArchivedCapsule;
use macroonz_harness::report::archive::AddressClaim;
use macroonz_harness::identity::ContentAddress;

fn promote(historical: ArchivedCapsule) -> ReplayCapsule {
    historical
}

fn mint(claim: AddressClaim) -> ContentAddress {
    claim
}

fn main() {}
