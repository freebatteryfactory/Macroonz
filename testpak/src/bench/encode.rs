//! Canonical encoding of one benchmark row's eight declared facts.

use super::types::{BENCH_ROW_KEY_TAG, BenchMeasurement, BenchReferences, BenchRowKey};
use crate::descriptor::{EncodeRefusal, NamespacedName};
use crate::identity::ContentAddress;

fn write_bytes(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), EncodeRefusal> {
    let length = u64::try_from(bytes.len()).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn write_name(output: &mut Vec<u8>, name: NamespacedName) -> Result<(), EncodeRefusal> {
    write_bytes(output, name.namespace().written().as_bytes())?;
    write_bytes(output, name.stem().written().as_bytes())
}

pub(super) fn derive_row_key(
    references: BenchReferences,
    measurement: &BenchMeasurement,
) -> Result<BenchRowKey, EncodeRefusal> {
    let mut preimage = Vec::new();
    write_name(&mut preimage, references.workload().name())?;
    let sizes = measurement.input_sizes().sizes();
    let size_count =
        u64::try_from(sizes.len()).map_err(|_| EncodeRefusal::LengthPastEncodingWidth)?;
    preimage.extend_from_slice(&size_count.to_be_bytes());
    for size in sizes {
        preimage.extend_from_slice(&size.to_be_bytes());
    }
    write_name(&mut preimage, references.preflight().name())?;
    write_name(&mut preimage, references.planted_worse().name())?;
    preimage.extend_from_slice(&measurement.budgets().samples().to_be_bytes());
    preimage.extend_from_slice(&measurement.budgets().warmups().to_be_bytes());
    preimage.extend_from_slice(&measurement.budgets().ratio().numerator().to_be_bytes());
    preimage.extend_from_slice(&measurement.budgets().ratio().denominator().to_be_bytes());
    match measurement.contention() {
        super::ContentionPosture::NoDeclaredContention => preimage.push(0u8),
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
