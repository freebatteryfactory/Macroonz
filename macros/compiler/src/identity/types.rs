//! The identity home's declarations: the subject roster, the role roster, one profile constant per preimage grammar, the transcript and its derivation record, the generator facts, and the two citation shapes.
//!
//! Declarations only.
//! Every constructor that must see a private field lives in `type_guard.rs`, declared below as this file's own child so the invariant nucleus and the fields it protects are never separated by a module boundary.

use crate::bounded::Bounded;
use core::marker::PhantomData;

#[path = "type_guard.rs"]
mod guard;

pub use guard::names_are_separating;
pub(crate) use guard::{human_projection, static_bytes};

/// The stem every subject and every grammar this compiler owns is declared under.
pub const MACROONZ_STEM: &str = "macroonz/identity";

/// One identity subject, by the name the derive-key grammar spells it with and the stem of whoever owns it.
///
/// The pair is what separates one subject's identities from another's, so both are DECLARED beside the marker rather than taken from the Rust spelling: a refactor that silently renamed every identity derived for a type would be a law change nobody wrote down.
/// The trait is open, and a consumer's name that happens to match this compiler's roster is a different key space rather than a collision.
pub trait Subject: Copy + 'static {
    /// The subject's declared segment of the derive-key context.
    const NAME: &'static str;

    /// The stem of whoever declared it.
    const STEM: &'static str;
}

/// Declares one roster of identity subjects under one stem, as the home's README shows.
///
/// Each row becomes a marker type carrying its declared name, and the roster settles both ways it could fail to separate while it compiles: a name outside the context grammar, and a name two rows declare.
/// A declared name is lowercase ASCII letters and digits in `-`-joined segments, with no leading, trailing, or doubled separator.
#[macro_export]
macro_rules! subjects {
    (stem = $stem:expr; $( $(#[$note:meta])* $name:ident = $declared:literal ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl $crate::identity::Subject for $name {
                const NAME: &'static str = $declared;
                const STEM: &'static str = $stem;
            }
        )+

        const _: () = ::core::assert!(
            $crate::identity::names_are_separating(&[$($declared),+]),
            "a subject name outside the derive-key grammar, or one two subjects declare",
        );
    };
}

subjects! {
    stem = MACROONZ_STEM;
    /// The token material one expansion was handed.
    CapturedDeclaration = "captured-declaration",
    /// What a request MEANT, ahead of anything decided about it.
    ProjectionIntent = "projection-intent",
    /// The canonical facts one kind-specific content value carries.
    ProjectionContent = "projection-content",
    /// One projection plan.
    Plan = "plan",
    /// One generated unit — the thing a plan declares it will materialize.
    GeneratedUnit = "generated-unit",
    /// One rendered unit — the thing a renderer actually materialized.
    RenderedUnit = "rendered-unit",
    /// The canonical bytes of one rendered unit.
    OutputBytes = "output-bytes",
    /// One proved closure between a plan's declared membership and what a renderer produced.
    Closure = "closure",
    /// One explanation, answered over a plan and the closure that proved its rendering.
    Explanation = "explanation",
    /// One closed expansion: the whole account one compilation produced.
    ClosedExpansion = "closed-expansion",
    /// One node of the origin graph.
    OriginNode = "origin-node",
    /// One subject a plan explicitly does not claim.
    Nonclaim = "nonclaim",
    /// One subject a decision trace entry is about.
    Traced = "traced",
    /// One stable name this compiler wrote down, standing for a value it declares.
    DeclaredName = "declared-name",
    /// One version of the generator itself.
    GeneratorVersion = "generator-version",
    /// One related issue a diagnostic points at.
    RelatedIssue = "related-issue",
    /// The whole refusal body one diagnostic's related set commits to, as opposed to any single issue inside it.
    /// A separate subject from [`RelatedIssue`] because one key space holding two LEVELS over one material collides by construction: a body's preimage is the framing of its issues, so an issue whose own material happened to be that framing would derive the identity of the body it aliased.
    RelatedBody = "related-body",
    /// One projection profile — the posture a request ran under.
    ProjectionProfile = "projection-profile",
    /// One projection kind, named by identity where a decoded route may name a kind this compiler does not implement.
    ProjectionKind = "projection-kind",
    /// One contract a diagnostic expected to hold.
    Contract = "contract",
    /// One callable entry point.
    ServiceEntry = "service-entry",
}

