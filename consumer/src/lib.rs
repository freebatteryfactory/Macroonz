//! An ordinary crate that adopts the machine from outside it.
//!
//! The package renames every ThreadPak dependency on its own dependency list —
//! `tp` for the machine, `tp_macros` for the derive shell, `harness` for the
//! judge — and reaches all three under no other name anywhere in this tree. That
//! rename is the whole point of the package: a reference the machine, the shell,
//! or the harness emits which resolves only under a published package name does
//! not resolve here.
//!
//! # What this library is
//!
//! A small owned vocabulary of the kind a real adopter writes: one request a
//! caller states, one bounded value its two lawful constructors answer with, and
//! the two refusal families those constructors refuse in. Nothing here is judge
//! machinery, and nothing here is clever — the moment this package starts
//! helping, it stops being evidence about what an ordinary consumer experiences.
//! Everything about testing lives in `tests/`, which is where a consumer's
//! testing lives.
//!
//! # The machine surface this library uses
//!
//! [`LotRefusal`] realizes the machine's two refusal contracts by hand —
//! `tp::refusal::RefusalFamily` and `tp::refusal::CauseOrderDeclaration`, whose
//! own page names a consumer outside the machine's crate as a lawful declarer of
//! a family. [`MergeRefusal`] realizes the same two contracts through the
//! shell's derive, off one declaration wearing `#[refusal(...)]`. Both roads
//! spell the machine `tp::` — the derived one because its declaration states the
//! binding this package reaches the machine by — so both are the standing
//! positive control for the rename: they resolve, or this package does not
//! build.
//!
//! # The pair
//!
//! One family realized by hand and one derived, in one crate, under one pair of
//! contracts. What the pair earns is a reading written ONCE over the contract
//! and taken of both: `tests/the_derived_road.rs` reads each family's declared
//! facts through a road generic in the contract, so a hand-realized family and a
//! derived one stand in the same seat rather than in two seats that agree.
//!
//! # What the derived road delivers here, exactly
//!
//! A refusal-family declaration delivers its two production implementations at
//! the declaration site, and this crate compiles them: they are what
//! `tests/the_derived_road.rs` reads back as values. The derived family's test
//! rows and its evaluation support arrive when the emission road delivers
//! carrier cargo to a rendered support shell; until that delivery exists, this
//! crate's derived-road evidence is the declaration-site surface read back as
//! values, which is what that file states and the whole of what it claims.
//!
//! # The seat's claim ceiling
//!
//! The seat's law is the harness README's and is not restated here. What this
//! package establishes today is CONSUMER-SHAPED: it is a workspace member, so it
//! shares this workspace's resolution and its lint wall. A true outsider shares
//! neither, and the packaged check that stands one up is the blessing day's.

/// What one caller states when it asks for a lot to be counted.
///
/// Unparsed on purpose: both seats are exactly as the caller wrote them, and
/// whether they name a lawful lot is [`Lot::counted`]'s question rather than
/// this value's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CountRequest {
    label: &'static str,
    items: u32,
}

impl CountRequest {
    /// The request, over the label and the count a caller stated.
    #[must_use]
    pub const fn stated(label: &'static str, items: u32) -> Self {
        Self { label, items }
    }

    /// The label the caller stated.
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.label
    }

    /// The count the caller stated.
    #[must_use]
    pub const fn items(self) -> u32 {
        self.items
    }
}

/// A counted lot: which lot was counted, and how many items it held.
///
/// # Construction
///
/// [`Lot::counted`] and [`Lot::merged`] are the only roads, and both stand
/// under the same ceiling — so a lot that names nothing and a lot larger than
/// this crate admits are values nobody holds, whichever road was walked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lot {
    label: &'static str,
    items: u32,
}

impl Lot {
    /// The largest count one lot admits.
    pub const CEILING: u32 = 1_000;

    /// This lot, over the label and the count it was given.
    ///
    /// # Errors
    ///
    /// Dependent checks in a declared order: the label is read first, so an
    /// unlabelled request never reaches the ceiling and exactly one cause is
    /// true of any refused request.
    pub const fn counted(label: &'static str, items: u32) -> Result<Self, LotRefusal> {
        if label.is_empty() {
            return Err(LotRefusal::NotLabelled);
        }
        if items > Self::CEILING {
            return Err(LotRefusal::OverLimit);
        }
        Ok(Self { label, items })
    }

