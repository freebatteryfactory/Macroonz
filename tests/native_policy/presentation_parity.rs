//! Recorded parity dispositions and caller bytes without new evaluation or encoding.

use crate::presentation_formats::{field, parsed};
use macroonz::harness::clock::HarnessClock;
use macroonz::harness::descriptor::archive::BindingArchiveLimits;
use macroonz::harness::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    NamespacedName, Origin, PopulationRef, Provenance, RevisionBinding, Row, SubjectRoute,
};
use macroonz::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz::harness::muterprater::interpretation_archive::{
    ParityArchiveLimits, ValueEncoder, retain_parity, retain_parity_standing,
};
use macroonz::harness::muterprater::{
    EvaluationBinding, EvaluationCall, EvaluationFamilyRef, EvaluationObservation, EvaluationPair,
    EvaluationSurface, MutationPolicy, MutationWitness, NoMutationParityReading, ProductionBinding,
    discover::lower_discoveries,
    interpret::{observe_no_mutation, qualify_no_mutation},
};
use macroonz::harness::properties::Agreement;
use macroonz::harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, FindingLocation, ForeignText,
    InvocationProfile, TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion,
    TrialFinding, TrialSite, archive::ArchiveLimits,
};
use macroonz::harness::runner::{Invocation, TrialBinding};
use macroonz::presentation;
use serde_json::{Value, json};
use std::cell::Cell;

pub(super) const LIMITS: ParityArchiveLimits = ParityArchiveLimits::declared(
    ArchiveLimits::declared(65_536, 16_384),
    BindingArchiveLimits::declared(8192, 1024, 8),
    ArchiveLimits::declared(8192, 4096),
    128,
    8,
);
const TAG: DomainTag = DomainTag::declared(
    "presentation-parity-control",
    IdentityProfileVersion::declared(1),
);

pub(super) struct Datum {
    byte: Cell<u8>,
    encodes: Cell<u32>,
}

pub(super) fn datum(byte: u8) -> Datum {
    Datum {
        byte: Cell::new(byte),
        encodes: Cell::new(0),
    }
}

pub(super) fn production(input: &Datum) -> Datum {
    datum(input.byte.get())
}

fn encoded(value: &Datum) -> Vec<u8> {
    value.encodes.set(value.encodes.get().saturating_add(1));
    if value.byte.get() == 0 {
        Vec::new()
    } else {
        vec![255, value.byte.get()]
    }
}

pub(super) fn encoder(name: &'static str) -> Result<ValueEncoder<Datum>, String> {
    Ok(ValueEncoder::declared(
        mapped(NamespacedName::named("parity-encoding", name))?,
        3,
        ContentAddress::derived(TAG, name.as_bytes()),
        RevisionBinding::declared(ContentAddress::derived(TAG, b"encoder")),
        |value| Ok(encoded(value)),
    ))
}

pub(super) fn mapped<T>(result: Result<T, impl std::fmt::Debug>) -> Result<T, String> {
    result.map_err(|error| format!("{error:?}"))
}

fn pair(call: EvaluationCall<Datum, Datum>) -> Result<EvaluationPair<Datum, Datum>, String> {
    let family = mapped(EvaluationFamilyRef::named("parity-display", "byte"))?;
    let policy = mapped(MutationPolicy::declared(family, Vec::new()))?;
    let (_, surface) = mapped(lower_discoveries(&policy, Vec::new()))?.into_parts();
    pair_for(&surface, call)
}

pub(super) fn pair_for(
    surface: &EvaluationSurface,
    call: EvaluationCall<Datum, Datum>,
) -> Result<EvaluationPair<Datum, Datum>, String> {
    mapped(EvaluationPair::paired(
        ProductionBinding::declared(
            surface.family(),
            RevisionBinding::declared(ContentAddress::derived(TAG, b"production")),
            production,
        ),
        EvaluationBinding::declared(
            surface,
            RevisionBinding::untracked(ContentAddress::derived(TAG, b"evaluation")),
            call,
        ),
        |left, right| {
            if left.byte.get() == right.byte.get() {
                Agreement::Agrees
            } else {
                Agreement::Differs
            }
        },
    ))
}

