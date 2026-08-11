//! Deriving a refusal family's declared facts: capture, plan, render, explain.
//!
//! # Ordinary callable Rust, and nothing else
//!
//! Nothing in this module knows a proc-macro exists. [`captured`] takes source
//! TEXT and returns a typed surface or a typed refusal; everything downstream
//! takes typed values. The Rust-facing shell is one caller of these functions;
//! a test is another; a future language frontend would be a third. A diagnostic
//! from here names [`ReproductionRoute::CallableServices`] because that route is
//! real.
//!
//! # The grammar is AUTHORED, and it is small
//!
//! ```text
//! #[refusal(shape = <shape-word>, order(<Variant> = "<stable-id>", ...))]
//! enum <FamilyName> { <Variant>, ... }
//! ```
//!
//! - `<shape-word>` is one of `single_cause`, `issue_collection`,
//!   `inseparable_pair`. The words map onto the machine's own
//!   [`FamilyShape`] roster; this module carries the spelling of the words and
//!   not a second roster of shapes.
//! - `order(...)` states the canonical selection order by naming each variant
//!   and the **stable identity** that variant's cause carries. The order clause
//!   is required exactly when the shape is `single_cause` and admitted only
//!   then, because band 00 declares the canonical order for that shape alone.
//!   Its order is the *selector's* order and need not match the order the
//!   variants happen to be written in.
//! - Variants carry nothing but their own names, and a stable identity is a
//!   quoted text with no escape sequence in it. Both restrictions exist so that
//!   what is captured can be rendered back without a quoting question ever
//!   arising.
//!
//! The caller never writes the selection-order STRINGS. It writes variants and
//! identities; the textual projection is emitted from the typed rows, which is
//! the whole point of band 00's split between a cause's identity and its
//! spelling.
//!
//! # Plan before render
//!
//! [`RefusalDeriveSurface::planned`] is the only road to a
//! [`RefusalFamilyDerivation`], and a derivation is the only thing that renders.
//! The declared output set is fixed before a byte of Rust exists, and
//! [`RenderedDerivation::membership`] is the same value the derivation declared
//! — the closure the services owe, stated as one equality rather than as a
//! promise.
//!
//! # What this module does not decide
//!
//! It decides no meaning. The three body shapes are band 00's; the selection
//! order's *content* is the author's; the stable identities are the author's;
//! the `RefusalFamily` and `CauseOrderDeclaration` contracts are band 00's. This
//! module reads a declaration and writes down what it already said.

use crate::diagnostics::{
    MacrocDiagnostic, MacrocPhase, ObservedClassification, ReleasePosture, RepairAction,
    ReproductionRoute,
};
use crate::explanation_protocol::{
    ExplanationAnswer, ExplanationCoverage, ProjectionExplanation, ProjectionExplanationView,
};
use crate::origin_graph::{
    DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
};
use crate::plane::{
    AssumptionLimit, ContractSubject, DeriveCauseLimit, DeriveSourceLimit, DerivedTypeSubject,
    ExactIdentity, GeneratedUnitSubject, HumanProjection, HumanTextLimit,
    ImplementedContractSubject, OriginNodeSubject, OutputBytesSubject, OwnerFactRef,
    ProjectionKindSubject, RefusalFamilySubject, RefusalReason, ServiceEntrySubject, TracedSubject,
};
use crate::planning::{
    DeriveImplContent, DeriveImplProjection, InvalidationTrigger, OutputIdentity,
    PlannedMembership, ProjectionContext, ProjectionDisposition, ProjectionPlan,
};
use crate::refusal::ProjectionPlanning;
use threadpak::declaration::types::{FragmentIdentityDomain, LinkedGraphDomain, SymbolDomain};
use threadpak::declaration::{CoordinateRole, SourceCoordinate};
use threadpak::evidence::CauseDisposition;
use threadpak::refusal::{
    CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape, RefusalFamily,
};
use threadpak::types::{Bounded, ConstLimit};

// ---------------------------------------------------------------------------
// The captured surface.
// ---------------------------------------------------------------------------

/// The authored shape word for a single-cause family.
pub const SHAPE_WORD_SINGLE_CAUSE: &str = "single_cause";

/// The authored shape word for an issue-collection family.
pub const SHAPE_WORD_ISSUE_COLLECTION: &str = "issue_collection";

/// The authored shape word for an inseparable-pair family.
pub const SHAPE_WORD_INSEPARABLE_PAIR: &str = "inseparable_pair";

/// One cause as the capture read it: the Rust variant that spells it and the
/// stable identity the author declared for it.
///
/// The owned text is the parse frontier's carrier and nothing more. It is not a
/// second answer to "what is a cause identity?" — that answer is band 00's
/// [`CauseId`], and the rendering emits exactly that type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedCause {
    spelling: String,
    stable_id: String,
}

impl CapturedCause {
    /// The Rust variant that spells this cause.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    /// The stable identity the author declared for this cause.
    #[must_use]
    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }
}

/// One refusal-family declaration, captured from source text.
///
/// The causes are non-empty exactly when the shape is
/// [`FamilyShape::SingleCause`]; the other two shapes declare no canonical
/// order, so there is nothing here to carry for them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalDeriveSurface {
    family_name: String,
    shape: FamilyShape,
    causes: Bounded<CapturedCause, DeriveCauseLimit>,
}

impl RefusalDeriveSurface {
    /// The declared family's Rust name.
    #[must_use]
    pub fn family_name(&self) -> &str {
        &self.family_name
    }

    /// The declared body shape.
    #[must_use]
    pub const fn shape(&self) -> FamilyShape {
        self.shape
    }

    /// The declared causes, in the canonical selection order the author stated.
    pub fn causes(&self) -> impl Iterator<Item = &CapturedCause> {
        self.causes.iter()
    }

