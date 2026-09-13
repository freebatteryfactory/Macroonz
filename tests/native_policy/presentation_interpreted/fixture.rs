//! Declared mutation observations for historical presentation controls.

use crate::presentation_mutation::manifest;
use crate::presentation_parity::{Datum, binding, datum, mapped, production};
use macroonz::harness::descriptor::{ClaimRef, MutationPointRef};
use macroonz::harness::muterprater::backend_archive::{
    BackendArchiveLimits, SuitePressureArchiveLimits,
};
use macroonz::harness::muterprater::discovery_archive::SurfaceArchiveLimits;
use macroonz::harness::muterprater::interpretation_archive::InterpretedArchiveLimits;
use macroonz::harness::muterprater::specimen_archive::ProjectionArchiveLimits;
use macroonz::harness::muterprater::verdict_archive::MutationRunArchiveLimits;
use macroonz::harness::muterprater::{
    ActivationSite, AdapterQualification, AlternativeDeclaration, CompiledSpecimenHost,
    CompiledSpecimenObservation, CompiledSpecimenRole, CompiledSuiteArtifactCustody,
    CompiledSuiteArtifactStanding, CompiledSuitePressure, DiscoveredMutationSite, EvaluationCall,
    EvaluationFamilyRef, EvaluationObservation, EvaluationSurface, GrammarStanding,
    MutationPermission, MutationPolicy, MutationSurfaceLowering, OperatorFamilyRef,
    OwnerClaimMapping, SpecimenMaterializerCall, discover::lower_discoveries,
};
use macroonz::harness::report::archive::ArchiveLimits;

const BYTES: ArchiveLimits = ArchiveLimits::declared(262_144, 131_072);
pub(super) const LIMITS: InterpretedArchiveLimits = InterpretedArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 8, 8),
    SuitePressureArchiveLimits::declared(
        BYTES,
        BackendArchiveLimits::declared(MutationRunArchiveLimits::declared(BYTES, 8), 8, 8, 8),
    ),
    ProjectionArchiveLimits::declared(BYTES, crate::presentation_parity::LIMITS, BYTES, BYTES, 128),
    BYTES,
    BYTES,
    128,
);
pub(super) const CONSOLE: &str = "Found 3 mutants\nok Unmutated baseline\nmissed fixture.rs:1:1: replace a with b\ncaught fixture.rs:2:1: replace <x>| with y\ncaught fixture.rs:3:1: replace z with q\n";
pub(super) const SOURCES: &[(&str, &[u8])] = &[("fixture.rs", b"\0\xffsource")];

pub(super) fn surface() -> Result<EvaluationSurface, String> {
    Ok(lowering()?.into_parts().1)
}

pub(super) fn lowering() -> Result<MutationSurfaceLowering, String> {
    let owner = binding()?.row().claim();
    let operator = OperatorFamilyRef::of_slug("comparison-boundaries").ok_or("operator missing")?;
    let permission = mapped(MutationPermission::declared(owner, vec![operator]))?;
    let policy = mapped(MutationPolicy::declared(
        mapped(EvaluationFamilyRef::named("parity-display", "byte"))?,
        vec![permission],
    ))?;
    let mut sites = Vec::new();
    let outside =
        OperatorFamilyRef::of_slug("boolean-operators").ok_or("outside operator missing")?;
    for (name, mapping, second) in [
        ("unmapped", OwnerClaimMapping::OwnerUnmapped, operator),
        ("second", OwnerClaimMapping::Mapped(owner), operator),
        (
            "unpermitted-claim",
            OwnerClaimMapping::Mapped(mapped(ClaimRef::named("other", "claim"))?),
            operator,
        ),
        (
            "unpermitted-family",
            OwnerClaimMapping::Mapped(owner),
            outside,
        ),
        ("first", OwnerClaimMapping::Mapped(owner), operator),
    ] {
        sites.push(mapped(DiscoveredMutationSite::discovered(
            mapped(MutationPointRef::named("display-mutation", name))?,
            mapping,
            vec![255],
            vec![
                AlternativeDeclaration::stated(operator, vec![0]),
                AlternativeDeclaration::stated(second, vec![13]),
            ],
            mapped(ActivationSite::named("display-mutation", "activation"))?,
        ))?);
    }
    mapped(lower_discoveries(&policy, sites))
}

pub(super) fn suite() -> Result<CompiledSuitePressure, String> {
    let manifest = manifest(CONSOLE)?;
    let qualification = mapped(AdapterQualification::of(
        manifest.reading(),
        GrammarStanding::Checked(manifest.invocation().version().clone()),
    ))?;
    let revisions = manifest.sources().to_vec();
    let custody = mapped(CompiledSuiteArtifactCustody::current(manifest, revisions))?;
    mapped(CompiledSuitePressure::demonstrated(
        CompiledSuiteArtifactStanding::Reported(&custody),
        &qualification,
    ))
}

pub(super) const KILLED: EvaluationCall<Datum, Datum> = |input, directive| {
    Ok(match directive.resolved() {
        Some(_) => EvaluationObservation::observed(datum(2), 7),
        None => EvaluationObservation::observed(production(input), 0),
    })
};
pub(super) const SURVIVED: EvaluationCall<Datum, Datum> = |input, directive| {
    Ok(match directive.resolved() {
        Some(_) => EvaluationObservation::observed(datum(3), 7),
        None => EvaluationObservation::observed(production(input), 0),
    })
};
pub(super) const MATERIALIZER: SpecimenMaterializerCall = |directive| {
    Ok(if directive.resolved().is_some() {
        vec![0, 255, 13]
    } else {
        Vec::new()
    })
};
pub(super) const HOST: CompiledSpecimenHost<Datum, Datum> = |request| {
    let meaning = match request.role() {
        CompiledSpecimenRole::Baseline => datum(1),
        CompiledSpecimenRole::Selected(_) => datum(2),
    };
    Ok(CompiledSpecimenObservation::executed(&request, meaning))
};
