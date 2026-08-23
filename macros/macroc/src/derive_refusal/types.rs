//! The refusal-family derive's public types: what was declared, what was
//! refused, what was planned, what one closed expansion binds, and the magnitude
//! a captured family's cause set is bounded by.
//!
//! Declarations only, with two deliberate exceptions.
//! `refusal_derive_facts!` declares every owner fact this home cites AND the
//! repair each one projects, and `capture_causes!` declares the capture family
//! AND writes its two contracts and its five per-cause tables — both in one
//! expansion, because the whole point of either declaration is that a row is
//! stated once and everything about it follows.

use crate::closure::{
    ClosedExpansion, ExpansionBindingRefusal, ProjectionClosureRefusal, RenderingRefusal,
};
use crate::diagnostics::{
    MachineAnchoring, MacrocDiagnostic, ObservedClassification, SiteCoordinate,
};
use crate::documentation::DocumentedItem;
use crate::explanation_protocol::ExplanationCoverage;
use crate::generated_support::{CarrierAssembly, ShellComposition};
use crate::mutation_descriptor::{MutationDeclaration, MutationDeclarationRefusal};
use crate::origin_graph::Nonclaim;
use crate::plane::{
    CapturedDeclarationSubject, CapturedTokenLimit, GeneratedTokenLimit, HumanProjection,
    HumanTextLimit, MembershipLimit, NonclaimLimit, OwnerFactRef, ProjectionIdentity,
    RenderedByteLimit, RenderedRole, SoleRenderedUnit, human_projection,
};
use crate::planning::{DeriveImplProjection, ProjectionDisposition};
use crate::refusal::ProjectionPlanning;
use crate::test_descriptor::{
    DescriptorPlanIssue, ShellDeclarationRefusal, TrialDeclarationRefusal, TrialTablePayload,
};
use crate::token::{SpanHandle, SpanTable, TextCapture};
use threadpak::declaration::SourceCoordinate;
use threadpak::refusal::{
    CauseId, CauseOrderDeclaration, CompletionPosture, DeclaredCause, DeclaredCauseOrder,
    FamilyShape, LocalCauseKey, RefusalFamily, RefusalFamilyId,
};
use threadpak::types::{Bounded, ConstLimit};

#[path = "type_guard.rs"]
mod guard;

pub use guard::{callable_entry, expected_contract};

// ---------------------------------------------------------------------------
// The magnitude.
//
// This home's own row, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on the row below
// are this home's, declared beside the capacity it governs.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many causes one captured refusal family may
    /// declare.
    ///
    /// # Bounds
    ///
    /// Sixty-four. A family's causes are the closed set a caller matches on, and
    /// a family past sixty-four has stopped being one refusal a reader can hold
    /// — the repair is a second family, not a longer cause list here. Past this
    /// the capture REFUSES rather than truncating: a family carrying some of its
    /// causes is a different family, and every contract rendered over it would
    /// be written about a declaration nobody made.
    ///
    /// # Nonclaims
    ///
    /// It bounds the DECLARED causes this home reads out of a captured surface,
    /// and it is not a bound on the token material that surface was read from —
    /// that is the token seam's four magnitudes, which the capture spends before
    /// this row is ever consulted.
    DeriveCauseLimit = 64,
}

// ---------------------------------------------------------------------------
// The authored grammar's vocabulary.
// ---------------------------------------------------------------------------

/// The authored shape word for a single-cause family.
pub const SHAPE_WORD_SINGLE_CAUSE: &str = "single_cause";

/// The authored shape word for an issue-collection family.
pub const SHAPE_WORD_ISSUE_COLLECTION: &str = "issue_collection";

/// The authored shape word for an inseparable-pair family.
pub const SHAPE_WORD_INSEPARABLE_PAIR: &str = "inseparable_pair";

/// The crate binding a consumer reaches the machine through, by default.
pub const DEFAULT_CRATE_BINDING: &str = "threadpak";

// ---------------------------------------------------------------------------
// The compiler-facing vocabulary.
// ---------------------------------------------------------------------------

/// The prefix every line this home hands a compiler opens with.
///
/// One owned spelling, cited by the one composing road
/// ([`composed`](crate::derive_refusal::diagnose::composed)) and by nothing
/// else.
/// A prefix spelled at each seam is a prefix that can be spelled two ways: a
/// reader filtering a build log on it would silently lose whichever seam drifted,
/// and nothing in the machine would notice, because no decision reads a
/// projection back.
pub const DIAGNOSTIC_PREFIX: &str = "threadpak refusal-family derive";

/// Declares every owner fact this home cites, once, with the home that owns it
/// and the repair it declares.
///
/// One row per fact.
/// A citation minted at a seam is a name nothing declares — two seams citing
/// "the same" fact under two spellings derive two different citation encodings,
/// and a reader chasing the fact finds neither. The rows below are the
/// declaration site, and the seams read them.
///
/// The repair travels in the same row as the citation because
/// [`RepairAction`](crate::diagnostics::RepairAction) states that the citation is
/// the load-bearing member and the text is a projection of it: a repair sentence
/// declared apart from the fact it projects is a sentence that can be shown
/// beside a fact that does not say it.
macro_rules! refusal_derive_facts {
    ($(
        $(#[$note:meta])*
        $variant:ident = $home:literal, $fact:literal, $repair:literal
    );+ $(;)?) => {
        /// Every owner fact one refusal-family derivation cites, by the declared
        /// names its home wrote down.
        ///
        /// # Nonclaims
        ///
        /// A row is a CITATION and never a mint.
        /// The refusal home's fact identities are not published to the compiler
        /// plane and this home derives none of them, so a row names the home and
        /// the fact and stops there — see
        /// [`OwnerFactRef`](crate::plane::OwnerFactRef).
        #[must_use = "a cited fact carries the home that declared it and the repair it declares"]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum RefusalDeriveFact {
            $( $(#[$note])* $variant ),+
        }

        impl RefusalDeriveFact {
            /// The complete roster, in the order the declaration states it.
            pub const ALL: &'static [Self] = &[$( Self::$variant ),+];

            /// The semantic home that declares this fact, by its declared stable
            /// name.
            #[must_use]
            pub const fn home(self) -> &'static str {
                match self {
                    $( Self::$variant => $home ),+
                }
            }

            /// This fact's own declared stable name inside that home.
            #[must_use]
            pub const fn stable_name(self) -> &'static str {
                match self {
                    $( Self::$variant => $fact ),+
                }
            }

            /// The citation the compiler plane carries: the home and the fact,
            /// by their declared stable names.
            #[must_use]
            pub const fn citation(self) -> OwnerFactRef {
                match self {
                    $( Self::$variant => OwnerFactRef::named($home, $fact) ),+
                }
            }

            /// The repair this fact declares, rendered for a person. A
            /// projection of the typed value: nothing reads it back.
            #[must_use]
            pub const fn described(self) -> &'static str {
                match self {
                    $( Self::$variant => $repair ),+
                }
            }

            /// The same repair as a bounded projection, proven to fit its limit
            /// family at compile time — so the repair road has no refusal to
            /// swallow and no empty fallback to fall into.
            #[must_use]
            pub fn repair(self) -> HumanProjection<HumanTextLimit> {
                match self {
                    $( Self::$variant => human_projection!(HumanTextLimit, $repair) ),+
                }
            }
        }
    };
}

