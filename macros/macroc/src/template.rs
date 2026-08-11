//! The declaration-template contract: what a ruled typed template declares,
//! what a binding may fill it with, and what one invocation of it is keyed by.
//!
//! # Category-typed holes
//!
//! A template's holes are typed by category, and the category travels on both
//! ends of every binding: a string can never become an identifier; a type
//! cannot enter an expression hole. The disagreement is a typed refusal at the
//! binding seam ([`TemplateBinding::bound`]), never a substitution nobody
//! notices.
//!
//! # Frontend-neutral, and owned elsewhere
//!
//! Templating is the machine's own authoring role — band 13 declares it as
//! [`threadpak::declaration::AuthoringRole::Quotation`], and that declaration
//! owns every semantic fact
//! this module speaks of: that a quoted fragment is typed data rather than
//! text, that splicing substitutes typed values only, that instantiation mints
//! no authority, and that produced material re-enters the ordinary validation
//! and linking path with no shortcut. This module summarizes and references
//! those facts as typed members; it answers none of them a second time.
//!
//! Any front door may offer a template surface, or none. Nothing here knows
//! which one is calling.
//!
//! # The three locks are members, not prose
//!
//! Band 13's [`threadpak::declaration::META_EVALUATION_LOCKS`] names the three
//! locks every meta
//! evaluation declares BEFORE evaluation. This module carries one typed member
//! per lock — [`SymbolicBoundFormula`], [`ProfileCeiling`], and
//! [`CheckedMeterPosture`] — so a template that declared none of them is
//! unrepresentable rather than refused. The lock roster's wording stays band
//! 13's; these members cite it.
//!
//! # The staged-meta laws, and where they live
//!
//! The stage a judgment stands at is band 13's [`Stage`], and the staged-meta
//! laws are band 13's too. A template records the stage its owner declared it
//! is evaluated at and nothing more: the plane never decides a stage, never
//! promotes material across one, and never mints Semantic Form — what a
//! template produces is declaration material that re-enters the machine's own
//! path untrusted, is judged at the instantiating site, and carries no live
//! authority of its own.
//!
//! # The meter is a mechanism, and mechanisms are gated
//!
//! [`CheckedMeterPosture`] is an obligation carrier and a stated nonclaim, not
//! a meter. The actual meter must refuse before over-limit allocation and must
//! never return a partial fragment set; that obligation is the owner's, the
//! mechanism is gated, and this module says which owner declared it rather
//! than pretending to run it.

use crate::origin_graph::Nonclaim;
use crate::plane::{
    ApplicationDistinctnessSubject, BoundFormulaSubject, ExactIdentity, FragmentDependencyLimit,
    InputDescriptorLimit, InputDescriptorSubject, LanguageProfileSubject, MetaBoundAxisLimit,
    MetaProfileSubject, OwnerFactRef, ProfileVersion, SourceSnapshotSubject,
    TemplateArgumentSubject, TemplateIssueLimit, TemplateParameterLimit, TemplateParameterSubject,
    TemplateSubject,
};
use threadpak::declaration::Stage;
use threadpak::declaration::types::{FragmentIdentityDomain, ProjectionConfigurationDomain};
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction};

// ---------------------------------------------------------------------------
// Category-typed holes.
// ---------------------------------------------------------------------------

/// The closed roster of splice categories: which kind of hole a template
/// declares, and which kind of material may fill it.
///
/// Category-typed holes are the law this roster exists for: a string can never
/// become an identifier; a type cannot enter an expression hole.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpliceCategory {
    /// A hole that evaluates to a value.
    Expression,
    /// A hole naming a type.
    Type,
    /// A hole standing where a pattern is matched.
    Pattern,
    /// A hole standing where a whole declaration goes.
    Declaration,
    /// A hole filled by declaration material assembled elsewhere.
    Fragment,
    /// A hole that binds a name — the category a string is never admitted to.
    IdentifierBinding,
}

/// The declared splice-category roster, in the order this contract states it.
pub const SPLICE_CATEGORIES: [SpliceCategory; 6] = [
    SpliceCategory::Expression,
    SpliceCategory::Type,
    SpliceCategory::Pattern,
    SpliceCategory::Declaration,
    SpliceCategory::Fragment,
    SpliceCategory::IdentifierBinding,
];

/// One typed hole a template declares: which category it admits, and which
/// parameter it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateParameter {
    /// The category of material this hole admits.
    pub category: SpliceCategory,
    /// The parameter's own identity.
    pub parameter: ExactIdentity<TemplateParameterSubject>,
}

/// One typed commitment offered to fill a hole: which category the material
/// is, and which exact argument it commits to.
///
/// The argument is a commitment, never a spelling: nothing here carries source
/// text, and no rendering of this value can be read back as one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateArgument {
    /// The category of the offered material.
    pub category: SpliceCategory,
    /// The argument's exact commitment.
    pub commitment: ExactIdentity<TemplateArgumentSubject>,
}

/// How one binding fails.
///
/// A single-cause family with one inhabited cause: binding one argument to one
/// parameter runs exactly one check, so one cause is all that can truthfully
/// exist and a collection body would claim checks that never ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateBindingIssue {
    /// The argument's category is not the parameter's category.
    CategoryMismatch {
        /// The category the parameter declared.
        expected: SpliceCategory,
        /// The category the argument offered.
        found: SpliceCategory,
    },
}

impl RefusalFamily for TemplateBindingIssue {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["CategoryMismatch"];
}

/// One argument bound to one parameter of the same category.
///
/// Holding one *is* the category proof: the only road here is
/// [`TemplateBinding::bound`], which refuses a category disagreement, so a
/// cross-category binding is not a value that exists and then fails a check —
/// it is a value nobody can build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateBinding {
    parameter: TemplateParameter,
    argument: TemplateArgument,
}