    /// Plan the derivation: fix the complete declared output set before
    /// anything is rendered.
    ///
    /// Total: every captured surface has a lawful membership, because the shape
    /// alone decides it. This is the one road to a [`RefusalFamilyDerivation`],
    /// which is the one thing that renders.
    #[must_use]
    pub fn planned(self) -> RefusalFamilyDerivation {
        let membership = match self.shape {
            FamilyShape::SingleCause => DerivedMembership::FamilyAndCauseOrder,
            FamilyShape::IssueCollection | FamilyShape::InseparablePair => {
                DerivedMembership::FamilyOnly
            }
        };
        RefusalFamilyDerivation {
            surface: self,
            membership,
        }
    }
}

// ---------------------------------------------------------------------------
// The capture refusal family.
// ---------------------------------------------------------------------------

/// The single-cause family for capturing a refusal-family declaration.
///
/// Single cause because the checks are dependent: there is no shape word to
/// admit until an attribute was found, no coverage to check until both the
/// order clause and the body were read, and no distinctness to check until the
/// identities were parsed. Claiming a result from a check that never ran is
/// unrepresentable here, which is exactly what the shape is for.
///
/// The canonical order below is the SELECTOR's order, not the execution
/// schedule: the magnitude check runs first and is selected last, because a
/// declaration that is not an enum at all is the more fundamental answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefusalDeriveCapture {
    /// The declared input is not an enum declaration this grammar admits: no
    /// `enum` keyword, an unbalanced body, an unterminated or escaped text
    /// literal, or a variant carrying anything beyond its own name.
    NotAnEnum,
    /// The `enum` keyword is not followed by a name.
    NotNamed,
    /// The enum body declares no variant.
    NotInhabited,
    /// No `#[refusal(shape = ...)]` was declared.
    NotShapeDeclared,
    /// The declared shape word is none of the three the machine's roster
    /// admits.
    NotAnAdmittedShape,
    /// The shape is `single_cause` and no `order(...)` clause was declared.
    NotOrderDeclared,
    /// The shape declares no canonical cause order and an `order(...)` clause
    /// was declared anyway.
    NotOrderAdmitted,
    /// The order clause and the enum body do not name the same causes.
    NotCovered,
    /// Two declared causes carry the same stable identity.
    NotDistinct,
    /// The declared input exceeds a declared magnitude.
    Unbounded,
}

impl RefusalFamily for RefusalDeriveCapture {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "NotAnEnum",
        "NotNamed",
        "NotInhabited",
        "NotShapeDeclared",
        "NotAnAdmittedShape",
        "NotOrderDeclared",
        "NotOrderAdmitted",
        "NotCovered",
        "NotDistinct",
        "Unbounded",
    ];
}

/// Hand-declared, and deliberately so: the services never derive their own
/// contracts. A generator that produced its own declared facts would be its own
/// oracle, and the parity this module is qualified by would compare it to
/// itself.
impl CauseOrderDeclaration for RefusalDeriveCapture {
    const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-an-enum"),
            "NotAnEnum",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-named"),
            "NotNamed",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-inhabited"),
            "NotInhabited",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-shape-declared"),
            "NotShapeDeclared",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-an-admitted-shape"),
            "NotAnAdmittedShape",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-order-declared"),
            "NotOrderDeclared",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-order-admitted"),
            "NotOrderAdmitted",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-covered"),
            "NotCovered",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.not-distinct"),
            "NotDistinct",
        ),
        DeclaredCause::declared(
            CauseId::declared("macroc.refusal-derive-capture.unbounded"),
            "Unbounded",
        ),
    ]);
}

impl RefusalDeriveCapture {
    /// How what was found differs from the contract that was expected.
    #[must_use]
    pub const fn observed(self) -> ObservedClassification {
        match self {
            Self::NotAnEnum
            | Self::NotAnAdmittedShape
            | Self::NotOrderAdmitted
            | Self::NotCovered => ObservedClassification::ContractDisagreement,
            Self::NotNamed
            | Self::NotInhabited
            | Self::NotShapeDeclared
            | Self::NotOrderDeclared => ObservedClassification::SeatAbsent,
            Self::NotDistinct => ObservedClassification::IdentityDisagreement,
            Self::Unbounded => ObservedClassification::BoundExceeded,
        }
    }

    /// The cause rendered for a person. A projection of the typed value:
    /// nothing reads it back, and no decision consults it.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::NotAnEnum => "the declared input is not an enum declaration this grammar admits",
            Self::NotNamed => "the `enum` keyword is not followed by a name",
            Self::NotInhabited => "the enum body declares no variant",
            Self::NotShapeDeclared => "no `#[refusal(shape = ...)]` was declared",
            Self::NotAnAdmittedShape => {
                "the declared shape word is none of `single_cause`, `issue_collection`, \
                 `inseparable_pair`"
            }
            Self::NotOrderDeclared => "a `single_cause` family declares no `order(...)` clause",
            Self::NotOrderAdmitted => {
                "this shape declares no canonical cause order, and an `order(...)` clause was \
                 declared anyway"
            }
            Self::NotCovered => "the `order(...)` clause and the enum body name different causes",
            Self::NotDistinct => "two declared causes carry the same stable identity",
            Self::Unbounded => "the declared input exceeds a declared magnitude",
        }
    }
}

/// One capture refusal: the established cause and where in the declared input
/// it sits.
///
/// Both seats are required. A refusal that could omit its coordinate would send
/// the caller looking, and a refusal that could omit its cause would be a
/// complaint rather than an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalDeriveRefusal {
    cause: RefusalDeriveCapture,
    coordinate: SourceCoordinate,
}

impl RefusalDeriveRefusal {
    /// The established cause.
    #[must_use]
    pub const fn cause(self) -> RefusalDeriveCapture {
        self.cause
    }

    /// Where in the declared input the observation sits, under its declared
    /// coordinate role.
    #[must_use]
    pub const fn coordinate(self) -> SourceCoordinate {
        self.coordinate
    }