refusal_derive_facts! {
    /// The refusal home's fact that a family's body is one of exactly three
    /// shapes.
    BodyShapesAreThreeAndClosed = "refusal", "family-shapes-are-three-and-closed",
        "a refusal family's body is one of exactly three declared shapes, and a declaration names \
         one of them";

    /// The refusal home's fact that the canonical cause order stands for
    /// single-cause families and for no other shape.
    CanonicalOrderStandsForSingleCauseAlone = "refusal",
        "canonical-order-stands-for-single-cause-alone",
        "a canonical cause order stands for a single-cause family and for no other shape, so the \
         order clause is declared exactly where the shape carries one and covers the body exactly";

    /// The refusal home's fact that a cause identity is the pair of its family's
    /// identity and its local key.
    CauseIdentityIsFamilyAndKey = "refusal", "cause-identity-is-family-and-key",
        "a cause identity is the pair of its family's declared identity and its own local key, \
         each spelled under the canonical grammar and each distinct inside its family";

    /// This home's own charter fact: what shape of Rust item this grammar reads
    /// a refusal family out of.
    AFamilyIsDeclaredAsABareVariantEnum = "macroc",
        "a-refusal-family-is-declared-as-a-bare-variant-enum",
        "a refusal family is declared as a named enum whose body carries bare variants, and this \
         grammar reads no other item form";

    /// This home's own charter fact: a declaration is read under the profile the
    /// derive declares, and a form outside it is refused rather than half-read.
    ADeclarationIsReadUnderTheDeclaredCompilerProfile = "macroc",
        "a-declaration-is-read-under-the-declared-compiler-profile",
        "the derive reads a declaration under its declared compiler profile, and a form that \
         profile does not read is refused rather than read in part";

    /// This home's own charter fact: a declared input that would pass a declared
    /// magnitude is refused whole.
    EveryDeclaredInputStandsUnderADeclaredMagnitude = "macroc",
        "every-declared-input-stands-under-a-declared-magnitude",
        "a declared input that would pass a declared magnitude is refused whole, because a \
         truncated capture is a different declaration";

    /// This home's own charter fact: a plan states its complete output set or it
    /// refuses.
    APlanStatesItsCompleteOutputSetOrRefuses = "macroc",
        "a-plan-states-its-complete-output-set-or-refuses",
        "a plan states its complete output set inside its declared magnitudes, once per role, or \
         it refuses";

    /// This home's own charter fact: the output firewall.
    NothingIsEmittedThatDidNotClose = "macroc", "nothing-is-emitted-that-did-not-close",
        "the membership rebuilt out of the rendered units must equal the plan's declared \
         membership, role for role and set for set, before a token exists";

    /// This home's own charter fact: every kind answers the explanation protocol
    /// about its own subject.
    EveryKindAnswersTheExplanationProtocol = "macroc",
        "every-kind-answers-the-explanation-protocol",
        "a projection answers every question its kind admits, exactly once, about its own subject \
         — an unbindable seat refuses rather than answering about a neighbouring value";

    /// This home's own charter fact: every rendered seat stands under a declared
    /// magnitude.
    EveryRenderedSeatStandsUnderADeclaredMagnitude = "macroc",
        "every-rendered-seat-stands-under-a-declared-magnitude",
        "a renderer that would emit past its declared magnitude refuses rather than materializing \
         part of a unit";

    /// This home's own charter fact: the row material a descriptor states about
    /// itself is the caller's declaration.
    ARowIsTheCallersDeclarationAndNeverTheProducers = "macroc",
        "a-row-is-the-callers-declaration-and-never-the-producers",
        "the claim, the suite, the roles, the tags, the subject, the check, the population, and \
         the callable a descriptor row states are the caller's own declarations, so a door handed \
         none declares no rows rather than inventing the material it would then prove";

    /// This home's own charter fact: what a trial declaration states, and the two
    /// sets of facts it does not.
    ATrialDeclarationStatesDescriptorMeaningAlone = "macroc",
        "a-trial-declaration-states-descriptor-meaning-alone",
        "a trial declaration states the support name, the stamped module, the authored table, and \
         one closed set of seats and rows — each seat naming its suite once, each row naming its \
         claim, roles, tags, subject, check, and population — and nothing else: the door, the \
         producer, the projection, and the schema are the producer's own act, and the revisions, \
         the callable, the budgets, the target, and the clock are the consumption target's host \
         facts, stated where that target invokes the carrier";

    /// This home's own charter fact: what a mutation declaration states, and
    /// which owner and execution facts remain outside it.
    AMutationDeclarationStatesEvaluationPolicyAlone = "macroc",
        "a-mutation-declaration-states-evaluation-policy-alone",
        "a mutation declaration states one output module, one evaluation family, the mapping from \
         each sealed generated fact to an owner claim, and the nonempty operator-family roster \
         that claim permits — the producer discovers candidate meaning, TestPak alone admits and \
         lowers executable alternatives, and the consumption target supplies every \
         callable, input, invocation, target, toolchain, clock, and trust observation";

    /// This home's own charter fact: material is delivered into a seat the
    /// carrier's published grammar actually writes.
    ACarrierSeatIsWrittenBeforeItIsFilled = "macroc",
        "a-carrier-seat-is-written-before-it-is-filled",
        "material is delivered into a seat the carrier's published grammar writes, so a crossing \
         whose seat is reserved and not yet written delivers nothing rather than riding another \
         crossing's seat into a target that does not run it";

    /// This home's own charter fact: every spelling a carrier renders in
    /// identifier position is one Rust identifier, distinct inside its
    /// namespace.
    ACarrierSpellingIsOneRustIdentifier = "macroc",
        "a-carrier-spelling-is-one-rust-identifier",
        "every spelling a carrier renders in identifier position is one Rust identifier and is \
         distinct inside the namespace it lands in, so an expansion never writes tokens a \
         consumer's compiler reads as something else";

    /// This home's own charter fact: one carrier carries one declaration's
    /// proved cargo, behind one pin.
    ///
    /// The VEHICLE is inside this fact and not beside it. A carrier plan for one
    /// declaration closing around another declaration's assembly is the same
    /// sentence read at the other end — one exported name delivering a second
    /// declaration's material — so it cites this row rather than a second one
    /// that would say half of what this already says.
    OneCarrierDeliversOneDeclarationsProvedCargo = "macroc",
        "one-carrier-delivers-one-declarations-proved-cargo",
        "every axis of one exported carrier carries cargo its own terminal proved, from the \
         partition that axis delivers from, under one root and one published expectation, and the \
         carrier plan the shell is rendered from stands under that same root — so no unproved \
         tokens, no second declaration's material, and no unit already compiled by the normal \
         build reach a consumption target";

    /// This home's own charter fact: the codec kind's blocking conjunction under
    /// the declared compiler profile.
    AByteRoleIsNotReadOutOfACapture = "macroc",
        "a-byte-role-is-not-read-out-of-a-captured-declaration",
        "a codec projection reads or writes one named byte role, which is the role an artifact's \
         canonical bytes are read under and a fact no declaration's tokens stand for — so the \
         repair is the machine's linked declaration path, where the byte role is minted";

    /// This home's own charter fact: the benchmark-descriptor kind's blocking
    /// conjunction under the declared compiler profile.
    AWorkCurrencyIsNotReadOutOfACapture = "macroc",
        "a-work-currency-is-not-read-out-of-a-captured-declaration",
        "a benchmark descriptor states its envelope in one named work currency, which is the \
         vocabulary a measurement is stated in and a fact no declaration's tokens stand for — so \
         the repair is the machine's linked declaration path, where the currency is minted";

    /// This home's own charter fact: the documentation kind's blocking
    /// conjunction — two seats, both independently true.
    AnAudienceAndAFacetElectionAreNotReadOutOfACapture = "macroc",
        "an-audience-and-a-facet-election-are-not-read-out-of-a-captured-declaration",
        "a documentation plan names the audience its prose is pitched at and the semantic facets \
         it covers, and this profile reads tokens rather than what prose means — so neither the \
         audience nor the facet election is answerable here, and both stand until a profile that \
         reads meaning offers the kind";

    /// This home's own charter fact: the host-wrapper kind's blocking
    /// conjunction — the contract seat and the target binding, independently.
    ABoundHostContractIsNotHeldByAnExpansion = "macroc",
        "a-bound-host-contract-is-not-held-by-an-expansion",
        "a host wrapper binds one named host contract and its kind requires the context to be \
         bound to one, and an expansion's context is target-free — so the contract seat and the \
         binding requirement are independently unmet and closing one would leave the other";

    /// This home's own charter fact: the remote-surface kind's blocking
    /// conjunction — three seats, all independently true.
    APortWireContractAndTargetAreNotHeldByAnExpansion = "macroc",
        "a-port-a-wire-contract-and-a-bound-target-are-not-held-by-an-expansion",
        "a remote surface projects one declared port over one wire contract and its kind requires \
         a bound host contract, and an expansion holds no port, no wire contract, and no target \
         binding — so all three stand and none of them is the primary one";

    /// This home's own charter fact: the pattern-stamp kind's blocking
    /// conjunction — the authored application, and where its member would land.
    APatternApplicationAndPublicationAreNotHeldByAnExpansion = "macroc",
        "an-authored-pattern-application-and-a-publication-posture-are-not-held-by-an-expansion",
        "a pattern stamp names an authored pattern, this instantiation of it, and its typed \
         arguments, and its member lands as a publication artifact written under a receipt — so \
         the application a caller supplies and the posture the delivery stands under are both \
         outside what an expansion holds";

    /// This home's own charter fact: the terminal binds what it hands out.
    NothingIsHandedOutThatDidNotBind = "macroc", "nothing-is-handed-out-that-did-not-bind",
        "a closed expansion binds the plan its proof was taken against and the explanation \
         answered over the two, so a proof or an explanation belonging to another expansion is \
         refused rather than bound under one identity";
}