impl TemplateBinding {
    /// Bind one argument to one parameter.
    ///
    /// # Errors
    ///
    /// Returns [`TemplateBindingIssue::CategoryMismatch`] naming both
    /// categories when the argument's category is not the parameter's. The
    /// mismatch is never coerced: a coercion here is exactly how a string
    /// becomes an identifier.
    pub fn bound(
        parameter: TemplateParameter,
        argument: TemplateArgument,
    ) -> Result<Self, TemplateBindingIssue> {
        if parameter.category == argument.category {
            Ok(Self {
                parameter,
                argument,
            })
        } else {
            Err(TemplateBindingIssue::CategoryMismatch {
                expected: parameter.category,
                found: argument.category,
            })
        }
    }

    /// The parameter filled.
    #[must_use]
    pub const fn parameter(&self) -> TemplateParameter {
        self.parameter
    }

    /// The argument that fills it.
    #[must_use]
    pub const fn argument(&self) -> TemplateArgument {
        self.argument
    }

    /// The category both ends agree on.
    #[must_use]
    pub const fn category(&self) -> SpliceCategory {
        self.parameter.category
    }
}

// ---------------------------------------------------------------------------
// The three locks.
// ---------------------------------------------------------------------------

/// The first lock: a symbolic bound formula stated over validated inputs.
///
/// This is a commitment to the formula, never a small language: the plane
/// names the formula its owner declared, names the owner fact that declares
/// it, and names the validated inputs it stands over. Nothing here evaluates,
/// parses, or rewrites a formula — evaluating one is a mechanism, and the
/// mechanism is not this module's.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolicBoundFormula {
    /// The declared formula, by identity.
    pub formula: ExactIdentity<BoundFormulaSubject>,
    /// The owner fact that declares it.
    pub declared_by: OwnerFactRef,
    /// The validated inputs the formula stands over — at least one, by shape:
    /// a formula over nothing bounds nothing.
    pub over_inputs: NonEmptyBounded<ExactIdentity<InputDescriptorSubject>, InputDescriptorLimit>,
}

/// The closed roster of meta bound axes a profile ceiling covers.
///
/// A ceiling that named "too big" without naming which magnitude would be an
/// unlocated bound, so every axis is a member of this roster and a ceiling
/// names each one exactly once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetaBoundAxis {
    /// Validated input descriptors one evaluation may range over.
    InputDescriptors,
    /// Work one evaluation may perform, in its declared currency.
    Work,
    /// Memory one evaluation may hold.
    Memory,
    /// Recursion depth one evaluation may reach.
    Recursion,
    /// Declarations one evaluation may produce.
    Declarations,
    /// Symbols one evaluation may introduce.
    Symbols,
    /// Diagnostics one evaluation may carry.
    Diagnostics,
    /// Output bytes one evaluation may commit to.
    OutputBytes,
}

/// The declared meta bound-axis roster, in the order this contract states it.
pub const META_BOUND_AXES: [MetaBoundAxis; 8] = [
    MetaBoundAxis::InputDescriptors,
    MetaBoundAxis::Work,
    MetaBoundAxis::Memory,
    MetaBoundAxis::Recursion,
    MetaBoundAxis::Declarations,
    MetaBoundAxis::Symbols,
    MetaBoundAxis::Diagnostics,
    MetaBoundAxis::OutputBytes,
];

/// One axis of a profile ceiling: which magnitude, how large, and on whose
/// declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxisCeiling {
    /// The magnitude this ceiling bounds.
    pub axis: MetaBoundAxis,
    /// The declared maximum on that axis.
    pub magnitude: u64,
    /// The owner fact that declares it.
    pub declared_by: OwnerFactRef,
}

/// The second lock: the hard profile ceiling, one magnitude per meta bound
/// axis.
///
/// Complete by construction: [`ProfileCeiling::declared`] admits a ceiling only
/// when every axis in [`META_BOUND_AXES`] appears exactly once, so a ceiling
/// silently missing an axis — the shape that lets one magnitude run unbounded
/// while the others look governed — cannot be held.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileCeiling {
    axes: Bounded<AxisCeiling, MetaBoundAxisLimit>,
}

impl ProfileCeiling {
    /// Declare the complete ceiling.
    ///
    /// # Errors
    ///
    /// Returns [`TemplateConstruction`] naming
    /// [`TemplateConstructionIssue::CeilingAxisAbsent`] for every axis nobody
    /// bounded and [`TemplateConstructionIssue::CeilingAxisDoubled`] for every
    /// axis bounded twice — all of them at once, because a caller repairing a
    /// ceiling one axis per attempt is a caller this seam failed.
    pub fn declared(axes: Vec<AxisCeiling>) -> Result<Self, TemplateConstruction> {
        let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
        for axis in META_BOUND_AXES {
            let stated = axes.iter().filter(|held| held.axis == axis).count();
            if stated == 0 {
                issues.push(TemplateConstructionIssue::CeilingAxisAbsent { axis });
            } else if stated > 1 {
                issues.push(TemplateConstructionIssue::CeilingAxisDoubled { axis });
            }
        }
        let mut established = issues.into_iter();
        if let Some(first) = established.next() {
            return Err(TemplateConstruction::co_established(
                first,
                established.collect(),
            ));
        }
        let observed = axes.len();
        Bounded::admitted_const(axes)
            .map(|axes| Self { axes })
            .map_err(|_| {
                TemplateConstruction::established(TemplateConstructionIssue::SeatBoundExceeded {
                    seat: TemplateSeat::AxisCeilings,
                    bound: u64::try_from(MetaBoundAxisLimit::MAX).unwrap_or(u64::MAX),
                    observed: u64::try_from(observed).unwrap_or(u64::MAX),
                })
            })
    }

