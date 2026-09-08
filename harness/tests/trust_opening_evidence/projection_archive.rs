//! Historical projection retention observed through the existing real compiler host.

use super::support::{
    COMPILED_SPECIMEN_HOST, CompiledRosterMeaning, EVALUATION, REVISION_TAG, SELECTED_OPERATION,
    SPECIMEN_HOST_CALLS, SPECIMEN_MATERIALIZER, SPECIMEN_MATERIALIZER_CALLS, active_selection,
    family, invocation, lock_specimen_tests, pair, qualification_of, qualified_no_mutation,
    surface_with, witness,
};
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::descriptor::{NamespacedName, RevisionBinding};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::muterprater::interpretation_archive::{
    ParityArchiveLimits, ParityArchiveRefusal, ValueEncoder, ValueEncodingRefusal, ValueRole,
};
use macroonz_harness::muterprater::specimen::demonstrate_compiled_projection;
use macroonz_harness::muterprater::specimen_archive::{
    ProjectionArchiveLimits, ProjectionArchiveRefusal, read_projection, retain_projection,
};
use macroonz_harness::muterprater::verdict_archive::retain_mutation;
use macroonz_harness::muterprater::{CompiledProjectionPressure, SpecimenMaterializerBinding};
use macroonz_harness::report::archive::{ArchiveLimits, retain_trial};
use std::error::Error;
use std::io::Read as _;
use std::sync::atomic::{AtomicU32, Ordering};

const BYTES: ArchiveLimits = ArchiveLimits::declared(131_072, 65536);
const PARITY: ParityArchiveLimits = ParityArchiveLimits::declared(
    ArchiveLimits::declared(65536, 32768),
    BindingArchiveLimits::declared(16384, 512, 8),
    BYTES,
    256,
    8,
);
const LIMITS: ProjectionArchiveLimits =
    ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 16384);

static ENCODER_CALLS: AtomicU32 = AtomicU32::new(0);
static OVERSIZE_AT: AtomicU32 = AtomicU32::new(0);

fn encoded_value(bytes: Vec<u8>) -> Vec<u8> {
    let ordinal = ENCODER_CALLS
        .fetch_add(1, Ordering::SeqCst)
        .saturating_add(1);
    if ordinal == OVERSIZE_AT.load(Ordering::SeqCst) {
        vec![0; 257]
    } else {
        bytes
    }
}

const INPUT: fn(&[u32; 3]) -> Result<Vec<u8>, ValueEncodingRefusal> = |input| {
    Ok(encoded_value(
        input.iter().flat_map(|word| word.to_be_bytes()).collect(),
    ))
};
const MEANING: fn(&CompiledRosterMeaning) -> Result<Vec<u8>, ValueEncodingRefusal> =
    |meaning| Ok(encoded_value(format!("{meaning:?}").into_bytes()));

fn encoder<Value>(
    name: &'static str,
    encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
) -> Result<ValueEncoder<Value>, Box<dyn Error>> {
    Ok(ValueEncoder::declared(
        NamespacedName::named("specimen-archive", name).map_err(failure)?,
        1,
        ContentAddress::derived(REVISION_TAG, name.as_bytes()),
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"projection-encoder")),
        encode,
    ))
}

fn failure(cause: impl core::fmt::Debug) -> std::io::Error {
    std::io::Error::other(format!("{cause:?}"))
}

fn refused_limits() -> [ProjectionArchiveLimits; 6] {
    [
        ProjectionArchiveLimits::declared(
            ArchiveLimits::declared(1, 65536),
            PARITY,
            BYTES,
            BYTES,
            16384,
        ),
        ProjectionArchiveLimits::declared(
            ArchiveLimits::declared(131_072, 31),
            PARITY,
            BYTES,
            BYTES,
            16384,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            ParityArchiveLimits::declared(
                ArchiveLimits::declared(1, 32768),
                PARITY.binding(),
                PARITY.trial(),
                256,
                8,
            ),
            BYTES,
            BYTES,
            16384,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            PARITY,
            ArchiveLimits::declared(1, 65536),
            BYTES,
            16384,
        ),
        ProjectionArchiveLimits::declared(
            BYTES,
            PARITY,
            BYTES,
            ArchiveLimits::declared(1, 65536),
            16384,
        ),
        ProjectionArchiveLimits::declared(BYTES, PARITY, BYTES, BYTES, 0),
    ]
}

