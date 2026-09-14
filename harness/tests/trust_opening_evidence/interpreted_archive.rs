//! Complete interpreted retention through actual compiled and active execution owners.

use super::support::{
    BACKEND_CONSOLE, CLAIM_MISMATCH_EVALUATION_CALLS, COMPILED_MUTANT_FILE, CURRENT_BACKEND_SOURCE,
    CompiledRosterMeaning, REVISION_TAG, SELECTED_OPERATION, SPECIMEN_HOST_CALLS,
    SPECIMEN_MATERIALIZER_CALLS, active_selection, compiled_suite_pressure, evaluation_counted,
    family, invocation, lock_specimen_tests, opened_trust, pair, qualification_of,
    qualified_no_mutation_under, standard_projection_under, surface_with, witness,
};
use macroonz_harness::clock::{ClockAttribution, HarnessClock};
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::descriptor::{NamespacedName, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::InterpretedMutationEvidence;
use macroonz_harness::muterprater::backend_archive::{
    BackendArchiveLimits, SuitePressureArchiveLimits,
};
use macroonz_harness::muterprater::discovery_archive::{SurfaceArchiveLimits, retain_surface};
use macroonz_harness::muterprater::interpret::{availability, execute_active};
use macroonz_harness::muterprater::interpretation_archive::{
    ArchivedInterpretedEvidence, InterpretedArchiveLimits, InterpretedArchiveRefusal,
    ParityArchiveLimits, ParityArchiveRefusal, ValueEncoder, ValueEncodingRefusal, ValueRole,
    read_interpreted, retain_interpreted, retain_interpreted_with_material,
};
use macroonz_harness::muterprater::specimen_archive::{
    ProjectionArchiveLimits, ProjectionArchiveRefusal,
};
use macroonz_harness::muterprater::verdict_archive::{MutationRunArchiveLimits, retain_mutation};
use macroonz_harness::report::ForeignText;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal, retain_trial};
use macroonz_harness::runner::Invocation;
use std::error::Error;
use std::io::Read as _;
use std::sync::atomic::{AtomicU32, Ordering};

const BYTES: ArchiveLimits = ArchiveLimits::declared(262_144, 131_072);
const PARITY: ParityArchiveLimits = ParityArchiveLimits::declared(
    BYTES,
    BindingArchiveLimits::declared(16384, 512, 8),
    BYTES,
    256,
    8,
);
const LIMITS: InterpretedArchiveLimits = InterpretedArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 8, 8),
    SuitePressureArchiveLimits::declared(
        BYTES,
        BackendArchiveLimits::declared(MutationRunArchiveLimits::declared(BYTES, 64), 64, 64, 64),
    ),
    ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 16384),
    BYTES,
    BYTES,
    256,
);
type Evidence<'surface, 'suite, 'projection, 'parity, 'pair, 'input> = InterpretedMutationEvidence<
    'surface,
    'suite,
    'projection,
    'parity,
    'pair,
    'input,
    [u32; 3],
    CompiledRosterMeaning,
>;

static ENCODER_CALLS: AtomicU32 = AtomicU32::new(0);
static OVERSIZE_AT: AtomicU32 = AtomicU32::new(0);
static REFUSE_AT: AtomicU32 = AtomicU32::new(0);

fn failure(cause: impl core::fmt::Debug) -> std::io::Error {
    std::io::Error::other(format!("{cause:?}"))
}

fn encoded_value(bytes: Vec<u8>) -> Result<Vec<u8>, ValueEncodingRefusal> {
    let ordinal = ENCODER_CALLS
        .fetch_add(1, Ordering::SeqCst)
        .saturating_add(1);
    if ordinal == REFUSE_AT.load(Ordering::SeqCst)
        && let Ok(cause) = NamespacedName::named("caller", "retention-refused")
    {
        return Err(ValueEncodingRefusal::refused(
            cause,
            Some(ForeignText::admitted(&[b'x', 255])),
        ));
    }
    Ok(if ordinal == OVERSIZE_AT.load(Ordering::SeqCst) {
        vec![0; 257]
    } else {
        bytes
    })
}

const INPUT: fn(&[u32; 3]) -> Result<Vec<u8>, ValueEncodingRefusal> =
    |input| encoded_value(input.iter().flat_map(|word| word.to_be_bytes()).collect());
const MEANING: fn(&CompiledRosterMeaning) -> Result<Vec<u8>, ValueEncodingRefusal> =
    |meaning| encoded_value(format!("{meaning:?}").into_bytes());

fn encoder<Value>(
    name: &'static str,
    encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
) -> Result<ValueEncoder<Value>, Box<dyn Error>> {
    Ok(ValueEncoder::declared(
        NamespacedName::named("interpreted-archive", name).map_err(failure)?,
        1,
        ContentAddress::derived(REVISION_TAG, name.as_bytes()),
        RevisionBinding::declared(ContentAddress::derived(
            REVISION_TAG,
            b"interpreted-encoder",
        )),
        encode,
    ))
}