    /// Read the declared axis ceilings.
    ///
    /// The order law applies: the ceiling is a set keyed by axis, so nothing
    /// identity-bearing is derived from the order this yields.
    pub fn iter(&self) -> impl Iterator<Item = &AxisCeiling> {
        self.axes.iter()
    }

    /// The number of axes bounded — the roster's cardinality, by construction.
    #[must_use]
    pub fn len(&self) -> usize {
        self.axes.len()
    }

    /// Always `false`: a ceiling covers every axis or does not exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.axes.is_empty()
    }
}

/// The third lock: the declared posture of the checked evaluation meter.
///
/// The obligation is the owner's — an actual meter refuses BEFORE over-limit
/// allocation and never returns a partial fragment set. The meter itself is a
/// gated mechanism, so this carrier states the obligation by citation and
/// states, as a [`Nonclaim`], that holding this value is not evidence a meter
/// ran. A posture that read as a measurement would be the plane answering a
/// question it has no standing to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckedMeterPosture {
    /// The owner fact declaring the checked-evaluation meter obligation.
    pub obliged_by: OwnerFactRef,
    /// What this carrier explicitly does not claim: that a meter ran here.
    pub unmeasured: Nonclaim,
}

// ---------------------------------------------------------------------------
// The template.
// ---------------------------------------------------------------------------

/// One profile named together with the version it was read at. Two versions of
/// two different profiles are not comparable and are never ranked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionedProfile<Profile> {
    /// The profile.
    pub profile: ExactIdentity<Profile>,
    /// That profile's version.
    pub version: ProfileVersion,
}

/// The seats a template seam can overrun. Only seats that can actually hold a
/// caller-supplied count appear; every other seat is bounded by the shape it
/// is built from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateSeat {
    /// The holes one template declares.
    DeclaredParameters,
    /// The bindings one application supplies.
    SuppliedBindings,
    /// The axis ceilings one profile ceiling carries.
    AxisCeilings,
}

/// The closed template-construction issue set.
///
/// No issue is payload-free: each names the parameter, axis, or seat it is
/// about, because a bare variant makes the caller guess which hole moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateConstructionIssue {
    /// Two declared holes claim the same parameter identity.
    DuplicateParameter {
        /// The doubled parameter.
        parameter: ExactIdentity<TemplateParameterSubject>,
    },
    /// A binding names a parameter this template does not declare.
    UnknownParameter {
        /// The unrecognized parameter.
        parameter: ExactIdentity<TemplateParameterSubject>,
    },
    /// A declared hole was left unbound.
    MissingBinding {
        /// The unbound parameter.
        parameter: ExactIdentity<TemplateParameterSubject>,
    },
    /// A declared hole was bound more than once.
    DuplicateBinding {
        /// The doubly bound parameter.
        parameter: ExactIdentity<TemplateParameterSubject>,
    },
    /// A binding names a declared parameter under the wrong category. Distinct
    /// from [`TemplateBindingIssue::CategoryMismatch`], which is the argument
    /// disagreeing with its own parameter: this is the supplied parameter
    /// disagreeing with the template's declaration of it.
    DeclaredCategoryDisagreement {
        /// The parameter both sides name.
        parameter: ExactIdentity<TemplateParameterSubject>,
        /// The category the template declared.
        declared: SpliceCategory,
        /// The category the binding carried.
        bound: SpliceCategory,
    },
    /// No ceiling bounds this axis.
    CeilingAxisAbsent {
        /// The unbounded axis.
        axis: MetaBoundAxis,
    },
    /// Two ceilings bound the same axis.
    CeilingAxisDoubled {
        /// The doubled axis.
        axis: MetaBoundAxis,
    },
    /// A seat's declared magnitude was exceeded.
    SeatBoundExceeded {
        /// Which seat overran.
        seat: TemplateSeat,
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The template-construction refusal family body.
///
/// Independent members: a template may double one parameter while leaving
/// another category-disagreeing, and an application may leave one hole unbound
/// while binding an unknown one. No primary issue is elected, and a zero-issue
/// refusal is unrepresentable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateConstruction {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<TemplateConstructionIssue, TemplateIssueLimit>,
    /// Whether every applicable check ran.
    pub posture: CompletionPosture,
}

impl RefusalFamily for TemplateConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl TemplateConstruction {
    /// The one-issue body. Total: the declared bound admits an item by
    /// compile-time proof, so refusing never needs an error road of its own.
    #[must_use]
    pub fn established(issue: TemplateConstructionIssue) -> Self {
        Self {
            issues: NonEmptyBounded::singleton(issue),
            posture: CompletionPosture::Complete,
        }
    }

    /// The several-issue body. When the supplied issues outrun the declared
    /// bound the body keeps the first and reports that enumeration stopped
    /// there — never a silent drop, never an unearned claim of completeness.
    #[must_use]
    pub fn co_established(
        first: TemplateConstructionIssue,
        rest: Vec<TemplateConstructionIssue>,
    ) -> Self {
        match NonEmptyBounded::admitted_const(first, rest) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}

/// One authored declaration template: its identity, its typed holes, the three
/// locks it declares before any evaluation, and the stage its owner declared it
/// is evaluated at.
///
/// Every seat is required. The parameter seat is structurally non-empty — a
/// template with no hole is a declaration, and the machine already has those.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclarationTemplate {
    identity: ExactIdentity<TemplateSubject>,
    parameters: NonEmptyBounded<TemplateParameter, TemplateParameterLimit>,
    formula: SymbolicBoundFormula,
    ceiling: ProfileCeiling,
    meter: CheckedMeterPosture,
    stage: Stage,
}