/// One identity this compiler derived, tagged by the subject it names.
///
/// Holding one means these thirty-two bytes came from a complete [`Transcript`] under the profile that transcript names, and would come out the same again from the same transcript on any machine.
///
/// # Authority
///
/// Collision resistance is claimed AS BLAKE3's, for the transcript as [`Transcript`] specifies it, at the [`Version`] the deriving [`Profile`] declares — and nothing broader.
///
/// # Construction
///
/// The only road is [`Identity::derived`], which takes a typed transcript; nothing wraps arbitrary bytes.
/// `S` is a `PhantomData` parameter, so an identity naming one subject is a different type than one naming another regardless of bytes, and their derive-key contexts differ too — the separation is a runtime fact and not only a compile-time one.
///
/// # Nonclaims
///
/// It does not claim that two things this compiler considers different always have different transcripts; that is the transcript's completeness, which each mint site owns and documents.
#[derive(Clone, Copy)]
pub struct Identity<S: Subject>([u8; 32], PhantomData<S>);

/// One projection plan's own identity.
pub type PlanId = Identity<Plan>;

/// One proved closure's own identity.
pub type ClosureId = Identity<Closure>;

/// One complete explanation's own identity.
pub type ExplanationId = Identity<Explanation>;

/// One closed expansion's own identity.
pub type ClosedExpansionId = Identity<ClosedExpansion>;

/// The seat one identity stands in inside its grammar.
///
/// A role is part of the derive-key context AND a member of every transcript, so two identities derived from one anchor under different roles are different twice over: separated before a byte of the transcript is read, and disagreeing inside it.
///
/// A row's declared name and slot are what the bytes carry, so a row is APPENDED and never renumbered — renumbering an occupied slot re-encodes transcripts that were already encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// The token material one expansion was handed.
    CapturedDeclaration,
    /// One projection plan.
    Plan,
    /// One node of the origin graph.
    OriginNode,
    /// One generated unit a plan declares it will materialize.
    GeneratedUnit,
    /// One rendered unit a renderer actually materialized.
    RenderedUnit,
    /// The canonical bytes of one rendered unit.
    OutputBytes,
    /// One bundle materialized across a single publication boundary.
    Bundle,
    /// One proved closure between a plan and its rendering.
    Closure,
    /// One closed expansion.
    ClosedExpansion,
    /// One projection intent — what a request meant, ahead of what it decided.
    ProjectionIntent,
    /// One explanation, answered over a plan and its closure.
    Explanation,
    /// The documentation rows one captured declaration carries, read as a second fact over the surface its semantic commitment already names.
    DeclarationDocumentation,
    /// One stable name this compiler wrote down.
    DeclaredName,
    /// The generator's declared name and the shape it renders.
    GeneratorVersion,
    /// One refusal body, or one issue inside it, as a diagnostic points at it.
    DiagnosticRelation,
    /// One helper attribute's material, read as an independent fact over the surface its semantic commitment already names.
    ///
    /// Several helpers may stand here at once; they are separated by the roster position each one is derived at, never by a grammar of their own.
    CapturedHelper,
    /// One kind-specific content commitment.
    ProjectionContent,
    /// One projection kind qualified by the producer that owns its generated names.
    ProjectionKind,
}

/// One position in one grammar's own order.
///
/// There is no `Ord`: positions of two different grammars are not comparable, and nothing here ranks them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(u32);

/// One preimage grammar: which members a mint site writes, in what order, carrying what material.
///
/// A grammar exists because a preimage is genuinely its own, never because a type is.
/// The stem sits ahead of the name, so one owner's `"plan"` and another's are two key spaces rather than one reached twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Profile {
    stem: &'static str,
    name: &'static str,
    version: Version,
}