pub(super) fn binding() -> Result<TrialBinding, String> {
    let row = mapped(Row::declared(
        mapped(ClaimRef::named("parity-display", "byte-contract"))?,
        mapped(ExecutionSuite::named("parity-display", "ordinary"))?,
        mapped(Classification::authored(Vec::new(), Vec::new()))?,
        mapped(SubjectRoute::named("parity-display", "production"))?,
        mapped(CheckRef::named("parity-display", "odd"))?,
        mapped(PopulationRef::named("parity-display", "declared"))?,
        Origin::HandWritten,
    ))?;
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        RevisionBinding::declared(ContentAddress::derived(TAG, b"subject-attachment")),
        RevisionBinding::declared(ContentAddress::derived(TAG, b"check-attachment")),
        |_invocation: &Invocation| TrialConclusion::Passed,
    );
    mapped(Binding::bound(row, attachment, Provenance::Unproduced))
}

fn reading<'pair, 'input>(
    pair: &'pair EvaluationPair<Datum, Datum>,
    input: &'input Datum,
    check: fn(&Datum) -> TrialConclusion,
) -> Result<NoMutationParityReading<'pair, 'input, Datum, Datum>, String> {
    let binding = binding()?;
    let check_ref = binding.row().check();
    let witness = mapped(MutationWitness::bound(binding, check_ref, check))?;
    mapped(observe_no_mutation(pair, witness, input, &invocation()))
}

pub(super) fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(128),
            TimeBudget::declared(0),
        ),
        TargetBinding::bound(
            TargetTriple::declared("explicit-parity-target"),
            ToolchainIdentity::declared("explicit-parity-tool"),
        ),
        TrialSite::located("outside", "parity.rs", 17, "both roads"),
        HarnessClock::unavailable(),
    )
}

const EVALUATE: EvaluationCall<Datum, Datum> =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 0));
const DIFFER: EvaluationCall<Datum, Datum> = |input, _directive| {
    Ok(EvaluationObservation::observed(
        datum(input.byte.get().wrapping_add(1)),
        0,
    ))
};
const FIRE: EvaluationCall<Datum, Datum> =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 7));
const PASS: fn(&Datum) -> TrialConclusion = |_value| TrialConclusion::Passed;

pub(super) fn odd(value: &Datum) -> TrialConclusion {
    if value.byte.get().is_multiple_of(2) {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::RefusedByCheck,
            FindingCause::named("independent", "even"),
            FindingLocation::at("witness.rs", 29),
            Some(ForeignText::admitted(b"<even>| refused")),
        ))
    } else {
        TrialConclusion::Passed
    }
}

#[test]
fn parity_presentation_keeps_raw_qualified_and_every_rejected_standing() -> Result<(), String> {
    for (evaluation, input, check, disposition, production_outcome, evaluation_outcome) in [
        (
            EVALUATE,
            1u8,
            PASS,
            json!({"kind":"qualified", "value":null}),
            "passed",
            "passed",
        ),
        (
            DIFFER,
            2,
            odd,
            json!({"kind":"rejected", "value":{"kind":"production-did-not-qualify", "value":null}}),
            "refused",
            "passed",
        ),
        (
            DIFFER,
            1,
            odd,
            json!({"kind":"rejected", "value":{"kind":"evaluation-did-not-qualify", "value":null}}),
            "passed",
            "refused",
        ),
        (
            FIRE,
            1,
            PASS,
            json!({"kind":"rejected", "value":{"kind":"no-mutation-activated", "value":7u32}}),
            "passed",
            "passed",
        ),
        (
            DIFFER,
            1,
            PASS,
            json!({"kind":"rejected", "value":{"kind":"meanings-disagreed", "value":null}}),
            "passed",
            "passed",
        ),
    ] {
        observe_disposition(
            evaluation,
            input,
            check,
            &disposition,
            production_outcome,
            evaluation_outcome,
        )?;
    }
    Ok(())
}