impl DeclarationTemplate {
    /// Declare one template.
    ///
    /// # Errors
    ///
    /// Returns [`TemplateConstruction`] naming every parameter identity
    /// declared twice, and the parameter seat when the hole set outgrows its
    /// declared magnitude. Two holes under one identity are refused rather than
    /// merged: merging them silently drops one hole's category.
    pub fn declared(
        identity: ExactIdentity<TemplateSubject>,
        first: TemplateParameter,
        rest: Vec<TemplateParameter>,
        formula: SymbolicBoundFormula,
        ceiling: ProfileCeiling,
        meter: CheckedMeterPosture,
        stage: Stage,
    ) -> Result<Self, TemplateConstruction> {
        let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
        let declared: Vec<TemplateParameter> = core::iter::once(first)
            .chain(rest.iter().copied())
            .collect();
        for (position, parameter) in declared.iter().enumerate() {
            let earlier = declared
                .iter()
                .take(position)
                .any(|other| other.parameter == parameter.parameter);
            let repeated = declared
                .iter()
                .skip(position.saturating_add(1))
                .any(|other| other.parameter == parameter.parameter);
            if repeated && !earlier {
                issues.push(TemplateConstructionIssue::DuplicateParameter {
                    parameter: parameter.parameter,
                });
            }
        }
        let mut established = issues.into_iter();
        if let Some(issue) = established.next() {
            return Err(TemplateConstruction::co_established(
                issue,
                established.collect(),
            ));
        }
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(first, rest)
            .map(|parameters| Self {
                identity,
                parameters,
                formula,
                ceiling,
                meter,
                stage,
            })
            .map_err(|_| {
                TemplateConstruction::established(TemplateConstructionIssue::SeatBoundExceeded {
                    seat: TemplateSeat::DeclaredParameters,
                    bound: u64::try_from(TemplateParameterLimit::MAX).unwrap_or(u64::MAX),
                    observed: u64::try_from(observed).unwrap_or(u64::MAX),
                })
            })
    }

    /// The template's own identity.
    #[must_use]
    pub const fn identity(&self) -> ExactIdentity<TemplateSubject> {
        self.identity
    }

    /// The guaranteed first declared hole.
    #[must_use]
    pub fn first_parameter(&self) -> TemplateParameter {
        *self.parameters.first()
    }

    /// Read the declared holes.
    ///
    /// The order law applies: the hole set is keyed by parameter identity, so
    /// nothing identity-bearing is derived from the order this yields.
    pub fn parameters(&self) -> impl Iterator<Item = &TemplateParameter> {
        self.parameters.iter()
    }

    /// The number of holes declared; structurally at least one.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.parameters.len()
    }

    /// The first lock: the symbolic bound formula over validated inputs.
    #[must_use]
    pub const fn formula(&self) -> &SymbolicBoundFormula {
        &self.formula
    }

    /// The second lock: the hard profile ceiling.
    #[must_use]
    pub const fn ceiling(&self) -> &ProfileCeiling {
        &self.ceiling
    }

    /// The third lock: the declared checked-meter posture.
    #[must_use]
    pub const fn meter(&self) -> CheckedMeterPosture {
        self.meter
    }

    /// The stage the owner declared this template is evaluated at.
    #[must_use]
    pub const fn stage(&self) -> Stage {
        self.stage
    }
}

// ---------------------------------------------------------------------------
// Application.
// ---------------------------------------------------------------------------

/// Whether one application is the applicative one or a deliberately distinct
/// one.
///
/// Not an option and not a flag: distinctness is an explicit, identity-bearing
/// declaration, so "these two applications are deliberately different" and
/// "somebody set a boolean" can never read the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicativeDistinctness {
    /// The ordinary posture: same template, same arguments, same profiles,
    /// same meaning.
    Applicative,
    /// Deliberately distinct from an otherwise identical application, under
    /// this declared distinctness identity.
    DeliberatelyDistinct(ExactIdentity<ApplicationDistinctnessSubject>),
}

/// One application of one template: which template, the canonical argument
/// commitments that fill its holes, the profiles it was applied under, and its
/// distinctness posture.
///
/// # What makes two applications the same
///
/// Same template + same arguments + same profiles = same meaning. Expansion
/// count, expansion order, formatting, an alias, and the position an
/// application sits at mint nothing: none of them is a member here, so none of
/// them can be read back as a difference.
///
/// The binding set is order-insensitive. Nothing identity-bearing may be
/// derived from the order [`TemplateApplication::bindings`] yields — an
/// applicative identity computed over these bindings canonicalizes by
/// parameter identity first, and testpak owes the permutation hostile: the
/// same bindings supplied in another order must yield the same application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateApplication {
    template: ExactIdentity<TemplateSubject>,
    bindings: NonEmptyBounded<TemplateBinding, TemplateParameterLimit>,
    language_profile: VersionedProfile<LanguageProfileSubject>,
    meta_profile: VersionedProfile<MetaProfileSubject>,
    distinctness: ApplicativeDistinctness,
}

