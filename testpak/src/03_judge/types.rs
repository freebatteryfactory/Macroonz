//! The judge seat's declarations: what one reading of one rendered artifact
//! concluded, what an artifact was read to declare, what a caller declared it
//! would declare, and the damages a judge inflicts on a lawful artifact.
//!
//! Declarations only. The readings themselves live beside their methods —
//! lane A in `byte_profile.rs`, lane B in `structural.rs`, the damage in
//! `mutation.rs` — and the mutation roster's closed tables live in
//! `type_contract.rs`. Nothing here decides anything, so a reader of this file
//! learns exactly what the seat can say and never how it says it.

/// What a judge found when it read one rendered projection against an
/// independently declared order.
///
/// Three answers, and none of them is silence: a reading that found nothing to
/// compare says so rather than passing.
///
/// # `Unreadable` is a failure class with its own alarm
///
/// [`RenderVerdict::Unreadable`] is not noise, not a skip, and not a softer
/// [`RenderVerdict::Deviates`]. It is its own failure class and it means one
/// specific thing: **the judge could not find the construct it anchors on.**
/// Either the artifact stopped stating that construct, or the artifact still
/// states it and the anchor no longer matches the text — a renamed constant, a
/// reformatted literal, a moved attribute.
///
/// Both of those are real findings, so a test asserting a lawful rendering
/// conforms MUST fail on `Unreadable`, and must never be written to accept it
/// alongside `Conforms`. A silent `Unreadable` is worse than a deviation: a
/// deviation says the renderer is wrong, while an ignored `Unreadable` says
/// nothing at all while every downstream assertion quietly stops testing
/// anything.
///
/// **The response to a false alarm is to fix the anchor deliberately, never to
/// loosen the reader.** When the artifact legitimately changes shape, the
/// anchor is re-stated to match the new shape, in one place, on purpose, and
/// the change is visible in the diff. Widening the reader until it matches
/// again — trimming whitespace, matching a prefix, falling back to a looser
/// pattern — buys a green run by making the judge cleverer, and a clever judge
/// starts agreeing with the renderer about what the text means, which is the
/// failure mode this whole package exists to avoid.
///
/// The reader is rehearsed against that alarm rather than trusted to raise it:
/// `testpak/tests/planted_defect.rs` shifts whitespace inside a lawful
/// rendering and requires `Unreadable`, so the alarm is known to sound before
/// anyone has to interpret one.
#[must_use = "a verdict is what the reading concluded about the rendering"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderVerdict {
    /// The rendering states exactly the declared order, spelling for spelling
    /// and identity for identity.
    Conforms,
    /// The rendering and the declared order disagree.
    Deviates,
    /// No projection could be read out of the rendering at all. A failure class
    /// of its own — see the type's documentation.
    Unreadable,
}

/// One cause row, as the artifact declares it: the four constructor paths it is
/// built through, the two seats of the stable identity minted for the cause, and
/// the spelling that cause is projected under.
///
/// The constructors are columns of the row and not decoration. A row spelling
/// the declared values through some other set of constructors declares something
/// else entirely, and a reader that kept only the strings would have called it
/// conforming.
///
/// There are four constructors and not two because a cause identity is a PAIR:
/// the row is built, the identity is minted, and each of the identity's two
/// seats is declared through its own type. A rendering that handed the joined
/// text to one seat, or minted the family through the local key's constructor,
/// is a different declaration and this reading says which column moved.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CauseRow {
    /// The path the row itself is constructed through.
    pub row_constructor: String,
    /// The path the row's stable identity is minted through.
    pub identity_constructor: String,
    /// The path the identity's family seat is declared through.
    pub family_constructor: String,
    /// The path the identity's local seat is declared through.
    pub local_constructor: String,
    /// The family the row's identity names.
    pub family: String,
    /// The local key the row's identity names inside that family.
    pub local: String,
    /// The spelling the row states.
    pub spelling: String,
}

/// One way an implementation may be WRITTEN beyond the plain form.
///
/// An implementation carries no visibility in Rust — there is no seat for one on
/// the item — so the postures a reader can be lied to about are these four, and
/// each of them changes what the artifact declares. A lawful rendering carries
/// none of them, which is why the declaration states an empty roster and any
/// posture at all is a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplPosture {
    /// `unsafe impl` — a contract with an obligation attached.
    Unsafely,
    /// `impl !Trait for Type` — the opposite of the contract the declaration
    /// named.
    Negative,
    /// `default impl` — a realization other implementations may replace.
    Defaulted,
    /// Generic parameters or a `where` clause: a family of implementations
    /// rather than the one the declaration named.
    Generic,
}

