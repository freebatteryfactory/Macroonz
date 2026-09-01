//! The request home's invariant nucleus: the roads that build one request, state its optional seats, and walk it.
//!
//! Declared inside `types.rs` as its own child, so a request's seats are reachable here and nowhere else.
//!
//! What lands here is what is about an ACT rather than about a value.
//! The walk is one function because the road is one road: a caller cannot arrive at a proof holding a rendering nobody planned, and cannot arrive at a binding holding an explanation answered over something else, because there is no seat between the steps to put a foreign value in.

use super::super::{decide, explain};
use super::{CrateBinding, Door, Producer, RUST_DECLARATION_PROFILE, Request, Selection};
use crate::closure::Closure;
use crate::diagnostic::{Diagnostic, Placement, Refused};
use crate::expansion::Expansion;
use crate::explanation::View;
use crate::identity::{
    self, Contract, Identity, OwnerFact, OwnerIdentity, Profile, Role, ServiceEntry, Transcript,
};
use crate::kind::{Kind, Question};
use crate::plan::Plan;
use crate::render::{Output, RenderError};
use crate::token::CapturedInput;

impl CrateBinding {
    /// The crate a consumer reaches this compiler's expansions through, by the word that consumer writes on its own dependency list.
    #[must_use]
    pub const fn declared(spelling: &'static str) -> Self {
        Self { spelling }
    }

    /// The word a path rendered through this binding opens with.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        self.spelling
    }
}

impl Door {
    /// One door, by the five facts a consumer declares once.
    ///
    /// A `const`, so a consumer writes it down beside its derive and passes it by reference from then on.
    #[must_use]
    pub const fn declared(
        prefix: &'static str,
        grammar: &'static str,
        entry: &'static str,
        binding: CrateBinding,
        producer: Producer,
    ) -> Self {
        Self {
            prefix,
            grammar,
            entry,
            binding,
            producer,
        }
    }

    /// The word every line composed through this door opens with.
    #[must_use]
    pub const fn prefix(&self) -> &'static str {
        self.prefix
    }

    /// The declaration grammar every diagnostic through this door expected to hold.
    ///
    /// Derived over the declared name's own bytes, rooted at [`Role::DeclaredName`], at position zero — the seat this compiler assigns a door's grammar.
    #[must_use]
    pub fn grammar(&self) -> Identity<Contract> {
        Identity::derived(Transcript::rooted(
            Role::DeclaredName,
            self.grammar.as_bytes(),
            0,
        ))
    }

    /// The callable entry point every diagnostic through this door reproduces at.
    ///
    /// Derived on [`Door::grammar`]'s terms, separated from it by its own subject and by its own content, at position one.
    #[must_use]
    pub fn entry(&self) -> Identity<ServiceEntry> {
        Identity::derived(Transcript::rooted(
            Role::DeclaredName,
            self.entry.as_bytes(),
            1,
        ))
    }

    /// The crate a path rendered through this door is rooted at.
    #[must_use]
    pub const fn binding(&self) -> CrateBinding {
        self.binding
    }

    /// Who is producing, for whatever this door's expansions are stamped into.
    #[must_use]
    pub const fn producer(&self) -> Producer {
        self.producer
    }
}

impl<'door, K: Kind> Request<'door, K> {
    /// The request one captured declaration and one kind's content amount to.
    ///
    /// Everything else has a stated default: nothing is depended on, the profile is [`RUST_DECLARATION_PROFILE`], nothing is assumed, no seat publishes to an address, and the kind's own questions are unanswered.
    /// A kind that declares questions or publishes to an address states those seats before rendering, or the road refuses at the step that needs them.
    pub fn over(capture: CapturedInput, content: K::Content, door: &'door Door) -> Self {
        Self {
            capture,
            content,
            door,
            dependencies: Vec::new(),
            profile: RUST_DECLARATION_PROFILE,
            assumptions: Vec::new(),
            addresses: Vec::new(),
            answers: Vec::new(),
            selection: Selection::All,
        }
    }