/// How the consumer names the machine on its own dependency list.
///
/// A consumer is allowed to rename its dependencies.
/// `tp = { package = "threadpak" }` is an ordinary Cargo edge, and in that crate
/// the machine is not called `threadpak` at all — so a rendering that spelled
/// `::threadpak` would name a crate the consumer does not have, and the
/// expansion would fail to compile for a reason that has nothing to do with the
/// declaration.
///
/// So the binding is part of what is CAPTURED.
/// It travels into the plan, into the explanation, into the rendering, and into
/// the invalidation set, because a consumer that renames its dependency has
/// changed what the rendering must say.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateBinding {
    spelling: String,
}

/// Which declaration one captured documentation row was written on.
///
/// Two arms, because the grammar admits documentation in two places and a
/// reader joining a row back to what it describes needs to know which: a row on
/// the FAMILY describes the declaration as a whole, and a row on a VARIANT
/// describes one cause of it.
/// A roster carrying rows with no such seat would be a pile of sentences that
/// have to be re-associated by position, and position is exactly what an
/// author's edit moves.
#[must_use = "a documentation row names the declaration it was written on"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocumentedDeclaration {
    /// The family declaration itself.
    Family,
    /// One declared variant, under the spelling the enum body writes it with.
    Variant(String),
}

/// One documentation row as the capture read it: what it was written on, the
/// text it carries, and the token it sits at.
///
/// # The attribute form
///
/// A documentation comment is an ATTRIBUTE by the time a declaration reaches
/// this grammar — `#[doc = "…"]`, one attribute per written line — so what is
/// captured here is exactly the form the language already produces, read
/// through the same token walk every other seat is read through. Nothing here
/// recognizes a comment: there are no comments left to recognize.
///
/// # Content
///
/// **The rows are what the DOCUMENTATION commitment stands over, and they are
/// exactly what the semantic one sets aside.** A captured declaration is named
/// twice: [`RefusalDeriveSurface::identity`] stands over the declaration's
/// tokens with these attributes dropped, and
/// [`RefusalDeriveSurface::documentation_identity`] stands over that name and
/// these rows in order. So a declaration whose prose changed keeps the name an
/// implementation projection is about and takes a new name a documentation
/// projection is about — which is the difference between "the same contract,
/// documented differently" and "a different contract".
///
/// Both are readings of ONE captured surface. The rows are cut from the same
/// token material the semantic commitment was derived over, so nothing here is a
/// second account of what the content is or of what it stands on.
///
/// # Nonclaims
///
/// It states what was written and where, and nothing about what the text MEANS:
/// no facet, no audience, no heading, and no section. Those are the
/// documentation projection's declarations, and a capture that decided any of
/// them would be deciding meaning it was handed as text.
#[must_use = "a captured documentation row is declared data the surface carries"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedDocumentation {
    declared_on: DocumentedDeclaration,
    text: String,
    token: SpanHandle,
}

/// One cause as the capture read it: the Rust variant that spells it, and the
/// LOCAL key the author declared for it.
///
/// The local key is not the cause identity.
/// The identity is band 00's pair — the family's declared identity in one seat
/// and this key in the other — and the derive mints it from the two rather than
/// asking the author to write a whole identity out, which is what keeps a
/// family's causes from drifting apart one hand-typed prefix at a time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedCause {
    spelling: String,
    local_key: String,
}

/// One refusal-family declaration, captured from a typed token tree.
///
/// The causes are non-empty exactly when the shape is
/// [`FamilyShape::SingleCause`]; the other two shapes declare no canonical
/// order, so there is nothing here to carry for them.
///
/// # The documentation rows
///
/// One roster, over the whole declaration, with every row naming the
/// declaration it was written on ([`DocumentedDeclaration`]). One roster rather
/// than one per variant, because the rows a shape carries no cause seat for are
/// rows all the same: a family whose shape declares no canonical cause order
/// still documents its variants, and a per-cause seat would drop exactly those.
///
/// The magnitude is [`CapturedTokenLimit`], which is the magnitude that already
/// governs the material these rows are cut from — a documentation row is one
/// captured attribute at one nesting level, and the trees at one nesting level
/// are what that bound admits. A second magnitude here would be a second
/// authority over one capacity.
///
/// # Two commitments, one surface, and which account carries which
///
/// **One captured surface, two authored facts, and no second account of content
/// dependencies anywhere.** The surface carries the SEMANTIC commitment — the
/// declaration with its documentation attributes set aside — and the
/// DOCUMENTATION commitment, which stands over that semantic name and the
/// ordered rows.
///
/// Each projection still receives exactly ONE
/// [`OwnerContentAccount`](crate::planning::OwnerContentAccount), and which of
/// the two commitments that account carries is decided by what the projection is
/// ABOUT:
///
/// - an implementation, test, codec, or any other projection over what the
///   declaration IS takes an account over the SEMANTIC commitment, with no
///   dependency declared — captured token material stands on nothing that has
///   been linked;
/// - a DOCUMENTATION projection takes an account over the DOCUMENTATION
///   commitment, and DECLARES the semantic commitment as its dependency, because
///   what the prose says stands on what the declaration is.
///
/// That second account is not a second reading of dependencies: it is one
/// account over one commitment, naming the one commitment it stands on, exactly
/// as every other account does. Nothing in these services holds a second list of
/// what content depends on.
///
/// # The trial declaration
///
/// A third reading, and the same shape the documentation reading has: the
/// `threadpak_trials` attribute is declaration material like any other token and
/// is exactly the material whose meaning is a statement about a consumer's TEST
/// target rather than about the production contract, so it is dropped from the
/// semantic walk and named under its own commitment. A declaration whose trial
/// rows moved keeps the name its implementation projection is about and takes a
/// new name its CARRIER is about.
///
/// It is a POSTURE rather than a roster, because the two are different facts: a
/// declaration that states no trials has no rows for a carrier to declare, and a
/// declaration that states an empty set of them is a shape the grammar refuses at
/// the payload's own door.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalDeriveSurface {
    family_name: String,
    family_id: String,
    binding: CrateBinding,
    shape: FamilyShape,
    causes: Bounded<CapturedCause, DeriveCauseLimit>,
    documentation: Bounded<CapturedDocumentation, CapturedTokenLimit>,
    trials: TrialDeclarationPosture,
    mutations: MutationDeclarationPosture,
    commitments: CapturedCommitments,
}