/// The grammar one captured declaration's SEMANTIC commitment is derived under.
///
/// The captured tree's own canonical encoding, rooted, with every documentation attribute dropped from the walk: a capture is the root of a derivation chain, and the material is the whole of what varies.
/// Spans enter nothing — a handle is the producer's own table index, and two producers reading one declaration issue different ones.
pub const CAPTURED_DECLARATION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "captured-declaration", Version::declared(1));

/// The grammar one captured declaration's DOCUMENTATION commitment is derived under.
///
/// The semantic commitment at the anchor, at full width, and over it the captured documentation rows in the order the walk read them — a second reading of one surface, so a declaration whose prose changes keeps its semantic name and takes a new documentation name.
pub const DECLARATION_DOCUMENTATION_PROFILE: Profile = Profile::declared(
    MACROONZ_STEM,
    "declaration-documentation",
    Version::declared(1),
);

/// The grammar one captured declaration's HELPER commitment is derived under.
///
/// The semantic commitment at the anchor, at full width, and over it the canonical bytes of one helper attribute's own captured trees, at the position that helper was declared at.
/// Helper material says how a declaration is exercised rather than what contract it realizes, so it is dropped from the semantic walk and enters here.
pub const CAPTURED_HELPER_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "captured-helper", Version::declared(1));

/// The grammar one DECLARED STABLE NAME's identity is derived under.
///
/// The name's own bytes, exactly as this compiler wrote them down, rooted, at the position the declaring seat states.
/// Several such names share this grammar and are separated by their subjects.
pub const DECLARED_NAME_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "declared-name", Version::declared(1));

/// The grammar one projection intent's identity is derived under.
///
/// The owner-qualified kind identity and the kind-specific content commitment it was meant over, rooted at position zero, and **nothing else** — no generator, no shape version, no delivery, no token grammar.
/// So an intent survives upgrading the machinery that realizes it, which is the whole reason the layer exists: it is the one layer two distinct requests are allowed to agree at.
pub const PROJECTION_INTENT_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "projection-intent", Version::declared(2));

/// The grammar one owner-qualified projection kind is derived under.
///
/// The producer namespace, the producer name, and the kind's declared name, each framed, rooted at position zero.
pub const PROJECTION_KIND_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "projection-kind", Version::declared(1));

/// The grammar one kind-specific content commitment is derived under.
///
/// The owner-qualified kind identity and the content's complete canonical bytes, anchored under the exact captured declaration the content was paired with, at position zero.
pub const PROJECTION_CONTENT_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "projection-content", Version::declared(1));

/// The grammar one plan's identity is derived under.
///
/// The intent, the dependency set the account declares beside it, the context, the complete membership in role order, the invalidation set, the decision trace, the origin trail, and the nonclaims — anchored on the address the content walked in carrying.
/// The context names the generator version the plan was produced under, so the generator reaches a plan's identity through the seat the plan declared it at and never through a member every grammar would have carried.
pub const PLAN_PROFILE: Profile = Profile::declared(MACROONZ_STEM, "plan", Version::declared(2));

/// The grammar one origin node's identity is derived under.
///
/// The declared material the node stands for, anchored on the address it is a node of, so one piece of content is one node wherever it is reached from.
pub const ORIGIN_NODE_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "origin-node", Version::declared(2));

/// The grammar one generated unit's semantic key is derived under.
///
/// The owner-qualified kind identity, the kind-specific content commitment, and the role's declared name, with the roster position of that role, anchored on what the plan hangs off — a member's LOGICAL identity, fixed before a byte of it exists.
pub const GENERATED_UNIT_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "generated-unit", Version::declared(2));

/// The grammar one rendered unit's identity and its output-bytes digest are both derived under.
///
/// The exact rendered bytes, under the semantic key they answer to, at the roster position of the role they were rendered under.
/// Two roles read here, on the terms [`Role::profile`] states.
pub const RENDERED_UNIT_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "rendered-unit", Version::declared(1));

/// The grammar one bundle's identity is derived under.
///
/// The member plans a bundle names, as the set it publishes as one unit.
pub const BUNDLE_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "bundle", Version::declared(1));

