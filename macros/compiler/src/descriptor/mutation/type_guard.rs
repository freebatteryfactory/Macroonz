//! The mutation home's invariant nucleus: every road that reaches a private field.
//!
//! A permission is admitted here, so a claim that permits nothing is not a value anybody can hold.
//! A policy's mappings are admitted here, so one owner fact cannot be mapped twice.
//! A site's alternatives are admitted here, so an alternative that names no operation does not exist.

use super::{
    ALTERNATIVE_LIMIT, Address, Alternative, Declaration, FactMapping, FamilySlug, MAPPING_LIMIT,
    MutationCaptureError, OPERATOR_FAMILY_LIMIT, PERMISSION_LIMIT, Permission, Policy, Site,
    Surface,
};
use crate::bounded::{Bounded, NonEmpty, first_duplicate_position};
use crate::descriptor::{CaptureCause, DeclarationError, Grammar, HelperRefusal, Name, Seat};
use crate::token::{GeneratedToken, SpanHandle};

impl FamilySlug {
    /// One operator family, by the slug the address resolves it under.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where the slug carries no characters: a family nobody named is a family the address cannot resolve.
    pub fn declared(slug: &str) -> Result<Self, DeclarationError> {
        if slug.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::OperatorFamily,
            });
        }
        Ok(Self(slug.to_owned()))
    }

    /// The slug itself.
    #[must_use]
    pub fn slug(&self) -> &str {
        self.0.as_str()
    }
}

impl Permission {
    /// Bind one claim to the operator families it permits.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where no family was named — a permission over nothing permits nothing and says so by not being written — [`DeclarationError::Doubled`] where one family is named twice, and [`DeclarationError::Unbounded`] where the roster outgrows [`OPERATOR_FAMILY_LIMIT`].
    pub fn permitted(claim: Name, families: Vec<FamilySlug>) -> Result<Self, DeclarationError> {
        if families.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::OperatorFamily,
            });
        }
        let offered = families.len();
        if first_duplicate_position(&families, |left, right| left == right).is_some() {
            return Err(DeclarationError::Doubled {
                seat: Seat::OperatorFamily,
            });
        }
        let admitted = NonEmpty::new(families).map_err(|_| {
            DeclarationError::unbounded(Seat::OperatorFamily, OPERATOR_FAMILY_LIMIT, offered)
        })?;
        Ok(Self {
            claim,
            families: admitted,
        })
    }

    /// The claim whose permission this row states.
    #[must_use]
    pub const fn claim(&self) -> &Name {
        &self.claim
    }

    /// The operator families this row permits; structurally at least one.
    #[must_use]
    pub fn families(&self) -> &NonEmpty<FamilySlug, OPERATOR_FAMILY_LIMIT> {
        &self.families
    }
}

impl Policy {
    /// Declare the complete policy one surface is lowered under.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Doubled`] where one owner fact is mapped twice or two permissions name one claim, and [`DeclarationError::Unbounded`] where either roster outgrows its declared magnitude.
    pub fn declared(
        family: Name,
        mappings: Vec<FactMapping>,
        permissions: Vec<Permission>,
    ) -> Result<Self, DeclarationError> {
        if first_duplicate_position(&mappings, |left, right| left.fact == right.fact).is_some() {
            return Err(DeclarationError::Doubled {
                seat: Seat::FactMapping,
            });
        }
        if first_duplicate_position(&permissions, |left, right| left.claim() == right.claim())
            .is_some()
        {
            return Err(DeclarationError::Doubled {
                seat: Seat::Permission,
            });
        }
        let offered_mappings = mappings.len();
        let admitted_mappings = Bounded::new(mappings).map_err(|_| {
            DeclarationError::unbounded(Seat::FactMapping, MAPPING_LIMIT, offered_mappings)
        })?;
        let offered_permissions = permissions.len();
        let admitted_permissions = Bounded::new(permissions).map_err(|_| {
            DeclarationError::unbounded(Seat::Permission, PERMISSION_LIMIT, offered_permissions)
        })?;
        Ok(Self {
            family,
            mappings: admitted_mappings,
            permissions: admitted_permissions,
        })
    }

    /// The evaluation family the rendered surface belongs to.
    #[must_use]
    pub const fn family(&self) -> &Name {
        &self.family
    }

    /// The owner-fact mappings, in the order they were declared.
    #[must_use]
    pub fn mappings(&self) -> &[FactMapping] {
        self.mappings.as_slice()
    }

    /// The permissions, in the order they were declared.
    #[must_use]
    pub fn permissions(&self) -> &[Permission] {
        self.permissions.as_slice()
    }

    /// The claim mapped to one owner fact, where the policy maps it.
    #[must_use]
    pub fn claim_for(&self, fact: &Name) -> Option<&Name> {
        self.mappings()
            .iter()
            .find(|mapping| mapping.fact == *fact)
            .map(|mapping| &mapping.claim)
    }
}

impl Alternative {
    /// Declare one alternative at a site.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Absent`] where the operation carries no bytes: an alternative that identifies no operation is one nothing can select.
    pub fn stated(
        family: FamilySlug,
        operation: Vec<u8>,
        meaning: Vec<GeneratedToken>,
    ) -> Result<Self, DeclarationError> {
        if operation.is_empty() {
            return Err(DeclarationError::Absent {
                seat: Seat::Alternative,
            });
        }
        Ok(Self {
            family,
            operation,
            meaning,
        })
    }