/// What one refusal-family declaration's HEAD and its attribute state: the Rust
/// name the enum carries, the stable family identity, the binding the consumer
/// reaches the machine by, and the declared body shape.
///
/// Four seats that travel as one value because they were read from one head and
/// one attribute, and because the road that takes them takes the declaration's
/// rosters, its trial posture, and its commitments beside them — which is past
/// the arity the lint wall admits, and past the arity a reader tells apart by
/// counting commas.
///
/// Every field is public and required, so a construction that leaves one out
/// stops compiling exactly where a missing argument used to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapturedFamilyFacts {
    /// The declared family's Rust name.
    pub family_name: String,
    /// The declared family's stable identity, as `<domain>.<family>`.
    pub family_id: String,
    /// How the consumer names the machine.
    pub binding: CrateBinding,
    /// The declared body shape.
    pub shape: FamilyShape,
}

/// One declaration's trial rows, and the commitment they are named under.
///
/// The two travel together because they are one reading: the commitment stands
/// over the exact token material the payload was read from, so a value holding
/// one without the other would be a name for rows nobody can see or rows with no
/// name of their own.
#[must_use = "declared trials carry the payload the carrier delivers and the commitment it is named under"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredTrials {
    commitment: ProjectionIdentity<CapturedDeclarationSubject>,
    payload: TrialTablePayload,
}

/// One declaration's mutation policy and mapping, under its independent commitment.
#[must_use = "declared mutations carry the helper reading and the commitment it is named under"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredMutations {
    commitment: ProjectionIdentity<CapturedDeclarationSubject>,
    declaration: MutationDeclaration,
}

/// Whether one captured declaration states trial rows.
///
/// Two postures, and they are different facts rather than one with a missing
/// half. A declaration that wrote no trial attribute has no rows for a carrier to
/// declare and no material for a third commitment to stand over; one that wrote
/// an EMPTY set of them is refused at the payload's own door, so the absent
/// posture is never a stand-in for a roster somebody left empty.
///
/// # Bounds
///
/// Neither posture is a refusal. A refusal-family declaration that states no
/// trials is exactly the declaration this derive has always compiled, and its
/// carrier renders an empty trials seat beside whatever it defers.
#[must_use = "a trial posture either carries a declaration's rows or states that it declared none"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrialDeclarationPosture {
    /// The declaration wrote no trial attribute.
    NotDeclared,
    /// The rows the declaration states, under the commitment they are named
    /// by.
    ///
    /// Boxed because a posture travels by value on every captured surface, and
    /// the largest answer must not set the size of the smaller one: a
    /// declaration that states no trials would otherwise pay a whole trial
    /// payload for a seat it does not fill.
    Declared(Box<DeclaredTrials>),
}

/// Whether one captured refusal declaration states generated mutation policy.
#[must_use = "a mutation posture either carries the helper reading or states that none was declared"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MutationDeclarationPosture {
    /// The declaration wrote no mutation helper.
    NotDeclared,
    /// The mutation helper's complete reading under its own commitment.
    Declared(Box<DeclaredMutations>),
}

/// How one declaration was not captured into a surface.
///
/// Two homes answer at this seam and each answer is carried whole, on exactly the
/// terms [`ShellComposition`](crate::generated_support::ShellComposition) sets:
/// whether the tokens say a lawful refusal-family declaration is the derive
/// grammar's question, and whether the trial attribute inside it says a lawful
/// carrier declaration belongs to the home that owns that vocabulary.
///
/// # Authority
///
/// **It is not a third refusal family and it declares no shape of its own.** Each
/// arm holds the body its own home established, unwrapped and unsummarized, so
/// nothing here is a second answer to either question — and the projection that
/// turns each into a diagnostic is that home's own.
#[must_use = "a capture refusal names which grammar refused and carries that grammar's body"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceCaptureRefusal {
    /// The refusal-family grammar refused. This home's own body, whole.
    Declaration(RefusalDeriveRefusal),
    /// The trial grammar refused, or the carrier's own vocabulary refused a value
    /// it read. The test-descriptor home's own body, whole.
    Trials(TrialDeclarationRefusal),
    /// The mutation helper grammar or carrier vocabulary refused.
    Mutations(MutationDeclarationRefusal),
}

impl From<RefusalDeriveRefusal> for SurfaceCaptureRefusal {
    fn from(refusal: RefusalDeriveRefusal) -> Self {
        Self::Declaration(refusal)
    }
}

impl From<TrialDeclarationRefusal> for SurfaceCaptureRefusal {
    fn from(refusal: TrialDeclarationRefusal) -> Self {
        Self::Trials(refusal)
    }
}

impl From<MutationDeclarationRefusal> for SurfaceCaptureRefusal {
    fn from(refusal: MutationDeclarationRefusal) -> Self {
        Self::Mutations(refusal)
    }
}

/// The two commitments one captured declaration derives, together.
///
/// They are ONE account and not two arguments that happen to arrive side by
/// side: the documentation commitment stands over the semantic one and names it
/// as its dependency, so a road holding either alone holds half of a pair whose
/// halves are ordered. Carrying them as a value is what makes the order a fact
/// about the type rather than a convention at every call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapturedCommitments {
    semantic: ProjectionIdentity<CapturedDeclarationSubject>,
    documentation: ProjectionIdentity<CapturedDeclarationSubject>,
}

// ---------------------------------------------------------------------------
// The capture refusal family.
// ---------------------------------------------------------------------------

