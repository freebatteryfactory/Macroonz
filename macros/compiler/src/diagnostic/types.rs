//! The diagnostic home's declarations: one diagnostic, the door it is asked through, the typed parts of its one summary line, the registries a refusal reads its vocabulary off, and the trait a refusing step implements.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, so the seats a caller may not write have exactly one way in.

use crate::bounded::{Bounded, Capping};
use crate::identity::{
    Contract, HumanProjection, Identity, OwnerFact, RelatedBody, RelatedIssue, ServiceEntry,
};
use crate::request::{CrateBinding, Producer};
use crate::token::{SourceCoordinate, SpanHandle, SpanResolutionRefusal, SpanTable};

#[path = "type_guard.rs"]
mod guard;

/// Owner-declared repairs one diagnostic may carry.
pub const REPAIR_LIMIT: usize = 8;

/// Identities one diagnostic's related set may carry.
///
/// A step may establish a wider body, so a body that outruns this is a case the road meets rather than one the bounds rule out.
pub const RELATED_ISSUE_LIMIT: usize = 64;

// ---------------------------------------------------------------------------
// The families.
//
// A family here is one refusal's issue space, and its name is what keeps two
// spaces' identical bytes from deriving one identity. The name is namespaced
// like an identity stem — the declarer's own name ahead of the space's — so an
// adopter declares its families in its own crate and two crates' spaces cannot
// collide without one wearing the other's name. A name is preimage material:
// renaming a family renames every related identity derived in it.
// ---------------------------------------------------------------------------

/// One refusal's issue space, named by its declarer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Family(&'static str);

/// Planning a projection.
pub const PLANNING_FAMILY: Family = Family::declared("macroonz/planning");

/// Proving a rendering closes over its plan.
pub const CLOSURE_FAMILY: Family = Family::declared("macroonz/closure");

/// Covering a kind's questions with an explanation.
pub const EXPLANATION_FAMILY: Family = Family::declared("macroonz/explanation");

/// Materializing a planned member.
pub const RENDERING_FAMILY: Family = Family::declared("macroonz/rendering");

/// Sealing a plan, a closure, and an explanation as one expansion.
pub const BINDING_FAMILY: Family = Family::declared("macroonz/binding");

/// Composing closed outputs into one carrier.
pub const ASSEMBLY_FAMILY: Family = Family::declared("macroonz/assembly");

/// Rendering the carrier shell itself.
pub const SHELL_FAMILY: Family = Family::declared("macroonz/shell");

/// The carrier's own declaration vocabulary.
pub const DECLARATION_FAMILY: Family = Family::declared("macroonz/descriptor-declaration");

/// The support home's own declaration vocabulary, which is not the descriptor's however alike the two read.
pub const SUPPORT_DECLARATION_FAMILY: Family = Family::declared("macroonz/support-declaration");

/// Reading a carrier's plan.
pub const DESCRIPTOR_PLAN_FAMILY: Family = Family::declared("macroonz/descriptor-plan");

/// The trial helper's captured grammar.
pub const FIRST_HELPER_FAMILY: Family = Family::declared("macroonz/trial-helper");

/// The mutation helper's captured grammar.
pub const SECOND_HELPER_FAMILY: Family = Family::declared("macroonz/mutation-helper");

/// The bench helper's captured grammar.
pub const BENCH_HELPER_FAMILY: Family = Family::declared("macroonz/bench-helper");

/// The shadow face's captured grammar.
pub const SHADOW_HELPER_FAMILY: Family = Family::declared("macroonz/shadow-helper");

/// The network declaration's captured grammar.
pub const NETWORK_HELPER_FAMILY: Family = Family::declared("macroonz/network-helper");

/// The concurrency declaration's captured grammar.
pub const CONCURRENCY_HELPER_FAMILY: Family = Family::declared("macroonz/concurrency-helper");

/// Reading a declared input into a captured surface.
pub const CAPTURE_FAMILY: Family = Family::declared("macroonz/capture");