fn reset(oversize: u32, refuse: u32) {
    ENCODER_CALLS.store(0, Ordering::SeqCst);
    OVERSIZE_AT.store(oversize, Ordering::SeqCst);
    REFUSE_AT.store(refuse, Ordering::SeqCst);
}

fn changed_outer(bytes: ArchiveLimits, active: usize) -> InterpretedArchiveLimits {
    InterpretedArchiveLimits::declared(
        bytes,
        LIMITS.surface(),
        LIMITS.suite(),
        LIMITS.projection(),
        BYTES,
        BYTES,
        active,
    )
}

fn known_limits() -> [InterpretedArchiveLimits; 7] {
    let small = ArchiveLimits::declared(1, 131_072);
    let surface = LIMITS.surface();
    let suite = LIMITS.suite();
    let projection = LIMITS.projection();
    [
        changed_outer(small, 256),
        changed_outer(ArchiveLimits::declared(262_144, 31), 256),
        InterpretedArchiveLimits::declared(
            BYTES,
            SurfaceArchiveLimits::declared(BYTES, 0, 8),
            suite,
            projection,
            BYTES,
            BYTES,
            256,
        ),
        InterpretedArchiveLimits::declared(
            BYTES,
            surface,
            SuitePressureArchiveLimits::declared(small, suite.backend()),
            projection,
            BYTES,
            BYTES,
            256,
        ),
        InterpretedArchiveLimits::declared(
            BYTES,
            surface,
            suite,
            ProjectionArchiveLimits::declared(small, PARITY, BYTES, BYTES, 16384),
            BYTES,
            BYTES,
            256,
        ),
        InterpretedArchiveLimits::declared(BYTES, surface, suite, projection, small, BYTES, 256),
        InterpretedArchiveLimits::declared(BYTES, surface, suite, projection, BYTES, small, 256),
    ]
}