    /// The compiler-facing rendering: one line naming the cause and the byte
    /// position it was established at.
    ///
    /// A projection of the typed value, produced here so that the expansion
    /// shell composes no sentence of its own.
    #[must_use]
    pub fn compiler_message(self) -> String {
        let described = self.cause.described();
        let position = self.coordinate.position;
        format!("threadpak refusal-family derive: {described} (at source byte {position})")
    }

    /// Project this refusal into the services' structured diagnostic.
    ///
    /// The identities are the caller's: the registered reason, the family, the
    /// declaring symbol, the fragment, the closed graph, the expected contract,
    /// the machine's cause posture, and the entry point that reproduces the
    /// observation. This module names none of them, because none of them is its
    /// to mint — the services classify what they OBSERVED
    /// ([`RefusalDeriveCapture::observed`]) and never mint the machine's cause
    /// commitment.
    #[must_use]
    pub fn diagnosed(self, anchors: &CaptureDiagnosticAnchors) -> MacrocDiagnostic {
        let description = HumanProjection::<HumanTextLimit>::projected(self.cause.described())
            .unwrap_or_else(|_| HumanProjection::empty());
        let repairs = Bounded::admitted_const(vec![RepairAction {
            declared_by: anchors.repair_declared_by,
            description,
        }])
        .unwrap_or_else(|_| Bounded::empty());
        MacrocDiagnostic {
            reason: anchors.reason,
            family: anchors.family,
            phase: MacrocPhase::Capture,
            coordinate: self.coordinate,
            declaration: anchors.declaration,
            fragment: anchors.fragment,
            graph: anchors.graph,
            expected: anchors.expected,
            observed: self.cause.observed(),
            cause: anchors.posture.clone(),
            related: Bounded::empty(),
            repairs,
            reproduction: ReproductionRoute::CallableServices {
                entry: anchors.entry,
            },
            release: ReleasePosture::NoReleasePromise,
        }
    }

    /// The established refusal at one byte position of the declared input.
    fn established(cause: RefusalDeriveCapture, at: u64) -> Self {
        Self {
            cause,
            coordinate: SourceCoordinate {
                role: CoordinateRole::Byte,
                position: at,
            },
        }
    }
}

/// The exact identities one capture diagnostic is projected against.
///
/// Every seat names something the machine owns. The services read them and
/// adapt nothing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaptureDiagnosticAnchors {
    /// The registered reason, as the machine's refusal home published it.
    pub reason: ExactIdentity<RefusalReason>,
    /// The refusal family that owns the reason.
    pub family: ExactIdentity<RefusalFamilySubject>,
    /// The declaring symbol.
    pub declaration: ExactIdentity<SymbolDomain>,
    /// The declaration fragment involved.
    pub fragment: ExactIdentity<FragmentIdentityDomain>,
    /// The closed graph the observation was made against.
    pub graph: ExactIdentity<LinkedGraphDomain>,
    /// The contract that was expected to hold.
    pub expected: ExactIdentity<ContractSubject>,
    /// The machine's cause posture. Narrowing is progress, never a forced
    /// verdict, and the services never elect one: the caller supplies the
    /// posture its investigation reached.
    pub posture: CauseDisposition,
    /// The callable entry point that reproduces the observation.
    pub entry: ExactIdentity<ServiceEntrySubject>,
    /// The owner fact that declares the repair.
    pub repair_declared_by: OwnerFactRef,
}

// ---------------------------------------------------------------------------
// The declared output set.
// ---------------------------------------------------------------------------

/// One item a refusal-family derivation may declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivedItem {
    /// The `RefusalFamily` implementation: shape and textual selection order.
    FamilyImplementation,
    /// The `CauseOrderDeclaration` implementation: the typed cause order.
    CauseOrderImplementation,
}

/// The complete declared output set of one derivation.
///
/// A closed sum rather than a bounded collection, because the set is decided by
/// the shape and there are exactly two answers. Neither answer is empty: a
/// derivation that would generate nothing is a disposition, not a derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivedMembership {
    /// The family implementation alone — the shape declares no cause order.
    FamilyOnly,
    /// The family implementation and the cause-order implementation.
    FamilyAndCauseOrder,
}

impl DerivedMembership {
    /// The declared items, in the order they are rendered.
    #[must_use]
    pub const fn items(self) -> &'static [DerivedItem] {
        match self {
            Self::FamilyOnly => &[DerivedItem::FamilyImplementation],
            Self::FamilyAndCauseOrder => &[
                DerivedItem::FamilyImplementation,
                DerivedItem::CauseOrderImplementation,
            ],
        }
    }

    /// The number of declared items; structurally at least one.
    #[must_use]
    pub const fn len(self) -> usize {
        self.items().len()
    }

    /// Always `false`: an empty declared output set is unrepresentable here.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        false
    }
}

/// Whether one derivation carries the typed cause order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CauseOrderStanding {
    /// The shape declares a canonical cause order, and the derivation carries
    /// it.
    Declared,
    /// The shape declares none — band 00 rules the canonical order for
    /// single-cause families alone.
    NotApplicableToShape,
}

// ---------------------------------------------------------------------------
// The derivation: plan, render, project, explain.
// ---------------------------------------------------------------------------

/// One planned refusal-family derivation.
///
/// Holding one means the declared output set is already fixed. Rendering reads
/// it; the identity-bearing projection reads it; neither invents a member.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalFamilyDerivation {
    surface: RefusalDeriveSurface,
    membership: DerivedMembership,
}

impl RefusalFamilyDerivation {
    /// The captured surface this derivation was planned from.
    #[must_use]
    pub const fn surface(&self) -> &RefusalDeriveSurface {
        &self.surface
    }

    /// The complete declared output set.
    #[must_use]
    pub const fn declared_membership(&self) -> DerivedMembership {
        self.membership
    }

    /// Whether the typed cause order stands for this family's shape.
    #[must_use]
    pub const fn cause_order_standing(&self) -> CauseOrderStanding {
        match self.membership {
            DerivedMembership::FamilyAndCauseOrder => CauseOrderStanding::Declared,
            DerivedMembership::FamilyOnly => CauseOrderStanding::NotApplicableToShape,
        }
    }

