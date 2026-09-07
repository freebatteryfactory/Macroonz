//! Live retention observes callback counts, actual dispositions and temporal limits.

use super::support::{REVISION_TAG, check_ref, family, foreign_invocation, policy, trial_binding};
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::descriptor::{NamespacedName, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::interpret::{observe_no_mutation, qualify_no_mutation};
use macroonz_harness::muterprater::interpretation_archive::{
    ArchivedParityDisposition, ArchivedValueConvention, ParityArchiveLimits, ParityArchiveRefusal,
    ValueEncoder, ValueEncodingRefusal, ValueRole, read_parity, retain_parity,
    retain_parity_standing,
};
use macroonz_harness::muterprater::{
    EvaluationBinding, EvaluationCall, EvaluationObservation, EvaluationPair, MutationWitness,
    NoMutationParityReading, ParityQualificationRefusal, ProductionBinding,
};
use macroonz_harness::properties::Agreement;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal, retain_trial};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, ForeignText, TrialConclusion, TrialFinding,
};
use std::cell::Cell;

const LIMITS: ParityArchiveLimits = ParityArchiveLimits::declared(
    ArchiveLimits::declared(32768, 8192),
    BindingArchiveLimits::declared(8192, 512, 8),
    ArchiveLimits::declared(8192, 4096),
    256,
    8,
);

#[derive(Clone, Copy)]
enum Mode {
    Bytes,
    Empty,
    Refuse,
    Large,
}

struct Value {
    byte: Cell<u8>,
    calls: Cell<u32>,
    mode: Cell<Mode>,
    cause: NamespacedName,
}

impl Value {
    fn declared(byte: u8, cause: NamespacedName) -> Self {
        Self {
            byte: Cell::new(byte),
            calls: Cell::new(0),
            mode: Cell::new(Mode::Bytes),
            cause,
        }
    }
}

fn encode(value: &Value) -> Result<Vec<u8>, ValueEncodingRefusal> {
    value.calls.set(value.calls.get().saturating_add(1));
    match value.mode.get() {
        Mode::Bytes => Ok(vec![value.byte.get()]),
        Mode::Empty => Ok(Vec::new()),
        Mode::Refuse => Err(ValueEncodingRefusal::refused(
            value.cause,
            Some(ForeignText::admitted(&[b'x', 0xff])),
        )),
        Mode::Large => Ok(vec![1; 257]),
    }
}

fn production(input: &Value) -> Value {
    Value::declared(input.byte.get(), input.cause)
}

const EVALUATE: EvaluationCall<Value, Value> =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 0));
const DISAGREE: EvaluationCall<Value, Value> = |input, _directive| {
    Ok(EvaluationObservation::observed(
        Value::declared(input.byte.get().wrapping_add(1), input.cause),
        0,
    ))
};
const ACTIVATE: EvaluationCall<Value, Value> =
    |input, _directive| Ok(EvaluationObservation::observed(production(input), 3));

fn same(left: &Value, right: &Value) -> Agreement {
    if left.byte.get() == right.byte.get() {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

fn passes(_value: &Value) -> TrialConclusion {
    TrialConclusion::Passed
}

fn refuses_even(value: &Value) -> TrialConclusion {
    if value.byte.get().is_multiple_of(2) {
        TrialConclusion::Refused(TrialFinding::established(
            FailureClass::RefusedByCheck,
            FindingCause::named("retention", "even"),
            FindingLocation::at("check.rs", 9),
            Some(ForeignText::admitted(b"even refused")),
        ))
    } else {
        TrialConclusion::Passed
    }
}

fn pair(evaluation: EvaluationCall<Value, Value>) -> Result<EvaluationPair<Value, Value>, ()> {
    let family = family("retention").map_err(|_| ())?;
    let policy = policy(family).map_err(|_| ())?;
    let (_, surface) = lower_discoveries(&policy, Vec::new())
        .map_err(|_| ())?
        .into_parts();
    EvaluationPair::paired(
        ProductionBinding::declared(
            family,
            RevisionBinding::declared(ContentAddress::derived(
                REVISION_TAG,
                b"production-encoding",
            )),
            production,
        ),
        EvaluationBinding::declared(
            &surface,
            RevisionBinding::untracked(ContentAddress::derived(
                REVISION_TAG,
                b"evaluation-encoding",
            )),
            evaluation,
        ),
        same,
    )
    .map_err(|_| ())
}

fn encoder() -> Result<ValueEncoder<Value>, ()> {
    Ok(ValueEncoder::declared(
        NamespacedName::named("encoding", "value").map_err(|_| ())?,
        7,
        ContentAddress::derived(REVISION_TAG, b"schema"),
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"encoder")),
        encode,
    ))
}

fn input(byte: u8) -> Result<Value, ()> {
    Ok(Value::declared(
        byte,
        NamespacedName::named("encoding", "refused").map_err(|_| ())?,
    ))
}

