//! The semantic spellings used by generated harness paths, calls, clauses, and target-supplied seats.
//!
//! [`HarnessName`] answers where an expression lands, while [`HarnessWord`] answers how stamped material names one grammar seat.
//! The enums carry no private state because each admitted variant is already one closed destination spelling.

use super::stamp::vocabulary;

vocabulary! {
/// One name macroonz-harness publishes that a descriptor emission spells.
///
/// Every module, type, arm, and constructor road the three renderings write stands here, so the address a rendered expression resolves at is one table a reader can join against the harness's own declarations rather than a search through three renderers.
pub enum HarnessName {
    /// The module the descriptor vocabulary lives under.
    Descriptor = "descriptor",
    /// The module the mutation surface lives under.
    Muterprater = "muterprater",
    /// The module discovery lowering lives under.
    Discover = "discover",
    /// The module the benchmark receiver lives under.
    Bench = "bench",
    /// The stamp a rendered bench payload is written in the grammar of.
    BenchTableStamp = "bench_table",
    /// The road a namespaced reference is parsed by.
    Named = "named",
    /// The road a row, a policy, a permission, or a budget set is declared by.
    Declared = "declared",
    /// The road a classification is taken as authored by.
    Authored = "authored",
    /// The road producer facts are stated by.
    Emitted = "emitted",
    /// The road an executable attachment is bound by.
    Attached = "attached",
    /// The road a binding is married by.
    Bound = "bound",
    /// The road to the published root schema declaration.
    Published = "published",
    /// The road from a root schema declaration to its derived identity.
    Identity = "identity",
    /// The road a work formula is taken over its declaration's bytes by.
    Encoded = "encoded",
    /// The road an alternative is stated by.
    Stated = "stated",
    /// The road a mutation site is discovered by.
    Discovered = "discovered",
    /// The road an evaluation observation is taken by.
    Observed = "observed",
    /// The road an operator family is resolved from its declared slug by.
    OfSlug = "of_slug",
    /// The road discovered sites are lowered by.
    LowerDiscoveries = "lower_discoveries",
    /// The reader from a resolved selection to the point it stands at.
    Point = "point",
    /// The reader from an identity to the name it carries.
    NameRoad = "name",
    /// The reader from a name to the owner that declares it.
    NamespaceRoad = "namespace",
    /// The reader from a name to the spelling it carries.
    StemRoad = "stem",
    /// The reader from a name part to the text it was written as.
    Written = "written",
    /// The reader from a resolved selection to the alternative it selected.
    Alternative = "alternative",
    /// The reader from an alternative to the operator family it belongs to.
    Family = "family",
    /// The reader from an operator family to its declared slug.
    Slug = "slug",
    /// The reader from an alternative to its semantic operation bytes.
    Operation = "operation",
    /// The reader from a resolved selection to the selection itself.
    Selection = "selection",
    /// The reader from a directive to the selection it resolved, where it resolved one.
    Resolved = "resolved",
    /// The root schema declaration a produced table pins against.
    Schema = "GeneratedSupportSchema",
    /// The stamp's own refusal type, which a table's provenance expression maps into.
    TableRefusal = "TrialTableRefusal",
    /// The claim a row serves.
    ClaimRef = "ClaimRef",
    /// The aggregate seat a row runs under.
    ExecutionSuite = "ExecutionSuite",
    /// One open classification a row carries.
    RoleRef = "Role",
    /// One open label a row carries beside its roles.
    TagRef = "Tag",
    /// The two open rosters, as the harness carries them.
    Classification = "Classification",
    /// The typed selection of what is under test.
    SubjectRoute = "SubjectRoute",
    /// The check that judges the subject.
    CheckRef = "CheckRef",
    /// The generated population that supplies a row's inputs.
    PopulationRef = "PopulationRef",
    /// The declaration door a generated row was authored through.
    DoorRef = "DoorRef",
    /// The projection that emitted a generated row.
    ProjectionRef = "ProjectionRef",
    /// What a producer's own act contributed to a generated row.
    ProducerFacts = "ProducerFacts",
    /// Where a row came from.
    Origin = "Origin",
    /// One row of the harness's denominator.
    RowType = "Row",
    /// What makes one row executable.
    Attachment = "ExecutableAttachment",
    /// Whether a producer stands behind one binding, and which schema it emitted against.
    ProvenanceType = "Provenance",
    /// The producer that emitted a binding against a published schema.
    ProducerName = "ProducerName",
    /// One row married to one executable attachment.
    BindingType = "Binding",
    /// One row of the bench-row vocabulary.
    BenchRow = "BenchRow",
    /// The four semantic references one benchmark row joins.
    BenchReferences = "BenchReferences",
    /// The input axis, budgets, contention posture, and optional formula one row declares.
    BenchMeasurement = "BenchMeasurement",
    /// The informed input-size axis.
    InputSizeAxis = "InputSizeAxis",
    /// What makes one benchmark row executable.
    BenchAttachment = "BenchAttachment",
    /// The reference naming what is measured.
    WorkloadRef = "WorkloadRef",
    /// The reference naming the correctness preflight.
    PreflightRef = "PreflightRef",
    /// The reference naming the planted-worse falsifier.
    PlantedWorseRef = "PlantedWorseRef",
    /// The neutral reference a row's complexity claim is stated through.
    ComplexityClaimRef = "ComplexityClaimRef",
    /// One work observation a benchmark callable may record.
    WorkObservationRef = "WorkObservationRef",
    /// The declared contention posture's own type.
    ContentionPosture = "ContentionPosture",
    /// The declared work formula's own type.
    WorkFormula = "WorkFormula",
    /// The gate's declared tolerances.
    DeclaredBudgets = "DeclaredBudgets",
    /// One bench row married to the callables the host order invokes.
    BenchBinding = "BenchBinding",
    /// One complete benchmark report.
    BenchReport = "BenchReport",
    /// How a namespaced reference refuses.
    NameRefusal = "NameRefusal",
    /// How a mutation permission refuses.
    PermissionRefusal = "PermissionRefusal",
    /// How a mutation policy refuses.
    PolicyRefusal = "PolicyRefusal",
    /// How a mutation discovery refuses.
    DiscoveryRefusal = "DiscoveryRefusal",
    /// How lowering a discovery refuses.
    DiscoveryLoweringRefusal = "DiscoveryLoweringRefusal",
    /// The evaluation family a generated surface belongs to.
    EvaluationFamilyRef = "EvaluationFamilyRef",
    /// One operator family a permission names.
    OperatorFamilyRef = "OperatorFamilyRef",
    /// One claim's permission over a roster of operator families.
    MutationPermission = "MutationPermission",
    /// The complete policy a surface is lowered under.
    MutationPolicy = "MutationPolicy",
    /// The lowering a surface hands back.
    MutationSurfaceLowering = "MutationSurfaceLowering",
    /// The point a mutation is discovered at.
    MutationPointRef = "MutationPointRef",
    /// The site an active selection is resolved against.
    ActivationSite = "ActivationSite",
    /// Whether an owner claim was mapped to the fact a discovery stands on.
    OwnerClaimMapping = "OwnerClaimMapping",
    /// One declared alternative at a discovered site.
    AlternativeDeclaration = "AlternativeDeclaration",
    /// One discovered mutation site.
    DiscoveredMutationSite = "DiscoveredMutationSite",
    /// The directive an evaluation is called under.
    EvaluationDirective = "EvaluationDirective",
    /// What one evaluation observed.
    EvaluationObservation = "EvaluationObservation",
    /// How an evaluation call refuses.
    EvaluationCallRefusal = "EvaluationCallRefusal",
    /// The stamp refusal arm a refused root schema declaration reaches.
    SchemaNotDeclared = "SchemaNotDeclared",
    /// The stamp refusal arm a refused schema encoding reaches.
    SchemaNotEncoded = "SchemaNotEncoded",
    /// The one origin arm a producer may emit.
    Generated = "Generated",
    /// The provenance arm a produced binding carries.
    ProducedProvenance = "Produced",
    /// The owner-claim arm a mapped discovery carries.
    Mapped = "Mapped",
    /// The owner-claim arm an unmapped discovery carries.
    OwnerUnmapped = "OwnerUnmapped",
    /// The evaluation refusal arm an unimplemented active selection reaches.
    ActiveSelectionNotImplemented = "ActiveSelectionNotImplemented",
    /// The contention arm a measurement with no declared competing work carries.
    NoDeclaredContention = "NoDeclaredContention",
}
spelling = "The spelling this name is published under.";
}

