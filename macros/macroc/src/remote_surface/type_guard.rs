//! The remote-surface home's invariant nucleus: every road that reaches a
//! private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's
//! walls structural: a pairing is seated by one road that refuses an empty
//! spelling, a spelling that is not an identifier, and two roads spelled alike,
//! and there is no second road that seats one — so a surface that wrote the wire
//! with the road it then read the wire with is a value nobody can write rather
//! than a state a reader has to notice.
//!
//! The surface is built here for the same reason. A remote surface exists only
//! where the plan declared a member under its role, only where that member is
//! written as a standalone artifact, only where the plan's context binds a host
//! contract, and only where the rendering fits the declared token magnitude — so
//! there is no half-composed delivery for a reader to mistake for a whole one.
//!
//! There is no refusal-body seat module here, and the absence is the honest
//! shape. This home's composition family is single-cause
//! ([`RemoteSurfaceIssue`]): every check is dependent on the one before it, so
//! there is no set of co-established issues for a body to carry and no private
//! seat for one to be built behind.

use super::super::plan::remote_surface_plan;
use super::super::render;
use super::{
    CodecPairing, IntegrationTargetLanding, PairedCodecRoad, RemoteSurface,
    RemoteSurfaceDeclarationRefusal, RemoteSurfaceIssue, RemoteSurfaceShape, SurfacePathRooting,
    SurfaceSignature, SurfaceTypePath,
};
use crate::origin_graph::OriginTrail;
use crate::plane::{
    AuthoringLimitProfile, ByteRoleSubject, GeneratedUnitSubject, OwnerIdentityRef, ProfileVersion,
    ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::{
    MemberDestination, ProjectionPlan, RemoteSurfaceProjection, SurfaceDirection,
};
use crate::token::GeneratedTree;
use threadpak::types::{NonEmptyBounded, PositiveLimit};

// ---------------------------------------------------------------------------
// The rendered vocabulary's nuclei.
// ---------------------------------------------------------------------------

impl SurfaceTypePath {
    /// One type path, rooted as the caller stated and spelled from the segments
    /// it named.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSurfaceDeclarationRefusal::PathSegmentsAbsent`] where no
    /// segment was supplied — a path naming nothing names nothing —
    /// [`RemoteSurfaceDeclarationRefusal::SegmentNotAnIdentifier`] where a segment
    /// is not one Rust identifier, and
    /// [`RemoteSurfaceDeclarationRefusal::PathSegmentsUnbounded`] where the
    /// segments outgrow the declared magnitude.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true of
    /// any refused path.
    pub fn spelled(
        rooting: SurfacePathRooting,
        segments: Vec<String>,
    ) -> Result<Self, RemoteSurfaceDeclarationRefusal> {
        let mut supplied = segments.into_iter();
        let Some(first) = supplied.next() else {
            return Err(RemoteSurfaceDeclarationRefusal::PathSegmentsAbsent);
        };
        let rest: Vec<String> = supplied.collect();
        if !is_surface_identifier(first.as_str())
            || rest.iter().any(|segment| !is_surface_identifier(segment))
        {
            return Err(RemoteSurfaceDeclarationRefusal::SegmentNotAnIdentifier);
        }
        let admitted = NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        )
        .map_err(|_| RemoteSurfaceDeclarationRefusal::PathSegmentsUnbounded)?;
        Ok(Self {
            rooting,
            segments: admitted,
        })
    }

    /// Where this path is rooted.
    #[must_use]
    pub const fn rooting(&self) -> SurfacePathRooting {
        self.rooting
    }

    /// The segments, from the root inward; structurally at least one.
    pub fn segments(&self) -> impl Iterator<Item = &String> {
        self.segments.iter()
    }

    /// How many segments the path carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.len()
    }
}

impl CodecPairing {
    /// Declare the codec pairing one surface rides.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSurfaceDeclarationRefusal::EmptyPairingRoad`] where either
    /// road states no spelling,
    /// [`RemoteSurfaceDeclarationRefusal::PairingRoadNotAnIdentifier`] where
    /// either is not one Rust identifier, and
    /// [`RemoteSurfaceDeclarationRefusal::PairingRoadsDoubled`] where the two
    /// carry one spelling.
    ///
    /// The checks are dependent and in that order — there is no pair to compare
    /// until both spellings are names — so exactly one cause is true of any
    /// refused pairing.
    pub fn paired(
        codec: SurfaceTypePath,
        encode: &str,
        decode: &str,
    ) -> Result<Self, RemoteSurfaceDeclarationRefusal> {
        if encode.is_empty() || decode.is_empty() {
            return Err(RemoteSurfaceDeclarationRefusal::EmptyPairingRoad);
        }
        if !is_surface_identifier(encode) || !is_surface_identifier(decode) {
            return Err(RemoteSurfaceDeclarationRefusal::PairingRoadNotAnIdentifier);
        }
        if encode == decode {
            return Err(RemoteSurfaceDeclarationRefusal::PairingRoadsDoubled);
        }
        Ok(Self {
            codec,
            encode: encode.to_owned(),
            decode: decode.to_owned(),
        })
    }

