//! One row's canonical preimage and the identity derived over it.

use super::{BENCH_ROW_KEY_TAG, BenchMeasurement, BenchReferences, BenchRowKey, ContentionPosture};
use crate::descriptor::EncodeRefusal;
use crate::descriptor::encode::{encode_declared_bytes, encode_declared_length};
use crate::identity::ContentAddress;

/// Derive one row's identity over the preimage the parent bench README spells out.
pub(super) fn derive_row_key(
    references: BenchReferences,
    measurement: &BenchMeasurement,
) -> Result<BenchRowKey, EncodeRefusal> {
    let mut preimage = Vec::new();
    references.workload().name().encode_into(&mut preimage);

    let sizes = measurement.input_sizes().sizes();
    encode_declared_length(sizes.len(), &mut preimage)?;
    for size in sizes {
        preimage.extend_from_slice(&size.to_be_bytes());
    }

    references.preflight().name().encode_into(&mut preimage);
    references.planted_worse().name().encode_into(&mut preimage);

    let budgets = measurement.budgets();
    preimage.extend_from_slice(&budgets.samples().to_be_bytes());
    preimage.extend_from_slice(&budgets.warmups().to_be_bytes());
    preimage.extend_from_slice(&budgets.ratio().numerator().to_be_bytes());
    preimage.extend_from_slice(&budgets.ratio().denominator().to_be_bytes());

    match measurement.contention() {
        ContentionPosture::NoDeclaredContention => preimage.push(0u8),
    }
    match measurement.formula() {
        None => preimage.push(0u8),
        Some(formula) => {
            preimage.push(1u8);
            encode_declared_bytes(formula.bytes(), &mut preimage)?;
        }
    }

    references.complexity().name().encode_into(&mut preimage);
    Ok(BenchRowKey::derived(ContentAddress::derived(
        BENCH_ROW_KEY_TAG,
        &preimage,
    )))
}
