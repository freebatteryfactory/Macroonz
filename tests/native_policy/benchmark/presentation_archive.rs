//! Nested benchmark archive causes keep their owned coordinates and full-width numbers.

use crate::presentation_formats::{field, parsed};
use macroonz::harness::bench::{InputSizeAxis, archive::BenchArchiveRefusal};
use macroonz::harness::descriptor::archive::{
    BindingArchiveRefusal, CandidateArchiveRefusal, RowArchiveRefusal,
};
use macroonz::presentation;
use serde_json::json;

#[test]
fn presentation_benchmark_archive_preserves_nested_axis_positions_and_provenance_slots()
-> Result<(), String> {
    let axis = InputSizeAxis::declared(vec![0, u64::MAX, u64::MAX])
        .err()
        .ok_or("duplicate axis admitted")?;
    let error = BenchArchiveRefusal::Axis(axis);
    let shown = parsed(&presentation::benchmark_archive_refusal(&error))?;
    assert_eq!(
        field(&shown, "/record/value/value")?,
        &json!({"size":u64::MAX,"first":1_usize,"duplicate":2_usize})
    );
    for (reason, expected) in [
        (
            BindingArchiveRefusal::Canonical(CandidateArchiveRefusal::LengthOutsidePlatform {
                declared: u64::MAX,
            }),
            json!({"kind":"canonical","value":{"kind":"length-outside-platform","value":u64::MAX}}),
        ),
        (
            BindingArchiveRefusal::Row(RowArchiveRefusal::InvalidOrigin { found: u8::MAX }),
            json!({"kind":"row","value":{"kind":"invalid-origin","value":u8::MAX}}),
        ),
    ] {
        let record = BenchArchiveRefusal::Provenance(reason);
        let projected = parsed(&presentation::benchmark_archive_refusal(&record))?;
        assert_eq!(field(&projected, "/record/kind")?, "provenance");
        assert_eq!(field(&projected, "/record/value")?, &expected);
    }
    Ok(())
}
