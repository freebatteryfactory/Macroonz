//! Lane C — compiled behaviour, over the mutants this plane produced.
//!
//! # The lane, and which half of it lives here
//!
//! Lane C's method is `rustc` compiling the artifact and the test reading its
//! trait constants back AS VALUES. The LAWFUL artifact's compiled seat is the
//! consumer fixtures under `xtask/fixtures/`, and it has to be: the question
//! "does a caller who holds the machine's types and wears the shell's derive
//! actually compile?" cannot be answered from inside either participant.
//!
//! A MUTANT has no such problem. The mutant is this plane's own damage, inflicted
//! on a lawful artifact by `mutated`; no participant is grading itself when the
//! judge hands its own damaged text to `rustc` and reads back what comes out. So
//! the two mutations the ownership ledger records as `CompiledBehaviour` get
//! their compiled seats here, and they were owed until now: the roster recorded
//! the claim and nothing compiled either mutant.
//!
//! # The two mechanisms, and why each is honest
//!
//! **`MalformedRust` — the mutant must FAIL to compile.** The materialized text
//! is checked in as a trybuild compile-fail fixture, under a header that states
//! its provenance, and the provenance is VERIFIED below rather than asserted: the
//! fixture must still be exactly what `mutated(lawful, MalformedRust)` produces.
//! A fixture written into the source tree by a running test would agree with
//! whatever the producer had just done, which is not evidence; a checked-in one
//! whose provenance is re-derived on every run is a text somebody can read and a
//! claim that can go stale loudly.
//!
//! **`ShapeAltered` — the mutant COMPILES and declares the wrong shape.** The
//! materialized text is checked in beside this file and included into the test
//! crate, so `rustc` compiles it exactly as it compiles any other item, and the
//! assertion below reads `SHAPE` back as a VALUE and compares it against the
//! shape this plane declares. Never against text: a text comparison here would
//! be lane A's method wearing lane C's name.
//!
//! Both directions, for both. The lawful artifact is materialized and compiled
//! the same way and reads back as declared — a lane that rejected everything
//! would catch both mutants and prove nothing.

use threadpak::refusal::{
    CauseId, CauseOrderDeclaration, DeclaredCauseOrder, FamilyShape, LocalCauseKey, RefusalFamily,
    RefusalFamilyId,
};
use threadpak_macroc::compile_refusal_text;
use threadpak_testpak::{ARTIFACT_MUTATIONS, ArtifactMutation, LaneOwnership, mutated};

/// The declaration handed to the services, stated here beside the values the
/// artifacts are held to.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// The body shape the declaration states.
const DECLARED_SHAPE: FamilyShape = FamilyShape::SingleCause;

/// The body shape the `ShapeAltered` mutant states instead.
const MUTATED_SHAPE: FamilyShape = FamilyShape::IssueCollection;

/// The declared spellings, in declared order.
const DECLARED_SPELLINGS: [&str; 3] = ["NotCanonical", "NotAdmitted", "Unbounded"];

/// The family every declared identity sits in.
const DECLARED_FAMILY: &str = "testpak.demo";

/// The declared local keys, in declared order. The family is stated once beside
/// them because the artifact states it once per row, and a lane that wrote the
/// joined name would be asserting over a value no compiled constant carries.
const DECLARED_LOCAL_KEYS: [&str; 3] = ["not-canonical", "not-admitted", "unbounded"];

/// The lawful artifact's compiled seat.
///
/// The enum is the declaration's own; everything after it is materialized text,
/// compiled by `rustc` as ordinary items.
mod lawful_artifact {
    /// The declared family. Visible to the whole test binary and no further:
    /// the seat is a compilation site for one materialized artifact, not an
    /// export.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum DemoFamily {
        /// A cause outside the machine's roster.
        NotAdmitted,
        /// A declared magnitude was exceeded.
        Unbounded,
        /// A key outside the canonical grammar.
        NotCanonical,
    }

    include!("compiled-mutant/lawful.rs");
}

/// The `ShapeAltered` mutant's compiled seat.
///
/// The same declaration, and an artifact that lies about one thing. It compiles,
/// which is exactly why the byte scan and the structural read are not the last
/// word: the disagreement is a VALUE, and only a compiler hands back values.
mod shape_altered_artifact {
    /// The declared family, unchanged — the mutation is in the artifact.
    /// Visible to the whole test binary and no further, exactly as the lawful
    /// seat's is.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum DemoFamily {
        /// A cause outside the machine's roster.
        NotAdmitted,
        /// A declared magnitude was exceeded.
        Unbounded,
        /// A key outside the canonical grammar.
        NotCanonical,
    }

    include!("compiled-mutant/shape-altered.rs");
}

/// The artifact one materialized fixture carries: its last line.
///
/// A materialized artifact is one line — the rendering's own projection carries
/// no newline — and the lines above it are the comment stating where the file
/// came from. Reading the last line rather than the whole file is what lets a
/// checked-in mutant explain itself without the explanation becoming part of
/// what is compared.
fn materialized_artifact(fixture: &str) -> &str {
    fixture.lines().last().unwrap_or_default()
}

/// The lawful rendering, as the receipt-rich road produced and closed over it.
fn lawful_rendering() -> String {
    compile_refusal_text(DECLARATION)
        .map(|(_, closed)| closed.inspected())
        .unwrap_or_default()
}

