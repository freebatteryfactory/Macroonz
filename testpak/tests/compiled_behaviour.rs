//! The compiled read-back: `rustc` compiles the artifact and the test reads its
//! trait constants back AS VALUES.
//!
//! # Seats
//!
//! The damaged seats are this harness's own, and they have to be. The services
//! carry no road that renders a defective artifact — a producer that writes its
//! own exam is rehearsed only against the defects it already imagined — so a
//! damaged artifact here is damage this plane inflicted on a lawful rendering,
//! and no participant is grading itself when that text goes to a compiler.
//!
//! The lawful compiled seat below is deliberate too: it is the control the
//! damaged ones are measured against, held inside testpak on purpose.
//! Outside-consumer parity — a crate that owns neither participant applying the
//! derive — is a separate seat, and it is absent.
//!
//! # Mechanisms
//!
//! **The malformed artifact must FAIL to compile.** Its materialized text is
//! checked in as a trybuild compile-fail fixture under a header that states
//! where it came from. It is checked in rather than written by a running test on
//! purpose: a fixture a test writes into the source tree agrees with whatever
//! the producer had just done, which is not evidence.
//!
//! **The shape-altered artifact COMPILES and declares the wrong shape.** Its
//! materialized text is checked in beside this file and included into the test
//! crate, so `rustc` compiles it exactly as it compiles any other item, and the
//! assertion below reads `SHAPE` back as a VALUE and compares it against the
//! shape this plane declares. Never against text: a text comparison here would
//! be a scan over bytes wearing this lane's name.
//!
//! Both directions. The lawful artifact is materialized and compiled the same
//! way and reads back as declared — a lane that rejected everything would catch
//! both damaged artifacts and prove nothing.
//!
//! # What holds the fixtures to the renderer, and what does not
//!
//! The LAWFUL fixture's provenance is re-derived below on every run: it must
//! still be, byte for byte, what the receipt-rich road produces from the
//! declaration stated here. That is what makes renderer drift loud — when the
//! rendering legitimately changes shape, this file says so by name.
//!
//! The two DAMAGED fixtures are not re-derived, and saying so is the honest
//! state rather than a gap. The road that damaged them was the retired judge
//! seat's string surgery, which cut against spellings a hand restated beside
//! the renderer's own output; it left the tree with that seat rather than being
//! copied here. What still holds the shape-altered fixture is the pair of
//! value-level readings below — it agrees with the lawful artifact in every
//! constant but the one it lies about — and the lawful control's own freshness:
//! a renderer that moves fails the control loudly, and a stale mutant beside a
//! failing control is never read as evidence. The malformed fixture carries
//! less than that: it proves a compiler refuses that text, and nothing here
//! ties it to today's renderer. Full custody returns when the generator owns
//! materialization and publishes each fixture with its receipt.

use threadpak::refusal::{
    CauseId, CauseOrderDeclaration, DeclaredCauseOrder, FamilyShape, LocalCauseKey, RefusalFamily,
    RefusalFamilyId,
};
use threadpak_macroc::compile_refusal_text;

/// The declaration handed to the services, stated here beside the values the
/// artifacts are held to.
const DECLARATION: &str = "#[refusal(family = \"testpak.demo\", shape = single_cause, \
    order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\", \
    Unbounded = \"unbounded\"))] enum DemoFamily { NotAdmitted, Unbounded, NotCanonical, }";

/// The body shape the declaration states.
const DECLARED_SHAPE: FamilyShape = FamilyShape::SingleCause;

/// The body shape the shape-altered artifact states instead.
const DAMAGED_SHAPE: FamilyShape = FamilyShape::IssueCollection;

/// The declared spellings, in declared order.
const DECLARED_SPELLINGS: [&str; 3] = ["NotCanonical", "NotAdmitted", "Unbounded"];

/// The family every declared identity sits in.
const DECLARED_FAMILY: &str = "testpak.demo";

/// The declared local keys, in declared order.
///
/// The family is stated once beside them because the artifact states it once per
/// row, and a lane that wrote the joined name would be asserting over a value no
/// compiled constant carries.
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

/// The shape-altered artifact's compiled seat.
///
/// The same declaration, and an artifact that lies about one thing. It compiles,
/// which is exactly why a structural read is not the last word: the
/// disagreement is a VALUE, and only a compiler hands back values.
mod shape_altered_artifact {
    /// The declared family, unchanged — the damage is in the artifact.
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
/// checked-in artifact explain itself without the explanation becoming part of
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
/// through a rendering would be a scan over bytes wearing this lane's name.
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
/// The control the damaged artifacts are measured against. Read as VALUES:
/// `SHAPE` is a `FamilyShape`, not the word `SingleCause` found in some bytes.
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

/// The shape-altered artifact compiles and declares a shape the declaration did
/// not name.
///
/// The disagreement is a value disagreement and is asserted as one. Everything
/// else the artifact declares still reads back as declared, which is what makes
/// the finding exactly "the shape" rather than "something over there is wrong"
/// — and it is also what holds this fixture to the lawful one it was cut from.
#[test]
fn the_shape_altered_artifact_compiles_and_declares_another_shape() {
    let read = <shape_altered_artifact::DemoFamily as RefusalFamily>::SHAPE;
    assert_ne!(read, DECLARED_SHAPE);
    assert_eq!(read, DAMAGED_SHAPE);
    assert_eq!(
        <shape_altered_artifact::DemoFamily as RefusalFamily>::SELECTION_ORDER,
        DECLARED_SPELLINGS
    );
    assert!(order_reads_as_declared(
        <shape_altered_artifact::DemoFamily as CauseOrderDeclaration>::DECLARED_ORDER
    ));
}

/// The lawful materialized artifact is still exactly what the road produces.
///
/// The freshness check, and the reason a checked-in artifact is honest. Where
/// the renderer legitimately changes shape this fixture goes stale and this test
/// says so, by name, in one place.
///
/// It is the only fixture in this lane whose provenance is re-derived; what
/// stands behind the two damaged ones, and what does not, is this file's page.
#[test]
fn the_lawful_materialized_artifact_is_the_roads_own_output() {
    let lawful = lawful_rendering();
    assert!(
        !lawful.is_empty(),
        "the lawful artifact did not compile through the receipt-rich road"
    );
    assert_eq!(
        materialized_artifact(include_str!("compiled-mutant/lawful.rs")),
        lawful.trim_end()
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
