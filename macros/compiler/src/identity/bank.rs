//! The authored profile bank for this compiler's identity grammars.

use super::{MACROONZ_STEM, Profile, Version};

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
/// Two roles read here, on the terms [`crate::identity::Role::profile`] states.
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
/// The generator's declared name, framed, then its shape position in four big-endian bytes, rooted at position zero; the package version is absent, for the reason [`crate::identity::ShapeVersion`] states.
pub const GENERATOR_VERSION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "generator-version", Version::declared(1));

/// The grammar a diagnostic's related identities are derived under, at both levels.
///
/// One refusal family's name and the framed material the level stands over — the issue's own canonical bytes at the issue level, and the framing of every issue in order at the body level — rooted at position zero.
/// The two levels are separated by their subjects, [`crate::identity::RelatedIssue`] and [`crate::identity::RelatedBody`], which is what keeps a body's preimage from being reachable as an issue's.
pub const DIAGNOSTIC_RELATION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "diagnostic-relation", Version::declared(1));
