use super::ReadBackError;
use crate::harness::oracle::{
    CompiledObservation, CompiledVerdict, DeclaredBehavior, ObservedMember,
};
use crate::native_process::{ProcessOutput, ProcessStop};

/// Decodes one successful native reader's stdout into a caller-owned value.
///
/// # Errors
/// Refuses interrupted execution or a nonzero exit before decoding, then preserves decoder refusal.
pub fn observed_read_back<Decoded>(
    output: &ProcessOutput,
    decode: impl FnOnce(&[u8]) -> Result<Decoded, String>,
) -> Result<Decoded, ReadBackError> {
    if output.stop() != &ProcessStop::Exited {
        return Err(ReadBackError::Interrupted(output.stop().clone()));
    }
    if !output.status().success() {
        return Err(ReadBackError::ProcessFailure);
    }
    decode(output.stdout().bytes()).map_err(ReadBackError::Decode)
}

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
    let members = observed_read_back(output, decode)?;
    Ok(crate::harness::oracle::compiled::compared(
        &CompiledObservation::ReadBack(members),
        declared,
    ))
}