    /// Render the declared output set as Rust source text.
    ///
    /// The textual selection order is emitted from the typed rows, never from
    /// anything the caller wrote as a string.
    #[must_use]
    pub fn rendered(&self) -> RenderedDerivation {
        RenderedDerivation {
            source: render_source(&self.surface, self.membership, None),
            membership: self.membership,
        }
    }

    /// Render the declared output set with one deliberate defect planted in it.
    ///
    /// # A qualification seam, and only that
    ///
    /// This road exists so that testpak has something to catch. It is public
    /// rather than test-gated because the judge lives in another package, and it
    /// is named for exactly what it does so that no caller reaches it by
    /// accident. Every macro family in this repository owes a planted defective
    /// expansion; this is this family's.
    ///
    /// The defect is in the RENDERING only. The captured surface and the typed
    /// order are untouched, which is what makes the defect detectable: the
    /// rendered text and the declaration it claims to project disagree.
    #[must_use]
    pub fn rendered_with_planted_defect(&self, defect: PlantedDefect) -> RenderedDerivation {
        RenderedDerivation {
            source: render_source(&self.surface, self.membership, Some(defect)),
            membership: self.membership,
        }
    }

    /// Project this derivation into an identity-bearing plan and the
    /// disposition of the cause-order projection.
    ///
    /// The plan's membership is the derivation's declared membership; the trail
    /// walks back to the authored declaration; the trace records the decisions
    /// in selection order, each citing a band 00 fact; the watch set names the
    /// identities whose change makes the plan stale.
    ///
    /// # Errors
    ///
    /// Returns the planning family when a declared magnitude is exceeded while
    /// assembling the cause set, the output set, the trail, the trace, or the
    /// watch set.
    pub fn projected(
        &self,
        anchors: &DerivationAnchors,
    ) -> Result<DerivedProjection, ProjectionPlanning> {
        let family_output = OutputIdentity {
            unit: anchors.family_unit,
            digest: anchors.family_digest,
            origin: OriginTrail::from_edge(OriginEdge {
                from: anchors.authored_node,
                relation: OriginRelation::SemanticDerivation,
                to: anchors.family_node,
            }),
        };
        let order_output = OutputIdentity {
            unit: anchors.order_unit,
            digest: anchors.order_digest,
            origin: OriginTrail::from_edge(OriginEdge {
                from: anchors.authored_node,
                relation: OriginRelation::SemanticDerivation,
                to: anchors.order_node,
            }),
        };
        let standing = self.cause_order_standing();
        let membership = match standing {
            CauseOrderStanding::Declared => {
                PlannedMembership::declared(family_output.clone(), vec![order_output.clone()])?
            }
            CauseOrderStanding::NotApplicableToShape => {
                PlannedMembership::from_output(family_output.clone())
            }
        };
        let shape_entry = TraceEntry {
            subject: anchors.traced,
            decision: TraceDecision::SelectedBecause(anchors.owner_facts.body_shapes),
        };
        let order_entry = TraceEntry {
            subject: anchors.traced,
            decision: match standing {
                CauseOrderStanding::Declared => TraceDecision::SelectedBecause(
                    anchors.owner_facts.canonical_order_is_shape_ruled,
                ),
                CauseOrderStanding::NotApplicableToShape => TraceDecision::OmittedBecause(
                    anchors.owner_facts.canonical_order_is_shape_ruled,
                ),
            },
        };
        let trace = DecisionTrace::recorded(shape_entry, vec![order_entry])?;
        let invalidation = InvalidationTrigger::watched(
            InvalidationTrigger::SourceDeclarationChanged {
                watched: *anchors.context.sources.first(),
            },
            vec![
                InvalidationTrigger::GraphIdentityChanged {
                    watched: anchors.context.graph,
                },
                InvalidationTrigger::GeneratorVersionChanged {
                    watched: anchors.context.generator,
                },
            ],
        )?;
        let assumptions = Bounded::<OwnerFactRef, AssumptionLimit>::admitted_const(vec![
            anchors.owner_facts.body_shapes,
            anchors.owner_facts.canonical_order_is_shape_ruled,
        ])
        .unwrap_or_else(|_| Bounded::empty());
        let plan = ProjectionPlan::<DeriveImplProjection>::planned(
            anchors.context.clone(),
            DeriveImplContent {
                derived_type: anchors.derived_type,
                contract: anchors.family_contract,
                assumptions,
            },
            membership,
            invalidation,
            trace,
            OriginTrail::from_edge(OriginEdge {
                from: anchors.authored_node,
                relation: OriginRelation::AuthoredDeclaration,
                to: anchors.family_node,
            }),
            Bounded::empty(),
        )?;
        let cause_order = match standing {
            CauseOrderStanding::Declared => ProjectionDisposition::Generated {
                output: order_output,
            },
            CauseOrderStanding::NotApplicableToShape => ProjectionDisposition::NotApplicable {
                because: anchors.owner_facts.canonical_order_is_shape_ruled,
            },
        };
        Ok(DerivedProjection { plan, cause_order })
    }

