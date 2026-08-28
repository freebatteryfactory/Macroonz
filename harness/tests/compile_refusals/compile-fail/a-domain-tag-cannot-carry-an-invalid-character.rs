use macroonz_harness::identity::{DomainTag, IdentityProfileVersion};

const INVALID_DOMAIN: DomainTag =
    DomainTag::declared("invalid_tag", IdentityProfileVersion::declared(1));

fn main() {
    let _invalid = INVALID_DOMAIN;
}
