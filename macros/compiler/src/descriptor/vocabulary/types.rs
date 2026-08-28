//! The semantic spellings used by generated harness paths, calls, clauses, and target-supplied seats.
//!
//! [`HarnessName`] answers where an expression lands, while [`HarnessWord`] answers how stamped material names one grammar seat.
//! The enums carry no private state because each admitted variant is already one closed destination spelling.

/// One name macroonz-harness publishes that a descriptor emission spells.
///
/// Every module, type, arm, and constructor road the three renderings write stands here, so the address a rendered expression resolves at is one table a reader can join against the harness's own declarations rather than a search through three renderers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessName {
    /// The module the descriptor vocabulary lives under.
    Descriptor,
    /// The module the mutation surface lives under.
    Muterprater,
    /// The module discovery lowering lives under.
    Discover,
    /// The module the benchmark receiver lives under.
    Bench,
    /// The stamp a rendered bench payload is written in the grammar of.
    BenchTableStamp,
    /// The road a namespaced reference is parsed by.
    Named,
    /// The road a row, a policy, a permission, or a budget set is declared by.
    Declared,
    /// The road a classification is taken as authored by.
    Authored,
    /// The road producer facts are stated by.
    Emitted,
    /// The road an executable attachment is bound by.
    Attached,
    /// The road a binding is married by.
    Bound,
    /// The road to the published root schema declaration.
    Published,
    /// The road from a root schema declaration to its derived identity.
    Identity,
    /// The road a work formula is taken over its declaration's bytes by.
    Encoded,
    /// The road an alternative is stated by.
    Stated,
    /// The road a mutation site is discovered by.
    Discovered,
    /// The road an evaluation observation is taken by.
    Observed,
    /// The road an operator family is resolved from its declared slug by.
    OfSlug,
    /// The road discovered sites are lowered by.
    LowerDiscoveries,
    /// The reader from a resolved selection to the point it stands at.
    Point,
    /// The reader from an identity to the name it carries.
    NameRoad,
    /// The reader from a name to the owner that declares it.
    NamespaceRoad,
    /// The reader from a name to the spelling it carries.
    StemRoad,
    /// The reader from a name part to the text it was written as.
    Written,
    /// The reader from a resolved selection to the alternative it selected.
    Alternative,
    /// The reader from an alternative to the operator family it belongs to.
    Family,
    /// The reader from an operator family to its declared slug.
    Slug,
    /// The reader from an alternative to its semantic operation bytes.
    Operation,
    /// The reader from a resolved selection to the selection itself.
    Selection,
    /// The reader from a directive to the selection it resolved, where it resolved one.
    Resolved,
    /// The root schema declaration a produced table pins against.
    Schema,
    /// The stamp's own refusal type, which a table's provenance expression maps into.
    TableRefusal,
    /// The claim a row serves.
    ClaimRef,
    /// The aggregate seat a row runs under.
    ExecutionSuite,
    /// One open classification a row carries.
    RoleRef,
    /// One open label a row carries beside its roles.
    TagRef,
    /// The two open rosters, as the harness carries them.
    Classification,
    /// The typed selection of what is under test.
    SubjectRoute,
    /// The check that judges the subject.
    CheckRef,
    /// The generated population that supplies a row's inputs.
    PopulationRef,
    /// The declaration door a generated row was authored through.
    DoorRef,
    /// The projection that emitted a generated row.
    ProjectionRef,
    /// What a producer's own act contributed to a generated row.
    ProducerFacts,
    /// Where a row came from.
    Origin,
    /// One row of the harness's denominator.
    RowType,
    /// What makes one row executable.
    Attachment,
    /// Whether a producer stands behind one binding, and which schema it emitted against.
    ProvenanceType,
    /// The producer that emitted a binding against a published schema.
    ProducerName,
    /// One row married to one executable attachment.
    BindingType,
    /// One row of the bench-row vocabulary.
    BenchRow,
    /// The four semantic references one benchmark row joins.
    BenchReferences,
    /// The input axis, budgets, contention posture, and optional formula one row declares.
    BenchMeasurement,
    /// The informed input-size axis.
    InputSizeAxis,
    /// What makes one benchmark row executable.
    BenchAttachment,
    /// The reference naming what is measured.
    WorkloadRef,
    /// The reference naming the correctness preflight.
    PreflightRef,
    /// The reference naming the planted-worse falsifier.
    PlantedWorseRef,
    /// The neutral reference a row's complexity claim is stated through.
    ComplexityClaimRef,
    /// One work observation a benchmark callable may record.
    WorkObservationRef,
    /// The declared contention posture's own type.
    ContentionPosture,
    /// The declared work formula's own type.
    WorkFormula,
    /// The gate's declared tolerances.
    DeclaredBudgets,
    /// One bench row married to the callables the host order invokes.
    BenchBinding,
    /// One complete benchmark report.
    BenchReport,
    /// How a namespaced reference refuses.
    NameRefusal,
    /// How a mutation permission refuses.
    PermissionRefusal,
    /// How a mutation policy refuses.
    PolicyRefusal,
    /// How a mutation discovery refuses.
    DiscoveryRefusal,
    /// How lowering a discovery refuses.
    DiscoveryLoweringRefusal,
    /// The evaluation family a generated surface belongs to.
    EvaluationFamilyRef,
    /// One operator family a permission names.
    OperatorFamilyRef,
    /// One claim's permission over a roster of operator families.
    MutationPermission,
    /// The complete policy a surface is lowered under.
    MutationPolicy,
    /// The lowering a surface hands back.
    MutationSurfaceLowering,
    /// The point a mutation is discovered at.
    MutationPointRef,
    /// The site an active selection is resolved against.
    ActivationSite,
    /// Whether an owner claim was mapped to the fact a discovery stands on.
    OwnerClaimMapping,
    /// One declared alternative at a discovered site.
    AlternativeDeclaration,
    /// One discovered mutation site.
    DiscoveredMutationSite,
    /// The directive an evaluation is called under.
    EvaluationDirective,
    /// What one evaluation observed.
    EvaluationObservation,
    /// How an evaluation call refuses.
    EvaluationCallRefusal,
    /// The stamp refusal arm a refused root schema declaration reaches.
    SchemaNotDeclared,
    /// The stamp refusal arm a refused schema encoding reaches.
    SchemaNotEncoded,
    /// The one origin arm a producer may emit.
    Generated,
    /// The provenance arm a produced binding carries.
    ProducedProvenance,
    /// The owner-claim arm a mapped discovery carries.
    Mapped,
    /// The owner-claim arm an unmapped discovery carries.
    OwnerUnmapped,
    /// The evaluation refusal arm an unimplemented active selection reaches.
    ActiveSelectionNotImplemented,
    /// The contention arm a measurement with no declared competing work carries.
    NoDeclaredContention,
}

