//! One row's canonical preimage, and the identity derived over it.

use super::types::{
    BENCH_ROW_KEY_TAG, BenchMeasurement, BenchReferences, BenchRowKey, ContentionPosture,
};
use crate::descriptor::{EncodeRefusal, NamespacedName};
use crate::identity::ContentAddress;

/// Append one length-prefixed byte string: eight big-endian length bytes, then the material.
fn write_bytes(into: &mut Vec<u8>, material: &[u8]) -> Result<(), EncodeRefusal> {
    let length =
        u64::try_from(material.len()).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    into.extend_from_slice(&length.to_be_bytes());
    into.extend_from_slice(material);
    Ok(())
}

/// Append one name: its namespace, then its stem.
fn write_name(into: &mut Vec<u8>, name: NamespacedName) -> Result<(), EncodeRefusal> {
    write_bytes(into, name.namespace().written().as_bytes())?;
    write_bytes(into, name.stem().written().as_bytes())
}

/// Derive one row's identity over the preimage this home's README spells out.
pub(super) fn derive_row_key(
    references: BenchReferences,
    measurement: &BenchMeasurement,
) -> Result<BenchRowKey, EncodeRefusal> {
    let mut preimage = Vec::new();
    write_name(&mut preimage, references.workload().name())?;

    let sizes = measurement.input_sizes().sizes();
    let count = u64::try_from(sizes.len()).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    preimage.extend_from_slice(&count.to_be_bytes());
    for size in sizes {
        preimage.extend_from_slice(&size.to_be_bytes());
    }

    write_name(&mut preimage, references.preflight().name())?;
    write_name(&mut preimage, references.planted_worse().name())?;

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
            write_bytes(&mut preimage, formula.bytes())?;
        }
    }

    write_name(&mut preimage, references.complexity().name())?;
    Ok(BenchRowKey::derived(ContentAddress::derived(
        BENCH_ROW_KEY_TAG,
        &preimage,
    )))
}
