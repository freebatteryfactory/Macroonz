//! An ordinary crate that adopts the machine from outside it.
//!
//! The package renames both of its ThreadPak dependencies on its own dependency
//! list — `tp` for the machine, `harness` for the judge — and reaches them under
//! no other name anywhere in this tree. That double rename is the whole point of
//! the package: a reference the machine or the harness emits which resolves only
//! under a published package name does not resolve here.
//!
//! # What this library is
//!
//! A small owned vocabulary of the kind a real adopter writes: one request a
//! caller states, one bounded value its lawful constructor answers with, and the
//! refusal family that constructor refuses in. Nothing here is judge machinery,
//! and nothing here is clever — the moment this package starts helping, it stops
//! being evidence about what an ordinary consumer experiences. Everything about
//! testing lives in `tests/`, which is where a consumer's testing lives.
//!
//! # The machine surface this library uses
//!
//! [`LotRefusal`] realizes the machine's two refusal contracts by hand —
//! `tp::refusal::RefusalFamily` and `tp::refusal::CauseOrderDeclaration`, whose
//! own page names a consumer outside the machine's crate as a lawful declarer of
//! a family. Both realizations are spelled through `tp::`, so they are also the
//! standing positive control for the rename: they resolve, or this package does
//! not build.
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
/// [`Lot::counted`] is the only road, so a lot that names nothing and a lot
/// larger than this crate admits are values nobody holds.
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