    /// Answer the explanation protocol over this derivation's projection.
    ///
    /// Nine seats: the eight every kind owes, plus the assumptions this kind
    /// declares. The why-NOT-generated seat is answered by the cause-order
    /// disposition — where the shape declares no canonical order, the answer
    /// names the band 00 fact rather than saying nothing.
    ///
    /// # Errors
    ///
    /// Returns [`ExplanationCoverage`] naming every unanswered, doubled, or
    /// inadmissible seat.
    pub fn explained(
        &self,
        projection: &DerivedProjection,
        anchors: &DerivationAnchors,
    ) -> Result<ProjectionExplanationView<DeriveImplProjection>, ExplanationCoverage> {
        let human = |text: &str| {
            HumanProjection::<HumanTextLimit>::projected(text)
                .unwrap_or_else(|_| HumanProjection::empty())
        };
        let plan = &projection.plan;
        ProjectionExplanationView::<DeriveImplProjection>::complete(vec![
            ProjectionExplanation::answered(
                ExplanationAnswer::Kind { kind: anchors.kind },
                human("an implementation projection over a declared refusal family"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Owner {
                    owner: anchors.owner_facts.body_shapes,
                },
                human("the refusal home requires a declared body shape"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::CausingDeclarations {
                    sources: plan.context().sources.clone(),
                },
                human("the enum declaration the caller wrote"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::GraphAndProfile {
                    graph: plan.context().graph,
                    profile: plan.context().profile,
                    version: plan.context().profile_version,
                },
                human("the closed graph and the selected projection profile"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::OutputAndDigest {
                    output: plan.membership().first().clone(),
                },
                human("the family implementation and the bytes it commits to"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::AssumptionsAndSpecializations {
                    assumptions: plan.content().assumptions.clone(),
                },
                human("the refusal home's shape and order facts"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Invalidators {
                    triggers: plan.invalidation().clone(),
                },
                human("the declaration, the graph, and the generator version"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::RelatedProjectionDisposition {
                    related: anchors.kind,
                    disposition: projection.cause_order.clone(),
                },
                human("what happened to the typed cause-order projection"),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Repairs {
                    repairs: Bounded::empty(),
                },
                human("nothing was refused, so no repair applies"),
            ),
        ])
    }
}

/// One rendered derivation: the Rust source text, and the declared output set it
/// materializes.
///
/// The membership rides alongside because the closure the services owe is an
/// equality, not a promise: what was rendered is what was planned.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderedDerivation {
    source: String,
    membership: DerivedMembership,
}

impl RenderedDerivation {
    /// The rendered Rust source text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// The declared output set this rendering materializes.
    #[must_use]
    pub const fn membership(&self) -> DerivedMembership {
        self.membership
    }
}

/// One deliberate defect a qualification rendering may carry.
///
/// Each one is a lie the rendered text tells about the declaration it projects,
/// and each is a lie a judge outside the services must catch. Neither variant
/// makes the rendering fail to compile: a defect that the compiler catches
/// proves nothing about whether anybody is watching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlantedDefect {
    /// The textual selection order is emitted in reverse while the typed order
    /// stands as declared — the projection no longer projects.
    SelectionOrderPermuted,
    /// Every cause is emitted under the first cause's stable identity —
    /// distinct causes made to share one identity.
    CauseIdentityRecycled,
}

/// The owner facts one refusal-family derivation cites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalOwnerFacts {
    /// The refusal home's fact that a family's body is one of exactly three
    /// shapes.
    pub body_shapes: OwnerFactRef,
    /// The refusal home's fact that the canonical cause order stands for
    /// single-cause families and for no other shape.
    pub canonical_order_is_shape_ruled: OwnerFactRef,
}

/// The exact identities one refusal-family derivation is projected against.
///
/// Every seat names something the machine owns — the closed graph, the profile,
/// the causing declaration, the derived type, the realized contract, the
/// generated units and their bytes, the origin nodes, the traced subject, and
/// the owner facts. The services mint none of them. In this phase the machine
/// mints identities only inside its own laws, so this record is reachable only
/// where such identities exist; that is a fact about the phase, not about the
/// shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DerivationAnchors {
    /// The shared plan context.
    pub context: ProjectionContext,
    /// This projection kind's identity.
    pub kind: ExactIdentity<ProjectionKindSubject>,
    /// The type the implementation is derived for.
    pub derived_type: ExactIdentity<DerivedTypeSubject>,
    /// The `RefusalFamily` contract the implementation realizes.
    pub family_contract: ExactIdentity<ImplementedContractSubject>,
    /// The authored declaration, as an origin node.
    pub authored_node: ExactIdentity<OriginNodeSubject>,
    /// The family implementation, as an origin node.
    pub family_node: ExactIdentity<OriginNodeSubject>,
    /// The family implementation's generated unit.
    pub family_unit: ExactIdentity<GeneratedUnitSubject>,
    /// That unit's canonical bytes.
    pub family_digest: ExactIdentity<OutputBytesSubject>,
    /// The cause-order implementation, as an origin node.
    pub order_node: ExactIdentity<OriginNodeSubject>,
    /// The cause-order implementation's generated unit.
    pub order_unit: ExactIdentity<GeneratedUnitSubject>,
    /// That unit's canonical bytes.
    pub order_digest: ExactIdentity<OutputBytesSubject>,
    /// The subject the plan's decisions are recorded about.
    pub traced: ExactIdentity<TracedSubject>,
    /// The owner facts the derivation rests on.
    pub owner_facts: RefusalOwnerFacts,
}

/// One projected derivation: the plan, and what happened to the cause-order
/// projection.
///
/// The second seat is the why-NOT-generated answer, carried beside the plan
/// rather than left for a caller to infer from a shorter membership.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedProjection {
    plan: ProjectionPlan<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
}

impl DerivedProjection {
    /// The identity-bearing plan.
    #[must_use]
    pub const fn plan(&self) -> &ProjectionPlan<DeriveImplProjection> {
        &self.plan
    }

    /// What happened to the typed cause-order projection.
    #[must_use]
    pub const fn cause_order(&self) -> &ProjectionDisposition {
        &self.cause_order
    }
}

// ---------------------------------------------------------------------------
// The two callable entry points.
// ---------------------------------------------------------------------------

/// What happened to one refusal-family derive request.
///
/// Silence is not a variant: a request either produced a rendering or was
/// refused with an established cause and a coordinate.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefusalDeriveDisposition {
    /// The implementation was generated; this is the rendering.
    Generated(RenderedDerivation),
    /// It was refused at capture.
    Refused(RefusalDeriveRefusal),
}

/// Capture one refusal-family declaration from source text.
///
/// # Errors
///
/// Returns [`RefusalDeriveRefusal`] carrying the established
/// [`RefusalDeriveCapture`] cause and the byte position it was established at.
pub fn captured(source: &str) -> Result<RefusalDeriveSurface, RefusalDeriveRefusal> {
    capture(source)
}

/// Capture, plan, and render one refusal-family declaration in one call — the
/// road an expansion surface takes.
///
/// Rendering is still reachable only through a plan: this function builds the
/// derivation and asks it to render, exactly as any other caller would.
#[must_use]
pub fn disposed(source: &str) -> RefusalDeriveDisposition {
    match capture(source) {
        Ok(surface) => RefusalDeriveDisposition::Generated(surface.planned().rendered()),
        Err(refusal) => RefusalDeriveDisposition::Refused(refusal),
    }
}

// ---------------------------------------------------------------------------
// The bounded hand-rolled reader.
// ---------------------------------------------------------------------------

/// One lexical token of the declared input.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// An identifier-shaped word.
    Word(String),
    /// A quoted text, without its quotes.
    Text(String),
    /// Any other single character.
    Mark(char),
}