/// The grammar one proved closure's identity is derived under.
///
/// The plan's identity at the anchor, and over it the complete planned membership in role order, the role roster's own length, the unit that stood under each role, and the partitioned emission's digests — the whole agreement rather than a sample of it.
pub const CLOSURE_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "closure", Version::declared(1));

/// The grammar one explanation's identity is derived under.
///
/// The closure's identity at the anchor, at full width, and over it the plan's identity, the number of answered seats, and every seat in the KIND's declared question order — the question's slot, the answer's discriminant, and that answer's typed material.
/// The order is the roster's and never the caller's, so two views answering one kind's questions with one set of answers derive one identity whichever order they were supplied in.
///
/// Human prose is excluded: a rendered line is a projection of a typed answer, so a preimage carrying one would commit to a rendering and would rename every explanation the day a sentence was reworded.
pub const EXPLANATION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "explanation", Version::declared(1));

/// The grammar one closed expansion's identity is derived under.
///
/// The closure's identity at the anchor, at full width, and over it exactly two members: the plan's identity and the explanation's.
/// Every other candidate is already inside one of the three — the partitioned emission is committed by the anchor and the kind by the plan's intent — and two spellings of one fact are how a preimage drifts.
pub const CLOSED_EXPANSION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "closed-expansion", Version::declared(1));

/// The grammar the GENERATOR VERSION identity is derived under.
///
/// The generator's declared name, framed, then its shape position in four big-endian bytes, rooted at position zero; the package version is absent, for the reason [`ShapeVersion`] states.
pub const GENERATOR_VERSION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "generator-version", Version::declared(1));

/// The grammar a diagnostic's related identities are derived under, at both levels.
///
/// One refusal family's name and the framed material the level stands over — the issue's own canonical bytes at the issue level, and the framing of every issue in order at the body level — rooted at position zero.
/// The two levels are separated by their subjects, [`RelatedIssue`] and [`RelatedBody`], which is what keeps a body's preimage from being reachable as an issue's.
pub const DIAGNOSTIC_RELATION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "diagnostic-relation", Version::declared(1));

/// What one transcript hangs off.
///
/// Each posture is written as a distinct byte ahead of its commitment, so a rooted transcript can never encode as an anchored one whose anchor happened to be empty.
/// The bytes are declared here rather than left in an encoder body, because an independent reader re-deriving a transcript needs them: [`Anchoring::Rooted`] is `0`, [`Anchoring::UnderOwner`] is `1`, [`Anchoring::UnderProjection`] is `2`, and a value is appended rather than renumbered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Anchoring {
    /// No anchor at all — the root of one derivation chain, where the material is the whole of what varies.
    Rooted,
    /// Anchored under an identity a CONSUMER minted, carried at full width.
    UnderOwner([u8; 32]),
    /// Anchored under another identity this compiler derived, carried at full width.
    UnderProjection([u8; 32]),
}

/// The COMPLETE preimage one [`Identity`] is derived from.
///
/// A transcript is the exact byte string handed to the digest, and the specification is complete: an independent implementation needs what follows and nothing else.
///
/// Two primitives.
/// `u32be(n)` and `u64be(n)` are the integer in four or eight big-endian bytes; `bytes(x)` is `u64be(x.len())` followed by the bytes of `x`, and every variable-length member is written that way, so no two member sequences can be cut at a different boundary and produce one byte string.
///
/// The members, in exactly this order, with no separators and no padding:
///
/// | # | member | encoding |
/// | - | ------ | -------- |
/// | 1 | profile stem | `bytes(utf8)` of [`Profile::stem`] |
/// | 2 | profile name | `bytes(utf8)` of [`Profile::name`] |
/// | 3 | profile version | `u32be`, that grammar's own position |
/// | 4 | subject | `bytes(utf8)` of [`Subject::NAME`] |
/// | 5 | role | `bytes(utf8)` of [`Role::name`] |
/// | 6 | role slot | one byte, [`Role::slot`] |
/// | 7 | anchoring | one byte, [`Anchoring::slot`] |
/// | 8 | anchor | `bytes(…)` — empty when rooted, else the full thirty-two |
/// | 9 | material | `bytes(…)` — the full material, never a fold |
/// | 10 | position | `u32be` |
///
/// The derive-key context is [`Profile::context_for`] over the same subject and role, and the identity is `blake3::derive_key(context, transcript)`.
/// The subject's stem is a segment of that context and is not a member here, so two subjects spelled alike under different stems derive under different keys.
/// The generator is not a member either: it is carried for the derivation record ([`Transcript::provenance`]) and written into no preimage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transcript<'material> {
    profile: Profile,
    generator: GeneratorIdentity,
    role: Role,
    anchoring: Anchoring,
    material: &'material [u8],
    position: u32,
}