fn observe_disposition(
    evaluation: EvaluationCall<Datum, Datum>,
    input: u8,
    check: fn(&Datum) -> TrialConclusion,
    disposition: &Value,
    production_outcome: &str,
    evaluation_outcome: &str,
) -> Result<(), String> {
    let pair = pair(evaluation)?;
    let input = datum(input);
    let reading = reading(&pair, &input, check)?;
    let raw = mapped(retain_parity(
        &reading,
        &encoder("input")?,
        &encoder("meaning")?,
        LIMITS,
    ))?;
    let raw = parsed(&presentation::archived_parity(&raw))?;
    assert_eq!(
        field(&raw, "/record/disposition")?,
        &json!({"kind":"raw", "value":null})
    );
    let standing = qualify_no_mutation(reading);
    let archive = mapped(retain_parity_standing(
        &standing,
        &encoder("input")?,
        &encoder("meaning")?,
        LIMITS,
    ))?;
    let shown = parsed(&presentation::archived_parity(&archive))?;
    assert_eq!(field(&shown, "/standing")?, "historical-unauthenticated");
    assert_eq!(field(&shown, "/record/disposition")?, disposition);
    assert_eq!(
        field(&shown, "/record/production_report/attempt/value/kind")?,
        production_outcome
    );
    assert_eq!(
        field(&shown, "/record/evaluation_report/attempt/value/kind")?,
        evaluation_outcome
    );
    for field_name in [
        "production_report",
        "evaluation_report",
        "witness",
        "conclusion",
        "substrate",
        "input",
        "production",
        "evaluation",
        "pair",
    ] {
        let path = format!("/record/{field_name}");
        assert_eq!(field(&raw, &path)?, field(&shown, &path)?);
    }
    if evaluation_outcome == "refused" {
        assert_eq!(
            field(
                &shown,
                "/record/evaluation_report/attempt/value/value/foreign/shown"
            )?,
            "<even>| refused"
        );
    }
    Ok(())
}

#[test]
fn parity_display_uses_retained_bytes_and_conventions_without_new_encoder_calls()
-> Result<(), String> {
    for byte in [0u8, 13] {
        let pair = pair(EVALUATE)?;
        let input = datum(byte);
        let reading = reading(&pair, &input, PASS)?;
        let archive = mapped(retain_parity(
            &reading,
            &encoder("input")?,
            &encoder("meaning")?,
            LIMITS,
        ))?;
        for value in [&input, reading.production(), reading.evaluation()] {
            assert_eq!(value.encodes.get(), 1);
            value.byte.set(91);
        }
        let shown = parsed(&presentation::archived_parity(&archive))?;
        for (role, convention) in [
            ("input", "input"),
            ("production", "meaning"),
            ("evaluation", "meaning"),
        ] {
            let path = format!("/record/{role}");
            assert_eq!(
                field(&shown, &format!("{path}/bytes"))?,
                if byte == 0 { "" } else { "ff0d" }
            );
            assert_eq!(
                field(&shown, &format!("{path}/convention/name/stem"))?,
                convention
            );
            assert_eq!(
                field(&shown, &format!("{path}/convention/version"))?,
                &json!(3u32)
            );
        }
        for value in [&input, reading.production(), reading.evaluation()] {
            assert_eq!(value.encodes.get(), 1);
        }
        assert_ne!(
            field(&shown, "/record/pair/production_revision")?,
            field(&shown, "/record/witness/subject_revision")?
        );
        assert_eq!(field(&shown, "/record/witness/row/roles")?, &json!([]));
        assert_eq!(
            field(&shown, "/record/witness/row/origin")?,
            &json!({"kind":"hand-written", "value":null})
        );
        assert_eq!(
            field(&shown, "/record/pair/evaluation_revision/posture")?,
            "unavailable-because-untracked"
        );
        assert_eq!(field(&shown, "/record/conclusion/value")?, &Value::Null);
    }
    Ok(())
}