/// Declares the capture family's causes once, and derives from that single
/// declaration the typed roster, the selection order, the stable identities, the
/// observed classification, the OWNER FACT each cause answers to, the text, and
/// the bounded human projection.
///
/// One literal per cause.
/// A second copy of any of these would be a second thing to keep true, and the
/// human projection in particular is proven to FIT its limit family at compile
/// time — so the explanation road has no refusal to swallow and no empty
/// fallback to fall into.
///
/// # The citation column
///
/// Every cause names the fact it is a violation OF, in its own row.
/// One citation shared by the whole family would be a blanket: a caller told
/// that a local key spelled with a capital letter violates "a family's body is
/// one of three shapes" is sent to read a rule that has nothing to do with what
/// was refused, and the repair beside it would be equally unrelated.
macro_rules! capture_causes {
    ($(
        $(#[$note:meta])*
        $variant:ident = $key:literal, $observed:expr, $fact:expr, $text:literal
    );+ $(;)?) => {
        /// The single-cause family for capturing a refusal-family declaration.
        ///
        /// Single cause because the checks are dependent: there is no shape word
        /// to admit until an attribute was found, no coverage to check until
        /// both the order clause and the body were read, and no distinctness to
        /// check until the keys were parsed.
        /// Claiming a result from a check that never ran is unrepresentable
        /// here, which is exactly what the shape is for.
        ///
        /// The canonical order below is the SELECTOR's order, not the execution
        /// schedule.
        #[must_use = "a capture refusal carries the established cause the declaration \
                      was not read"]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum RefusalDeriveCapture {
            $( $(#[$note])* $variant ),+
        }

        impl RefusalFamily for RefusalDeriveCapture {
            const SHAPE: FamilyShape = FamilyShape::SingleCause;
            const SELECTION_ORDER: &'static [&'static str] = &[
                $( stringify!($variant) ),+
            ];
        }

        /// Hand-declared, and deliberately so: the services never derive their
        /// own contracts.
        /// A generator that produced its own declared facts would be its own
        /// oracle.
        impl CauseOrderDeclaration for RefusalDeriveCapture {
            const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
                $(
                    DeclaredCause::declared(
                        CauseId::declared(
                            RefusalDeriveCapture::FAMILY,
                            LocalCauseKey::declared($key),
                        ),
                        stringify!($variant),
                    )
                ),+
            ]);
        }

        impl RefusalDeriveCapture {
            /// This family's own stable identity.
            pub const FAMILY: RefusalFamilyId =
                RefusalFamilyId::declared("macroc.refusal-derive-capture");

            /// One cause's stable identity: this family, and the cause's local
            /// key inside it.
            #[must_use]
            pub const fn id(self) -> CauseId {
                CauseId::declared(
                    Self::FAMILY,
                    match self {
                        $( Self::$variant => LocalCauseKey::declared($key) ),+
                    },
                )
            }

            /// How what was found differs from the contract that was expected.
            #[must_use]
            pub const fn observed(self) -> ObservedClassification {
                match self {
                    $( Self::$variant => $observed ),+
                }
            }

            /// The owner fact this cause is a violation of — the fact whose
            /// repair a caller is pointed at.
            ///
            /// Per cause, never per family: see the citation column above.
            pub const fn declared_by(self) -> RefusalDeriveFact {
                match self {
                    $( Self::$variant => $fact ),+
                }
            }

            /// The cause rendered for a person. A projection of the typed value:
            /// nothing reads it back, and no decision consults it.
            #[must_use]
            pub const fn described(self) -> &'static str {
                match self {
                    $( Self::$variant => $text ),+
                }
            }

            /// The same rendering as a bounded projection, proven to fit its
            /// limit family at compile time.
            #[must_use]
            pub fn description(self) -> HumanProjection<HumanTextLimit> {
                match self {
                    $( Self::$variant => human_projection!(HumanTextLimit, $text) ),+
                }
            }
        }
    };
}

capture_causes! {
    /// The declared input carries no item this grammar recognizes at all.
    NotAnEnum = "not-an-enum", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "the declared input carries no item declaration this grammar recognizes";

    /// A real Rust item arrived that is not an enum — a struct, a union, a
    /// trait, or a function. It is a real declaration and it is the wrong FORM,
    /// which is a different answer than "this is not an enum at all".
    UnsupportedDeclarationForm = "unsupported-declaration-form",
        ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "a refusal family is declared as an enum, and this declaration is a different item form";

    /// The `enum` keyword is not followed by a name.
    NotNamed = "not-named", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "the `enum` keyword is not followed by a name";

    /// A real enum arrived carrying a form this compiler profile does not read:
    /// generic parameters, or a `where` clause.
    UnavailableUnderCompilerProfile = "unavailable-under-compiler-profile",
        ObservedClassification::ProfileDisagreement,
        RefusalDeriveFact::ADeclarationIsReadUnderTheDeclaredCompilerProfile,
        "this declaration carries generics or a `where` clause, which the derive's declared \
         compiler profile does not read";

    /// The enum declares no body at all.
    NotBodied = "not-bodied", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "the enum declares no body";

    /// The enum body declares no variant.
    NotInhabited = "not-inhabited", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "the enum body declares no variant";

    /// A real variant arrived carrying a payload. The grammar admits bare
    /// variants only, so what is captured renders back without a construction
    /// question ever arising.
    UnsupportedVariantPayload = "unsupported-variant-payload",
        ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::AFamilyIsDeclaredAsABareVariantEnum,
        "a variant carries a payload, and this grammar admits bare variants only";

    /// The declaration carries the `refusal` helper more than once, or one of
    /// its closed clauses is declared more than once.
    NotDeclaredOnce = "not-declared-once", ObservedClassification::IdentityDisagreement,
        RefusalDeriveFact::ADeclarationIsReadUnderTheDeclaredCompilerProfile,
        "the `refusal` helper and each clause inside it are declared at most once";

    /// One comma-delimited group inside `refusal` is not a complete assignment
    /// or the complete `order(...)` form.
    NotAClause = "not-a-clause", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::ADeclarationIsReadUnderTheDeclaredCompilerProfile,
        "one `refusal` group is not a complete `<key> = <value>` assignment or `order(...)` clause";

    /// One clause inside `refusal` names no seat this grammar declares.
    NotADeclarableClause = "not-a-declarable-clause",
        ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::ADeclarationIsReadUnderTheDeclaredCompilerProfile,
        "one `refusal` clause is outside the closed `crate`, `family`, `shape`, and `order` roster";

    /// No `#[refusal(family = ...)]` was declared.
    NotFamilyDeclared = "not-family-declared", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::CauseIdentityIsFamilyAndKey,
        "no `#[refusal(family = \"<domain>.<family>\")]` was declared";

    /// The declared family identity does not follow the canonical grammar.
    NotFamilyGrammatical = "not-family-grammatical",
        ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::CauseIdentityIsFamilyAndKey,
        "the declared family identity is not two lowercase kebab-case segments joined by a dot";

    /// No `#[refusal(shape = ...)]` was declared.
    NotShapeDeclared = "not-shape-declared", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::BodyShapesAreThreeAndClosed,
        "no `#[refusal(shape = ...)]` was declared";

    /// The declared shape word is none of the three the machine's roster admits.
    NotAnAdmittedShape = "not-an-admitted-shape", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::BodyShapesAreThreeAndClosed,
        "the declared shape word is none of `single_cause`, `issue_collection`, \
         `inseparable_pair`";

    /// The shape is `single_cause` and no `order(...)` clause was declared.
    NotOrderDeclared = "not-order-declared", ObservedClassification::SeatAbsent,
        RefusalDeriveFact::CanonicalOrderStandsForSingleCauseAlone,
        "a `single_cause` family declares no `order(...)` clause";

    /// The shape declares no canonical cause order and an `order(...)` clause
    /// was declared anyway.
    NotOrderAdmitted = "not-order-admitted", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::CanonicalOrderStandsForSingleCauseAlone,
        "this shape declares no canonical cause order, and an `order(...)` clause was declared \
         anyway";

    /// The order clause and the enum body do not name the same causes.
    NotCovered = "not-covered", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::CanonicalOrderStandsForSingleCauseAlone,
        "the `order(...)` clause and the enum body name different causes";

    /// Two declared causes carry the same local key.
    NotDistinct = "not-distinct", ObservedClassification::IdentityDisagreement,
        RefusalDeriveFact::CauseIdentityIsFamilyAndKey,
        "two declared causes carry the same local key";

    /// A declared local key does not follow the canonical grammar.
    NotKeyed = "not-keyed", ObservedClassification::ContractDisagreement,
        RefusalDeriveFact::CauseIdentityIsFamilyAndKey,
        "a declared local key is not one lowercase kebab-case segment";

    /// The declared input exceeds a declared magnitude.
    Unbounded = "unbounded", ObservedClassification::BoundExceeded,
        RefusalDeriveFact::EveryDeclaredInputStandsUnderADeclaredMagnitude,
        "the declared input exceeds a declared magnitude";
}