/// The inspectable record of ONE derivation.
///
/// The identity answers "which thing is this?" and is thirty-two bytes; the record answers "where did those thirty-two bytes come from?" and is inspection material.
/// They are separate values so neither constrains the other: the transcript can be complete because it is not stored, and the record can be honest because it is written once where the derivation happened rather than copied everywhere the identity goes.
///
/// The material is stated by its LENGTH and not carried, because material is unbounded and a record that copied it would double every rendering in memory to say something the rendered unit already holds.
/// That length is not a fold and identifies nothing; the identity is what commits to the material, at full width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Provenance {
    subject_stem: &'static str,
    subject: &'static str,
    role: Role,
    profile: Profile,
    generator: GeneratorIdentity,
    anchoring: Anchoring,
    material_length: u64,
    position: u32,
}

/// The version of the SHAPE a generator renders: a different token layout, a different set of roles, a different contract realized.
///
/// It is deliberately not the package version, which moves for reasons that cannot reach the output and is worthless as the fact a reader judges staleness by.
/// **It is not a segment of any preimage** either: a bump renames no identity, because which generator rendered a thing is a fact ABOUT the derivation and rides [`Provenance`], while what the thing IS rides the preimage its grammar declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShapeVersion(u32);

/// Which generator produced an identity, and under which rendered shape.
///
/// The name and the shape version are the two load-bearing facts a staleness comparison reads.
/// The package version is recorded and read back but compared by nothing, because a report of "a different generator" on a version bump nobody's output noticed is noise dressed as provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GeneratorIdentity {
    name: &'static str,
    shape: ShapeVersion,
    package: &'static str,
}

/// This generator, as every derivation record here names it.
pub const GENERATOR: GeneratorIdentity = GeneratorIdentity::declared(
    "macroonz",
    ShapeVersion::declared(1),
    env!("CARGO_PKG_VERSION"),
);

/// One identity a CONSUMER minted, cited by the subject the consumer names it under.
///
/// This compiler mints nothing for a consumer and checks nothing here: the bytes cross unchanged, and holding one says the compiler refers exactly to that identity and says nothing else — nothing about authority, freshness, availability, or equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerIdentity {
    /// The subject the minting side names it under.
    pub subject: &'static str,
    /// The identity's declared raw-byte storage order.
    pub bytes: [u8; 32],
}

/// One owning home and one fact it declares, by the stable names that home wrote down.
///
/// Every selection, omission, exclusion, and non-applicability in this compiler cites one.
/// A bare boolean would say a decision happened without saying whose fact decided it, which is exactly the explanation the compiler owes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerFact {
    /// The owning home, by its declared name.
    pub home: &'static str,
    /// The fact that home declares, by its declared stable name.
    pub name: &'static str,
}

/// Bytes one human projection may carry.
///
/// A projection that does not fit refuses rather than truncating, so the magnitude is the length past which a sentence is a different sentence and not a longer one.
pub const HUMAN_TEXT_LIMIT: usize = 512;

/// One bounded human-readable rendering of a typed value.
///
/// It is a projection and only a projection: derived from typed values, carried for a person to read, and never read back.
/// No decision, no identity, and no refusal anywhere in this compiler consults one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HumanProjection(Bounded<u8, HUMAN_TEXT_LIMIT>);