/// One token and the byte position it starts at.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Lexed {
    token: Token,
    at: u64,
}

/// The byte position of one token, or zero where there is no such token.
fn position_at(tokens: &[Lexed], index: usize) -> u64 {
    tokens.get(index).map_or(0, |lexed| lexed.at)
}

/// The word one token spells, where it is a word.
fn word_at(tokens: &[Lexed], index: usize) -> Option<&str> {
    match tokens.get(index).map(|lexed| &lexed.token) {
        Some(Token::Word(word)) => Some(word.as_str()),
        _ => None,
    }
}

/// The character one token spells, where it is a mark.
fn mark_at(tokens: &[Lexed], index: usize) -> Option<char> {
    match tokens.get(index).map(|lexed| &lexed.token) {
        Some(Token::Mark(mark)) => Some(*mark),
        _ => None,
    }
}

/// The text one token carries, where it is a quoted text.
fn text_at(tokens: &[Lexed], index: usize) -> Option<&str> {
    match tokens.get(index).map(|lexed| &lexed.token) {
        Some(Token::Text(text)) => Some(text.as_str()),
        _ => None,
    }
}

/// Cut the declared input into tokens.
fn lex(source: &str) -> Result<Vec<Lexed>, RefusalDeriveRefusal> {
    let mut tokens: Vec<Lexed> = Vec::new();
    let mut characters = source.char_indices().peekable();
    while let Some((offset, character)) = characters.next() {
        let at = u64::try_from(offset).unwrap_or(u64::MAX);
        if character.is_whitespace() {
            continue;
        }
        if character.is_alphabetic() || character == '_' {
            let mut word = String::new();
            word.push(character);
            while let Some(&(_, next)) = characters.peek() {
                if next.is_alphanumeric() || next == '_' {
                    word.push(next);
                    let _consumed = characters.next();
                } else {
                    break;
                }
            }
            tokens.push(Lexed {
                token: Token::Word(word),
                at,
            });
            continue;
        }
        if character == '"' {
            let mut text = String::new();
            let mut closed = false;
            for (_, next) in characters.by_ref() {
                if next == '"' {
                    closed = true;
                    break;
                }
                if next == '\\' {
                    return Err(RefusalDeriveRefusal::established(
                        RefusalDeriveCapture::NotAnEnum,
                        at,
                    ));
                }
                text.push(next);
            }
            if !closed {
                return Err(RefusalDeriveRefusal::established(
                    RefusalDeriveCapture::NotAnEnum,
                    at,
                ));
            }
            tokens.push(Lexed {
                token: Token::Text(text),
                at,
            });
            continue;
        }
        tokens.push(Lexed {
            token: Token::Mark(character),
            at,
        });
    }
    Ok(tokens)
}