impl TemplateApplication {
    /// Apply one template to one complete set of bindings.
    ///
    /// # Errors
    ///
    /// Returns [`TemplateConstruction`] naming every declared hole left
    /// unbound, every hole bound twice, every binding naming a hole this
    /// template does not declare, and every binding whose parameter category
    /// disagrees with the template's declaration of it. All of them are
    /// reported together, and the binding seat is named when the supplied set
    /// outgrows its declared magnitude.
    pub fn applied(
        template: &DeclarationTemplate,
        bindings: Vec<TemplateBinding>,
        language_profile: VersionedProfile<LanguageProfileSubject>,
        meta_profile: VersionedProfile<MetaProfileSubject>,
        distinctness: ApplicativeDistinctness,
    ) -> Result<Self, TemplateConstruction> {
        let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
        for declared in template.parameters() {
            let mut supplied = bindings
                .iter()
                .filter(|binding| binding.parameter().parameter == declared.parameter);
            match (supplied.next(), supplied.next()) {
                (None, _) => issues.push(TemplateConstructionIssue::MissingBinding {
                    parameter: declared.parameter,
                }),
                (Some(bound), None) => {
                    if bound.category() != declared.category {
                        issues.push(TemplateConstructionIssue::DeclaredCategoryDisagreement {
                            parameter: declared.parameter,
                            declared: declared.category,
                            bound: bound.category(),
                        });
                    }
                }
                (Some(_), Some(_)) => issues.push(TemplateConstructionIssue::DuplicateBinding {
                    parameter: declared.parameter,
                }),
            }
        }
        for binding in &bindings {
            let known = template
                .parameters()
                .any(|declared| declared.parameter == binding.parameter().parameter);
            if !known {
                issues.push(TemplateConstructionIssue::UnknownParameter {
                    parameter: binding.parameter().parameter,
                });
            }
        }
        let mut established = issues.into_iter();
        if let Some(issue) = established.next() {
            return Err(TemplateConstruction::co_established(
                issue,
                established.collect(),
            ));
        }
        let observed = bindings.len();
        let mut supplied = bindings.into_iter();
        let Some(first) = supplied.next() else {
            // Foreclosed above: a template declares at least one hole, so an
            // empty binding set has already established a missing binding.
            return Err(TemplateConstruction::established(
                TemplateConstructionIssue::MissingBinding {
                    parameter: template.first_parameter().parameter,
                },
            ));
        };
        NonEmptyBounded::admitted_const(first, supplied.collect())
            .map(|bindings| Self {
                template: template.identity(),
                bindings,
                language_profile,
                meta_profile,
                distinctness,
            })
            .map_err(|_| {
                TemplateConstruction::established(TemplateConstructionIssue::SeatBoundExceeded {
                    seat: TemplateSeat::SuppliedBindings,
                    bound: u64::try_from(TemplateParameterLimit::MAX).unwrap_or(u64::MAX),
                    observed: u64::try_from(observed).unwrap_or(u64::MAX),
                })
            })
    }

    /// The template applied.
    #[must_use]
    pub const fn template(&self) -> ExactIdentity<TemplateSubject> {
        self.template
    }

    /// Read the canonical argument commitments, one per declared hole.
    ///
    /// The order law applies as stated on the type: this order is not
    /// identity-bearing.
    pub fn bindings(&self) -> impl Iterator<Item = &TemplateBinding> {
        self.bindings.iter()
    }

    /// The number of holes filled; structurally at least one, and equal to the
    /// template's arity by construction.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.bindings.len()
    }

    /// The language profile and version this application was made under.
    #[must_use]
    pub const fn language_profile(&self) -> VersionedProfile<LanguageProfileSubject> {
        self.language_profile
    }

    /// The meta profile and version this application was made under.
    #[must_use]
    pub const fn meta_profile(&self) -> VersionedProfile<MetaProfileSubject> {
        self.meta_profile
    }

    /// Whether this application is applicative or deliberately distinct.
    #[must_use]
    pub const fn distinctness(&self) -> ApplicativeDistinctness {
        self.distinctness
    }
}

// ---------------------------------------------------------------------------
// The invocation key.
// ---------------------------------------------------------------------------

/// The semantic invocation key: the complete set of lawful inputs one template
/// invocation is keyed by.
///
/// Seven members, every one of them a typed commitment. A cached expansion is
/// disposable: it is an optimization keyed by this value and never a
/// replacement for recomputing under it, so discarding every cache anywhere
/// changes no meaning at all.
///
/// What may never participate is [`INVOCATION_KEY_NEVER`], and it is a roster
/// rather than a warning: none of those facts is a member of this record, so a
/// key that varied with the checkout path or the wall clock is not a key
/// somebody must remember not to build.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateInvocationKey {
    /// The template's semantic identity.
    pub template: ExactIdentity<TemplateSubject>,
    /// The validated input descriptors this invocation commits to.
    pub inputs: Bounded<ExactIdentity<InputDescriptorSubject>, InputDescriptorLimit>,
    /// The exact source snapshot the invocation was read against.
    pub source_snapshot: ExactIdentity<SourceSnapshotSubject>,
    /// The declaration fragments this invocation depends on.
    pub fragment_dependencies:
        Bounded<ExactIdentity<FragmentIdentityDomain>, FragmentDependencyLimit>,
    /// The language profile and version.
    pub language_profile: VersionedProfile<LanguageProfileSubject>,
    /// The meta profile and version.
    pub meta_profile: VersionedProfile<MetaProfileSubject>,
    /// The configuration commitment in force.
    pub configuration: ExactIdentity<ProjectionConfigurationDomain>,
}

/// The closed roster of facts that never participate in an invocation key.
///
/// Each is a way the same declared input could yield two different answers,
/// which is exactly what determinism forbids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForbiddenKeyFact {
    /// Where the tree happens to be checked out.
    CheckoutPath,
    /// Which directory the caller happened to run from.
    CurrentDirectory,
    /// When a file was last written.
    ModificationTime,
    /// Which process is running.
    ProcessIdentity,
    /// Whatever the ambient environment happens to say.
    AmbientEnvironment,
    /// What the wall clock reads.
    WallTime,
    /// Anything drawn from entropy.
    Entropy,
    /// Which host the work runs on.
    HostAddress,
    /// The order a hash map happened to iterate in.
    MapIterationOrder,
}

/// The declared never-roster of an invocation key, in the order this contract
/// states it.
pub const INVOCATION_KEY_NEVER: [ForbiddenKeyFact; 9] = [
    ForbiddenKeyFact::CheckoutPath,
    ForbiddenKeyFact::CurrentDirectory,
    ForbiddenKeyFact::ModificationTime,
    ForbiddenKeyFact::ProcessIdentity,
    ForbiddenKeyFact::AmbientEnvironment,
    ForbiddenKeyFact::WallTime,
    ForbiddenKeyFact::Entropy,
    ForbiddenKeyFact::HostAddress,
    ForbiddenKeyFact::MapIterationOrder,
];

