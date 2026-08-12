//! The damage: how a judge makes a lawful artifact lie.
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
//! So the road is gone from the services and the damage lives here. testpak
//! takes a LAWFUL artifact — one the receipt-rich road produced and closed over
//! — and damages it itself.
//!
//! # This file cuts; it does not decide
//!
//! Every function below is string surgery over one lawful text, and each one
//! returns `None` rather than a text it did not manage to damage. What the
//! roster IS is declared in `types.rs`, and which lane owns catching each
//! damage is the closed table in `type_contract.rs`. Keeping the cutting apart
//! from the ledger is what stops a mutation from being quietly re-owned by
//! whichever lane happened to notice it.

use super::types::ArtifactMutation;

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
        ArtifactMutation::ImplMemberDuplicated => duplicated_member(lawful),
        ArtifactMutation::ImplMemberUnexpected => replaced_once(
            lawful,
            "; } impl",
            "; fn nobody_planned_this ( ) { } } impl",
        ),
        ArtifactMutation::ConstructorPathAltered => replaced_once(
            lawful,
            "DeclaredCause :: declared",
            "DeclaredCause :: adopted",
        ),
        ArtifactMutation::ImplPostureAltered => {
            replaced_once(lawful, "impl :: threadpak", "unsafe impl :: threadpak")
        }
        ArtifactMutation::MeaningBearingAttributeAdded => Some(format!(
            "#[cfg(feature = \"nobody-declared-this\")] {lawful}"
        )),
        ArtifactMutation::MalformedRust => replaced_once(lawful, "{", "{{{"),
    }
}

/// Emit the first member constant a second time, immediately after itself.
///
/// The copy is byte-identical, which is the point: a reader that filed each
/// named constant into one seat would write the second reading over the first
/// and report nothing at all.
fn duplicated_member(lawful: &str) -> Option<String> {
    const OPENING: &str = "const SHAPE";
    let at = lawful.find(OPENING)?;
    let end = lawful
        .get(at..)?
        .find(';')?
        .checked_add(at)?
        .checked_add(1)?;
    let member = lawful.get(at..end)?;
    let head = lawful.get(..end)?;
    let tail = lawful.get(end..)?;
    Some(format!("{head} {member}{tail}"))
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