    /// The lot this count is about.
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.label
    }

    /// The count this lot carries.
    #[must_use]
    pub const fn items(self) -> u32 {
        self.items
    }

    /// Two counts of one lot, merged into the single count they add up to.
    ///
    /// # Errors
    ///
    /// Dependent checks in a declared order: the labels are read first, so a
    /// pair naming two different lots never reaches the ceiling and exactly one
    /// cause is true of any refused pair.
    pub fn merged(self, other: Self) -> Result<Self, MergeRefusal> {
        if self.label != other.label {
            return Err(MergeRefusal::NotTheSameLot);
        }
        let items = self.items.saturating_add(other.items);
        if items > Self::CEILING {
            return Err(MergeRefusal::OverLimit);
        }
        Ok(Self {
            label: self.label,
            items,
        })
    }
}

/// How one lot refused the request it was given.
///
/// Single cause, because the checks are dependent: the ceiling is meaningful
/// only once the request names a lot at all.
#[must_use = "a refusal is the reason a lot was not counted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LotRefusal {
    /// The request states an empty label, so the lot names nothing.
    NotLabelled,
    /// The count is past [`Lot::CEILING`].
    OverLimit,
}

/// How merging two counted lots refused the pair it was given.
///
/// # The shape this family takes, and why
///
/// Single cause, because the checks are dependent: two counts of different lots
/// have no sum worth taking, so the ceiling becomes a meaningful question only
/// once the pair is known to be about one lot. The roster's other two shapes
/// each say something this road does not — an issue collection stands for
/// independent facts that can all be true at once, and an inseparable pair
/// stands for exactly two questions neither of which means anything alone —
/// and one cause is all that can truthfully exist here.
///
/// # What is declared once
///
/// The identity this family is known by, the binding this package reaches the
/// machine under, the body shape, and the canonical order over the causes are
/// stated in the attribute below and nowhere else. The two contract
/// implementations are derived from that one statement, so the family's
/// declared facts and the family's declaration are the same sentence rather
/// than two that have to be kept agreeing.
///
/// The causes' identities are minted from the family identity and each cause's
/// own local key, so nothing here writes a shared prefix out by hand. The key
/// `over-limit` is a word this family and [`LotRefusal`] both spell, and the two
/// causes are different identities because their family seats differ — which is
/// exactly what a shared word buys, and no more.
#[must_use = "a refusal is the reason two lots were not merged"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, tp_macros::RefusalFamily)]
#[refusal(
    crate = tp,
    family = "consumer.lot-merge",
    shape = single_cause,
    order(NotTheSameLot = "not-the-same-lot", OverLimit = "over-limit")
)]
pub enum MergeRefusal {
    /// The two counts name different lots, so there is nothing to add.
    NotTheSameLot,
    /// The two counts are of one lot and add up past [`Lot::CEILING`].
    OverLimit,
}

/// This crate's stable identity for the lot family.
///
/// The domain segment is this consumer's, never the machine's: a family
/// identity states who owns the family, and nothing about it is inherited.
const LOT_FAMILY: tp::refusal::RefusalFamilyId =
    tp::refusal::RefusalFamilyId::declared("consumer.lot");

impl tp::refusal::RefusalFamily for LotRefusal {
    const SHAPE: tp::refusal::FamilyShape = tp::refusal::FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotLabelled", "OverLimit"];
}

impl tp::refusal::CauseOrderDeclaration for LotRefusal {
    const DECLARED_ORDER: tp::refusal::DeclaredCauseOrder =
        tp::refusal::DeclaredCauseOrder::declared(&[
            tp::refusal::DeclaredCause::declared(
                tp::refusal::CauseId::declared(
                    LOT_FAMILY,
                    tp::refusal::LocalCauseKey::declared("not-labelled"),
                ),
                "NotLabelled",
            ),
            tp::refusal::DeclaredCause::declared(
                tp::refusal::CauseId::declared(
                    LOT_FAMILY,
                    tp::refusal::LocalCauseKey::declared("over-limit"),
                ),
                "OverLimit",
            ),
        ]);
}