/// A codec shape's own declaration vocabulary.
pub const CODEC_DECLARATION_FAMILY: Family = Family::declared("macroonz/codec-declaration");

/// Which step of the road was running when the disagreement was observed.
///
/// Declared in the order the steps run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    /// Reading the declared input into a captured surface.
    Capture,
    /// Naming the complete output set, before a token of Rust exists.
    Planning,
    /// Materializing a planned member.
    Rendering,
    /// Proving the rendering closes over the plan it claims to materialize.
    Closure,
    /// Answering the questions the kind owes.
    Explanation,
    /// Sealing the plan, the closure, and the explanation as one expansion.
    Binding,
    /// Composing closed outputs into one exported carrier.
    Assembly,
}

/// Which class of refusal one composed line is about.
///
/// The class is the second clause of every line and is READ off this roster rather than written at the seam that refused, so two seams reporting one class do not read as two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefusalClass {
    /// The declared input was not read into a captured surface.
    DeclarationNotRead,
    /// Planning refused before a token of Rust existed.
    PlanNotStated,
    /// A renderer did not produce the unit the plan named.
    RenderingNotProduced,
    /// The rendering does not close over the plan it claims to materialize.
    RenderingNotClosed,
    /// The written explanation does not cover its kind's questions.
    ExplanationNotCovered,
    /// The explanation had no subject to write its seats about.
    ExplanationNotBound,
    /// A rendering would have passed a declared magnitude.
    MagnitudeNotHeld,
    /// The three values a binding seals do not belong to one expansion.
    ExpansionNotBound,
    /// A set of closed outputs does not compose into one exported carrier.
    CarrierNotAssembled,
    /// The carrier's own vocabulary was not declared.
    CarrierNotDeclared,
    /// A class an adopter declared, for a refusal none of the rows above say.
    ///
    /// The two spellings are the adopter's own, and a line about it reads them exactly as a line about any row above reads that row's.
    Declared {
        /// The stable kebab-case name.
        name: &'static str,
        /// The second clause of a composed line about it.
        described: &'static str,
    },
}

/// How what was observed differs from the contract that was expected.
///
/// A typed classification, never a sentence: the sentence is a projection of this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Observed {
    /// A required seat was unfurnished.
    SeatAbsent,
    /// What was present disagrees with the expected contract.
    ContractDisagreement,
    /// An identity that had to match did not.
    IdentityDisagreement,
    /// The material was presented under a profile that does not offer it.
    ProfileDisagreement,
    /// A declared magnitude was exceeded.
    BoundExceeded,
    /// Generated material arrived with no origin.
    OriginAbsent,
    /// A difference an adopter declared, for an observation none of the rows above classify.
    Declared {
        /// The stable kebab-case name.
        name: &'static str,
        /// How the difference reads in a composed line.
        described: &'static str,
    },
}

/// One declared magnitude a rendering can pass, and the thing it governs.
///
/// The prose belongs to the magnitude rather than to whichever refusal named it; the number belongs to the home that declares the bound, so a refusal carries it and this roster never restates it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderedMagnitude {
    /// The bytes one rendered unit may carry.
    RenderedBytes,
    /// The units one rendering may carry.
    RenderedUnits,
    /// The tokens one generated tree may carry at one nesting level.
    GeneratedTokens,
}

/// The seat one explanation could not bind its subject to.
///
/// Named seats rather than one "something was missing": a caller repairing a derivation needs to know whether the PLAN failed to declare the member, the CLOSURE failed to prove its bytes, or the plan cited no owner fact at all, and those are three different repairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationSeat {
    /// The planned member standing under the role the seat is about.
    PlannedMember,
    /// The digest the closure proved over that member's rendered bytes.
    ProvedDigest,
    /// The first owner fact the plan declares as an assumption.
    DeclaredAssumption,
}