/// One word the harness's stamp grammar reads, as a clause key or a seat name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessWord {
    /// The clause carrying a table's stated provenance.
    Provenance,
    /// The word a produced provenance opens with.
    Produced,
    /// The word the schema a table pins against follows.
    Against,
    /// The clause carrying the consumer's declared budgets.
    Invocation,
    /// The clause carrying the target and toolchain the runs stand on.
    Target,
    /// The clause carrying the wall-measurement source.
    Clock,
    /// The word one aggregate seat's group opens with.
    Suite,
    /// The clause each declared bench row is written under.
    Row,
    /// The roster of benchmark bindings one stamped table carries.
    Bindings,
    /// The target-owned report reader.
    Reporter,
    /// The target-owned measured callable for one benchmark row.
    Measured,
    /// The target-owned planted-worse callable for one benchmark row.
    PlantedWorse,
    /// The target-owned work judge binding for one benchmark row.
    Judge,
    /// The target-owned complete preflight trial for one benchmark row.
    Preflight,
    /// The attachment seat carrying one row's subject revision commitment.
    SubjectRevision,
    /// The attachment seat carrying one row's check revision commitment.
    CheckRevision,
    /// The attachment seat carrying the callable that reaches one row's conclusion.
    Call,
    /// The provenance seat naming the producer.
    Producer,
    /// The provenance seat naming the schema identity.
    Schema,
}

