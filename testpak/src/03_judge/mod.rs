//! Seat 03 — judge: the readers that state a verdict over one rendered
//! artifact, and the mutations they are rehearsed against.
//!
//! # What a judge here may read, and what it may not
//!
//! A judge reads an ARTIFACT — the rendered text a service produced — and
//! compares it against a declared order the caller states independently. It
//! never asks the service under judgement what the answer was, because a
//! comparison between a value and itself proves that the value equals itself.
//!
//! # Three lanes, and a verdict belongs to exactly one of them
//!
//! **Lane A — the byte-profile scan** (`byte_profile.rs`). It finds one declared
//! textual form in the rendered text and reports what it found, and that is a
//! claim about BYTES: nothing about what item carries them, and nothing that
//! more anchors would turn into structure.
//!
//! **Lane B — the structural read** (`structural.rs`). What item is this, what
//! does the implementation target, which trait does it realize, what are its
//! members, and are the cause rows the declared ones. Answering that means
//! parsing Rust, which the byte scan deliberately does not do — so the lane
//! hands the text to a parser nobody here wrote and reads the tree back. Its
//! dependency is admitted in this package's README, which states what the lane
//! reads, what it refuses to claim, and which producer components it shares
//! nothing with.
//!
//! **Lane C — the compiled behaviour.** `rustc` compiles the artifact and the
//! test reads its trait constants AS VALUES. There the compiler is the
//! independent decoder: it parses by its own rules, with no anchor of ours in
//! the path, and hands back typed values rather than substrings.
//!
//! The LAWFUL artifact's seat is the consumer-fixture parity tests at
//! `xtask/fixtures/macro-consumer` and `xtask/fixtures/renamed-consumer`, which
//! apply the shell's derive in crates owning neither participant and compare the
//! derived `SHAPE`, `SELECTION_ORDER`, and `DECLARED_ORDER` against hand-written
//! twins. The MUTANTS' seats are this package's `tests/compiled_behaviour.rs`: a
//! mutant is this plane's own damage, so no participant is grading itself when
//! the judge hands its own damaged text to a compiler and reads back a refusal
//! to compile and a disagreeing value.
//!
//! No lane subsumes another and none is a weaker version of another. **A verdict
//! is method-specific**, exactly as the machine's evidence law requires:
//! "the permuted rendering was rejected by the byte scan over these two declared
//! orders" and "the derived implementation equals its hand-written twin under
//! compilation" are two claims, each true of its own method and neither standing
//! in for the other. Reporting one as though it came from another is the
//! collapse the whole plane exists to refuse.
//!
//! # The readers are dumb on purpose, and the reason is not "simplicity"
//!
//! A cleverer reader would have to decide what the text MEANS, and the only way
//! to decide that is to implement the same understanding the renderer already
//! has. Two implementations of one understanding, written by the same hands
//! against the same document, agree because they SHARE THE CHALLENGED
//! IMPLEMENTATION — not because either of them understands Rust. Their agreement
//! is therefore correlated evidence, and correlated evidence about a renderer is
//! not independent of that renderer. Lane C escapes this because `rustc` is a
//! decoder nobody here wrote.
//!
//! # The seats
//!
//! `types.rs` declares everything this seat can say — both verdicts, everything
//! lane B recovers, what a caller declares against, and the mutation roster —
//! and `type_contract.rs` states that roster's closed tables, so a lane's
//! ownership is read in one place rather than inferred from whichever reader
//! noticed a damage. `byte_profile.rs`, `structural.rs`, and `mutation.rs` are
//! the three operations: scan, parse, cut.

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
