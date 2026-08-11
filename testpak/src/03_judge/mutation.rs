//! The mutations, and who owns catching each one.
//!
//! # Mutation is the JUDGE's job, and it used to be the generator's
//!
//! The services used to carry the planted defect themselves: a public
//! `rendered_with_planted_defect` road that produced a deliberately wrong
//! artifact for testpak to catch. That is the generator writing its own exam.
//! Whatever defect the generator can imagine is the defect the judge is
//! rehearsed against, and the defects the generator cannot imagine are exactly
//! the ones nobody looks for.
//!
//! So the road is gone from the services and the mutations live here. testpak
//! takes a LAWFUL artifact — one the receipt-rich road produced and closed over
//! — and damages it itself.
//!
//! # A mutation names the lane that owns it, and the lanes that do not
//!
//! **Not every lane catches every mutation, and pretending otherwise is the
//! defect this record exists to prevent.** A byte scan anchored on
//! `const SELECTION_ORDER` cannot tell a real constant from the same bytes
//! sitting inside a comment, and it has no opinion at all about what item the
//! constant is a member of. Recording "lane A catches this" for a mutation lane
//! A cannot see would turn the whole ledger into a green wall that measures
//! nothing.
//!
//! So [`ArtifactMutation::owned_by`] states, per mutation, which lane the
//! catching claim belongs to. Two of the three lanes are seated in this package
//! — the byte scan in [`crate::judge`] and the structural read in
//! [`crate::judge::structural`] — and each is held, in
//! `tests/planted_defect.rs`, to exactly the mutations recorded against it and
//! to no others.

/// Which lane's claim covers catching one mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaneOwnership {
    /// Lane A — the byte-profile scan. It catches this because the mutation
    /// changes the exact declared textual form the scan anchors on.
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
/// project. None of them is invented by the thing under judgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactMutation {
    /// The textual selection order is reversed while the typed order stands as
    /// declared — the projection no longer projects.
    OrderPermuted,
    /// Every cause is emitted under the first cause's identity — distinct causes
    /// made to share one identity.
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
    /// The artifact stops being well-formed Rust.
    MalformedRust,
}

/// The declared mutation roster, in the order this seat states it.
pub const ARTIFACT_MUTATIONS: [ArtifactMutation; 10] = [
    ArtifactMutation::OrderPermuted,
    ArtifactMutation::IdentityRecycled,
    ArtifactMutation::PlannedOutputOmitted,
    ArtifactMutation::UnplannedOutputAdded,
    ArtifactMutation::ImplTargetAltered,
    ArtifactMutation::ShapeAltered,
    ArtifactMutation::OutputDuplicated,
    ArtifactMutation::TraitPathWrong,
    ArtifactMutation::DecoyInComment,
    ArtifactMutation::MalformedRust,
];

impl ArtifactMutation {
    /// Which lane's claim covers catching this mutation.
    ///
    /// Read this as a ledger of what is CLAIMED, not of what is comfortable. The
    /// four that name [`LaneOwnership::Structural`] and the two that name
    /// [`LaneOwnership::CompiledBehaviour`] are not caught by the byte scan and
    /// are not recorded as though they were.
    ///
    /// Ownership is the seat of the CLAIM, not an exclusivity boast: the
    /// structural read happens to notice a permuted order too, and says nothing
    /// about it, because that verdict is stated over lane A's method and belongs
    /// to lane A's row.
    #[must_use]
    pub const fn owned_by(self) -> LaneOwnership {
        match self {
            // The first pair changes the exact spellings or the exact identities
            // the scan reads out of the anchored forms. The second pair changes
            // how MANY `CauseId` forms the artifact carries, which the scan's
            // magnitude check sees. Different reasons, one lane.
            Self::OrderPermuted
            | Self::IdentityRecycled
            | Self::PlannedOutputOmitted
            | Self::OutputDuplicated => LaneOwnership::ByteProfile,
            // What item is this, what does it target, which trait does it
            // realize, and is that constant a member of it — none of those is a
            // question about bytes.
            Self::ImplTargetAltered
            | Self::TraitPathWrong
            | Self::UnplannedOutputAdded
            | Self::DecoyInComment => LaneOwnership::Structural,
            // A changed shape word and a malformed artifact are both caught
            // where the artifact is compiled and read back as values.
            Self::ShapeAltered | Self::MalformedRust => LaneOwnership::CompiledBehaviour,
        }
    }

    /// The mutation rendered for a person. A projection: nothing reads it back.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::OrderPermuted => "the textual selection order is reversed",
            Self::IdentityRecycled => "every cause is emitted under one identity",
            Self::PlannedOutputOmitted => "a planned output is deleted",
            Self::UnplannedOutputAdded => "an unplanned output is appended",
            Self::ImplTargetAltered => "the implementation targets a different type",
            Self::ShapeAltered => "the declared body shape is changed",
            Self::OutputDuplicated => "a planned output is emitted twice",
            Self::TraitPathWrong => "the trait path names a different contract",
            Self::DecoyInComment => "the anchored bytes are planted in a comment",
            Self::MalformedRust => "the artifact stops being well-formed Rust",
        }
    }
}

