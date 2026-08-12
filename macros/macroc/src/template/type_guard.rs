//! The template home's invariant nucleus: every road that reaches a private
//! field.
//!
//! Declared inside `types.rs` as its own child, which is what makes each of this
//! home's proofs structural. A binding is made HERE, after the two categories
//! agreed, so a string that became an identifier is a value nobody can build. A
//! ceiling is declared HERE, after every axis was accounted for exactly once, so
//! a ceiling that leaves one magnitude unbounded while the others look governed
//! does not exist. A template and an application are built HERE for the same
//! reason. There is no other seam in the crate that can produce any of them.

use super::super::establish::{binding_issues, ceiling_issues, parameter_issues, refused};
use super::{
    ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate, ProfileCeiling,
    SpliceCategory, SymbolicBoundFormula, TemplateApplication, TemplateArgument, TemplateBinding,
    TemplateBindingIssue, TemplateConstruction, TemplateConstructionIssue, TemplateParameter,
    TemplateSeat, VersionedProfile,
};
use crate::plane::{
    AuthoringLimitProfile, LanguageProfileSubject, MetaBoundAxisLimit, MetaProfileSubject,
    OwnerIdentityRef, TemplateParameterLimit, TemplateSubject,
};
use threadpak::declaration::Stage;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit, NonEmptyBounded, PositiveLimit};

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
