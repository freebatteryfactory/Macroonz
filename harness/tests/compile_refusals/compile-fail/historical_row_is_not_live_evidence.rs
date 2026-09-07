//! Historical rows, origins and byte slices cannot mint live descriptors or admission.

use macroonz_harness::descriptor::archive::{ArchivedOrigin, ArchivedRow};
use macroonz_harness::descriptor::{CanonicalRowBytes, Origin, Row};

fn promote(historical: ArchivedRow) -> Row {
    historical
}

fn promote_origin(historical: ArchivedOrigin) -> Origin {
    historical
}

fn promote_bytes(historical: &ArchivedRow) -> &CanonicalRowBytes {
    historical.canonical_bytes()
}

fn main() {}
