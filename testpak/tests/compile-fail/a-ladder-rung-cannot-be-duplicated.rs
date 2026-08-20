//! The image ladder's affinity reversal: no rung on the ladder is duplicable.
//!
//! Band 16 declares the WHOLE ladder affine — five rungs, and "each transition
//! is a sealed constructor consuming `self` and returning the stronger type or a
//! typed refusal". A duplication trait on any one of them contradicts that
//! sentence at the declaration: the caller hands the weaker rung to the
//! transition and still holds one afterwards, so the two rungs stand side by
//! side and the ladder is a suggestion.
//!
//! There are two ways to still hold one and five rungs to hold, so the claim is
//! TEN facts and this fixture asks all ten. `Copy` duplicates silently at the
//! call site; `Clone` duplicates on request, which is no better for a typestate
//! whose entire content is that the weaker state is gone.
//!
//! The reversal this one supersedes moved `ExecutableImage` twice, and moving
//! asks exactly one of the ten. `Clone` restored on that same rung, or either
//! trait restored on any of the other four, left it failing for its original
//! reason — so the gate stayed green, the full-ladder claim read as discharged,
//! and nine of the ten facts had nothing standing behind them.
//!
//! The BOUND is the reading here, because the bound is the claim.
//! `duplicated_at_the_call_site` is satisfied by a `Copy` type and by no other,
//! `duplicated_on_request` by a `Clone` type and by no other, so each rung's two
//! lines refuse exactly while that rung carries neither trait and stop refusing
//! the moment it carries one. Nothing is constructed to ask it: the ladder has
//! no public constructor, and a bound needs no value.
//!
//! # Why the move is not asked here as well
//!
//! Moving a rung twice was tried in this file and taken back out. `E0382` is a
//! BORROW-check finding, and the borrow checker does not run on a body whose
//! type check already failed — so behind ten unsatisfied bounds the move road
//! emitted nothing, in this tree and in every tree where a rung is still
//! affine. A road that can only speak once the fixture has stopped working is a
//! road that proves nothing, which is the defect this file was written to
//! repair. What the move road demonstrated — that `ExecutableImage` is not
//! `Copy` — is the first of the ten bounds below, stated where the compiler
//! answers it.

fn duplicated_at_the_call_site<Rung: Copy>() {}

fn duplicated_on_request<Rung: Clone>() {}

fn main() {
    duplicated_at_the_call_site::<threadpak::image::UntrustedImageBytes>();
    duplicated_on_request::<threadpak::image::UntrustedImageBytes>();

    duplicated_at_the_call_site::<threadpak::image::BoundedDecodedImage>();
    duplicated_on_request::<threadpak::image::BoundedDecodedImage>();

    duplicated_at_the_call_site::<threadpak::image::SemanticImage>();
    duplicated_on_request::<threadpak::image::SemanticImage>();

    duplicated_at_the_call_site::<threadpak::image::AgreementCheckedImage>();
    duplicated_on_request::<threadpak::image::AgreementCheckedImage>();

    duplicated_at_the_call_site::<threadpak::image::ExecutableImage>();
    duplicated_on_request::<threadpak::image::ExecutableImage>();
}