impl HarnessName {
    /// The spelling this name is published under.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Descriptor => "descriptor",
            Self::Muterprater => "muterprater",
            Self::Discover => "discover",
            Self::Bench => "bench",
            Self::BenchTableStamp => "bench_table",
            Self::Named => "named",
            Self::Declared => "declared",
            Self::Authored => "authored",
            Self::Emitted => "emitted",
            Self::Attached => "attached",
            Self::Bound => "bound",
            Self::Published => "published",
            Self::Identity => "identity",
            Self::Encoded => "encoded",
            Self::Stated => "stated",
            Self::Discovered => "discovered",
            Self::Observed => "observed",
            Self::OfSlug => "of_slug",
            Self::LowerDiscoveries => "lower_discoveries",
            Self::Point => "point",
            Self::NameRoad => "name",
            Self::NamespaceRoad => "namespace",
            Self::StemRoad => "stem",
            Self::Written => "written",
            Self::Alternative => "alternative",
            Self::Family => "family",
            Self::Slug => "slug",
            Self::Operation => "operation",
            Self::Selection => "selection",
            Self::Resolved => "resolved",
            Self::Schema => "GeneratedSupportSchema",
            Self::TableRefusal => "TrialTableRefusal",
            Self::ClaimRef => "ClaimRef",
            Self::ExecutionSuite => "ExecutionSuite",
            Self::RoleRef => "Role",
            Self::TagRef => "Tag",
            Self::Classification => "Classification",
            Self::SubjectRoute => "SubjectRoute",
            Self::CheckRef => "CheckRef",
            Self::PopulationRef => "PopulationRef",
            Self::DoorRef => "DoorRef",
            Self::ProjectionRef => "ProjectionRef",
            Self::ProducerFacts => "ProducerFacts",
            Self::Origin => "Origin",
            Self::RowType => "Row",
            Self::Attachment => "ExecutableAttachment",
            Self::ProvenanceType => "Provenance",
            Self::ProducerName => "ProducerName",
            Self::BindingType => "Binding",
            Self::BenchRow => "BenchRow",
            Self::BenchReferences => "BenchReferences",
            Self::BenchMeasurement => "BenchMeasurement",
            Self::InputSizeAxis => "InputSizeAxis",
            Self::BenchAttachment => "BenchAttachment",
            Self::WorkloadRef => "WorkloadRef",
            Self::PreflightRef => "PreflightRef",
            Self::PlantedWorseRef => "PlantedWorseRef",
            Self::ComplexityClaimRef => "ComplexityClaimRef",
            Self::WorkObservationRef => "WorkObservationRef",
            Self::ContentionPosture => "ContentionPosture",
            Self::WorkFormula => "WorkFormula",
            Self::DeclaredBudgets => "DeclaredBudgets",
            Self::BenchBinding => "BenchBinding",
            Self::BenchReport => "BenchReport",
            Self::NameRefusal => "NameRefusal",
            Self::PermissionRefusal => "PermissionRefusal",
            Self::PolicyRefusal => "PolicyRefusal",
            Self::DiscoveryRefusal => "DiscoveryRefusal",
            Self::DiscoveryLoweringRefusal => "DiscoveryLoweringRefusal",
            Self::EvaluationFamilyRef => "EvaluationFamilyRef",
            Self::OperatorFamilyRef => "OperatorFamilyRef",
            Self::MutationPermission => "MutationPermission",
            Self::MutationPolicy => "MutationPolicy",
            Self::MutationSurfaceLowering => "MutationSurfaceLowering",
            Self::MutationPointRef => "MutationPointRef",
            Self::ActivationSite => "ActivationSite",
            Self::OwnerClaimMapping => "OwnerClaimMapping",
            Self::AlternativeDeclaration => "AlternativeDeclaration",
            Self::DiscoveredMutationSite => "DiscoveredMutationSite",
            Self::EvaluationDirective => "EvaluationDirective",
            Self::EvaluationObservation => "EvaluationObservation",
            Self::EvaluationCallRefusal => "EvaluationCallRefusal",
            Self::SchemaNotDeclared => "SchemaNotDeclared",
            Self::SchemaNotEncoded => "SchemaNotEncoded",
            Self::Generated => "Generated",
            Self::ProducedProvenance => "Produced",
            Self::Mapped => "Mapped",
            Self::OwnerUnmapped => "OwnerUnmapped",
            Self::ActiveSelectionNotImplemented => "ActiveSelectionNotImplemented",
            Self::NoDeclaredContention => "NoDeclaredContention",
        }
    }
}

impl HarnessWord {
    /// The spelling this word is read under.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::Provenance => "provenance",
            Self::Produced => "produced",
            Self::Against => "against",
            Self::Invocation => "invocation",
            Self::Target => "target",
            Self::Clock => "clock",
            Self::Suite => "suite",
            Self::Row => "row",
            Self::Bindings => "bindings",
            Self::Reporter => "reporter",
            Self::Measured => "measured",
            Self::PlantedWorse => "planted_worse",
            Self::Judge => "judge",
            Self::Preflight => "preflight",
            Self::SubjectRevision => "subject_revision",
            Self::CheckRevision => "check_revision",
            Self::Call => "call",
            Self::Producer => "producer",
            Self::Schema => "schema",
        }
    }
}