fn encoder_controls(
    evidence: &Evidence<'_, '_, '_, '_, '_, '_>,
    input: &ValueEncoder<[u32; 3]>,
    meaning: &ValueEncoder<CompiledRosterMeaning>,
) -> Result<(), Box<dyn Error>> {
    reset(0, 0);
    for limits in known_limits() {
        assert!(retain_interpreted(evidence, input, meaning, &limits).is_err());
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 0);
    }
    assert!(
        retain_interpreted_with_material(evidence, input, meaning, BACKEND_CONSOLE, &[], &LIMITS)
            .is_err()
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 0);
    for (ordinal, role) in [
        (1u32, ValueRole::Input),
        (2, ValueRole::Production),
        (3, ValueRole::Evaluation),
    ] {
        reset(ordinal, 0);
        assert_eq!(
            retain_interpreted(evidence, input, meaning, &LIMITS),
            Err(InterpretedArchiveRefusal::Projection(
                ProjectionArchiveRefusal::Parity(ParityArchiveRefusal::ValueTooLarge { role })
            ))
        );
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), ordinal);
    }
    reset(4, 0);
    assert_eq!(
        retain_interpreted(evidence, input, meaning, &LIMITS),
        Err(InterpretedArchiveRefusal::ActiveValueTooLarge)
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    reset(0, 4);
    let Err(InterpretedArchiveRefusal::ActiveEncoder(refusal)) =
        retain_interpreted(evidence, input, meaning, &LIMITS)
    else {
        return Err(failure("active role absent").into());
    };
    assert_eq!(refusal.cause().namespace().written(), "caller");
    assert_eq!(refusal.cause().stem().written(), "retention-refused");
    assert_eq!(
        refusal
            .foreign()
            .ok_or_else(|| failure("foreign absent"))?
            .bytes(),
        &[b'x', 255]
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    Ok(())
}

fn size_controls(
    evidence: &Evidence<'_, '_, '_, '_, '_, '_>,
    input: &ValueEncoder<[u32; 3]>,
    meaning: &ValueEncoder<CompiledRosterMeaning>,
    record: &ArchivedInterpretedEvidence,
) -> Result<(), Box<dyn Error>> {
    let exact = changed_outer(
        ArchiveLimits::declared(record.encoded().len(), BYTES.field()),
        record.meaning().bytes().len(),
    );
    reset(0, 0);
    assert_eq!(
        retain_interpreted(evidence, input, meaning, &exact).map_err(failure)?,
        *record
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    let before_active = changed_outer(
        ArchiveLimits::declared(
            record
                .encoded()
                .len()
                .saturating_sub(record.meaning().bytes().len())
                .saturating_sub(1),
            BYTES.field(),
        ),
        256,
    );
    reset(0, 0);
    assert_eq!(
        retain_interpreted(evidence, input, meaning, &before_active),
        Err(InterpretedArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 3);
    let after_active = changed_outer(
        ArchiveLimits::declared(record.encoded().len().saturating_sub(1), BYTES.field()),
        256,
    );
    reset(0, 0);
    assert_eq!(
        retain_interpreted(evidence, input, meaning, &after_active),
        Err(InterpretedArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    Ok(())
}

fn controls(evidence: &Evidence<'_, '_, '_, '_, '_, '_>) -> Result<[Vec<u8>; 2], Box<dyn Error>> {
    let input = encoder("u32-array-be", INPUT)?;
    let meaning = encoder("debug-roster", MEANING)?;
    encoder_controls(evidence, &input, &meaning)?;
    reset(0, 0);
    let record = retain_interpreted(evidence, &input, &meaning, &LIMITS).map_err(failure)?;
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    let projection = record.trust().projection();
    for report in [
        projection.parity().production_report(),
        projection.parity().evaluation_report(),
        projection.baseline_report(),
        projection.selected_report(),
        record.report(),
    ] {
        assert_eq!(report.clock_attribution(), ClockAttribution::Synthetic);
    }
    assert_eq!(record.meaning().bytes(), b"Unstated");
    assert_eq!(
        record.trust().projection().parity().production().bytes(),
        b"Stated(1)"
    );
    assert_eq!(
        record.trust().projection().parity().evaluation().bytes(),
        b"Stated(1)"
    );
    assert_eq!(
        record.trust().projection().parity().input().bytes(),
        &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        record.report(),
        &retain_trial(evidence.report(), BYTES).map_err(failure)?
    );
    assert_eq!(
        record.mutation(),
        &retain_mutation(evidence.mutation(), BYTES).map_err(failure)?
    );
    assert_eq!(
        record.trust().surface(),
        &retain_surface(evidence.trust().surface(), LIMITS.surface()).map_err(failure)?
    );
    assert_eq!(
        read_interpreted(record.encoded(), &LIMITS).map_err(failure)?,
        record
    );
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    size_controls(evidence, &input, &meaning, &record)?;
    reset(0, 0);
    let original = retain_interpreted_with_material(
        evidence,
        &input,
        &meaning,
        BACKEND_CONSOLE,
        &[(COMPILED_MUTANT_FILE, CURRENT_BACKEND_SOURCE)],
        &LIMITS,
    )
    .map_err(failure)?;
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 4);
    assert_eq!(
        original.trust().suite().manifest().original_console(),
        Some(BACKEND_CONSOLE.as_bytes())
    );
    assert_eq!(original.trust().projection(), record.trust().projection());
    Ok([record.encoded().to_vec(), original.encoded().to_vec()])
}

#[test]
fn actual_compiled_and_active_execution_retains_complete_evidence_once()
-> Result<(), Box<dyn Error>> {
    let _specimen_guard = lock_specimen_tests().map_err(failure)?;
    let family = family("interpreted-archive").map_err(failure)?;
    let surface = surface_with(family, vec![SELECTED_OPERATION]).map_err(failure)?;
    let pair = pair(family, &surface, evaluation_counted).map_err(failure)?;
    let input = [1u32, 0, 0];
    let base = invocation().map_err(failure)?;
    let measured = Invocation::declared(
        base.profile(),
        base.target().clone(),
        base.site(),
        HarnessClock::reading_as(|| 0, ClockAttribution::Synthetic),
    );
    let standing =
        qualified_no_mutation_under(&pair, witness().map_err(failure)?, &input, &measured)
            .map_err(failure)?;
    let qualification = qualification_of(&standing).map_err(failure)?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    let projection = standard_projection_under(
        &surface,
        qualification,
        &pair,
        active_selection(&surface).map_err(failure)?,
        &measured,
    )
    .map_err(failure)?;
    let suite = compiled_suite_pressure().map_err(failure)?;
    let trust = opened_trust(availability(
        Some(&surface),
        Some(&suite),
        Some(&projection),
    ))
    .map_err(failure)?;
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    let evidence = execute_active(&trust, &measured).map_err(failure)?;
    for report in [
        qualification.reading().production_report(),
        qualification.reading().evaluation_report(),
        projection.baseline_report(),
        projection.selected_report(),
        evidence.report(),
    ] {
        assert_eq!(report.clock_attribution(), ClockAttribution::Synthetic);
    }
    let records = controls(&evidence)?;
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    drop(evidence);
    drop(projection);
    drop(standing);
    drop(surface);
    drop(suite);
    for bytes in records {
        assert_eq!(
            super::archive_process::round_trip(
                &bytes,
                "interpreted_archive::child_loads_actual_interpreted"
            )?,
            bytes
        );
    }
    Ok(())
}

#[test]
#[ignore = "driven by the actual interpreted archive process-boundary claim"]
fn child_loads_actual_interpreted() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(262_145)
        .read_to_end(&mut bytes)?;
    let record = read_interpreted(&bytes, &LIMITS).map_err(failure)?;
    drop(bytes);
    super::archive_process::publish(record.encoded())
}