/// One trait implementation the artifact declares, as lane B read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationStructure {
    /// The type the implementation targets, spelled as its path.
    pub target: String,
    /// The trait path the implementation realizes, spelled with its leading
    /// `::` when it carries one.
    pub trait_path: String,
    /// The postures the implementation is written under, in roster order.
    pub postures: Vec<ImplPosture>,
    /// The attributes the implementation and its members carry that decide
    /// something — every attribute that is not a doc comment, by path.
    pub meaning_bearing_attributes: Vec<String>,
    /// The body-shape word the member constant `SHAPE` states, where this
    /// implementation states one.
    pub shape: Option<String>,
    /// The spellings the member constant `SELECTION_ORDER` states, in order,
    /// where this implementation states it.
    pub selection_order: Option<Vec<String>>,
    /// The path the member constant `DECLARED_ORDER` is constructed through,
    /// where this implementation states it.
    pub order_constructor: Option<String>,
    /// The cause rows the member constant `DECLARED_ORDER` states, in order,
    /// where this implementation states it.
    pub cause_rows: Option<Vec<CauseRow>>,
    /// The members that are not one of the three expected constants, described
    /// by what each one is.
    pub unexpected_members: Vec<String>,
    /// The expected constants this implementation states more than once, by
    /// name. The second reading is recorded here and never written over the
    /// first.
    pub duplicated_members: Vec<String>,
}

/// Everything lane B recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The trait implementations the artifact declares, in the order it
    /// declares them.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not trait implementations
    /// of a named type. Nothing lawful renders one, so any count above zero is
    /// a finding rather than a detail.
    pub other_items: usize,
}

/// What the caller states the artifact should declare, written independently of
/// the thing under judgement.
///
/// Every roster here is authored by the caller beside the declaration it handed
/// to the producer. Nothing in this structure is obtained by asking the producer
/// what it did.
#[derive(Debug, Clone, Copy)]
pub struct DeclaredStructure<'a> {
    /// The one type every declared implementation targets.
    pub target: &'a str,
    /// The trait paths the artifact declares, in the order it declares them.
    pub traits: &'a [&'a str],
    /// The postures every declared implementation is written under.
    pub postures: &'a [ImplPosture],
    /// The attributes the declaration admits on an implementation or on one of
    /// its members, by path. Doc comments are not attributes for this purpose
    /// and never appear here.
    pub attributes: &'a [&'a str],
    /// The body-shape word exactly one implementation states.
    pub shape: &'a str,
    /// The cause spellings, in declared order. Both the selection order and the
    /// cause rows are held to this one roster, because both project the same
    /// declared causes.
    pub spellings: &'a [&'a str],
    /// The stable cause identities, in declared order, each as the
    /// `(family, local)` pair the artifact spells. Stated as a pair rather than
    /// as a joined name because the artifact declares a pair: a caller who wrote
    /// the join would be asserting over a value the artifact does not carry.
    pub identities: &'a [(&'a str, &'a str)],
    /// The path the declared order is constructed through.
    pub order_constructor: &'a str,
    /// The path every cause row is constructed through.
    pub row_constructor: &'a str,
    /// The path every row's stable identity is minted through.
    pub identity_constructor: &'a str,
    /// The path every identity's family seat is declared through.
    pub family_constructor: &'a str,
    /// The path every identity's local seat is declared through.
    pub local_constructor: &'a str,
}

/// Which structural fact the artifact and the declaration disagree about.
///
/// One finding, named. A verdict that only said "no" would leave every caller
/// guessing which of a dozen questions came back wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralDisagreement {
    /// The artifact declares an item that is not a trait implementation of a
    /// named type.
    UnexpectedItem,
    /// One trait-and-target pair is implemented more than once.
    DuplicateImplementation,
    /// The artifact declares a different number of implementations than the
    /// declaration names.
    OutputCardinality,
    /// An implementation targets a type the declaration did not name.
    ImplementationTarget,
    /// An implementation realizes a trait path the declaration did not name, or
    /// names them in another order.
    TraitPath,
    /// An implementation is written `unsafe`, negative, `default`, or generic
    /// where the declaration names none of those.
    ImplPosture,
    /// An implementation or one of its members carries an attribute that
    /// decides something and that the declaration did not name.
    MeaningBearingAttribute,
    /// An implementation carries a member that is not one of the expected
    /// associated constants.
    UnexpectedImplMember,
    /// An implementation states one of the expected constants more than once.
    DuplicateMember,
    /// The stated body-shape word is not the declared one, or is stated by no
    /// implementation or by more than one.
    FamilyShape,
    /// The stated selection order is not the declared roster, in order.
    SelectionOrder,
    /// A declared value is carried through a constructor path the declaration
    /// did not name.
    ConstructorPath,
    /// The stated cause rows are not the declared identities and spellings, in
    /// order.
    CauseRows,
}