    /// Selects the structurally nonempty subset this request plans from the kind's complete role roster.
    ///
    /// The first role is separate so an empty selection cannot be stated.
    /// Selection order does not carry meaning and is canonicalized by the kind's role roster before planning.
    /// Foreign, doubled, and overlarge selections remain typed planning refusals under the existing membership owner.
    /// Stating a selection again replaces the earlier statement.
    pub fn selecting(mut self, first: K::Role, rest: Vec<K::Role>) -> Self {
        self.selection = Selection::Declared { first, rest };
        self
    }

    /// States the captures this content declares it stands on.
    ///
    /// The set is canonicalized where the account is built, so two callers declaring one set in two orders reach one plan.
    pub fn depending_on(
        mut self,
        dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    ) -> Self {
        self.dependencies = dependencies;
        self
    }

    /// States the profile this request is decided under.
    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
    }

    /// States the owner facts this projection rests on.
    ///
    /// They are the assumptions the explanation answers with and the decisions the trace records, which is one statement read twice rather than two a caller could disagree with itself about.
    pub fn assuming(mut self, assumptions: Vec<OwnerFact>) -> Self {
        self.assumptions = assumptions;
        self
    }

    /// States the address the unit under one seat is written to.
    ///
    /// Stating a seat's address twice keeps the last statement: an address is one fact about one seat, and two of them would leave the plan electing.
    /// The seat must be one a publication act consumes — planning refuses an address on a seat that never publishes, so a stated address is never an inert claim riding the identities.
    pub fn publishing_at(mut self, role: K::Role, address: OwnerIdentity) -> Self {
        self.addresses.retain(|(seat, _)| *seat != role);
        self.addresses.push((role, address));
        self
    }

    /// States the answers to the questions the kind itself declares.
    ///
    /// The universal questions every kind owes are answered by this road; these are the kind's own.
    pub fn answering(mut self, answers: Vec<<K::Question as Question>::Answer>) -> Self {
        self.answers = answers;
        self
    }

    /// Walk the road: plan, render, close, explain, bind.
    ///
    /// The renderer is called once, against the plan, and writes one unit per seat the plan declares.
    /// Each step hands the next a value the next one cannot forge, so the order is not a convention a caller could take in another sequence.
    ///
    /// # Errors
    ///
    /// Returns one [`Diagnostic`], composed under this request's door, wherever any step refuses: the plan the kind's roster and the caller's seats amount to, the renderer's own refusal, the units it wrote, the proof that they close over the plan, the coverage of the questions the kind owes, or the binding of the three.
    /// Every one of them happens before a token is reachable, because tokens are reachable only from the expansion this road returns.
    pub fn render(
        self,
        renderer: impl FnOnce(&Plan<K>, &mut Output<'_, K>) -> Result<(), RenderError>,
    ) -> Result<Expansion<K>, Diagnostic> {
        let Self {
            capture,
            content,
            door,
            dependencies,
            profile,
            assumptions,
            addresses,
            answers,
            selection,
        } = self;
        let statements = decide::Statements::from_request(&assumptions, &addresses, &selection);
        let plan =
            decide::planned::<K>(&capture, content, door, dependencies, profile, &statements)
                .map_err(|refusal| refused(&refusal, door))?;

        let mut out = Output::over(&plan);
        renderer(&plan, &mut out).map_err(|refusal| refused(&refusal, door))?;
        let units = out.rendered().map_err(|refusal| refused(&refusal, door))?;

        let closure = Closure::proved(&plan, units).map_err(|refusal| refused(&refusal, door))?;
        let universal = explain::universal(door, &plan, &closure, &assumptions)
            .map_err(|refusal| refused(&refusal, door))?;
        let view = View::complete(&plan, &closure, universal, answers)
            .map_err(|refusal| refused(&refusal, door))?;
        Expansion::bound(plan, closure, view).map_err(|refusal| refused(&refusal, door))
    }
}

/// The one projection this road makes: any step's own refusal, under this request's door, about the declaration as a whole.
///
/// The placement is never a token, and that is a claim rather than a shortcut: every refusal reachable here is established at or after planning, which is downstream of a capture that already succeeded, so there is no clause of the caller's grammar left to point at.
fn refused<E: Refused>(refusal: &E, door: &Door) -> Diagnostic {
    Diagnostic::refused(refusal, door, &Placement::WholeDeclaration)
}
