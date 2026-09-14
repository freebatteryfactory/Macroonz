//! Bounded historical binding encoding without callable serialization.

use super::{
    ArchivedBinding, BindingArchiveLimits, BindingArchiveRefusal, CandidateArchiveRefusal,
    read_binding,
};
use crate::descriptor::{Binding, NamespacedName, Provenance, RevisionBinding, RevisionPosture};
use crate::identity::encode_bytes;

/// Retain the complete descriptor and attachment standing of an existing binding.
///
/// # Errors
///
/// Refuses independent byte, name/address and row-label ceilings before encoding.
pub fn retain_binding<Invocation, Conclusion>(
    binding: &Binding<Invocation, Conclusion>,
    limits: BindingArchiveLimits,
) -> Result<ArchivedBinding, BindingArchiveRefusal> {
    let total = binding_size(binding, limits)?;
    let mut bytes = Vec::with_capacity(total);
    bytes.extend_from_slice(&1u32.to_be_bytes());
    encode_bytes(binding.row().canonical_bytes().as_bytes(), &mut bytes);
    let attachment = binding.attachment();
    attachment.subject().name().encode_into(&mut bytes);
    attachment.check().name().encode_into(&mut bytes);
    write_revision(attachment.subject_revision(), &mut bytes);
    write_revision(attachment.check_revision(), &mut bytes);
    write_provenance(binding.provenance(), &mut bytes);
    read_binding(&bytes, limits)
}

pub(crate) fn write_provenance(provenance: Provenance, bytes: &mut Vec<u8>) {
    match provenance {
        Provenance::Unproduced => bytes.push(0),
        Provenance::Produced { producer, schema } => {
            bytes.push(1);
            producer.name().encode_into(bytes);
            encode_bytes(schema.address().as_bytes(), bytes);
        }
    }
}

pub(crate) fn binding_size<Invocation, Conclusion>(
    binding: &Binding<Invocation, Conclusion>,
    limits: BindingArchiveLimits,
) -> Result<usize, BindingArchiveRefusal> {
    use BindingArchiveRefusal::Canonical;
    if limits.field() < 32 {
        return Err(Canonical(CandidateArchiveRefusal::FieldTooLarge));
    }
    let row = binding.row();
    if row.roles().len() > limits.labels() || row.tags().len() > limits.labels() {
        return Err(Canonical(CandidateArchiveRefusal::TooManyLabels));
    }
    // The row reader checks every retained origin/name using the same limits before any caller encoder runs.
    super::retain_row(row, limits).map_err(BindingArchiveRefusal::Row)?;
    let attachment = binding.attachment();
    let producer = provenance_size(binding.provenance(), limits)?;
    let total = sum(&[
        110,
        row.canonical_bytes().as_bytes().len(),
        name_size(attachment.subject().name(), limits)?,
        name_size(attachment.check().name(), limits)?,
        producer,
    ])?;
    if total > limits.bytes() {
        return Err(Canonical(CandidateArchiveRefusal::BytesTooLarge));
    }
    Ok(total)
}

pub(crate) fn provenance_size(
    provenance: Provenance,
    limits: BindingArchiveLimits,
) -> Result<usize, BindingArchiveRefusal> {
    match provenance {
        Provenance::Unproduced => Ok(1),
        Provenance::Produced {
            producer,
            schema: _,
        } => {
            if limits.field() < 32 {
                return Err(BindingArchiveRefusal::Canonical(
                    CandidateArchiveRefusal::FieldTooLarge,
                ));
            }
            sum(&[41, name_size(producer.name(), limits)?])
        }
    }
}

fn name_size(
    name: NamespacedName,
    limits: BindingArchiveLimits,
) -> Result<usize, BindingArchiveRefusal> {
    let namespace = name.namespace().written().len();
    let stem = name.stem().written().len();
    if namespace > limits.field() || stem > limits.field() {
        return Err(BindingArchiveRefusal::Canonical(
            CandidateArchiveRefusal::FieldTooLarge,
        ));
    }
    sum(&[16, namespace, stem])
}

fn sum(parts: &[usize]) -> Result<usize, BindingArchiveRefusal> {
    parts.iter().try_fold(0usize, |total, part| {
        total
            .checked_add(*part)
            .ok_or(BindingArchiveRefusal::SizeOutsidePlatform)
    })
}

pub(crate) fn write_revision(binding: RevisionBinding, bytes: &mut Vec<u8>) {
    // A revision record is a framed address and a posture, itself framed for bounded reuse.
    bytes.extend_from_slice(&41u64.to_be_bytes());
    encode_bytes(binding.revision().as_bytes(), bytes);
    bytes.push(match binding.posture() {
        RevisionPosture::Derived => 0,
        RevisionPosture::Declared => 1,
        RevisionPosture::Untracked => 2,
    });
}