/// What one structural reading concluded.
///
/// Three answers, and none of them is silence — see `structural.rs` for why
/// `Unparsable` is its own failure class.
#[must_use = "a verdict is what the structural reading concluded"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralVerdict {
    /// The artifact declares exactly what the caller declared it would.
    Conforms,
    /// The artifact and the declaration disagree, about this.
    Deviates(StructuralDisagreement),
    /// The text is not parseable Rust, so nothing structural was read at all.
    Unparsable,
}

/// Which lane's claim covers catching one mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaneOwnership {
    /// Lane A — the byte-profile scan ([`crate::judge::byte_profile`]). It
    /// catches this because the mutation changes the exact declared textual
    /// form the scan anchors on.
    ByteProfile,
    /// Lane B — the structural read ([`crate::judge::structural`]). Catching
    /// this needs an answer about what the artifact DECLARES, which no scan over
    /// bytes can give.
    Structural,
    /// Lane C — compiled behaviour. Catching this needs `rustc` to reject the
    /// artifact or to hand back a different value.
    CompiledBehaviour,
}

/// One deliberate damage a judge inflicts on a lawful artifact.
///
/// Each is a lie the mutated text tells about the declaration it claims to
/// project. None of them is invented by the thing under judgement. Which lane
/// owns catching each one is the closed table in `type_contract.rs`; the damage
/// itself is `mutation.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactMutation {
    /// The textual selection order is reversed while the typed order stands as
    /// declared — the projection no longer projects.
    OrderPermuted,
    /// Every cause is emitted under the first cause's local key — distinct
    /// causes inside one family made to share one identity.
    IdentityRecycled,
    /// One planned output is deleted from the artifact.
    PlannedOutputOmitted,
    /// An output nobody planned is appended.
    UnplannedOutputAdded,
    /// The implementation targets a different type than the one declared.
    ImplTargetAltered,
    /// The declared body shape is changed.
    ShapeAltered,
    /// A planned output is emitted twice.
    OutputDuplicated,
    /// The trait path names a contract the declaration did not realize.
    TraitPathWrong,
    /// A decoy carrying the anchored bytes is planted inside a comment while the
    /// real constant is damaged.
    DecoyInComment,
    /// One planned member constant is emitted twice inside one implementation.
    ImplMemberDuplicated,
    /// A member nobody planned is added inside one implementation.
    ImplMemberUnexpected,
    /// A declared value is carried through a constructor the declaration did
    /// not name.
    ConstructorPathAltered,
    /// The implementation is written under a posture the declaration did not
    /// name.
    ImplPostureAltered,
    /// An attribute that decides something is added to an implementation.
    MeaningBearingAttributeAdded,
    /// The artifact stops being well-formed Rust.
    MalformedRust,
}

/// The declared mutation roster, in the order this seat states it.
pub const ARTIFACT_MUTATIONS: [ArtifactMutation; 15] = [
    ArtifactMutation::OrderPermuted,
    ArtifactMutation::IdentityRecycled,
    ArtifactMutation::PlannedOutputOmitted,
    ArtifactMutation::UnplannedOutputAdded,
    ArtifactMutation::ImplTargetAltered,
    ArtifactMutation::ShapeAltered,
    ArtifactMutation::OutputDuplicated,
    ArtifactMutation::TraitPathWrong,
    ArtifactMutation::DecoyInComment,
    ArtifactMutation::ImplMemberDuplicated,
    ArtifactMutation::ImplMemberUnexpected,
    ArtifactMutation::ConstructorPathAltered,
    ArtifactMutation::ImplPostureAltered,
    ArtifactMutation::MeaningBearingAttributeAdded,
    ArtifactMutation::MalformedRust,
];