    /// The codec's own type, whose two roads carry the wire contract's bytes.
    #[must_use]
    pub const fn codec(&self) -> &SurfaceTypePath {
        &self.codec
    }

    /// One of the pairing's two roads, by the roster the facing table answers in.
    ///
    /// Read through the roster rather than through two accessors, so the road a
    /// facing names and the road the rendering calls are one lookup and cannot be
    /// crossed at a call site.
    #[must_use]
    pub fn road(&self, road: PairedCodecRoad) -> &str {
        match road {
            PairedCodecRoad::Encode => self.encode.as_str(),
            PairedCodecRoad::Decode => self.decode.as_str(),
        }
    }
}

impl SurfaceSignature {
    /// State the signature the rendered surface road stands at.
    ///
    /// Total: every seat is a path that already refused everything a path can be
    /// refused for, and there is nothing left for this road to check — so it has
    /// no error branch for a caller to fill.
    #[must_use]
    pub fn stated(
        accepts: SurfaceTypePath,
        answers: SurfaceTypePath,
        refusal: SurfaceTypePath,
    ) -> Self {
        Self {
            accepts,
            answers,
            refusal,
        }
    }

    /// What the rendered road accepts.
    #[must_use]
    pub const fn accepts(&self) -> &SurfaceTypePath {
        &self.accepts
    }

    /// What the rendered road answers with.
    #[must_use]
    pub const fn answers(&self) -> &SurfaceTypePath {
        &self.answers
    }

    /// The refusal every checked call on the road is converted into.
    #[must_use]
    pub const fn refusal(&self) -> &SurfaceTypePath {
        &self.refusal
    }
}

impl RemoteSurfaceShape {
    /// Declare one complete remote-surface shape.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSurfaceDeclarationRefusal::EmptyPortRoad`] where the port
    /// states no road,
    /// [`RemoteSurfaceDeclarationRefusal::PortRoadNotAnIdentifier`] where that
    /// road is not one Rust identifier,
    /// [`RemoteSurfaceDeclarationRefusal::EmptyEntrySpelling`] where the entry
    /// states no spelling, and
    /// [`RemoteSurfaceDeclarationRefusal::EntrySpellingNotAnIdentifier`] where
    /// that spelling is not one Rust identifier.
    ///
    /// The checks are dependent and in that order, so exactly one cause is true of
    /// any refused shape.
    pub fn declared(
        port: SurfaceTypePath,
        call: &str,
        pairing: CodecPairing,
        signature: SurfaceSignature,
        entry: &str,
    ) -> Result<Self, RemoteSurfaceDeclarationRefusal> {
        if call.is_empty() {
            return Err(RemoteSurfaceDeclarationRefusal::EmptyPortRoad);
        }
        if !is_surface_identifier(call) {
            return Err(RemoteSurfaceDeclarationRefusal::PortRoadNotAnIdentifier);
        }
        if entry.is_empty() {
            return Err(RemoteSurfaceDeclarationRefusal::EmptyEntrySpelling);
        }
        if !is_surface_identifier(entry) {
            return Err(RemoteSurfaceDeclarationRefusal::EntrySpellingNotAnIdentifier);
        }
        Ok(Self {
            port,
            call: call.to_owned(),
            pairing,
            signature,
            entry: entry.to_owned(),
        })
    }

    /// The type realizing the port declaration at the address.
    #[must_use]
    pub const fn port(&self) -> &SurfaceTypePath {
        &self.port
    }

    /// The port's own road the surface calls between the pairing's two.
    #[must_use]
    pub fn call(&self) -> &str {
        self.call.as_str()
    }

    /// The codec pairing this surface rides.
    #[must_use]
    pub const fn pairing(&self) -> &CodecPairing {
        &self.pairing
    }

    /// The signature the rendered road stands at.
    #[must_use]
    pub const fn signature(&self) -> &SurfaceSignature {
        &self.signature
    }

    /// The spelling the rendered surface is declared under.
    #[must_use]
    pub fn entry(&self) -> &str {
        self.entry.as_str()
    }
}

