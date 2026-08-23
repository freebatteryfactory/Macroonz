//! Smart constructors and readers for one captured mutation declaration.

use super::{
    GeneratedMutationFamily, MutationDeclaration, MutationModuleName, MutationOrderCause,
    MutationOrderDeclaration, MutationOwnerFact, MutationProjectionRequest,
    OperatorPermissionDeclaration, OwnerClaimDeclaration,
};
use crate::test_descriptor::{
    ShellDeclarationRefusal, SupportMacroName, WallName, is_rendered_identifier,
};
use crate::token::SpanHandle;

impl MutationModuleName {
    /// Parse the module identifier one mutation delivery writes.
    ///
    /// # Errors
    ///
    /// Refuses a spelling that is not one Rust identifier.
    pub(crate) fn declared(spelling: &str) -> Result<Self, ShellDeclarationRefusal> {
        if !is_rendered_identifier(spelling) {
            return Err(ShellDeclarationRefusal::SpellingNotAnIdentifier);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The Rust identifier spelling this module carries.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl MutationOwnerFact {
    /// Resolve one helper spelling to the sealed generated fact it names.
    #[must_use]
    pub const fn of_spelling(spelling: &str) -> Option<Self> {
        match spelling.as_bytes() {
            b"declared_order" => Some(Self::DeclaredOrder),
            _ => None,
        }
    }

    /// The one helper spelling this fact is declared under.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::DeclaredOrder => "declared_order",
        }
    }
}

impl GeneratedMutationFamily {
    /// Resolve one declared slug to a family this producer can materialize.
    #[must_use]
    pub const fn of_slug(slug: &str) -> Option<Self> {
        match slug.as_bytes() {
            b"declared-order-permutation" => Some(Self::DeclaredOrderPermutation),
            _ => None,
        }
    }

    /// The `TestPak` operator-bank slug this generated family names.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::DeclaredOrderPermutation => "declared-order-permutation",
        }
    }
}

impl OwnerClaimDeclaration {
    /// Bind one sealed generated fact to one owner claim.
    #[must_use]
    pub(crate) const fn mapped(fact: MutationOwnerFact, claim: WallName) -> Self {
        Self { fact, claim }
    }

    /// The generated fact this row maps.
    #[must_use]
    pub const fn fact(&self) -> MutationOwnerFact {
        self.fact
    }

    /// The owner claim this row maps the fact to.
    #[must_use]
    pub const fn claim(&self) -> &WallName {
        &self.claim
    }
}

impl OperatorPermissionDeclaration {
    /// Bind one owner claim to the nonempty generated-family roster it permits.
    #[must_use]
    pub(crate) fn permitted(
        claim: WallName,
        first: GeneratedMutationFamily,
        mut remaining: Vec<GeneratedMutationFamily>,
    ) -> Self {
        let mut families = Vec::with_capacity(remaining.len().saturating_add(1));
        families.push(first);
        families.append(&mut remaining);
        Self { claim, families }
    }

    /// The owner claim whose permission this row states.
    #[must_use]
    pub const fn claim(&self) -> &WallName {
        &self.claim
    }

    /// The generated operator families this row permits.
    #[must_use]
    pub fn families(&self) -> &[GeneratedMutationFamily] {
        &self.families
    }
}

impl MutationDeclaration {
    /// Bind the completely parsed helper into one declaration.
    pub(crate) fn captured(
        at: SpanHandle,
        module: MutationModuleName,
        support: Option<(SupportMacroName, SpanHandle)>,
        family: WallName,
        mappings: Vec<OwnerClaimDeclaration>,
        permissions: Vec<OperatorPermissionDeclaration>,
    ) -> Self {
        Self {
            at,
            module,
            support,
            family,
            mappings,
            permissions,
        }
    }

    /// The helper attribute token that authored this declaration.
    #[must_use]
    pub(crate) const fn site(&self) -> SpanHandle {
        self.at
    }