fn reading<'pair, 'input>(
    pair: &'pair EvaluationPair<Value, Value>,
    input: &'input Value,
    check: fn(&Value) -> TrialConclusion,
) -> Result<NoMutationParityReading<'pair, 'input, Value, Value>, ()> {
    let witness = MutationWitness::bound(
        trial_binding().map_err(|_| ())?,
        check_ref().map_err(|_| ())?,
        check,
    )
    .map_err(|_| ())?;
    observe_no_mutation(pair, witness, input, &foreign_invocation()).map_err(|_| ())
}

#[test]
fn retention_is_one_encoding_per_value_and_loading_executes_nothing() -> Result<(), ()> {
    let pair = pair(EVALUATE)?;
    let input = input(7)?;
    let reading = reading(&pair, &input, passes)?;
    let encoder = encoder()?;
    let input_encoder = ValueEncoder::declared(
        NamespacedName::named("input-encoding", "distinct").map_err(|_| ())?,
        11,
        ContentAddress::derived(REVISION_TAG, b"input-schema"),
        RevisionBinding::untracked(ContentAddress::derived(REVISION_TAG, b"input-encoder")),
        encode,
    );
    let record = retain_parity(&reading, &input_encoder, &encoder, LIMITS).map_err(|_| ())?;
    assert_eq!(
        (
            input.calls.get(),
            reading.production().calls.get(),
            reading.evaluation().calls.get()
        ),
        (1, 1, 1)
    );
    assert_eq!(record.disposition(), ArchivedParityDisposition::Raw);
    assert_eq!(record.input().bytes(), &[7]);
    assert_eq!(record.production().bytes(), &[7]);
    assert_eq!(record.evaluation().bytes(), &[7]);
    for (retained, declared) in [
        (record.input(), &input_encoder),
        (record.production(), &encoder),
        (record.evaluation(), &encoder),
    ] {
        observe_convention(retained.convention(), declared);
    }
    assert_eq!(
        record.production_report(),
        &retain_trial(reading.production_report(), LIMITS.trial()).map_err(|_| ())?
    );
    assert_eq!(
        record.evaluation_report(),
        &retain_trial(reading.evaluation_report(), LIMITS.trial()).map_err(|_| ())?
    );
    assert_ne!(
        record.pair().production_revision().revision(),
        record.witness().subject_revision().revision()
    );
    assert_ne!(
        record.pair().evaluation_revision().revision(),
        record.witness().check_revision().revision()
    );
    assert_eq!(
        read_parity(record.encoded(), LIMITS).map_err(|_| ())?,
        record
    );
    assert_eq!(
        (
            input.calls.get(),
            reading.production().calls.get(),
            reading.evaluation().calls.get()
        ),
        (1, 1, 1)
    );
    let exact = ParityArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len(), 8192),
        LIMITS.binding(),
        LIMITS.trial(),
        256,
        8,
    );
    assert_eq!(
        retain_parity(&reading, &input_encoder, &encoder, exact).map_err(|_| ())?,
        record
    );
    let short = ParityArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len().saturating_sub(1), 8192),
        LIMITS.binding(),
        LIMITS.trial(),
        256,
        8,
    );
    assert_eq!(
        retain_parity(&reading, &input_encoder, &encoder, short),
        Err(ParityArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    Ok(())
}

fn observe_convention(convention: &ArchivedValueConvention, declared: &ValueEncoder<Value>) {
    assert_eq!(
        convention.name().namespace(),
        declared.convention().namespace().written()
    );
    assert_eq!(
        convention.name().stem(),
        declared.convention().stem().written()
    );
    assert_eq!(convention.version(), declared.version());
    assert_eq!(convention.schema().as_bytes(), declared.schema().as_bytes());
    assert_eq!(
        convention.revision().revision(),
        declared.revision().revision().as_bytes()
    );
    assert_eq!(
        convention.revision().posture(),
        declared.revision().posture()
    );
}

#[test]
fn qualified_and_each_rejected_live_standing_retains_its_actual_reading() -> Result<(), ()> {
    let passing: fn(&Value) -> TrialConclusion = passes;
    for (evaluation, byte, check, expected) in [
        (EVALUATE, 1u8, passing, ArchivedParityDisposition::Qualified),
        (
            DISAGREE,
            2,
            refuses_even,
            ArchivedParityDisposition::Rejected(
                ParityQualificationRefusal::ProductionDidNotQualify,
            ),
        ),
        (
            DISAGREE,
            1,
            refuses_even,
            ArchivedParityDisposition::Rejected(
                ParityQualificationRefusal::EvaluationDidNotQualify,
            ),
        ),
        (
            ACTIVATE,
            1,
            passes,
            ArchivedParityDisposition::Rejected(ParityQualificationRefusal::NoMutationActivated {
                firings: 3,
            }),
        ),
        (
            DISAGREE,
            1,
            passes,
            ArchivedParityDisposition::Rejected(ParityQualificationRefusal::MeaningsDisagreed),
        ),
    ] {
        let pair = pair(evaluation)?;
        let input = input(byte)?;
        let standing = qualify_no_mutation(reading(&pair, &input, check)?);
        let encoder = encoder()?;
        let record =
            retain_parity_standing(&standing, &encoder, &encoder, LIMITS).map_err(|_| ())?;
        assert_eq!(record.disposition(), expected);
        assert_eq!(record.input().bytes(), &[byte]);
        assert_eq!(
            read_parity(record.encoded(), LIMITS).map_err(|_| ())?,
            record
        );
    }
    Ok(())
}

#[test]
fn each_callback_refusal_and_oversize_stops_at_its_role_without_consuming_evidence()
-> Result<(), ()> {
    for (role, mode) in [
        ValueRole::Input,
        ValueRole::Production,
        ValueRole::Evaluation,
    ]
    .into_iter()
    .flat_map(|role| {
        [Mode::Refuse, Mode::Large]
            .into_iter()
            .map(move |mode| (role, mode))
    }) {
        let pair = pair(EVALUATE)?;
        let input = input(1)?;
        let reading = reading(&pair, &input, passes)?;
        let encoder = encoder()?;
        let selected = match role {
            ValueRole::Input => &input,
            ValueRole::Production => reading.production(),
            ValueRole::Evaluation => reading.evaluation(),
        };
        selected.mode.set(mode);
        let result = retain_parity(&reading, &encoder, &encoder, LIMITS);
        match mode {
            Mode::Refuse => {
                let Err(ParityArchiveRefusal::Encoder { role: found, cause }) = result else {
                    return Err(());
                };
                assert_eq!(found, role);
                assert_eq!(cause.cause(), input.cause);
                assert_eq!(cause.foreign().ok_or(())?.bytes(), &[b'x', 0xff]);
            }
            Mode::Large => {
                assert_eq!(result, Err(ParityArchiveRefusal::ValueTooLarge { role }));
            }
            Mode::Bytes | Mode::Empty => return Err(()),
        }
        let expected = match role {
            ValueRole::Input => (1, 0, 0),
            ValueRole::Production => (1, 1, 0),
            ValueRole::Evaluation => (1, 1, 1),
        };
        assert_eq!(
            (
                input.calls.get(),
                reading.production().calls.get(),
                reading.evaluation().calls.get()
            ),
            expected
        );
        selected.mode.set(Mode::Bytes);
        assert!(retain_parity(&reading, &encoder, &encoder, LIMITS).is_ok());
    }
    Ok(())
}

#[test]
fn every_known_metadata_bound_refuses_before_callbacks() -> Result<(), ()> {
    let pair = pair(EVALUATE)?;
    let input = input(1)?;
    let reading = reading(&pair, &input, passes)?;
    let encoder = encoder()?;
    for limits in [
        ParityArchiveLimits::declared(
            ArchiveLimits::declared(1, 8192),
            LIMITS.binding(),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            ArchiveLimits::declared(32768, 40),
            LIMITS.binding(),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            LIMITS.bytes(),
            BindingArchiveLimits::declared(1, 512, 8),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            LIMITS.bytes(),
            LIMITS.binding(),
            ArchiveLimits::declared(1, 4096),
            256,
            8,
        ),
        ParityArchiveLimits::declared(LIMITS.bytes(), LIMITS.binding(), LIMITS.trial(), 256, 1),
    ] {
        assert!(retain_parity(&reading, &encoder, &encoder, limits).is_err());
    }
    assert_eq!(
        (
            input.calls.get(),
            reading.production().calls.get(),
            reading.evaluation().calls.get()
        ),
        (0, 0, 0)
    );
    Ok(())
}

#[test]
fn encoding_at_retention_does_not_claim_an_earlier_interior_state_snapshot() -> Result<(), ()> {
    let pair = pair(EVALUATE)?;
    let input = input(1)?;
    let reading = reading(&pair, &input, passes)?;
    input.byte.set(9);
    reading.production().byte.set(8);
    reading.evaluation().byte.set(7);
    let encoder = encoder()?;
    let record = retain_parity(&reading, &encoder, &encoder, LIMITS).map_err(|_| ())?;
    assert_eq!(record.input().bytes(), &[9]);
    assert_eq!(record.production().bytes(), &[8]);
    assert_eq!(record.evaluation().bytes(), &[7]);
    assert_eq!(reading.conclusion(), &TrialConclusion::Passed);
    for value in [&input, reading.production(), reading.evaluation()] {
        value.mode.set(Mode::Empty);
    }
    let empty = retain_parity(&reading, &encoder, &encoder, LIMITS).map_err(|_| ())?;
    assert!(empty.input().bytes().is_empty());
    assert!(empty.production().bytes().is_empty());
    assert!(empty.evaluation().bytes().is_empty());
    Ok(())
}
