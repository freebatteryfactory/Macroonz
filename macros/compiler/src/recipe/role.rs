//! The one complete account of syntax, availability, destination, and placement for every recipe role.

use super::types::{
    EVIDENCE_ROLES, PROJECTION_ROLES, ProjectionStanding, RecipeRoleAvailability,
    RecipeRoleEntrance, RecipeRoleOutput, RecipeRolePlacement, RecipeRoleProfile,
};
use super::{PROJECTION_LIMIT, RecipeRole};
use crate::kind::Destination;

impl RecipeRole {
    /// Reads the complete compiler-owned profile for this role.
    pub(super) fn profile(self) -> RecipeRoleProfile {
        match self {
            Self::Companions => profile(0, "companions", always(), baked(0), None),
            Self::RelationTables => profile(1, "relation_tables", always(), baked(1), None),
            Self::Dispatch => profile(2, "dispatch", always(), baked(3), None),
            Self::CompileContract => profile(3, "compile_contract", harness(), support(), None),
            Self::DeclarationConformance => {
                profile(4, "declaration_conformance", harness(), support(), None)
            }
            Self::Typestate => profile(5, "typestate", always(), baked(4), None),
            Self::Trials => evidence_profile(6, "trials", root(0), 0),
            Self::Mutation => evidence_profile(7, "mutation", root(1), 1),
            Self::Benchmarks => evidence_profile(8, "benchmarks", root(2), 2),
            Self::Network => evidence_profile(9, "network", baked(5), 3),
            Self::Concurrency => evidence_profile(10, "concurrency", baked(6), 4),
            Self::Codec => profile(11, "codec", always(), baked(2), None),
        }
    }

    pub(super) fn from_syntax(spelling: &str, entrance: RecipeRoleEntrance) -> Option<Self> {
        let roles = match entrance {
            RecipeRoleEntrance::Projection => PROJECTION_ROLES,
            RecipeRoleEntrance::Evidence => EVIDENCE_ROLES,
        };
        roles
            .iter()
            .copied()
            .find(|role| role.profile().syntax == spelling)
    }

    pub(super) fn roles_at(placement: RecipeRolePlacement) -> impl Iterator<Item = Self> {
        let mut roles = Self::ALL
            .iter()
            .copied()
            .filter(|role| role.profile().output.placement == placement)
            .collect::<Vec<_>>();
        roles.sort_by_key(|role| role.profile().output.placement_position);
        roles.into_iter()
    }

    pub(super) fn evidence_roles() -> impl Iterator<Item = Self> {
        EVIDENCE_ROLES.iter().copied()
    }

    pub(super) fn standing(
        self,
        standings: &[ProjectionStanding; PROJECTION_LIMIT],
    ) -> &ProjectionStanding {
        let Some(standing) = standings.get(self.profile().position) else {
            unreachable!("the complete role profile position fits the projection account")
        };
        standing
    }

    pub(super) fn standing_mut(
        self,
        standings: &mut [ProjectionStanding; PROJECTION_LIMIT],
    ) -> &mut ProjectionStanding {
        let Some(standing) = standings.get_mut(self.profile().position) else {
            unreachable!("the complete role profile position fits the projection account")
        };
        standing
    }
}

fn profile(
    position: usize,
    syntax: &'static str,
    availability: RecipeRoleAvailability,
    output: RecipeRoleOutput,
    evidence_position: Option<usize>,
) -> RecipeRoleProfile {
    RecipeRoleProfile {
        position,
        syntax,
        entrance: RecipeRoleEntrance::Projection,
        availability,
        output,
        evidence_position,
    }
}

fn evidence_profile(
    position: usize,
    syntax: &'static str,
    output: RecipeRoleOutput,
    evidence_position: usize,
) -> RecipeRoleProfile {
    RecipeRoleProfile {
        position,
        syntax,
        entrance: RecipeRoleEntrance::Evidence,
        availability: RecipeRoleAvailability::Harness,
        output,
        evidence_position: Some(evidence_position),
    }
}

fn always() -> RecipeRoleAvailability {
    RecipeRoleAvailability::Always
}

fn harness() -> RecipeRoleAvailability {
    RecipeRoleAvailability::Harness
}

fn baked(position: usize) -> RecipeRoleOutput {
    RecipeRoleOutput {
        destination: Destination::DeclarationSite,
        placement: RecipeRolePlacement::BakedModule,
        placement_position: Some(position),
    }
}

fn root(position: usize) -> RecipeRoleOutput {
    RecipeRoleOutput {
        destination: Destination::DeclarationSite,
        placement: RecipeRolePlacement::DeclarationRoot,
        placement_position: Some(position),
    }
}

fn support() -> RecipeRoleOutput {
    RecipeRoleOutput {
        destination: Destination::TestCarrier,
        placement: RecipeRolePlacement::SupportCarrier,
        placement_position: None,
    }
}
