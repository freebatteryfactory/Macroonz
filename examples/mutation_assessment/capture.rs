//! Explicit saved-result bytes and current typed checks over those bytes.

use super::{checks, types::CapturedResult};
use macroonz::harness::descriptor::{
    ExecutableAttachment, NamespacedName, Origin, Provenance, SynthesisFacts,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz::harness::report::TrialConclusion;
use macroonz::harness::runner::{Invocation, TrialBinding};
use std::cell::Cell;

pub(super) const LIMITS: InputLimits = InputLimits::declared(512, 40);

std::thread_local! {
    static DECODES: Cell<u32> = const { Cell::new(0) };
    static CHECKS: Cell<u32> = const { Cell::new(0) };
}

pub(super) fn decoder() -> Result<InputBinding<CapturedResult>, String> {
    let profile = InputProfile::declared(
        NamespacedName::named("nonzero", "captured-selected-result").map_err(super::debug)?,
        1,
        ContentAddress::derived(
            DomainTag::declared("nonzero-input", IdentityProfileVersion::declared(1)),
            b"source32-input-u32be-result-u32be",
        ),
    );
    Ok(InputBinding::declared(
        profile,
        checks::revision(b"source32-input-u32be-result-u32be-v1"),
        |bytes| {
            DECODES.set(DECODES.get().saturating_add(1));
            let mut source = [0u8; 32];
            source.copy_from_slice(bytes.bytes(32)?);
            let mut input = [0u8; 4];
            input.copy_from_slice(bytes.bytes(4)?);
            let mut meaning = [0u8; 4];
            meaning.copy_from_slice(bytes.bytes(4)?);
            Ok(CapturedResult {
                source,
                input: u32::from_be_bytes(input),
                meaning: u32::from_be_bytes(meaning),
            })
        },
    ))
}

pub(super) fn bound(
    bytes: &[u8],
    invocation: Invocation,
) -> Result<Invocation<BoundInput<CapturedResult>>, String> {
    let decoder = decoder()?;
    let input = decoder
        .decode(pack(decoder.profile(), bytes, LIMITS).map_err(super::debug)?)
        .map_err(super::debug)?;
    Ok(invocation.with_input(input))
}

pub(super) fn candidate() -> Result<TrialBinding<BoundInput<CapturedResult>>, String> {
    binding(
        "positive-input",
        candidate_origin()?,
        b"positive-input",
        strict,
    )
}

pub(super) fn weakened() -> Result<TrialBinding<BoundInput<CapturedResult>>, String> {
    binding(
        "positive-input",
        candidate_origin()?,
        b"positive-input-weakened-to-well-formed-v2",
        weak,
    )
}

pub(super) fn parent() -> Result<TrialBinding<BoundInput<CapturedResult>>, String> {
    binding("well-formed", Origin::HandWritten, b"well-formed", weak)
}

pub(super) fn observations() -> (u32, u32) {
    (DECODES.get(), CHECKS.get())
}

fn candidate_origin() -> Result<Origin, String> {
    Ok(Origin::Candidate(SynthesisFacts::Survivor(
        super::declaration::point()?,
    )))
}

fn strict(invocation: &Invocation<BoundInput<CapturedResult>>) -> TrialConclusion {
    CHECKS.set(CHECKS.get().saturating_add(1));
    checks::positive(invocation.input().value().meaning)
}

fn weak(invocation: &Invocation<BoundInput<CapturedResult>>) -> TrialConclusion {
    CHECKS.set(CHECKS.get().saturating_add(1));
    checks::well_formed(invocation.input().value().meaning)
}

fn binding(
    name: &'static str,
    origin: Origin,
    revision: &[u8],
    call: fn(&Invocation<BoundInput<CapturedResult>>) -> TrialConclusion,
) -> Result<TrialBinding<BoundInput<CapturedResult>>, String> {
    let row = checks::row(name, origin)?;
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        checks::revision(b"nonzero-v1"),
        checks::revision(revision),
        call,
    );
    TrialBinding::bound(row, attachment, Provenance::Unproduced).map_err(super::debug)
}