impl IntegrationTargetLanding {
    /// The landing one plan declared: the integration target's own file, under
    /// the byte role the planned member is written as an artifact beneath.
    ///
    /// Reached from `plan.rs`, which reads the byte role off the planned member's
    /// destination. There is no road that invents one: a byte role this home chose
    /// would be this home deciding which bytes somebody else's target carries.
    #[must_use]
    pub const fn in_integration_target(byte_role: OwnerIdentityRef<ByteRoleSubject>) -> Self {
        Self { byte_role }
    }

    /// The byte role the artifact is written under.
    #[must_use]
    pub const fn byte_role(&self) -> OwnerIdentityRef<ByteRoleSubject> {
        self.byte_role
    }

    /// The destination this landing IS, rebuilt as the plan's own vocabulary.
    ///
    /// Composed rather than stored, so a landing whose destination disagreed with
    /// its byte role is not a value anybody can hold.
    #[must_use]
    pub const fn destination(&self) -> MemberDestination {
        MemberDestination::AsArtifact {
            byte_role: self.byte_role,
        }
    }
}

// ---------------------------------------------------------------------------
// The composed surface.
// ---------------------------------------------------------------------------

impl RemoteSurface {
    /// Compose one remote surface.
    ///
    /// The order is the road: what the plan decided, then the rendering under the
    /// facing the plan declared — so a surface never exists that the reading did
    /// not agree on.
    ///
    /// # Errors
    ///
    /// Returns [`RemoteSurfaceIssue`] naming the plan's disagreement (the role was
    /// not planned, its member is spliced at the declaration site rather than
    /// written as an artifact, or the context binds no host contract) or the
    /// rendering's (a surface past the declared token magnitude). The checks are
    /// dependent, so exactly one cause is ever established — which is why the
    /// family is single-cause and there is no body to collect.
    pub fn composed(
        plan: &ProjectionPlan<RemoteSurfaceProjection>,
        shape: &RemoteSurfaceShape,
    ) -> Result<Self, RemoteSurfaceIssue> {
        let stated = remote_surface_plan(plan)?;
        let tree = render::surface_road(shape, stated.direction)?;
        Ok(Self {
            role: stated.role,
            semantic_key: stated.semantic_key,
            profile: stated.profile,
            profile_version: stated.profile_version,
            origin: stated.origin,
            landing: stated.landing,
            faces: stated.direction,
            tree,
        })
    }

    /// The rendered role this surface stands under.
    #[must_use]
    pub const fn role(&self) -> SoleRenderedUnit {
        self.role
    }

    /// The planned member's semantic key this surface answers to.
    #[must_use]
    pub const fn semantic_key(&self) -> ProjectionIdentity<GeneratedUnitSubject> {
        self.semantic_key
    }

    /// The profile the plan expected to render it.
    #[must_use]
    pub const fn profile(&self) -> ProjectionIdentity<ProjectionProfileSubject> {
        self.profile
    }

    /// That profile's version.
    #[must_use]
    pub const fn profile_version(&self) -> ProfileVersion {
        self.profile_version
    }

    /// The trail this surface walks back along to authored material.
    #[must_use]
    pub const fn origin(&self) -> &OriginTrail {
        &self.origin
    }

    /// Where the rendered artifact lands.
    #[must_use]
    pub const fn landing(&self) -> &IntegrationTargetLanding {
        &self.landing
    }

    /// Which way this surface faces, and therefore which of the pairing's roads
    /// opened it.
    ///
    /// # Nonclaims
    ///
    /// A facing says which end of the wire the rendered road stands at and
    /// nothing about who calls it: an inbound surface is not a server and an
    /// outbound one is not a client, because neither this home nor the plan it
    /// read says anything about who holds the road.
    #[must_use]
    pub const fn faces(&self) -> SurfaceDirection {
        self.faces
    }

    /// The rendered surface — the entry road, its two pairing calls, and the
    /// port's own call between them.
    #[must_use]
    pub const fn tree(&self) -> &GeneratedTree {
        &self.tree
    }
}

// ---------------------------------------------------------------------------
// The alphabet.
// ---------------------------------------------------------------------------

/// Whether one spelling is a single Rust identifier this home is willing to
/// render.
///
/// ASCII only, and `_` alone is refused because it is the wildcard pattern rather
/// than a name. Published from `types.rs` so every road that renders a spelling
/// reads one alphabet.
#[must_use]
pub fn is_surface_identifier(spelling: &str) -> bool {
    let mut characters = spelling.chars();
    let Some(head) = characters.next() else {
        return false;
    };
    if !head.is_ascii_alphabetic() && head != '_' {
        return false;
    }
    if spelling == "_" {
        return false;
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}
