//! `ProgramImage`: the self-explaining executable package, its identities, the
//! component table, the packaging profiles, the validation ladder, and the
//! admission pipeline.
//!
//! # The standalone-reader law
//!
//! A standalone reader must determine what the program means, what it can
//! read, request, or publish, which capabilities and bounds it requires,
//! which uncertainty can remain, and whether the current implementation can
//! validate it — WITHOUT the original compiler process, the Rust source tree,
//! repository prose, chat history, an ambient registry, or an online linker
//! service.
//!
//! # No provisional encoding
//!
//! A `ProgramImage` is a canonical executable artifact, not a nickname for an
//! in-memory prototype: no image is generated or admitted from a provisional
//! or process-local encoding — the canonical byte profile (target-independent
//! grammar, explicit widths, ordering, normalization, duplicate/unknown-field
//! policy, depth/size bounds; never Rust `repr`) must close first. The byte
//! row is this home's own `img` frame profile citing the bytes home's
//! primitives; decode rides the shared sixteen decode maxima.
//!
//! # Neutral inspection
//!
//! Every image has a neutral, read-only, effect-free inspection surface —
//! identities, signatures, both forms' operations, source/cut requirements,
//! capabilities/effects, bounds, recursion witnesses, imports/kernels,
//! explanation structures, validation findings. Neutral disassembly renders
//! Execution Form without inventing source syntax. INSPECTION IS NOT
//! ADMISSION.

use crate::bytes::ContentRegionId;
use crate::execution::KernelRequirementSet;
use crate::identity::{
    AuthorityPosition, ByteIdentity, CreationLaw, IdentityClass, IdentityRole, Occurrence,
};
use crate::types::{Bounded, EvidenceRef, Limit};

// ---------------------------------------------------------------------------
// The image identities.
// ---------------------------------------------------------------------------

/// The byte-role marker for exact image bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageByteRole;

/// The exact serialized-bytes identity — Class B. Never substitutable for
/// the semantic home's meaning digest: a digest proves only the exact byte
/// role named by its own preimage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageDigest(ByteIdentity<ImageByteRole>);

impl IdentityRole for ImageDigest {
    const CLASS: IdentityClass = IdentityClass::ByteDigest;
    const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
}

impl ImageDigest {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn of(digest: ByteIdentity<ImageByteRole>) -> Self {
        Self(digest)
    }
}

/// The claim marker for program-image references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramImageClaim;

/// A reference to one program image — Class E: the referent, its version, and
/// the availability and integrity postures ride the root evidence-reference
/// shape; the claim marker is the role. A digest-shaped value with no
/// declared role is not a lawful reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramImageRef(pub EvidenceRef<ProgramImageClaim>);

/// The identity role marker for image families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageFamilyRole;

/// One image family — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageFamilyId(Occurrence<ImageFamilyRole>);

impl IdentityRole for ImageFamilyId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl ImageFamilyId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<ImageFamilyRole>) -> Self {
        Self(occurrence)
    }
}

/// One image-family format version — Class C, ordered ONLY within its family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageFamilyFormatVersion(pub AuthorityPosition<ImageFamilyId>);

/// The identity role marker for image profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageProfileRole;

/// One image profile — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageProfileId(Occurrence<ImageProfileRole>);

impl IdentityRole for ImageProfileId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl ImageProfileId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<ImageProfileRole>) -> Self {
        Self(occurrence)
    }
}

/// One image-profile version — Class C, ordered ONLY within its profile.
/// Each identity carries its own compatibility claim: semantic, execution,
/// image-bytes, runtime, and release support do not move together; an unknown
/// operation, version, profile, import, or kernel is refused, never silently
/// ignored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageProfileVersion(pub AuthorityPosition<ImageProfileId>);

/// The identity role marker for admitted programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedProgramRole;

/// One admitted invocation subject — Class D, minted ONLY by the admission
/// pipeline's final stage; admission does not mutate image identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittedProgramId(Occurrence<AdmittedProgramRole>);

impl IdentityRole for AdmittedProgramId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

// ---------------------------------------------------------------------------
// The component table.
// ---------------------------------------------------------------------------

