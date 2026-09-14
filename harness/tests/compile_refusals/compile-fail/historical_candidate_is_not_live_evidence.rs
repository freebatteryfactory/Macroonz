//! Historical candidate data cannot mint live rows, canonical row evidence or static names.

use macroonz_harness::descriptor::archive::{ArchivedCandidate, ArchivedName};
use macroonz_harness::descriptor::{CanonicalRowBytes, NamespacedName, Row};

fn promote(historical: ArchivedCandidate) -> Row {
    historical
}

fn promote_name(historical: ArchivedName) -> NamespacedName {
    historical
}

fn promote_bytes(historical: &ArchivedCandidate) -> &CanonicalRowBytes {
    historical.canonical_bytes()
}

fn main() {}