#[cfg(test)]
mod laws {
    use super::{
        ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate,
        ForbiddenKeyFact, INVOCATION_KEY_NEVER, META_BOUND_AXES, MetaBoundAxis, ProfileCeiling,
        SPLICE_CATEGORIES, SpliceCategory, SymbolicBoundFormula, TemplateApplication,
        TemplateArgument, TemplateBinding, TemplateBindingIssue, TemplateConstruction,
        TemplateConstructionIssue, TemplateInvocationKey, TemplateParameter, TemplateSeat,
        VersionedProfile,
    };
    use crate::origin_graph::Nonclaim;
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use threadpak::declaration::Stage;
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// The closed splice-category roster, proven closed by an exhaustive match:
    /// a new category stops compiling here until it is placed.
    const fn category_index(category: SpliceCategory) -> usize {
        match category {
            SpliceCategory::Expression => 0,
            SpliceCategory::Type => 1,
            SpliceCategory::Pattern => 2,
            SpliceCategory::Declaration => 3,
            SpliceCategory::Fragment => 4,
            SpliceCategory::IdentifierBinding => 5,
        }
    }

    /// The closed meta bound-axis roster, proven closed by an exhaustive match.
    const fn axis_index(axis: MetaBoundAxis) -> usize {
        match axis {
            MetaBoundAxis::InputDescriptors => 0,
            MetaBoundAxis::Work => 1,
            MetaBoundAxis::Memory => 2,
            MetaBoundAxis::Recursion => 3,
            MetaBoundAxis::Declarations => 4,
            MetaBoundAxis::Symbols => 5,
            MetaBoundAxis::Diagnostics => 6,
            MetaBoundAxis::OutputBytes => 7,
        }
    }

    /// The closed forbidden-fact roster, proven closed by an exhaustive match.
    const fn forbidden_index(fact: ForbiddenKeyFact) -> usize {
        match fact {
            ForbiddenKeyFact::CheckoutPath => 0,
            ForbiddenKeyFact::CurrentDirectory => 1,
            ForbiddenKeyFact::ModificationTime => 2,
            ForbiddenKeyFact::ProcessIdentity => 3,
            ForbiddenKeyFact::AmbientEnvironment => 4,
            ForbiddenKeyFact::WallTime => 5,
            ForbiddenKeyFact::Entropy => 6,
            ForbiddenKeyFact::HostAddress => 7,
            ForbiddenKeyFact::MapIterationOrder => 8,
        }
    }

    /// The closed template-seat roster, proven closed by an exhaustive match.
    const fn seat_index(seat: TemplateSeat) -> usize {
        match seat {
            TemplateSeat::DeclaredParameters => 0,
            TemplateSeat::SuppliedBindings => 1,
            TemplateSeat::AxisCeilings => 2,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([80; 32]),
            fact: ExactIdentity::decoded([81; 32]),
        }
    }

    /// One declared hole under the category and identity byte the caller names.
    fn parameter(category: SpliceCategory, tag: u8) -> TemplateParameter {
        TemplateParameter {
            category,
            parameter: ExactIdentity::decoded([tag; 32]),
        }
    }

    /// One offered commitment under the category and identity byte named.
    fn argument(category: SpliceCategory, tag: u8) -> TemplateArgument {
        TemplateArgument {
            category,
            commitment: ExactIdentity::decoded([tag; 32]),
        }
    }