    /// The operator family this alternative belongs to.
    #[must_use]
    pub const fn family(&self) -> &FamilySlug {
        &self.family
    }

    /// The semantic bytes that identify this operation.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        self.operation.as_slice()
    }

    /// The value this alternative means, as the tokens that spell it.
    #[must_use]
    pub fn meaning(&self) -> &[GeneratedToken] {
        self.meaning.as_slice()
    }
}

impl Site {
    /// Declare one mutation site.
    ///
    /// # Errors
    ///
    /// Returns [`DeclarationError::Doubled`] where two alternatives carry one operation — two selections over one operation are one selection nobody can tell apart — and [`DeclarationError::Unbounded`] where the alternatives outgrow [`ALTERNATIVE_LIMIT`].
    pub fn declared(
        point: Name,
        fact: Name,
        order: Vec<GeneratedToken>,
        production: Vec<GeneratedToken>,
        unchanged: Vec<u8>,
        alternatives: Vec<Alternative>,
    ) -> Result<Self, DeclarationError> {
        if first_duplicate_position(&alternatives, |left, right| {
            left.operation() == right.operation()
        })
        .is_some()
        {
            return Err(DeclarationError::Doubled {
                seat: Seat::Alternative,
            });
        }
        let offered = alternatives.len();
        let admitted = Bounded::new(alternatives).map_err(|_| {
            DeclarationError::unbounded(Seat::Alternative, ALTERNATIVE_LIMIT, offered)
        })?;
        Ok(Self {
            point,
            fact,
            order,
            production,
            unchanged,
            alternatives: admitted,
        })
    }

    /// The point this site is discovered at.
    #[must_use]
    pub const fn point(&self) -> &Name {
        &self.point
    }

    /// The owner fact this site's alternatives press on.
    #[must_use]
    pub const fn fact(&self) -> &Name {
        &self.fact
    }

    /// The type every alternative and the production are values of.
    #[must_use]
    pub fn order(&self) -> &[GeneratedToken] {
        self.order.as_slice()
    }

    /// The expression the unchanged declaration answers with.
    #[must_use]
    pub fn production(&self) -> &[GeneratedToken] {
        self.production.as_slice()
    }

    /// The semantic bytes of the operation nothing pressed.
    #[must_use]
    pub fn unchanged(&self) -> &[u8] {
        self.unchanged.as_slice()
    }

    /// The alternatives, in the order they were declared.
    #[must_use]
    pub fn alternatives(&self) -> &[Alternative] {
        self.alternatives.as_slice()
    }
}

impl Declaration {
    /// Bind what one helper body stated.
    ///
    /// Total: every part was admitted by its own road before it reached this one.
    #[must_use]
    pub const fn captured(address: Address, policy: Policy, point: Name, fact: Name) -> Self {
        Self {
            address,
            policy,
            point,
            fact,
        }
    }

    /// Where the rendered surface lands and what it is invoked by.
    #[must_use]
    pub const fn address(&self) -> &Address {
        &self.address
    }

    /// The policy the surface is lowered under.
    #[must_use]
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }

    /// The point this declaration's site is discovered at.
    #[must_use]
    pub const fn point(&self) -> &Name {
        &self.point
    }

    /// The owner fact this declaration's site stands on.
    #[must_use]
    pub const fn fact(&self) -> &Name {
        &self.fact
    }

    /// Complete this declaration with the material only the door that captured it can compute.
    ///
    /// # Errors
    ///
    /// Returns whatever [`Site::declared`] refuses with.
    pub fn completed(
        self,
        order: Vec<GeneratedToken>,
        production: Vec<GeneratedToken>,
        unchanged: Vec<u8>,
        alternatives: Vec<Alternative>,
    ) -> Result<Surface, DeclarationError> {
        let site = Site::declared(
            self.point,
            self.fact,
            order,
            production,
            unchanged,
            alternatives,
        )?;
        Ok(Surface::declared(self.address, self.policy, site))
    }
}

impl Surface {
    /// Declare the complete payload one mutation surface is written from.
    ///
    /// Total: the address, the policy, and the site were each admitted by their own road before they reached this one, and nothing here is a fact about more than one of them.
    #[must_use]
    pub const fn declared(address: Address, policy: Policy, site: Site) -> Self {
        Self {
            address,
            policy,
            site,
        }
    }

    /// Where the rendered surface lands and what it is invoked by.
    #[must_use]
    pub const fn address(&self) -> &Address {
        &self.address
    }

    /// The policy the surface is lowered under.
    #[must_use]
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }

    /// The site whose alternatives an evaluation selects between.
    #[must_use]
    pub const fn site(&self) -> &Site {
        &self.site
    }
}

impl MutationCaptureError {
    /// One refusal the mutation grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// One refusal the vocabulary established over a value this grammar read.
    pub const fn vocabulary_refused(
        grammar: Grammar,
        refusal: DeclarationError,
        at: SpanHandle,
    ) -> Self {
        Self(HelperRefusal::vocabulary_refused(grammar, refusal, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}