/// Damage one lawful artifact.
///
/// Returns the mutated text, or `None` where the lawful artifact does not carry
/// what this mutation needs to damage. `None` is a real answer and is never
/// treated as "the mutation was applied and nothing happened": a test that
/// accepted it would be rehearsing against text it never changed.
#[must_use]
pub fn mutated(lawful: &str, mutation: ArtifactMutation) -> Option<String> {
    match mutation {
        ArtifactMutation::OrderPermuted => reversed_list(lawful),
        ArtifactMutation::IdentityRecycled => recycled_identities(lawful),
        ArtifactMutation::PlannedOutputOmitted => {
            let at = lawful.find("impl :: threadpak :: refusal :: CauseOrderDeclaration")?;
            lawful.get(..at).map(str::to_owned)
        }
        ArtifactMutation::UnplannedOutputAdded => Some(format!(
            "{lawful} impl :: threadpak :: refusal :: RefusalFamily for NobodyPlannedThis {{ }}"
        )),
        ArtifactMutation::ImplTargetAltered => {
            replaced_once(lawful, "for DemoFamily", "for SomeOtherType")
        }
        ArtifactMutation::ShapeAltered => replaced_once(
            lawful,
            "FamilyShape :: SingleCause",
            "FamilyShape :: IssueCollection",
        ),
        ArtifactMutation::OutputDuplicated => {
            let at = lawful.find("impl :: threadpak :: refusal :: CauseOrderDeclaration")?;
            let tail = lawful.get(at..)?;
            Some(format!("{lawful} {tail}"))
        }
        ArtifactMutation::TraitPathWrong => replaced_once(
            lawful,
            "refusal :: RefusalFamily",
            "refusal :: SomethingElse",
        ),
        ArtifactMutation::DecoyInComment => decoy_in_comment(lawful),
        ArtifactMutation::MalformedRust => replaced_once(lawful, "{", "{{{"),
    }
}

/// Reverse the quoted items of the first bracketed list.
fn reversed_list(lawful: &str) -> Option<String> {
    let open = lawful.find("= &")?;
    let bracket = lawful.get(open..)?.find('[')?.checked_add(open)?;
    let close = lawful.get(bracket..)?.find(']')?.checked_add(bracket)?;
    let inner = lawful.get(bracket.checked_add(1)?..close)?;
    let mut items: Vec<&str> = inner.split(',').map(str::trim).collect();
    items.reverse();
    let head = lawful.get(..bracket.checked_add(1)?)?;
    let tail = lawful.get(close..)?;
    Some(format!("{head}{}{tail}", items.join(", ")))
}

/// Emit every cause identity under the first one.
fn recycled_identities(lawful: &str) -> Option<String> {
    const OPENING: &str = "CauseId :: declared ( \"";
    let first_at = lawful.find(OPENING)?.checked_add(OPENING.len())?;
    let first_end = lawful.get(first_at..)?.find('"')?.checked_add(first_at)?;
    let first = lawful.get(first_at..first_end)?.to_owned();
    let mut rebuilt = String::new();
    let mut rest = lawful;
    while let Some(at) = rest.find(OPENING) {
        let opening_end = at.checked_add(OPENING.len())?;
        rebuilt.push_str(rest.get(..opening_end)?);
        rebuilt.push_str(&first);
        let after = rest.get(opening_end..)?;
        let end = after.find('"')?;
        rest = after.get(end..)?;
    }
    rebuilt.push_str(rest);
    Some(rebuilt)
}

/// Plant the anchored bytes inside a comment while damaging the real constant.
fn decoy_in_comment(lawful: &str) -> Option<String> {
    let damaged = reversed_list(lawful)?;
    let anchor_at = lawful.find("const SELECTION_ORDER")?;
    // The list's own bracket, not the slice type's: the anchored form carries
    // `[ & 'static str ]` before it, so the first `]` closes the type.
    let assignment = lawful
        .get(anchor_at..)?
        .find("= &")?
        .checked_add(anchor_at)?;
    let close = lawful
        .get(assignment..)?
        .find(']')?
        .checked_add(assignment)?;
    let decoy = lawful.get(anchor_at..=close)?;
    Some(format!("// {decoy} ;\n{damaged}"))
}

/// Replace the first occurrence of one form, or `None` where it is absent.
fn replaced_once(lawful: &str, from: &str, to: &str) -> Option<String> {
    let at = lawful.find(from)?;
    let head = lawful.get(..at)?;
    let tail = lawful.get(at.checked_add(from.len())?..)?;
    Some(format!("{head}{to}{tail}"))
}