/// What one composed line is a summary OF.
///
/// A single-cause refusal establishes one cause and enumerates nothing: there is no remainder to count and no bound anything could have been capped at, so a line reporting "and 0 further issues, complete" would answer a question never asked of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineBody {
    /// One established cause, with nothing enumerated beside it.
    SingleCause,
    /// A body of independent issues.
    Body {
        /// Established issues past the one the line states in full.
        further: usize,
        /// Whether the body kept every issue it established.
        capping: Capping,
    },
}

/// The typed parts one compiler line is composed from.
///
/// They travel as one value because they are one line: a class handed to [`composed`](crate::diagnostic::composed) beside another refusal's first established issue composes a sentence that is well formed, complete-looking, and about nothing in particular.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line<'issue> {
    /// Which class of refusal the line is about.
    pub class: RefusalClass,
    /// The first established issue, stated in full.
    pub first: &'issue str,
    /// What the line is a summary of.
    pub body: LineBody,
}

/// Whether a composed line says where the refusal sits.
///
/// Not an option: a whole-declaration refusal is a STATED posture and not a site somebody forgot to supply, and adding a position to its line would send a reader to an arbitrary spot inside a declaration the refusal is not about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineSite {
    /// The refusal is about the declaration as a whole, and the line adds nothing.
    WholeDeclaration,
    /// The refusal sits somewhere the producer can name, and the line says where — or says that the producer's table does not reach it.
    At(SiteCoordinate),
}

/// Where one diagnostic's token sits, or why the producer's table could not say.
///
/// A seat that cannot be furnished states the posture rather than being filled with a stand-in: a coordinate written where a table did not reach would read exactly like a coordinate the table resolved, and the reader has no third value to compare it against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SiteCoordinate {
    /// The position the producer's table resolved, in the role it speaks.
    Resolved(SourceCoordinate),
    /// The producer's table does not reach this handle, and the refusal states how far it does reach.
    NotReached(SpanResolutionRefusal),
}

/// Where one diagnostic points.
///
/// A diagnostic about a CAPTURED declaration names the offending token in the producer's own span table, so the producer can put a compiler error on exactly that token rather than on the declaration's first one.
/// A diagnostic established BEFORE any capture has no token to name — no table was built and no handle was issued — and carries the byte it was born at instead.
/// A diagnostic about the declaration AS A WHOLE has nowhere narrower to point, and says exactly that: the whole-declaration arm carries no token and no coordinate, because any it carried would send a reader to a spot the refusal is not about.
///
/// # Nonclaims
///
/// **The pre-capture and whole-declaration arms mint no handle, and that is the substitution this sum removes.**
/// A required handle seat forces handle zero onto an observation that issued none, and handle zero reads exactly like an honest answer pointing at the declaration's first token.
#[must_use = "a site names the token it points at, the byte it was born at, or the whole declaration"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Site {
    /// One token of a captured declaration, and where the producer's table put it.
    AtToken {
        /// The offending token, as a handle into the producer's own span table.
        token: SpanHandle,
        /// Where that token sits, or the typed statement that the table does not reach the handle.
        coordinate: SiteCoordinate,
    },
    /// One byte of the text a read refused on, before any capture existed to issue a handle.
    BeforeCapture {
        /// The byte the observation was born at, in the role its own text counts in.
        coordinate: SourceCoordinate,
    },
    /// The declaration as a whole: a stated posture with no token seat to fabricate one into.
    WholeDeclaration,
}

/// One identity a related set carries, at the level it is about.
///
/// The two levels are two types rather than two positions, because position is not a fact a reader can check and because one subject over two levels collides by construction: the body's preimage is the framing of its issues, so an issue whose own material happened to be that framing would derive the byte-for-byte identity of the body it aliased.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelatedIdentity {
    /// The whole refusal body, as one commitment to every issue it established.
    Body(Identity<RelatedBody>),
    /// One established issue, on its own.
    Issue(Identity<RelatedIssue>),
}