/// Where one capture refusal was established.
///
/// Two arms, and they are different observations rather than one with a missing
/// half.
/// A declaration that was CAPTURED has a span table and a handle into it, and
/// the refusal names the offending token so a producer can put a compiler error
/// on exactly that token.
/// A text read that refused BEFORE any capture has neither: no table was built,
/// no handle was issued, and there is nothing for a handle to index. What it
/// does have is the byte it was born at — see
/// [`TextReadRefusal::coordinate`](crate::token::TextReadRefusal::coordinate) —
/// so the refusal carries that instead.
///
/// # Nonclaims
///
/// The pre-capture arm mints no handle.
/// A [`SpanHandle`] means "the token at this index of the table the producer
/// built while capturing"; where no table was built, every index means nothing,
/// and handle zero in particular reads exactly like an honest answer pointing at
/// the first token. That is the substitution this sum removes.
#[must_use = "a refusal site names the token it sits at, or the byte it was born at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefusalSite {
    /// One token of a captured declaration, as a handle into the producer's own
    /// span table.
    AtToken(SpanHandle),
    /// One byte of the text a read refused on, before any capture existed to
    /// issue a handle.
    BeforeCapture(SourceCoordinate),
}

/// One capture refusal, published from this file and DECLARED in
/// `type_guard.rs`'s `seat` module, beside the only roads that reach its two
/// seats.
///
/// Rust's privacy is MODULE-scoped, so a seat declared here would be private to
/// everything else this file declares as well.
pub use guard::RefusalDeriveRefusal;

// ---------------------------------------------------------------------------
// The declared output set.
// ---------------------------------------------------------------------------

/// The complete declared output set of one derivation.
///
/// A closed sum rather than a bounded collection, because the set is decided by
/// the shape and there are exactly two answers.
/// Neither answer is empty: a derivation that would generate nothing is a
/// disposition, not a derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivedMembership {
    /// The family implementation alone — the shape declares no cause order.
    FamilyOnly,
    /// The family implementation and the cause-order implementation.
    FamilyAndCauseOrder,
    /// The family implementation and one generated mutation evaluation.
    FamilyAndMutationEvaluation,
    /// The two production implementations and one generated mutation evaluation.
    FamilyCauseOrderAndMutationEvaluation,
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

/// The membership-only view of one derivation: what was captured, and the
/// complete output set the shape fixes.
///
/// A draft states what the shape fixed and nothing else, and it renders nothing.
/// The road to emitted tokens runs through
/// [`compile_refusal`](crate::derive_refusal::compile_refusal), which builds the
/// plan, the origin graph, the trace, the rendering, the closure, and the
/// explanation, in that order, and refuses before any of them is skipped.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalDerivationDraft {
    surface: RefusalDeriveSurface,
    membership: DerivedMembership,
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
    /// The refusal home's fact that a cause identity is the pair of its
    /// family's identity and its local key.
    pub cause_key_grammar: OwnerFactRef,
}

/// What one live compilation needs supplied to it, and nothing more.
///
/// Every seat is something the CALLER genuinely has.
/// There is no seat here for an identity the machine has not published, because
/// the honest answer to "which closed graph?" inside an expansion is that there
/// is none — and the plan says so in its own anchoring rather than being handed
/// a fiction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalCompileContext {
    /// How the producer resolves span handles.
    pub spans: SpanTable,
    /// Whether the machine's own identities stand behind a diagnostic raised
    /// here.
    pub machine: MachineAnchoring,
    /// The owner facts the derivation cites.
    pub owner_facts: RefusalOwnerFacts,
    /// What this compilation explicitly does not claim.
    pub nonclaims: Bounded<Nonclaim, NonclaimLimit>,
}

// ---------------------------------------------------------------------------
// The refusal family's view over the closed expansion.
// ---------------------------------------------------------------------------

/// This family's view over the closed expansion one live compilation ended at,
/// together with the two facts the terminal does not carry.
///
/// # A view, and not a second account
///
/// The plan, the proof, the explanation, the identity, and every emission are
/// the CLOSED EXPANSION's — [`ClosedExpansion`] is the terminal every projection
/// kind's door ends at, and this value holds one and reads it. Every road below
/// that answers about them delegates, so there is nothing here that could answer
/// differently from the terminal it stands over, and no second identity for one
/// expansion.
///
/// What it adds is what the refusal-family road knows and the generic terminal
/// does not: the CAPTURED SURFACE the declaration was read into, and the
/// disposition of the typed cause-order projection.
/// Both are facts about this family's own declaration, so a terminal seat for
/// either would be a seat every other kind carries empty.
///
/// # The name
///
/// It is the REFUSAL FAMILY's expansion, and the terminal it stands over is the
/// closed expansion. The two used to share the second name, which made "the
/// closed expansion" mean one thing in this home and another everywhere else —
/// and the family view is the narrower value, so it is the one that says whose
/// view it is.
///
/// # The one road to emitted tokens
///
/// A caller cannot hold this without the plan, the rendering, the proved
/// closure, and the complete explanation all having been produced, having
/// agreed, and having been bound: [`RefusalFamilyExpansion::bound`] is
/// crate-internal and builds through [`ClosedExpansion::bound`], which refuses a
/// closure proved against another plan and an explanation answered over another
/// plan or another closure. There is no constructor that skips a step and no
/// other public value in this home that carries a token tree.
///
/// # Inspection and emission
///
/// [`RefusalFamilyExpansion::plan`] and [`RefusalFamilyExpansion::closure`] are
/// the SAME values [`RefusalFamilyExpansion::emitted`] is read off.
/// There is no parallel plan built for inspection and no synthetic sibling built
/// for emission, so "what does it say it did" and "what did it do" cannot drift.
///
/// This value holds no tokens of its own.
/// The partitioned emission belongs to the CLOSURE, which built it as part of
/// proving and committed to each emission's digest inside its own identity; the
/// terminal borrows it and this view reads the terminal.
#[must_use = "a refusal-family expansion is this family's whole view over the closed expansion it ended at"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusalFamilyExpansion {
    surface: RefusalDeriveSurface,
    expansion: ClosedExpansion<DeriveImplProjection>,
    cause_order: ProjectionDisposition,
}

