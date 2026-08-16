//! Seat 03 — judge: the readers that state a verdict over one rendered
//! artifact, and the mutations they are rehearsed against.
//!
//! A judge reads an ARTIFACT — the rendered text a service produced — and
//! compares it against a declared order the caller states independently.
//! It never asks the service under judgement what the answer was, because a
//! comparison between a value and itself proves that the value equals itself.
//!
//! # Three lanes, and a verdict belongs to exactly one of them
//!
//! Lane A — the byte-profile scan (`byte_profile.rs`). It finds one declared
//! textual form in the rendered text and reports what it found, and that is a
//! claim about BYTES: nothing about what item carries them, and nothing that
//! more anchors would turn into structure.
//!
//! Lane B — the structural read (`structural.rs`). What item is this, what does
//! the implementation target, which trait does it realize, what are its
//! members, and are the cause rows the declared ones. Answering that means
//! parsing Rust, which the byte scan deliberately does not do, so the lane
//! hands the text to a parser nobody here wrote and reads the tree back. The
//! package README admits that dependency.
//!
//! Lane C — the compiled behaviour. `rustc` compiles the artifact and the test
//! reads its trait constants AS VALUES: the compiler parses by its own rules,
//! with no anchor of ours in the path, and hands back typed values rather than
//! substrings. The compiled seat is this package's
//! `tests/compiled_behaviour.rs`, where a lawful control and the two mutations
//! recorded against this lane are handed to `rustc`. A mutant is this plane's
//! own damage, so no participant is grading itself when the judge hands its own
//! damaged text to a compiler.
//!
//! No lane subsumes another and none is a weaker version of another. A verdict
//! is method-specific, exactly as the machine's evidence law requires: "the
//! permuted rendering was rejected by the byte scan over these two declared
//! orders" and "the compiled implementation reads back the declared shape" are
//! two claims, each true of its own method and neither standing in for the
//! other. Reporting one as though it came from another is the collapse the
//! whole plane exists to refuse.
//!
//! # The readers are dumb on purpose
//!
//! A cleverer reader would have to decide what the text MEANS, and the only way
//! to decide that is to implement the same understanding the renderer already
//! has. Two implementations of one understanding, written by the same hands
//! against the same document, agree because they SHARE THE CHALLENGED
//! IMPLEMENTATION rather than because either understands Rust — correlated
//! evidence about a renderer, not evidence independent of it. Lanes B and C
//! escape this because `syn` and `rustc` are decoders nobody here wrote.
//!
//! # The seats
//!
//! `types.rs` declares everything this seat can say, and `type_contract.rs`
//! states which lane owns catching each mutation, so ownership is read in one
//! place rather than inferred from whichever reader noticed a damage.
//! `byte_profile.rs`, `structural.rs`, and `mutation.rs` are the three
//! operations: scan, parse, cut.

pub mod byte_profile;
pub mod mutation;
pub mod structural;
mod type_contract;
pub mod types;

pub use byte_profile::{cause_identities_in, judge_declared_order, selection_order_in};
pub use mutation::mutated;
pub use structural::{judge_structure, structure_of};
pub use types::{
    ARTIFACT_MUTATIONS, ArtifactMutation, ArtifactStructure, CauseRow, DeclaredStructure,
    ImplPosture, ImplementationStructure, LaneOwnership, RenderVerdict, StructuralDisagreement,
    StructuralVerdict,
};