fn controls(
    pressure: &CompiledProjectionPressure<'_, '_, '_, [u32; 3], CompiledRosterMeaning>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let input = encoder("u32-array-be", INPUT)?;
    let meaning = encoder("debug-roster", MEANING)?;
    ENCODER_CALLS.store(0, Ordering::SeqCst);
    OVERSIZE_AT.store(0, Ordering::SeqCst);
    for limits in refused_limits() {
        assert!(retain_projection(pressure, &input, &meaning, limits).is_err());
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 0);
    }
    for (ordinal, role) in [
        (1u32, ValueRole::Input),
        (2, ValueRole::Production),
        (3, ValueRole::Evaluation),
    ] {
        ENCODER_CALLS.store(0, Ordering::SeqCst);
        OVERSIZE_AT.store(ordinal, Ordering::SeqCst);
        assert_eq!(
            retain_projection(pressure, &input, &meaning, LIMITS),
            Err(ProjectionArchiveRefusal::Parity(
                ParityArchiveRefusal::ValueTooLarge { role }
            ))
        );
        assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), ordinal);
    }
    ENCODER_CALLS.store(0, Ordering::SeqCst);
    OVERSIZE_AT.store(0, Ordering::SeqCst);
    let record = retain_projection(pressure, &input, &meaning, LIMITS).map_err(failure)?;
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 3);
    assert_eq!(record.baseline_content(), pressure.baseline_content());
    assert_eq!(record.selected_content(), pressure.selected_content());
    assert_eq!(record.standing().artifact(), pressure.standing().artifact());
    assert_eq!(
        record.baseline_report(),
        &retain_trial(pressure.baseline_report(), BYTES).map_err(failure)?
    );
    assert_eq!(
        record.selected_report(),
        &retain_trial(pressure.selected_report(), BYTES).map_err(failure)?
    );
    assert_eq!(
        record.mutation(),
        &retain_mutation(pressure.mutation(), BYTES).map_err(failure)?
    );
    assert_eq!(
        record.parity().input().bytes(),
        &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(record.parity().production().bytes(), b"Stated(1)");
    assert_eq!(record.parity().evaluation().bytes(), b"Stated(1)");
    let detached = read_projection(record.encoded(), LIMITS).map_err(failure)?;
    assert_eq!(record, detached);
    assert_eq!(ENCODER_CALLS.load(Ordering::SeqCst), 3);
    let exact = ProjectionArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len(), 65536),
        PARITY,
        BYTES,
        BYTES,
        pressure
            .baseline_content()
            .bytes()
            .len()
            .max(pressure.selected_content().bytes().len()),
    );
    assert_eq!(
        retain_projection(pressure, &input, &meaning, exact).map_err(failure)?,
        record
    );
    let short = ProjectionArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len().saturating_sub(1), 65536),
        PARITY,
        BYTES,
        BYTES,
        16384,
    );
    assert!(retain_projection(pressure, &input, &meaning, short).is_err());
    Ok(record.encoded().to_vec())
}

#[test]
fn actual_compiler_host_pressure_retains_once_with_all_known_bounds_before_encoders()
-> Result<(), Box<dyn Error>> {
    let _specimen_guard = lock_specimen_tests().map_err(failure)?;
    let family = family("projection-archive").map_err(failure)?;
    let surface = surface_with(family, vec![SELECTED_OPERATION]).map_err(failure)?;
    let pair = pair(family, &surface, EVALUATION).map_err(failure)?;
    let input = [1u32, 0, 0];
    let standing =
        qualified_no_mutation(&pair, witness().map_err(failure)?, &input).map_err(failure)?;
    let qualification = qualification_of(&standing).map_err(failure)?;
    let selection = active_selection(&surface).map_err(failure)?;
    SPECIMEN_MATERIALIZER_CALLS.store(0, Ordering::SeqCst);
    SPECIMEN_HOST_CALLS.store(0, Ordering::SeqCst);
    let pressure = demonstrate_compiled_projection(
        &surface,
        qualification,
        &SpecimenMaterializerBinding::bound(&pair, SPECIMEN_MATERIALIZER),
        selection,
        &invocation().map_err(failure)?,
        COMPILED_SPECIMEN_HOST,
    )
    .map_err(failure)?;
    let bytes = controls(&pressure)?;
    assert_eq!(SPECIMEN_MATERIALIZER_CALLS.load(Ordering::SeqCst), 2);
    assert_eq!(SPECIMEN_HOST_CALLS.load(Ordering::SeqCst), 2);
    drop(pressure);
    drop(standing);
    drop(surface);
    assert_eq!(
        super::archive_process::round_trip(
            &bytes,
            "projection_archive::child_loads_actual_projection"
        )?,
        bytes
    );
    Ok(())
}

#[test]
#[ignore = "driven by the actual compiled projection archive process-boundary claim"]
fn child_loads_actual_projection() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(131_073)
        .read_to_end(&mut bytes)?;
    let record = read_projection(&bytes, LIMITS).map_err(failure)?;
    drop(bytes);
    super::archive_process::publish(record.encoded())
}