// ---------------------------------------------------------------------------
// The compiler-facing line, and the seats an explanation binds.
//
// These stand here because a public type belongs to its home's declaration
// registry, not to the pure-function file that is their only reader today.
// `diagnose.rs` composes lines and `explain.rs` writes explanations; neither
// owns a vocabulary a caller outside this home can name.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// Which class of refusal one composed line is about.
    ///
    /// The class is the second clause of every line this home composes, and it
    /// is READ off this roster rather than written at the seam that refused.
    /// A class phrase spelled at a seam is a phrase only that seam knows about:
    /// two seams reporting one class then read as two classes, and a reader
    /// grouping a build log by what went wrong groups them apart.
    pub enum RefusalClass {
        /// The declared input was not read into a captured surface.
        DeclarationNotRead = "declaration-not-read", "the declaration was not read";
        /// Planning refused before a token of Rust existed.
        PlanNotStated = "plan-not-stated", "planning refused";
        /// The rendering does not close over the plan it claims to materialize.
        RenderingNotClosed = "rendering-not-closed",
            "the rendering does not close over the plan it claims to materialize";
        /// The written explanation does not cover its kind's questions.
        ExplanationNotCovered = "explanation-not-covered",
            "the explanation does not cover its kind's questions";
        /// The explanation had no subject to write its seats about.
        ExplanationNotBound = "explanation-not-bound",
            "the explanation cannot bind its subject";
        /// A rendering would have passed a declared magnitude.
        MagnitudeNotHeld = "magnitude-not-held", "a rendering would pass a declared magnitude";
        /// The three values the terminal binds do not belong to one expansion.
        ExpansionNotBound = "expansion-not-bound",
            "the three values do not belong to one expansion";
        /// A set of closed outputs does not compose into one exported carrier.
        CarrierNotAssembled = "carrier-not-assembled",
            "the closed outputs do not compose into one carrier";
        /// The carrier's own vocabulary was not declared.
        CarrierNotDeclared = "carrier-not-declared",
            "the carrier's own vocabulary was not declared";
    }
}

/// What a composed line is a summary OF.
///
/// Two shapes, and they are different facts rather than one with the numbers
/// zeroed out.
/// A single-cause refusal establishes one cause and enumerates nothing: there is
/// no remainder to count and no examination bound anything could have stopped
/// at, so a line reporting "and 0 further issues, examination complete" would be
/// answering a question that was never asked of it.
/// A collection-shaped body has both, and the line carries both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineBody {
    /// One established cause, with nothing enumerated beside it.
    SingleCause,
    /// A refusal body: how many further issues it established beyond the first,
    /// and whether it examined everything it could.
    Body {
        /// Established issues past the one the line states in full.
        further: usize,
        /// Whether the body examined every site, and what it did with what did
        /// not fit.
        posture: CompletionPosture,
    },
}

/// The typed parts one compiler line is composed from.
///
/// They travel as one value because they are one line: a class handed to
/// [`composed`](super::composed) beside another refusal's first established
/// issue would compose a sentence that is well formed, complete-looking, and
/// about nothing in particular.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalLine<'issue> {
    /// Which class of refusal the line is about.
    pub class: RefusalClass,
    /// The first established issue, stated in full.
    pub first: &'issue str,
    /// What the line is a summary of.
    pub body: LineBody,
}

/// Whether a composed line says where the refusal sits.
///
/// Not an option: a whole-declaration refusal is a STATED posture, not a site
/// somebody forgot to supply.
/// A refusal about the declaration as a whole has nowhere narrower to point, and
/// its typed [`DiagnosticSite`](crate::diagnostics::DiagnosticSite) already
/// carries that; adding a position to its line would send a reader to an
/// arbitrary spot inside a declaration the refusal is not about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineSite {
    /// The refusal is about the declaration as a whole, and the line adds
    /// nothing.
    WholeDeclaration,
    /// The refusal sits at one place the producer can name, and the line says
    /// where — or says that the producer's table does not reach it.
    At(SiteCoordinate),
}

threadpak::closed_register! {
    /// One declared magnitude a rendering can pass, and the thing it governs.
    ///
    /// The prose belongs to the MAGNITUDE rather than to whichever refusal named
    /// it: two refusal families reach the same magnitudes, and a phrase written
    /// at each of them is a phrase that can disagree with itself about what one
    /// number bounds.
    pub enum RenderedMagnitude {
        /// The rendered-byte magnitude one materialized unit stands under.
        RenderedBytes = "rendered-bytes", "the bytes one rendered unit may carry";
        /// The membership magnitude one rendering stands under.
        RenderedUnits = "rendered-units", "the units one rendering may carry";
        /// The generated-token magnitude one tree level stands under.
        GeneratedTokens = "generated-tokens",
            "the tokens one generated tree may carry at one nesting level";
    }
}

impl RenderedMagnitude {
    /// The declared magnitude itself, read off the plane's limits roster.
    ///
    /// Read rather than restated: a number written here would be a second
    /// authority on a bound the plane already declares, and a diagnostic naming
    /// a bound the code does not enforce is evidence about nothing.
    #[must_use]
    pub const fn declared(self) -> usize {
        match self {
            Self::RenderedBytes => RenderedByteLimit::MAX,
            Self::RenderedUnits => MembershipLimit::MAX,
            Self::GeneratedTokens => GeneratedTokenLimit::MAX,
        }
    }
}

threadpak::closed_register! {
    /// The seat one explanation could not bind its subject to.
    ///
    /// Named seats rather than one "something was missing": a caller repairing
    /// a derivation needs to know whether the PLAN failed to declare the member,
    /// the CLOSURE failed to prove its bytes, or the plan cited no owner fact at
    /// all, and those are three different repairs.
    pub enum ExplanationSeat {
        /// The planned member standing under the family implementation's role.
        PlannedFamilyMember = "planned-family-member",
            "the planned member under the family role";
        /// The digest the closure proved over that member's rendered bytes.
        ProvedFamilyDigest = "proved-family-digest",
            "the digest the closure proved over the family bytes";
        /// The first owner fact the plan declares as an assumption.
        DeclaredAssumption = "declared-assumption", "the first owner fact the plan declares";
        /// The planned member standing under the carrier's one rendered role.
        ///
        /// Its own seat rather than the family implementation's, because the two
        /// are members of two different plans: a caller told the family member is
        /// absent while the carrier's is the one missing would repair the wrong
        /// projection.
        PlannedCarrierMember = "planned-carrier-member",
            "the planned member under the carrier's one role";
    }
}

/// How writing one explanation refuses.
///
/// Two postures, and they are different observations.
/// A view that could not be BOUND never reached the coverage check — there was
/// no subject to write nine seats about.
/// A view that was written and does not cover its kind's questions reached it
/// and failed it.
#[must_use = "a refusal carries the unbound seat or the coverage the view failed"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExplanationBindingRefusal {
    /// A required seat's subject is absent. The explanation refuses rather than
    /// answering about a neighbouring value.
    RequiredOutputAbsent {
        /// Which seat had no subject.
        seat: ExplanationSeat,
    },
    /// The written view does not cover the kind's applicable questions.
    Coverage(ExplanationCoverage),
}

// ---------------------------------------------------------------------------
// What the road's own steps hand back.
//
// A step's outcome is vocabulary a caller names, so it belongs to the home's
// registry rather than to the file that produces it.
// ---------------------------------------------------------------------------

///
/// Two postures, and they are different observations rather than one with a
/// missing half. A declaration whose FAMILY seat carries an admissible sentence
/// has documentation material a projection can be planned over; one that carries
/// no family-seat row at all has none, and this home composes none — a summary
/// invented here would be a claim about the owner's declaration the owner did
/// not make.
///
/// Neither posture is a refusal. A declaration that documented nothing is a
/// lawful declaration, and the derive's own road does not stop for it.
#[must_use = "a documentation reading is either material a projection is planned over or the stated absence of it"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapturedDocumentationReading {
    /// The family seat carried a line the one-plain-sentence law admits, so the
    /// item is that sentence and no earned section.
    Documented {
        /// The documentation material, ready for a documentation projection's
        /// composition.
        item: DocumentedItem,
        /// What happened to facet election, which decides what a section is
        /// earned by.
        facets: ProjectionDisposition,
    },
    /// The declaration carries no family-seat row, so there is no owner sentence
    /// for an item to open with.
    NotDocumented {
        /// Why no item was read. Nobody asked for one: the author wrote no
        /// family-level prose, and this home writes none on an author's behalf.
        because: ProjectionDisposition,
    },
}