/// The component-role roster — AUTHORED here: the image's eighteen bound
/// facts are law, and which component roles carry them is this home's
/// decision. One role per separable bound-fact carrier; identities ride
/// the root frame header, not a component. The wire form is the `img` row's
/// registered role `u16`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentRole {
    /// The normalized Semantic Form.
    SemanticForm,
    /// The agreed Execution Form.
    ExecutionForm,
    /// Types, schemas, definitions, and operator contracts.
    ContractsAndDefinitions,
    /// Constants.
    Constants,
    /// Declared inputs and outputs.
    DeclaredInputsAndOutputs,
    /// Event and effect declarations.
    EventAndEffectDeclarations,
    /// Capability requirements.
    CapabilityRequirements,
    /// Source and historical-cut requirements.
    SourceAndCutRequirements,
    /// The portable bounds.
    Bounds,
    /// Explanation structures.
    ExplanationStructures,
    /// Completed public-operation and closed-function judgments.
    CompletedJudgments,
    /// Bounded capture records.
    CaptureRecords,
    /// The import and immutable-resource closure.
    ImportClosure,
    /// The required kernel-interface closure and binding policy.
    KernelRequirements,
    /// Entrypoints.
    Entrypoints,
    /// Compatibility and extension posture.
    CompatibilityPosture,
    /// Optional origin and source maps, where the profile admits them.
    OriginMaps,
    /// Optional authenticity and attestation references.
    AuthenticityReferences,
    /// Optional qualification references.
    QualificationReferences,
}

/// How a component is carried. An unresolvable or digest-mismatched
/// reference REFUSES THE IMAGE. Self-contained packaging inlines each
/// component as a frame under the physical cap — a larger component must be
/// a content region (the tiering law).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentCarriage {
    /// Carried inline.
    Inline,
    /// Referenced by immutable content identity — the reference IS the
    /// region's digest.
    ImmutableReference,
}

/// One component of the image's root binding — the `img` row: registered
/// role, registered profile (`u16`), exact content digest, length, carriage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramImageComponent {
    /// The component role.
    pub role: ComponentRole,
    /// The registered component profile.
    pub profile: u16,
    /// The exact content digest.
    pub content: ContentRegionId,
    /// The byte length.
    pub length: u64,
    /// The carriage.
    pub carriage: ComponentCarriage,
}

/// The three packaging profiles — all lawful, all satisfying the same
/// dual-form closure and standalone-inspection requirement. RULED (D-IMG-2):
/// `SelfContained` is the selected paved-road default — offline verification,
/// regulated and air-gapped deployment, agent handoff, reproducibility (the
/// self-explaining-artifact north star). Selecting the default narrows
/// nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackagingProfile {
    /// Every component inline — the paved-road default.
    SelfContained,
    /// Components referenced by content identity from an immutable store.
    ImmutableBound,
    /// A mix.
    Hybrid,
}

/// Limit family for an image's components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentLimit;
impl Limit for ComponentLimit {}

/// One directly executable program's package: a root binding a set of typed
/// components plus its import/immutable-resource/required-kernel closure.
/// Every executable image contains or immutably binds BOTH forms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramImage {
    /// The image-family format version.
    pub family: ImageFamilyFormatVersion,
    /// The image-profile version.
    pub profile: ImageProfileVersion,
    /// The packaging profile.
    pub packaging: PackagingProfile,
    /// The component table.
    pub components: Bounded<ProgramImageComponent, ComponentLimit>,
    /// The complete required kernel closure.
    pub kernel_requirements: KernelRequirementSet,
}

/// The eighteen bound facts every image contains or immutably binds.
pub const BOUND_FACT_ROSTER: [&str; 18] = [
    "image-role-profile-version-identities",
    "normalized-semantic-form",
    "agreed-execution-form",
    "types-schemas-definitions-operator-contracts",
    "constants",
    "declared-inputs-and-outputs",
    "event-and-effect-declarations",
    "capability-requirements",
    "source-and-historical-cut-requirements",
    "portable-bounds",
    "explanation-structures",
    "completed-judgments",
    "bounded-capture-records",
    "import-and-immutable-resource-closure",
    "kernel-requirement-set",
    "entrypoints",
    "compatibility-and-extension-posture",
    "optional-origin-authenticity-qualification-references",
];

/// The program-image artifact suffix — the artifact-kind register's row: one
/// `TPAK` magic for every binary artifact (the bytes home's frame grammar),
/// with the registered role `u16` distinguishing kinds.
pub const PROGRAM_IMAGE_EXTENSION: &str = ".program.tpk";