/// Whether one compiled declared order carries the declared identities and
/// spellings, in order, read back as values.
///
/// The identity each row must carry is BUILT here, out of the two seats this
/// file declares, and compared as a value. Nothing reads a name back out of the
/// compiled constant and nothing compares text: an identity comparison that went
/// through a rendering would be lane A's method wearing lane C's name.
fn order_reads_as_declared(order: DeclaredCauseOrder) -> bool {
    let family = RefusalFamilyId::declared(DECLARED_FAMILY);
    order.len() == DECLARED_LOCAL_KEYS.len()
        && order
            .iter()
            .zip(DECLARED_LOCAL_KEYS.iter().zip(DECLARED_SPELLINGS.iter()))
            .all(|(row, (local, spelling))| {
                row.id() == CauseId::declared(family, LocalCauseKey::declared(local))
                    && row.id().family() == family
                    && row.spelling() == *spelling
            })
}

/// The lawful artifact compiles, and every constant reads back as the
/// declaration states it.
///
/// The control lane C's mutants are measured against. Read as VALUES: `SHAPE` is
/// a `FamilyShape`, not the word `SingleCause` found in some bytes.
#[test]
fn the_lawful_artifact_compiles_and_reads_back_as_declared() {
    assert_eq!(
        <lawful_artifact::DemoFamily as RefusalFamily>::SHAPE,
        DECLARED_SHAPE
    );
    assert_eq!(
        <lawful_artifact::DemoFamily as RefusalFamily>::SELECTION_ORDER,
        DECLARED_SPELLINGS
    );
    assert!(order_reads_as_declared(
        <lawful_artifact::DemoFamily as CauseOrderDeclaration>::DECLARED_ORDER
    ));
}

/// The `ShapeAltered` mutant compiles and declares a shape the declaration did
/// not name.
///
/// The disagreement is a value disagreement and is asserted as one. Everything
/// else the artifact declares still reads back as declared, which is what makes
/// the finding exactly "the shape" rather than "something over there is wrong".
#[test]
fn the_shape_altered_mutant_compiles_and_declares_another_shape() {
    let read = <shape_altered_artifact::DemoFamily as RefusalFamily>::SHAPE;
    assert_ne!(read, DECLARED_SHAPE);
    assert_eq!(read, MUTATED_SHAPE);
    assert_eq!(
        <shape_altered_artifact::DemoFamily as RefusalFamily>::SELECTION_ORDER,
        DECLARED_SPELLINGS
    );
    assert!(order_reads_as_declared(
        <shape_altered_artifact::DemoFamily as CauseOrderDeclaration>::DECLARED_ORDER
    ));
}

/// Every materialized artifact in this lane is still exactly what this plane's
/// own mutation produces.
///
/// The provenance check, and the reason a checked-in mutant is honest. Where a
/// renderer legitimately changes shape the fixtures go stale and this test says
/// so, by name, in one place — the same discipline lane A's anchor stands under.
#[test]
fn the_materialized_artifacts_are_this_planes_own_output() {
    let lawful = lawful_rendering();
    assert!(!lawful.is_empty());
    assert_eq!(
        materialized_artifact(include_str!("compiled-mutant/lawful.rs")),
        lawful.trim_end()
    );

    let shape_altered = mutated(&lawful, ArtifactMutation::ShapeAltered).unwrap_or_default();
    assert!(!shape_altered.is_empty() && shape_altered != lawful);
    assert_eq!(
        materialized_artifact(include_str!("compiled-mutant/shape-altered.rs")),
        shape_altered.trim_end()
    );

    let malformed = mutated(&lawful, ArtifactMutation::MalformedRust).unwrap_or_default();
    assert!(!malformed.is_empty() && malformed != lawful);
    assert!(
        include_str!("compile-fail/a-materialized-malformed-mutant.rs")
            .contains(malformed.trim_end()),
        "the compile-fail fixture is no longer the mutant it says it is"
    );
}

/// Lane C owns exactly the two mutations that have a compiled seat here.
///
/// The ledger and the evidence are held to each other in one assertion. A third
/// mutation recorded as `CompiledBehaviour` without a compiled seat would fail
/// here rather than sitting in the roster looking like coverage.
#[test]
fn lane_c_owns_exactly_the_mutations_with_a_compiled_seat() {
    let owned: Vec<ArtifactMutation> = ARTIFACT_MUTATIONS
        .into_iter()
        .filter(|mutation| mutation.owned_by() == LaneOwnership::CompiledBehaviour)
        .collect();
    assert_eq!(
        owned,
        vec![
            ArtifactMutation::ShapeAltered,
            ArtifactMutation::MalformedRust
        ]
    );
}

/// Both materialized families are still ordinary Rust enums: the artifact added
/// declared facts and took nothing away.
#[test]
fn the_materialized_families_are_still_ordinary_enums() {
    let lawful = [
        lawful_artifact::DemoFamily::NotCanonical,
        lawful_artifact::DemoFamily::NotAdmitted,
        lawful_artifact::DemoFamily::Unbounded,
    ];
    let altered = [
        shape_altered_artifact::DemoFamily::NotCanonical,
        shape_altered_artifact::DemoFamily::NotAdmitted,
        shape_altered_artifact::DemoFamily::Unbounded,
    ];
    assert_eq!(lawful.len(), altered.len());
    assert_ne!(lawful.first(), lawful.get(1));
    assert_ne!(altered.first(), altered.get(1));
}