/// One diagnostic's related set: the identities it carries, married to how it was capped.
///
/// A capping that can be carried away from its set is a claim that can be told about a different one, so the set-building road is the only road in and the seats are private.
///
/// # Ordering
///
/// The body rides first, and a reader does not depend on that: an identity states its own level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelatedSet {
    carried: Bounded<RelatedIdentity, RELATED_ISSUE_LIMIT>,
    capping: Capping,
}

/// One repair the owner declared, projected for a person to read.
///
/// The citation is the load-bearing member.
/// The text is a projection of it, and nothing here ever composes a repair the owner did not declare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Repair {
    /// The owner fact that declares this repair.
    pub declared_by: OwnerFact,
    /// The repair rendered for a person.
    pub description: HumanProjection,
}

/// How to reach the same observation again.
///
/// The road is the callable entry point the door names, which needs no proc macro anywhere in the path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Route {
    entry: Identity<ServiceEntry>,
}

/// The one value that says who is asking.
///
/// The two declared names are spellings rather than identities so a door is a `const` a consumer writes down once; the identities they stand for are derived on read, under the declared-name grammar at the two positions this compiler assigns.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Door {
    prefix: &'static str,
    grammar: &'static str,
    entry: &'static str,
    binding: CrateBinding,
    producer: Producer,
}

/// Where one projected refusal sits.
///
/// A refusal about the DECLARATION has nowhere narrower to point, and a line naming a position inside it would send a reader to an arbitrary spot; a refusal about one CLAUSE of an authored attribute has exactly one place, and the reader is sent there.
#[derive(Debug, Clone, Copy)]
pub enum Placement<'table> {
    /// The refusal is about the declaration as a whole.
    WholeDeclaration,
    /// The refusal sits at one token, resolved through the producer's own table.
    AtToken {
        /// The token it sits at.
        token: SpanHandle,
        /// The table the producer resolves handles through.
        spans: &'table SpanTable,
    },
}

/// One diagnostic.
///
/// Every seat is required and every one of them is readable: a diagnostic that could omit its phase, its site, its expected contract, or its observed classification would be one that sometimes says less than it knows.
/// The seats a caller reads after deciding what refused ride behind one pointer, so every `Result` that answers with a diagnostic stays small on its passing side.
#[must_use = "a diagnostic carries the observation, its site, and the owner-declared repair"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    phase: Phase,
    site: Site,
    observed: Observed,
    carried: Box<DiagnosticSeats>,
}

/// The seats one diagnostic carries behind its pointer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiagnosticSeats {
    summary: String,
    expected: Identity<Contract>,
    related: RelatedSet,
    repairs: Bounded<Repair, REPAIR_LIMIT>,
    route: Route,
}

/// How one step of the road says no.
///
/// A step refuses in the vocabulary of the home that owns it and implements this to say how that vocabulary reads; [`Diagnostic::refused`] is the one road from any implementation to a diagnostic.
/// The two associated constants are facts about the error's TYPE and not about a call site, for the reason this home's README gives.
pub trait Refused {
    /// The step of the road this refusal is raised at.
    const PHASE: Phase;

    /// The family whose issue space this refusal's related identities derive in.
    ///
    /// An adopter declares its own, under its own namespace; the compiler's are the [`Family`] constants beside this trait.
    const FAMILY: Family;

    /// Which class of refusal the summary line opens with.
    fn class(&self) -> RefusalClass;

    /// How the first established issue reads for a person.
    fn first(&self) -> String;

    /// How what was observed differs from the expected contract, read off that same first issue.
    fn observed(&self) -> Observed;

    /// What the summary line is a summary of.
    fn body(&self) -> LineBody;

    /// One canonical byte string per issue established BEYOND the primary cause, in the order the body established them.
    ///
    /// The primary cause is the summary's own subject and never a member of its related set, so a single-cause refusal answers with nothing — which is what [`LineBody::SingleCause`]'s "enumerates nothing" means, at the machine seat as well as in the line.
    /// Two bodies that differ in any typed member must answer with different bytes; that completeness is the implementing home's.
    fn related(&self) -> Vec<Vec<u8>>;

    /// The owner-declared repairs that apply.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT>;
}