// ---------------------------------------------------------------------------
// The validation ladder and admission pipeline.
// ---------------------------------------------------------------------------

/// The domain marker for validation-stage content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageStageDomain;

/// Untrusted image bytes — the ladder's only entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UntrustedImageBytes {
    stage: crate::identity::Commitment<ImageStageDomain>,
}

impl UntrustedImageBytes {
    /// The stage content's commitment.
    #[must_use]
    pub fn stage(&self) -> &crate::identity::Commitment<ImageStageDomain> {
        &self.stage
    }
}

/// A bounded, canonically decoded image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundedDecodedImage {
    stage: crate::identity::Commitment<ImageStageDomain>,
}

impl BoundedDecodedImage {
    /// The stage content's commitment.
    #[must_use]
    pub fn stage(&self) -> &crate::identity::Commitment<ImageStageDomain> {
        &self.stage
    }
}

/// A semantically validated image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticImage {
    stage: crate::identity::Commitment<ImageStageDomain>,
}

impl SemanticImage {
    /// The stage content's commitment.
    #[must_use]
    pub fn stage(&self) -> &crate::identity::Commitment<ImageStageDomain> {
        &self.stage
    }
}

/// An agreement-checked image — minted ONLY by the independent agreement
/// verifier, never by literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgreementCheckedImage {
    stage: crate::identity::Commitment<ImageStageDomain>,
}

impl AgreementCheckedImage {
    /// The stage content's commitment.
    #[must_use]
    pub fn stage(&self) -> &crate::identity::Commitment<ImageStageDomain> {
        &self.stage
    }
}

/// An executable image — minted ONLY by the independent agreement verifier.
/// Each ladder transition is a sealed constructor consuming `self`, returning
/// the stronger type or a typed refusal — affine typestate, because these
/// states are small, stable, and known at the call site. From here onward
/// states are durable and crash-recoverable and are NOT compile-time
/// typestate: invocation admission produces a separate authority-bound value;
/// execution, suspension, and termination are runtime records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutableImage {
    stage: crate::identity::Commitment<ImageStageDomain>,
}

impl ExecutableImage {
    /// The stage content's commitment.
    #[must_use]
    pub fn stage(&self) -> &crate::identity::Commitment<ImageStageDomain> {
        &self.stage
    }
}

/// The DURABLE record of the reached validation phase — not the live handle.
/// A decoded record re-enters live use only through re-validation, never by
/// construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageValidation {
    /// Untrusted bytes.
    UntrustedBytes,
    /// Bounded-decoded.
    BoundedDecoded,
    /// Semantically validated.
    Semantic,
    /// Agreement-checked.
    AgreementChecked,
    /// Executable.
    Executable,
}

/// The sixteen dependency-ordered admission stages — no stage is skipped; no
/// program runs from parsed bytes, a digest match, a compiler assertion, a
/// signature alone, an available kernel alone, or a capability claim alone.
/// Cheap safe refusals may precede expensive proof work, but reordering may
/// never create an undeclared oracle or side channel.
pub const ADMISSION_PIPELINE: [&str; 16] = [
    "bounded-canonical-decode",
    "role-profile-version-extension-validation",
    "component-length-offset-count-allocation-validation",
    "exact-identity-and-import-resource-closure",
    "type-schema-definition-semantic-form-validation",
    "recursion-and-aggregate-resource-witness-validation",
    "independent-semantic-to-execution-re-lowering",
    "bound-form-agreement",
    "execution-form-type-region-control-suspension-validation",
    "effect-capability-requirement-source-closure",
    "work-and-output-bound-validation",
    "kernel-interface-binding-qualification-closure",
    "authenticity-trust-admission-policy-where-required",
    "executable-image",
    "invocation-specific-admission",
    "admitted-program",
];

/// The eight facts admission proves. Observed effects stay a SUBSET of the
/// declared and admitted closure, and capability authorizes an operation
/// without proving it committed, completed, or was atomic.
pub const ADMISSION_PROVES: [&str; 8] = [
    "every-semantic-operation-is-profile-allowed",
    "every-requested-effect-is-declared",
    "every-capability-is-satisfied-by-the-admitted-grant",
    "every-port-kernel-dependency-closes",
    "every-source-cut-requirement-is-lawful",
    "every-resource-bound-fits-the-reserved-envelope",
    "every-recursive-effect-and-suspension-total-closes",
    "observed-effects-stay-a-subset-of-the-admitted-closure",
];