/// The index of the token closing the group that opens at `open_index`.
fn matching(tokens: &[Lexed], open_index: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for index in open_index..tokens.len() {
        let Some(mark) = mark_at(tokens, index) else {
            continue;
        };
        if mark == open {
            depth = depth.saturating_add(1);
        } else if mark == close {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

/// The token range inside the `#[refusal(...)]` attribute, where it exists.
fn refusal_attribute(tokens: &[Lexed]) -> Option<(usize, usize)> {
    for index in 0..tokens.len() {
        let opens = index
            .checked_sub(1)
            .is_some_and(|bracket| mark_at(tokens, bracket) == Some('['))
            && index
                .checked_sub(2)
                .is_some_and(|hash| mark_at(tokens, hash) == Some('#'));
        if word_at(tokens, index) == Some("refusal")
            && opens
            && mark_at(tokens, index.saturating_add(1)) == Some('(')
        {
            let open = index.saturating_add(1);
            let close = matching(tokens, open, '(', ')')?;
            return Some((open.saturating_add(1), close));
        }
    }
    None
}

/// The declared shape word inside the attribute body, with its byte position.
fn shape_word(tokens: &[Lexed], start: usize, end: usize) -> Option<(&str, u64)> {
    for index in start..end {
        if word_at(tokens, index) == Some("shape")
            && mark_at(tokens, index.saturating_add(1)) == Some('=')
            && let Some(word) = word_at(tokens, index.saturating_add(2))
        {
            return Some((word, position_at(tokens, index.saturating_add(2))));
        }
    }
    None
}

/// The token range inside the `order(...)` clause, where it exists.
fn order_clause(tokens: &[Lexed], start: usize, end: usize) -> Option<(usize, usize)> {
    for index in start..end {
        if word_at(tokens, index) == Some("order")
            && mark_at(tokens, index.saturating_add(1)) == Some('(')
        {
            let open = index.saturating_add(1);
            let close = matching(tokens, open, '(', ')')?;
            return Some((open.saturating_add(1), close));
        }
    }
    None
}

/// The `Variant = "stable-id"` pairs inside an order clause.
fn order_pairs(tokens: &[Lexed], start: usize, end: usize) -> Option<Vec<CapturedCause>> {
    let mut pairs: Vec<CapturedCause> = Vec::new();
    let mut index = start;
    while index < end {
        let spelling = word_at(tokens, index)?;
        if mark_at(tokens, index.saturating_add(1)) != Some('=') {
            return None;
        }
        let stable_id = text_at(tokens, index.saturating_add(2))?;
        pairs.push(CapturedCause {
            spelling: spelling.to_owned(),
            stable_id: stable_id.to_owned(),
        });
        index = index.saturating_add(3);
        if index < end {
            if mark_at(tokens, index) != Some(',') {
                return None;
            }
            index = index.saturating_add(1);
        }
    }
    Some(pairs)
}

/// The variant names inside an enum body opening at `brace_index`.
fn body_variants(tokens: &[Lexed], brace_index: usize) -> Option<Vec<String>> {
    let close = matching(tokens, brace_index, '{', '}')?;
    let mut variants: Vec<String> = Vec::new();
    let mut group: Vec<usize> = Vec::new();
    let mut depth = 0usize;
    for index in brace_index.saturating_add(1)..close {
        match mark_at(tokens, index) {
            Some('(' | '[' | '{') => {
                depth = depth.saturating_add(1);
                group.push(index);
            }
            Some(')' | ']' | '}') => {
                depth = depth.saturating_sub(1);
                group.push(index);
            }
            Some(',') if depth == 0 => {
                if !close_group(tokens, &group, &mut variants) {
                    return None;
                }
                group.clear();
            }
            _ => group.push(index),
        }
    }
    if !close_group(tokens, &group, &mut variants) {
        return None;
    }
    Some(variants)
}

/// Close one comma-separated group: empty groups are trailing commas, a lone
/// word is a variant, and anything else is outside this grammar.
fn close_group(tokens: &[Lexed], group: &[usize], variants: &mut Vec<String>) -> bool {
    let Some((first, rest)) = group.split_first() else {
        return true;
    };
    if !rest.is_empty() {
        return false;
    }
    match word_at(tokens, *first) {
        Some(word) => {
            variants.push(word.to_owned());
            true
        }
        None => false,
    }
}

/// The machine's body shape one authored word names.
fn admitted_shape(word: &str) -> Option<FamilyShape> {
    match word {
        SHAPE_WORD_SINGLE_CAUSE => Some(FamilyShape::SingleCause),
        SHAPE_WORD_ISSUE_COLLECTION => Some(FamilyShape::IssueCollection),
        SHAPE_WORD_INSEPARABLE_PAIR => Some(FamilyShape::InseparablePair),
        _ => None,
    }
}

/// Whether the ordered causes and the body variants name the same set, each
/// exactly once.
fn covers(causes: &[CapturedCause], variants: &[String]) -> bool {
    causes.len() == variants.len()
        && causes.iter().all(|cause| {
            variants
                .iter()
                .filter(|variant| variant.as_str() == cause.spelling())
                .count()
                == 1
        })
        && variants.iter().all(|variant| {
            causes
                .iter()
                .filter(|cause| cause.spelling() == variant.as_str())
                .count()
                == 1
        })
}

/// Whether every declared stable identity is distinct.
fn distinct(causes: &[CapturedCause]) -> bool {
    causes.iter().enumerate().all(|(index, cause)| {
        causes
            .iter()
            .skip(index.saturating_add(1))
            .all(|other| other.stable_id() != cause.stable_id())
    })
}

/// One established capture refusal at one byte position.
fn refuse(cause: RefusalDeriveCapture, at: u64) -> RefusalDeriveRefusal {
    RefusalDeriveRefusal::established(cause, at)
}

/// The enum declaration as it was read: its name, its variants, and the byte
/// its body opens at.
struct DeclaredEnum {
    /// The declared family's Rust name.
    family_name: String,
    /// The variant names, in the order the body writes them.
    variants: Vec<String>,
    /// The byte position the body opens at.
    brace: u64,
}

/// The attribute as it was read: the admitted shape, where the shape word sits,
/// and the order clause's token range where one was declared.
struct DeclaredAttribute {
    /// The machine's body shape the declared word names.
    shape: FamilyShape,
    /// The byte position the shape word sits at.
    at: u64,
    /// The order clause's token range, where one was declared.
    order: Option<(usize, usize)>,
}

/// Read the enum declaration: the keyword, the name, and the body's variants.
fn read_enum(tokens: &[Lexed]) -> Result<DeclaredEnum, RefusalDeriveRefusal> {
    let enum_index = (0..tokens.len())
        .find(|index| word_at(tokens, *index) == Some("enum"))
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAnEnum, 0))?;
    let name_index = enum_index.saturating_add(1);
    let family_name = word_at(tokens, name_index)
        .ok_or_else(|| {
            refuse(
                RefusalDeriveCapture::NotNamed,
                position_at(tokens, enum_index),
            )
        })?
        .to_owned();
    let brace_index = (name_index..tokens.len())
        .find(|index| mark_at(tokens, *index) == Some('{'))
        .ok_or_else(|| {
            refuse(
                RefusalDeriveCapture::NotAnEnum,
                position_at(tokens, name_index),
            )
        })?;
    let brace = position_at(tokens, brace_index);
    let variants = body_variants(tokens, brace_index)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotAnEnum, brace))?;
    if variants.is_empty() {
        return Err(refuse(RefusalDeriveCapture::NotInhabited, brace));
    }
    if variants.len() > DeriveCauseLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, brace));
    }
    Ok(DeclaredEnum {
        family_name,
        variants,
        brace,
    })
}