/// How one PLANNED MEMBER was not materialized: the role it was refused at, and
/// the home that refused.
///
/// # Authority
///
/// **A helper answers in the vocabulary of what it did, and the diagnostic is
/// composed once at the door.** Every road on the rendering half used to return
/// [`MacrocDiagnostic`] — a seat-complete value the size of the whole
/// compiler-facing account — merely because the public door eventually returns
/// one. That flattened error ownership at the first helper: which layer refused
/// became prose inside a large record rather than a fact the type carried, and
/// four separate seats each needed the same lint exemption to say so.
///
/// The role travels with the cause because a refusal on this road is always
/// about one member: a caller told only that "the rendering failed" has four
/// roles to inspect and no reason to prefer any of them.
///
/// # Bounds
///
/// Generic in the ROLE, because the two roads that reach it have two rosters: an
/// implementation projection materializes four roles and the carrier
/// materializes one. A refusal fixed to either roster would have to name the
/// other's member under a role it does not stand at, which is the neighbouring
/// value this whole home refuses to answer with.
#[must_use = "a member-render refusal names the role and the home that refused"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberRenderRefusal<R: RenderedRole> {
    /// The role the member stands under.
    pub role: R,
    /// Which home refused, carrying that home's own body.
    pub cause: MemberRenderCause,
}

/// Which home refused while one planned member was being materialized.
///
/// Two homes, two bodies, each carried whole: the DERIVE's renderer states what a
/// tree could not be rendered under, and the CLOSURE home's materialization
/// states which declared magnitude a unit's bytes passed. A single roster over
/// both would give a body that observes its own target and a byte count one shape
/// and one related-identity tag.
#[must_use = "a member-render cause names which home refused and carries that home's body"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemberRenderCause {
    /// The renderer refused. This home's own body.
    Rendered(RenderRefusal),
    /// Materializing the rendered tree refused. The closure home's own body.
    Materialized(RenderingRefusal),
}

/// How the CARRIER road refuses, with each home's own body carried whole.
///
/// # Authority
///
/// **Nine arms, nine homes, and not one summary.** The carrier road walks the
/// same eight public steps the implementation road walks — an account, a context,
/// a plan, a rendering, a proof, an explanation, a terminal — with the physical
/// assembly between them, and every one of those steps refuses in the vocabulary
/// of the home that owns it. A road that answered in one global diagnostic at
/// every step would be deciding, at the first helper, that which step failed is a
/// sentence rather than a value.
///
/// The projection into a diagnostic happens ONCE, at the door
/// ([`compile_declaration`](super::compile_declaration)), through each home's own
/// projection. Nothing here composes a line.
///
/// # Bounds
///
/// It says which step of the carrier road refused and carries what that step
/// said. It says nothing about the implementation road beside it, which ends at
/// its own terminal and refuses in its own vocabulary.
#[must_use = "a carrier-road refusal names which step refused and carries that step's body"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CarrierRoadRefusal {
    /// Planning the carrier refused.
    Planned(ProjectionPlanning),
    /// The carrier's own declaration vocabulary refused a spelling this home
    /// declared.
    Declared(ShellDeclarationRefusal),
    /// The proved outputs do not compose into one carrier.
    ///
    /// Boxed because the composition body carries a bounded issue roster, and the
    /// largest answer on this road must not set the size of the smallest.
    Assembled(Box<CarrierAssembly>),
    /// Reading the carrier's plan disagreed with the plan.
    PlanNotRead(DescriptorPlanIssue),
    /// The carrier plan and the assembly are not one declaration's, or the
    /// carrier's tokens do not fit its declared magnitude.
    ///
    /// Boxed on exactly [`CarrierRoadRefusal::Assembled`]'s terms: each arm of
    /// the composition answer carries a home's own bounded body.
    Composed(Box<ShellComposition>),
    /// Materializing the shell as the plan's one member refused.
    Rendered(MemberRenderRefusal<SoleRenderedUnit>),
    /// The rendering does not close over the plan it claims to materialize.
    ///
    /// Boxed on the same terms.
    Closed(Box<ProjectionClosureRefusal<SoleRenderedUnit>>),
    /// The explanation could not bind its subject, or does not cover this kind's
    /// questions.
    Explained(ExplanationBindingRefusal),
    /// The three values the terminal binds do not belong to one expansion.
    Bound(ExpansionBindingRefusal),
}

impl From<ProjectionPlanning> for CarrierRoadRefusal {
    fn from(refusal: ProjectionPlanning) -> Self {
        Self::Planned(refusal)
    }
}

impl From<ShellDeclarationRefusal> for CarrierRoadRefusal {
    fn from(refusal: ShellDeclarationRefusal) -> Self {
        Self::Declared(refusal)
    }
}

impl From<CarrierAssembly> for CarrierRoadRefusal {
    fn from(refusal: CarrierAssembly) -> Self {
        Self::Assembled(Box::new(refusal))
    }
}

impl From<DescriptorPlanIssue> for CarrierRoadRefusal {
    fn from(issue: DescriptorPlanIssue) -> Self {
        Self::PlanNotRead(issue)
    }
}

impl From<ShellComposition> for CarrierRoadRefusal {
    fn from(refusal: ShellComposition) -> Self {
        Self::Composed(Box::new(refusal))
    }
}

impl From<MemberRenderRefusal<SoleRenderedUnit>> for CarrierRoadRefusal {
    fn from(refusal: MemberRenderRefusal<SoleRenderedUnit>) -> Self {
        Self::Rendered(refusal)
    }
}

impl From<ProjectionClosureRefusal<SoleRenderedUnit>> for CarrierRoadRefusal {
    fn from(refusal: ProjectionClosureRefusal<SoleRenderedUnit>) -> Self {
        Self::Closed(Box::new(refusal))
    }
}

impl From<ExplanationBindingRefusal> for CarrierRoadRefusal {
    fn from(refusal: ExplanationBindingRefusal) -> Self {
        Self::Explained(refusal)
    }
}

impl From<ExpansionBindingRefusal> for CarrierRoadRefusal {
    fn from(refusal: ExpansionBindingRefusal) -> Self {
        Self::Bound(refusal)
    }
}

/// How one rendering failed to assemble.
#[must_use = "a rendering refusal names what the tree could not be rendered under"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderRefusal {
    /// The rendered tree exceeds the declared token magnitude.
    Unbounded,
}

/// How the callable text route refused.
///
/// Two postures, and they are genuinely different observations.
/// A text that cannot be cut into tokens never reached the grammar at all and
/// has no span table to point into; a text that cut fine and said the wrong
/// thing has both.
/// Folding them together would hand a caller a diagnostic whose site indexes a
/// table that was never built.
#[must_use = "a refusal names which of the two ways the callable text route refused"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextCompileRefusal {
    /// The text could not be cut into tokens.
    NotReadable(crate::token::TextReadRefusal),
    /// The text was read, and the compilation refused. The capture rides along
    /// so the diagnostic's token handle resolves against the same table the read
    /// issued.
    Refused(Box<(TextCapture, MacrocDiagnostic)>),
}