    /// The output module written into the consumption target.
    #[must_use]
    pub const fn module(&self) -> &MutationModuleName {
        &self.module
    }

    /// The public support macro this helper owns, where no trial helper owns it.
    #[must_use]
    pub fn support(&self) -> Option<&SupportMacroName> {
        self.support.as_ref().map(|(name, _)| name)
    }

    /// The helper token that authored the optional support address.
    #[must_use]
    pub(crate) fn support_site(&self) -> Option<SpanHandle> {
        self.support.as_ref().map(|(_, at)| *at)
    }

    /// The evaluation family the generated surface belongs to.
    #[must_use]
    pub const fn family(&self) -> &WallName {
        &self.family
    }

    /// The sealed generated-fact mappings in authored order.
    pub fn mappings(&self) -> impl Iterator<Item = &OwnerClaimDeclaration> {
        self.mappings.iter()
    }

    /// The owner permissions in authored order.
    pub fn permissions(&self) -> impl Iterator<Item = &OperatorPermissionDeclaration> {
        self.permissions.iter()
    }

    /// The owner claim mapped to one sealed fact, where the declaration maps it.
    #[must_use]
    pub fn mapping(&self, fact: MutationOwnerFact) -> Option<&WallName> {
        self.mappings
            .iter()
            .find(|mapping| mapping.fact() == fact)
            .map(OwnerClaimDeclaration::claim)
    }
}

impl MutationOrderCause {
    /// Bind one captured cause's operation coordinates to its rendered row.
    pub(crate) fn informed(
        local_key: &str,
        spelling: &str,
        row: Vec<crate::token::GeneratedToken>,
    ) -> Self {
        Self {
            local_key: local_key.to_owned(),
            spelling: spelling.to_owned(),
            row,
        }
    }

    /// The owner-declared local key used in mutation-operation identity.
    pub(crate) fn local_key(&self) -> &str {
        &self.local_key
    }

    /// The Rust spelling used in mutation-operation identity.
    pub(crate) fn spelling(&self) -> &str {
        &self.spelling
    }

    /// The destination-shaped typed row the frontend prepared.
    pub(crate) fn row(&self) -> &[crate::token::GeneratedToken] {
        &self.row
    }
}

impl MutationProjectionRequest {
    /// Bind every fact the mechanical renderer needs into one informed request.
    pub(crate) fn informed(
        declaration: &MutationDeclaration,
        point: WallName,
        order_type: Vec<crate::token::GeneratedToken>,
        production_expression: Vec<crate::token::GeneratedToken>,
        order_constructor: Vec<crate::token::GeneratedToken>,
        order: MutationOrderDeclaration,
        alternative_count: u64,
    ) -> Self {
        Self {
            declaration: declaration.clone(),
            point,
            order_type,
            production_expression,
            order_constructor,
            order,
            alternative_count,
        }
    }

    /// The helper declaration that owns module, policy, and support addressing.
    pub(crate) const fn declaration(&self) -> &MutationDeclaration {
        &self.declaration
    }

    /// The admitted namespaced point stem derived by the frontend.
    pub(crate) const fn point(&self) -> &WallName {
        &self.point
    }

    /// The destination-shaped declared-order type.
    pub(crate) fn order_type(&self) -> &[crate::token::GeneratedToken] {
        &self.order_type
    }

    /// The actual owner declaration's typed order expression.
    pub(crate) fn production_expression(&self) -> &[crate::token::GeneratedToken] {
        &self.production_expression
    }

    /// The destination-shaped typed-order constructor.
    pub(crate) fn order_constructor(&self) -> &[crate::token::GeneratedToken] {
        &self.order_constructor
    }

    /// The informed order posture this request carries.
    pub(crate) const fn order(&self) -> &MutationOrderDeclaration {
        &self.order
    }

    /// The compiler-admitted array magnitude for adjacent alternatives.
    pub(crate) const fn alternative_count(&self) -> u64 {
        self.alternative_count
    }
}