/// Read the `#[refusal(...)]` attribute: the shape word and the order clause.
fn read_attribute(tokens: &[Lexed]) -> Result<DeclaredAttribute, RefusalDeriveRefusal> {
    let (body_start, body_end) = refusal_attribute(tokens)
        .ok_or_else(|| refuse(RefusalDeriveCapture::NotShapeDeclared, 0))?;
    let (word, at) = shape_word(tokens, body_start, body_end).ok_or_else(|| {
        refuse(
            RefusalDeriveCapture::NotShapeDeclared,
            position_at(tokens, body_start),
        )
    })?;
    let shape =
        admitted_shape(word).ok_or_else(|| refuse(RefusalDeriveCapture::NotAnAdmittedShape, at))?;
    Ok(DeclaredAttribute {
        shape,
        at,
        order: order_clause(tokens, body_start, body_end),
    })
}

/// Read the declared causes: the order clause where the shape carries one, and
/// then coverage and distinctness against the body.
fn read_causes(
    tokens: &[Lexed],
    attribute: &DeclaredAttribute,
    declared: &DeclaredEnum,
) -> Result<Vec<CapturedCause>, RefusalDeriveRefusal> {
    let causes = match (attribute.shape, attribute.order) {
        (FamilyShape::SingleCause, None) => {
            return Err(refuse(RefusalDeriveCapture::NotOrderDeclared, attribute.at));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, Some((start, _))) => {
            return Err(refuse(
                RefusalDeriveCapture::NotOrderAdmitted,
                position_at(tokens, start),
            ));
        }
        (FamilyShape::IssueCollection | FamilyShape::InseparablePair, None) => {
            return Ok(Vec::new());
        }
        (FamilyShape::SingleCause, Some((start, end))) => order_pairs(tokens, start, end)
            .ok_or_else(|| refuse(RefusalDeriveCapture::NotCovered, position_at(tokens, start)))?,
    };
    if !covers(&causes, &declared.variants) {
        return Err(refuse(RefusalDeriveCapture::NotCovered, declared.brace));
    }
    if !distinct(&causes) {
        return Err(refuse(RefusalDeriveCapture::NotDistinct, declared.brace));
    }
    Ok(causes)
}

/// The whole capture, one dependent check after another.
fn capture(source: &str) -> Result<RefusalDeriveSurface, RefusalDeriveRefusal> {
    if source.len() > DeriveSourceLimit::MAX {
        return Err(refuse(RefusalDeriveCapture::Unbounded, 0));
    }
    let tokens = lex(source)?;
    let declared = read_enum(&tokens)?;
    let attribute = read_attribute(&tokens)?;
    let causes = read_causes(&tokens, &attribute, &declared)?;
    let causes = Bounded::admitted_const(causes)
        .map_err(|_| refuse(RefusalDeriveCapture::Unbounded, declared.brace))?;
    Ok(RefusalDeriveSurface {
        family_name: declared.family_name,
        shape: attribute.shape,
        causes,
    })
}

// ---------------------------------------------------------------------------
// The rendering.
// ---------------------------------------------------------------------------

/// The machine's shape variant one body shape spells.
const fn shape_variant(shape: FamilyShape) -> &'static str {
    match shape {
        FamilyShape::SingleCause => "SingleCause",
        FamilyShape::IssueCollection => "IssueCollection",
        FamilyShape::InseparablePair => "InseparablePair",
    }
}

/// Render the declared output set as Rust source text.
fn render_source(
    surface: &RefusalDeriveSurface,
    membership: DerivedMembership,
    defect: Option<PlantedDefect>,
) -> String {
    let mut rendered = String::new();
    render_family_impl(&mut rendered, surface, defect);
    if membership == DerivedMembership::FamilyAndCauseOrder {
        render_cause_order_impl(&mut rendered, surface, defect);
    }
    rendered
}

/// Render the `RefusalFamily` implementation.
fn render_family_impl(
    rendered: &mut String,
    surface: &RefusalDeriveSurface,
    defect: Option<PlantedDefect>,
) {
    rendered.push_str("impl ::threadpak::refusal::RefusalFamily for ");
    rendered.push_str(surface.family_name());
    rendered.push_str(" {\n    const SHAPE: ::threadpak::refusal::FamilyShape =\n");
    rendered.push_str("        ::threadpak::refusal::FamilyShape::");
    rendered.push_str(shape_variant(surface.shape()));
    rendered.push_str(";\n    const SELECTION_ORDER: &'static [&'static str] = &[");
    let mut spellings: Vec<&str> = surface.causes().map(CapturedCause::spelling).collect();
    if defect == Some(PlantedDefect::SelectionOrderPermuted) {
        spellings.reverse();
    }
    for (position, spelling) in spellings.iter().enumerate() {
        if position > 0 {
            rendered.push_str(", ");
        }
        rendered.push('"');
        rendered.push_str(spelling);
        rendered.push('"');
    }
    rendered.push_str("];\n}\n");
}

/// Render the `CauseOrderDeclaration` implementation.
fn render_cause_order_impl(
    rendered: &mut String,
    surface: &RefusalDeriveSurface,
    defect: Option<PlantedDefect>,
) {
    let recycled = surface.causes().next().map(CapturedCause::stable_id);
    rendered.push_str("impl ::threadpak::refusal::CauseOrderDeclaration for ");
    rendered.push_str(surface.family_name());
    rendered.push_str(" {\n    const DECLARED_ORDER: ::threadpak::refusal::DeclaredCauseOrder =\n");
    rendered.push_str("        ::threadpak::refusal::DeclaredCauseOrder::declared(&[\n");
    for cause in surface.causes() {
        let stable_id = match (defect, recycled) {
            (Some(PlantedDefect::CauseIdentityRecycled), Some(first)) => first,
            _ => cause.stable_id(),
        };
        rendered.push_str("            ::threadpak::refusal::DeclaredCause::declared(\n");
        rendered.push_str("                ::threadpak::refusal::CauseId::declared(\"");
        rendered.push_str(stable_id);
        rendered.push_str("\"),\n                \"");
        rendered.push_str(cause.spelling());
        rendered.push_str("\",\n            ),\n");
    }
    rendered.push_str("        ]);\n}\n");
}