    /// The complete ceiling: every axis bounded exactly once.
    fn complete_ceiling() -> Result<ProfileCeiling, TemplateConstruction> {
        ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 64,
                    declared_by: owner_fact(),
                })
                .collect(),
        )
    }

    /// The first lock, over one validated input.
    fn formula() -> SymbolicBoundFormula {
        SymbolicBoundFormula {
            formula: ExactIdentity::decoded([82; 32]),
            declared_by: owner_fact(),
            over_inputs: NonEmptyBounded::singleton(ExactIdentity::decoded([83; 32])),
        }
    }

    /// The third lock, as an obligation and a stated nonclaim.
    fn meter() -> CheckedMeterPosture {
        CheckedMeterPosture {
            obliged_by: owner_fact(),
            unmeasured: Nonclaim {
                unclaimed: ExactIdentity::decoded([84; 32]),
                because: owner_fact(),
            },
        }
    }

    /// One template over the holes the caller names.
    fn template(
        first: TemplateParameter,
        rest: Vec<TemplateParameter>,
    ) -> Result<DeclarationTemplate, TemplateConstruction> {
        complete_ceiling().and_then(|ceiling| {
            DeclarationTemplate::declared(
                ExactIdentity::decoded([85; 32]),
                first,
                rest,
                formula(),
                ceiling,
                meter(),
                Stage::Meta,
            )
        })
    }

    /// The language profile, at a declared version.
    fn language() -> VersionedProfile<crate::plane::LanguageProfileSubject> {
        VersionedProfile {
            profile: ExactIdentity::decoded([86; 32]),
            version: ProfileVersion::declared(4),
        }
    }

    /// The meta profile, at a declared version.
    fn meta() -> VersionedProfile<crate::plane::MetaProfileSubject> {
        VersionedProfile {
            profile: ExactIdentity::decoded([87; 32]),
            version: ProfileVersion::declared(5),
        }
    }

    /// law: template.splice-categories-are-six-and-closed — the hole categories
    /// are a closed roster whose members are pairwise distinct and declared in
    /// one order.
    /// Owed reversal: adding a category without placing it must break this law.
    #[test]
    fn splice_categories_are_six_and_closed() {
        assert_eq!(SPLICE_CATEGORIES.len(), 6);
        let indexes: Vec<usize> = SPLICE_CATEGORIES
            .iter()
            .copied()
            .map(category_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.a-binding-agrees-on-category-or-refuses — an argument
    /// enters a hole only when both ends name the same category, and the
    /// refusal names both categories rather than saying "wrong kind".
    /// Owed reversal (red twin): a constructor that coerced the argument's
    /// category must break this law.
    #[test]
    fn a_binding_agrees_on_category_or_refuses() {
        let bound = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::IdentifierBinding, 2),
        );
        assert!(bound.is_ok_and(|binding| {
            matches!(binding.category(), SpliceCategory::IdentifierBinding)
                && binding.argument().commitment == ExactIdentity::decoded([2; 32])
                && binding.parameter().parameter == ExactIdentity::decoded([1; 32])
        }));

        let refused = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::Expression, 2),
        );
        assert!(refused.is_err_and(|issue| matches!(
            issue,
            TemplateBindingIssue::CategoryMismatch {
                expected: SpliceCategory::IdentifierBinding,
                found: SpliceCategory::Expression
            }
        )));
    }

    /// law: template.the-two-families-declare-their-shapes — the binding seam
    /// runs one check and takes the single-cause shape with a declared
    /// selection order; the construction seam co-establishes and takes the
    /// collection shape, electing no primary issue.
    /// Owed reversal (red twin): swapping the two shapes must break this law.
    #[test]
    fn the_two_families_declare_their_shapes() {
        assert!(matches!(
            TemplateBindingIssue::SHAPE,
            FamilyShape::SingleCause
        ));
        assert_eq!(TemplateBindingIssue::SELECTION_ORDER, &["CategoryMismatch"]);
        assert!(matches!(
            TemplateConstruction::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TemplateConstruction::SELECTION_ORDER.is_empty());
    }

    /// law: template.a-ceiling-covers-every-meta-bound-axis — the axis roster is
    /// closed at eight, a complete ceiling reads back one magnitude per axis,
    /// and a ceiling missing or doubling an axis refuses naming that axis.
    /// Owed reversal (red twin): a ceiling admitting a subset of the axes must
    /// break this law.
    #[test]
    fn a_ceiling_covers_every_meta_bound_axis() {
        assert_eq!(META_BOUND_AXES.len(), 8);
        let indexes: Vec<usize> = META_BOUND_AXES.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        assert!(complete_ceiling().is_ok_and(|ceiling| {
            ceiling.len() == 8
                && !ceiling.is_empty()
                && ceiling.iter().count() == 8
                && ceiling
                    .iter()
                    .all(|held| held.magnitude == 64 && held.declared_by == owner_fact())
                && META_BOUND_AXES
                    .iter()
                    .all(|axis| ceiling.iter().any(|held| held.axis == *axis))
        }));

        let short = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .filter(|axis| *axis != MetaBoundAxis::Memory)
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(short.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::CeilingAxisAbsent {
                axis: MetaBoundAxis::Memory
            }
        )));

        let doubled = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .chain(core::iter::once(MetaBoundAxis::Work))
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::CeilingAxisDoubled {
                axis: MetaBoundAxis::Work
            }
        )));
    }

    /// law: template.a-template-carries-its-three-locks-and-its-stage — a
    /// declared template holds the symbolic formula over validated inputs, the
    /// complete ceiling, the checked-meter obligation with its stated nonclaim,
    /// and the stage its owner declared; two holes under one identity refuse.
    /// Owed reversal (red twin): omitting any lock seat must not compile.
    #[test]
    fn a_template_carries_its_three_locks_and_its_stage() {
        let declared = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 11)],
        );
        assert!(declared.is_ok_and(|template| {
            template.arity() == 2
                && template.parameters().count() == 2
                && template.identity() == ExactIdentity::decoded([85; 32])
                && template.formula().over_inputs.len() == 1
                && template.formula().declared_by == owner_fact()
                && template.ceiling().len() == 8
                && template.meter().obliged_by == owner_fact()
                && template.meter().unmeasured.because == owner_fact()
                && matches!(template.stage(), Stage::Meta)
                && matches!(template.first_parameter().category, SpliceCategory::Type)
        }));

        let doubled = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 10)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DuplicateParameter { .. }
        )));
    }

    /// The two-hole template the application laws range over: a type hole at
    /// parameter identity 20 and an expression hole at parameter identity 21.
    fn two_hole_template() -> Result<DeclarationTemplate, TemplateConstruction> {
        template(
            parameter(SpliceCategory::Type, 20),
            vec![parameter(SpliceCategory::Expression, 21)],
        )
    }

    /// The bindings for the holes named, each built through the checked binding
    /// seam. A category disagreement at that seam yields no binding at all, so
    /// a law that expected one fails on the count it asserts rather than on a
    /// road this helper invented.
    fn bindings(named: &[(SpliceCategory, u8, u8)]) -> Vec<TemplateBinding> {
        named
            .iter()
            .filter_map(|(category, hole, commitment)| {
                TemplateBinding::bound(
                    parameter(*category, *hole),
                    argument(*category, *commitment),
                )
                .ok()
            })
            .collect()
    }

    /// Apply the two-hole template to the bindings supplied.
    fn apply(supplied: Vec<TemplateBinding>) -> Result<TemplateApplication, TemplateConstruction> {
        two_hole_template().and_then(|template| {
            TemplateApplication::applied(
                &template,
                supplied,
                language(),
                meta(),
                ApplicativeDistinctness::Applicative,
            )
        })
    }

    /// law: template.an-application-binds-every-hole-exactly-once — a complete
    /// application reads its bindings back whole under both profiles, an
    /// unbound hole refuses, and a doubly bound hole refuses.
    /// Owed reversal: an application seam that accepted a partial binding set
    /// must break this law.
    #[test]
    fn an_application_binds_every_hole_exactly_once() {
        let supplied = bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]);
        assert_eq!(supplied.len(), 2);
        let applied = apply(supplied);
        assert!(applied.is_ok_and(|application| {
            application.arity() == 2
                && application.bindings().count() == 2
                && application.template() == ExactIdentity::decoded([85; 32])
                && application.language_profile().version.position() == 4
                && application.meta_profile().version.position() == 5
                && matches!(
                    application.distinctness(),
                    ApplicativeDistinctness::Applicative
                )
        }));

        let unbound = apply(bindings(&[(SpliceCategory::Type, 20, 30)]));
        assert!(unbound.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::MissingBinding { .. }
        )));

        let doubled = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Type, 20, 33),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DuplicateBinding { .. }
        )));
    }

    /// law: template.an-application-refuses-a-stranger-or-a-recategorized-hole —
    /// a binding naming a hole this template does not declare refuses, and a
    /// binding naming a declared hole under another category refuses naming both
    /// the declared category and the bound one.
    /// Owed reversal: an application seam that ignored an unknown binding, or
    /// one that trusted the binding's own category over the template's, must
    /// break this law.
    #[test]
    fn an_application_refuses_a_stranger_or_a_recategorized_hole() {
        let stranger = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
            (SpliceCategory::Pattern, 99, 98),
        ]));
        assert!(stranger.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::UnknownParameter { .. }
        )));

        let recategorized = apply(bindings(&[
            (SpliceCategory::Pattern, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(recategorized.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DeclaredCategoryDisagreement {
                declared: SpliceCategory::Type,
                bound: SpliceCategory::Pattern,
                ..
            }
        )));
    }

    /// law: template.deliberate-distinctness-is-identity-bearing — two
    /// applications of one template over the same bindings and profiles differ
    /// only when a distinctness identity says so; the applicative posture and a
    /// declared distinctness never read the same.
    /// Owed reversal (red twin): a boolean distinctness flag must break this
    /// law.
    #[test]
    fn deliberate_distinctness_is_identity_bearing() {
        let holes = template(parameter(SpliceCategory::Fragment, 40), Vec::new());
        assert!(holes.is_ok_and(|template| {
            let binding = TemplateBinding::bound(
                parameter(SpliceCategory::Fragment, 40),
                argument(SpliceCategory::Fragment, 41),
            );
            let pair = binding.map_err(|_| ()).and_then(|binding| {
                let applicative = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::Applicative,
                );
                let twin = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::Applicative,
                );
                let distinct = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::DeliberatelyDistinct(ExactIdentity::decoded([42; 32])),
                );
                applicative
                    .and_then(|applicative| {
                        twin.and_then(|twin| distinct.map(|distinct| (applicative, twin, distinct)))
                    })
                    .map_err(|_| ())
            });
            pair.is_ok_and(|(applicative, twin, distinct)| {
                applicative == twin && applicative != distinct
            })
        }));
    }

    /// law: template.the-invocation-key-names-seven-lawful-inputs — the key
    /// carries the template identity, the validated inputs, the source
    /// snapshot, the fragment dependencies, both profile versions, and the
    /// configuration commitment, and two keys differing only in a lawful input
    /// are different keys.
    /// Owed reversal: a key that dropped the configuration commitment must
    /// break this law.
    #[test]
    fn the_invocation_key_names_seven_lawful_inputs() {
        let key = TemplateInvocationKey {
            template: ExactIdentity::decoded([50; 32]),
            inputs: Bounded::empty(),
            source_snapshot: ExactIdentity::decoded([51; 32]),
            fragment_dependencies: Bounded::empty(),
            language_profile: language(),
            meta_profile: meta(),
            configuration: ExactIdentity::decoded([52; 32]),
        };
        let reconfigured = TemplateInvocationKey {
            configuration: ExactIdentity::decoded([53; 32]),
            ..key.clone()
        };
        assert_ne!(key, reconfigured);
        assert_eq!(key, key.clone());
        assert!(key.inputs.is_empty() && key.fragment_dependencies.is_empty());
        assert_eq!(key.language_profile.version.position(), 4);
        assert_eq!(key.meta_profile.version.position(), 5);
        assert_eq!(key.source_snapshot.as_bytes(), &[51_u8; 32]);
    }

    /// law: template.forbidden-key-facts-are-nine-and-closed — the never-roster
    /// is closed at nine, each member distinct, and none of them is a member of
    /// the key record.
    /// Owed reversal: adding a forbidden fact without placing it must break
    /// this law.
    #[test]
    fn forbidden_key_facts_are_nine_and_closed() {
        assert_eq!(INVOCATION_KEY_NEVER.len(), 9);
        let indexes: Vec<usize> = INVOCATION_KEY_NEVER
            .iter()
            .copied()
            .map(forbidden_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.seat-bounds-name-the-seat-that-overran — a bound refusal
    /// names which seat exceeded its magnitude, the declared bound, and the
    /// observed count, and the seat roster is closed.
    /// Owed reversal: a payload-free bound issue must break this law.
    #[test]
    fn seat_bounds_name_the_seat_that_overran() {
        let seats = [
            TemplateSeat::DeclaredParameters,
            TemplateSeat::SuppliedBindings,
            TemplateSeat::AxisCeilings,
        ];
        let indexes: Vec<usize> = seats.iter().copied().map(seat_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        let overrun: Vec<TemplateParameter> = (0..40_u8)
            .map(|tag| parameter(SpliceCategory::Expression, tag.saturating_add(100)))
            .collect();
        let refused = template(parameter(SpliceCategory::Expression, 99), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::SeatBoundExceeded {
                seat: TemplateSeat::DeclaredParameters,
                bound: 32,
                observed: 41
            }
        )));
    }
}
