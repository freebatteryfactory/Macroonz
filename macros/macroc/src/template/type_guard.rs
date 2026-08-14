//! The template home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes each of this
//! home's proofs structural. A binding is made HERE, after the two categories
//! agreed, so a string that became an identifier is a value nobody can build. A
//! ceiling is declared HERE, after every axis was accounted for exactly once, so
//! a ceiling that leaves one magnitude unbounded while the others look governed
//! does not exist. A template and an application are built HERE for the same
//! reason, and so is the refusal BODY: its seat is private, so this file is the
//! only module in the workspace that can spell the literal. There is no other
//! seam in the crate that can produce any of them.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: `establish.rs` beside it, anywhere else in the
//! services, and any crate downstream cannot write the literal, and the compiler
//! says so with `E0451`. It does not exclude DESCENDANTS — a module declared
//! inside this one would construct as freely as these roads do, so a
//! `#[cfg(test)] mod` under the guard would reopen exactly what the guard closes,
//! and the reversals for this seat are testpak's compile-fail fixtures instead.
//!
//! And it excludes the literal only. The refusal body's two mints are
//! module-private for the other half of the same claim: a private seat reached
//! by a public generic constructor lets any holder of an issue produce a body no
//! pass established, and lets a holder of the borrowed body clone its issues out
//! and reseat them. Both roads sit beside the three passes that raise them.

use super::super::establish::{binding_issues, ceiling_issues, parameter_issues};
use super::{
    ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate, ProfileCeiling,
    SpliceCategory, SymbolicBoundFormula, TemplateApplication, TemplateArgument, TemplateBinding,
    TemplateBindingIssue, TemplateConstruction, TemplateConstructionIssue, TemplateParameter,
    TemplateSeat, VersionedProfile,
};
use crate::plane::{
    AuthoringLimitProfile, LanguageProfileSubject, MetaBoundAxisLimit, MetaProfileSubject,
    OwnerIdentityRef, TemplateIssueLimit, TemplateParameterLimit, TemplateSubject,
};
use threadpak::declaration::Stage;
use threadpak::refusal::{AdmittedPrefix, StopBound};
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
fn refused(issues: Vec<TemplateConstructionIssue>) -> Option<TemplateConstruction> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(TemplateConstruction::co_established(
        first,
        established.collect(),
    ))
}

impl TemplateConstruction {
    /// The one-issue body. Total: the declared bound admits an item by
    /// compile-time proof, so refusing never needs an error road of its own.
    ///
    /// Module-private, like every other road in this file: the three passes that
    /// establish these issues are the three checked constructors below, and a
    /// body exists only where one of them ran. A public road here would let any
    /// holder of an issue mint a refusal no pass raised, which is the same
    /// opening a public SEAT would be — the seat and the mint are two halves of
    /// one claim, and closing one of them closes neither.
    fn established(issue: TemplateConstructionIssue) -> Self {
        Self {
            body: AdmittedPrefix::carrying_one(issue),
        }
    }

    /// The several-issue body.
    ///
    /// The three passes in `establish.rs` run their rosters to the end before a
    /// body exists, so the posture here is about the REPORT and never about the
    /// passes. Where every established issue fits the declared bound the body
    /// carries all of them; where it does not, the body carries what the bound
    /// holds and names how many established issues stand outside it — never a
    /// silent drop, never an unearned claim of completeness, and never a claim
    /// that nobody looked.
    ///
    /// Module-private, on the same terms as the one-issue road.
    fn co_established(
        first: TemplateConstructionIssue,
        rest: Vec<TemplateConstructionIssue>,
    ) -> Self {
        Self {
            body: AdmittedPrefix::examined_completely(
                first,
                rest,
                &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                StopBound::DeclaredIssueBound,
            ),
        }
    }

    /// The established issues and what this refusal says about its own coverage
    /// of them.
    ///
    /// Borrowed and never owned, for the reason band 00 borrows its carry: an
    /// owned body is a value a caller can seat under another refusal, which is
    /// the pairing the coupled seat exists to end.
    pub const fn body(&self) -> &AdmittedPrefix<TemplateConstructionIssue, TemplateIssueLimit> {
        &self.body
    }
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
        if let Some(refusal) = refused(ceiling_issues(&axes)) {
            return Err(refusal);
        }
        let observed = axes.len();
        Bounded::admitted_const(
            axes,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
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
        identity: OwnerIdentityRef<TemplateSubject>,
        first: TemplateParameter,
        rest: Vec<TemplateParameter>,
        formula: SymbolicBoundFormula,
        ceiling: ProfileCeiling,
        meter: CheckedMeterPosture,
        stage: Stage,
    ) -> Result<Self, TemplateConstruction> {
        let declared: Vec<TemplateParameter> = core::iter::once(first)
            .chain(rest.iter().copied())
            .collect();
        if let Some(refusal) = refused(parameter_issues(&declared)) {
            return Err(refusal);
        }
        let observed = rest.len().saturating_add(1);
        NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
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
    pub const fn identity(&self) -> OwnerIdentityRef<TemplateSubject> {
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
        if let Some(refusal) = refused(binding_issues(template, &bindings)) {
            return Err(refusal);
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
        NonEmptyBounded::admitted_const(
            first,
            supplied.collect(),
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map(|bounded_bindings| Self {
            template: template.identity(),
            bindings: bounded_bindings,
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
    pub const fn template(&self) -> OwnerIdentityRef<TemplateSubject> {
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
