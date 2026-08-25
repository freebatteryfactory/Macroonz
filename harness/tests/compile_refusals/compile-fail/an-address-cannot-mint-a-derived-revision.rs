use macroonz_harness::descriptor::RevisionBinding;
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};

const FOREIGN_DOMAIN: DomainTag =
    DomainTag::declared("foreign-revision", IdentityProfileVersion::declared(1));

fn main() {
    let address = ContentAddress::derived(FOREIGN_DOMAIN, b"caller-made");
    let _binding = RevisionBinding::derived(address);
}
