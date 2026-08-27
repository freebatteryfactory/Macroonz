//! The expansion home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the home's central claim structural rather than reviewed.
//! An expansion is bound here, after the three values agreed about their parentage, and it is the only value in the crate that hands a closure's proved deliveries out.
//! An account is built here too, and it accepts only the kind home's complete-set witness so a consumer-owned record cannot smuggle silence past the accounting boundary.

use super::{Accounted, BindError, Expansion};
use crate::closure::{Closure, PartitionCargo, PartitionedEmission};
use crate::explanation::View;
use crate::identity::{self, ClosedExpansionId, Provenance, Transcript, encode_bytes};
use crate::kind::{Destination, DispositionSet, Kind, KindSet};
use crate::plan::Plan;
use crate::render::RenderedUnit;

impl<K: Kind> Expansion<K> {
    /// Bind one expansion: the plan, the closure proved against it, and the explanation answered over the two.
    ///
    /// The road every kind's request terminates at.
    /// A caller that walked the steps arrives with three unforgeable values and leaves with the one account emission is reachable from; a caller that skipped a step has nothing to hand in.
    ///
    /// # Construction
    ///
    /// The identity is derived at [`Role::ClosedExpansion`](crate::identity::Role::ClosedExpansion), anchored on the CLOSURE — an expansion exists only where a closure does — over a content transcript of exactly two members: the plan's identity, then the explanation's.
    ///
    /// Nothing else enters, and each absence is the no-double-entry law.
    /// The deliveries are inside the anchor, because a closure's identity commits to their digests; the kind and the account are inside member one, because a plan's identity commits to its intent.
    /// A second spelling of either here would write one fact twice and let the two spellings drift.
    ///
    /// # Errors
    ///
    /// Returns [`BindError`] naming the pair that disagreed and both of its identities.
    /// Nothing is elected out of any pair: an expansion naming one plan while carrying another's proof, or another's explanation, would answer every question correctly about the wrong expansion.
    pub fn bound(
        plan: Plan<K>,
        closure: Closure<K::Role>,
        explanation: View<K>,
    ) -> Result<Self, BindError> {
        let planned = plan.identity();
        let proved = closure.plan();
        if planned != proved {
            return Err(BindError::ClosureProvedAgainstAnotherPlan { planned, proved });
        }
        let answered_over_plan = explanation.plan();
        if planned != answered_over_plan {
            return Err(BindError::ExplanationAnsweredOverAnotherPlan {
                planned,
                answered: answered_over_plan,
            });
        }
        let anchor = closure.identity();
        let answered_over_closure = explanation.closure();
        if anchor != answered_over_closure {
            return Err(BindError::ExplanationAnsweredOverAnotherClosure {
                proved: anchor,
                answered: answered_over_closure,
            });
        }
        let mut content = Vec::new();
        encode_bytes(planned.as_bytes(), &mut content);
        encode_bytes(explanation.identity().as_bytes(), &mut content);
        let (derived, provenance) = ClosedExpansionId::derived_with_provenance(
            Transcript::under_projection(identity::Role::ClosedExpansion, &anchor, &content, 0),
        );
        Ok(Self {
            identity: derived,
            provenance,
            plan,
            closure,
            explanation,
        })
    }

    /// This expansion's own identity: the name of the whole account.
    #[must_use]
    pub const fn identity(&self) -> ClosedExpansionId {
        self.identity
    }

    /// The record of how that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }

    /// The complete plan: account, context, content, membership, watch set, trace, trail, and nonclaims.
    pub const fn plan(&self) -> &Plan<K> {
        &self.plan
    }

    /// The proof that what was rendered is what was planned.
    pub const fn closure(&self) -> &Closure<K::Role> {
        &self.closure
    }

    /// Every question this kind owes, answered over that plan and that proof.
    pub const fn explain(&self) -> &View<K> {
        &self.explanation
    }

    /// The deliveries this expansion carries, split by destination.
    ///
    /// The closure's own proved value, borrowed rather than copied: this expansion keeps no second emission, so what is delivered is what was proved and there is no pair of values to drift apart.
    pub const fn emission(&self) -> &PartitionedEmission {
        self.closure.emission()
    }

    /// What the declaration site expands into — the only tokens the consumer's normal build compiles.
    pub const fn emit(&self) -> &PartitionCargo {
        self.emission().declaration_site()
    }

    /// The deferred cargo the consumer's test target invokes.
    pub const fn test_carrier(&self) -> &PartitionCargo {
        self.emission().test_carrier()
    }

    /// The deferred cargo the consumer's bench target invokes.
    pub const fn bench_carrier(&self) -> &PartitionCargo {
        self.emission().bench_carrier()
    }

    /// Every unit this expansion publishes as a standalone artifact, in seat order, each carrying the address its own planned output names.
    ///
    /// Read off the proved rendering rather than copied into a record beside it: a published artifact IS its rendered unit at an address, and a second value restating that unit's tree, digest, and seat would be a second answer to one question.
    pub fn published(&self) -> impl Iterator<Item = &RenderedUnit<K::Role>> {
        self.closure
            .rendered()
            .units_to(Destination::PublicationArtifact)
    }
}

impl<K: Kind, Set: KindSet> Accounted<K, Set> {
    /// Seat one door's complete disposition witness beside the expansion that door produced.
    ///
    /// Public, because a door is the consumer's: a crate-internal road here would mean only this crate could ever answer for a set of kinds, and this compiler declares none.
    /// Which row says generated remains the caller's claim — which is exactly what a door decides — while [`DispositionSet`] makes omission structurally unavailable here.
    pub const fn seated(expansion: Expansion<K>, dispositions: DispositionSet<Set>) -> Self {
        Self {
            expansion,
            dispositions,
        }
    }

    /// What this door produced, whole.
    ///
    /// Read through, never restated: what it planned, what it proved, what it explains, and what each build receives are that value's own answers.
    pub const fn expansion(&self) -> &Expansion<K> {
        &self.expansion
    }

    /// What happened to every kind of the set, whole and declaration-ordered.
    ///
    /// The witness pairs each declared name with the disposition surrendered at that position only after every surrendered name and the whole row count matched the complete set.
    pub const fn dispositions(&self) -> &DispositionSet<Set> {
        &self.dispositions
    }
}