vocabulary! {
/// One word the harness's stamp grammar reads, as a clause key or a seat name.
pub enum HarnessWord {
    /// The clause carrying a table's stated provenance.
    Provenance = "provenance",
    /// The word a produced provenance opens with.
    Produced = "produced",
    /// The word the schema a table pins against follows.
    Against = "against",
    /// The clause carrying the consumer's declared budgets.
    Invocation = "invocation",
    /// The clause carrying the target and toolchain the runs stand on.
    Target = "target",
    /// The clause carrying the wall-measurement source.
    Clock = "clock",
    /// The word one aggregate seat's group opens with.
    Suite = "suite",
    /// The clause each declared bench row is written under.
    Row = "row",
    /// The roster of benchmark bindings one stamped table carries.
    Bindings = "bindings",
    /// The target-owned report reader.
    Reporter = "reporter",
    /// The target-owned measured callable for one benchmark row.
    Measured = "measured",
    /// The target-owned planted-worse callable for one benchmark row.
    PlantedWorse = "planted_worse",
    /// The target-owned work judge binding for one benchmark row.
    Judge = "judge",
    /// The target-owned complete preflight trial for one benchmark row.
    Preflight = "preflight",
    /// The attachment seat carrying one row's subject revision commitment.
    SubjectRevision = "subject_revision",
    /// The attachment seat carrying one row's check revision commitment.
    CheckRevision = "check_revision",
    /// The attachment seat carrying the callable that reaches one row's conclusion.
    Call = "call",
    /// The provenance seat naming the producer.
    Producer = "producer",
    /// The provenance seat naming the schema identity.
    Schema = "schema",
}
spelling = "The spelling this word is read under.";
}
