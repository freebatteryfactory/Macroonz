use super::ReadBackError;
use crate::harness::oracle::{
    CompiledObservation, CompiledVerdict, DeclaredBehavior, ObservedMember,
};
use crate::native_process::{ProcessOutput, ProcessStop};

/// Decodes an actual successful reader's stdout and delegates member judgment to the existing oracle.
///
/// The caller owns value decoding and independently declares expected members.
///
/// # Errors
/// Refuses interrupted execution, a nonzero reader exit or a failed value decoder.
pub fn compared_read_back(
    output: &ProcessOutput,
    decode: impl FnOnce(&[u8]) -> Result<Vec<ObservedMember>, String>,
    declared: &DeclaredBehavior<'_>,
) -> Result<CompiledVerdict, ReadBackError> {
    if output.stop() != &ProcessStop::Exited {
        return Err(ReadBackError::Interrupted(output.stop().clone()));
    }
    if !output.status().success() {
        return Err(ReadBackError::ProcessFailure);
    }
    let members = decode(output.stdout().bytes()).map_err(ReadBackError::Decode)?;
    Ok(crate::harness::oracle::compiled::compared(
        &CompiledObservation::ReadBack(members),
        declared,
    ))
}
