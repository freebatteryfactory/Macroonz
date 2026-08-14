//! The production scope guards' laundering reversal: the guards the machine
//! actually ships seal their position, not merely the demo roles a stamp is
//! rehearsed on.
//!
//! `a-stamped-representation-cannot-be-laundered.rs` proves the stamped SHAPE
//! keeps its seat private, on two roles this fixture's own module stamps. That
//! leaves one thing unproven, and it is the thing that matters: a production
//! guard is only sealed if it IS that shape. This fixture asks the question of a
//! guard the machine declares — `FrameVersion`, band 11's version of a reference
//! frame — from outside the crate that declares it, where no module-privacy help
//! exists in either direction.
//!
//! Both halves of the laundering are attempted, each on its own road. Taking the
//! position OUT of the role is `version.0`; putting one back IN under the role
//! is the tuple form `FrameVersion(position)`. Each refuses on its own, and
//! neither refusal depends on the other.
//!
//! # What this file establishes, exactly
//!
//! REPRESENTATION PRIVACY, and nothing wider: the seat is not a field an outside
//! caller can read and the tuple constructor is not one it can call. It does NOT
//! establish that the guard has no road out. Add a public `position()` or
//! `into_position()` to the stamp and both attempts below emit the same two
//! errors they emit today, byte for byte — the field is still private and the
//! constructor is still unreachable — so this file would keep passing while the
//! sealed position walked out through a road with a name. That was executed
//! rather than reasoned about: the accessor was added, this fixture stayed
//! green, and the whole compile-refusal suite with it.
//!
//! *No road out exists* is not a sentence Rust can be asked to refuse, so it is
//! not asked here. `cargo xtask check`'s `stamped-guards-seal-their-position`
//! derives it — over every type the stamp is invoked for, and over the stamp's
//! own transcriber, where one accessor would unseal all of them at once. The
//! count that gate prints is the population; no number stands in this file,
//! because a count kept beside a derivation is a second thing to keep true.
//!
//! No value is constructed and none could be: `AuthorityPosition::assigned` is
//! the authority-side mint and `ReferenceFrameId` has no outside road either, so
//! the signatures and the field access alone are the proof.

fn main() {
    let out_of_the_role: fn(
        threadpak::navigation::FrameVersion,
    ) -> threadpak::identity::AuthorityPosition<
        threadpak::navigation::ReferenceFrameId,
    > = |version| version.0;

    let into_the_role: fn(
        threadpak::identity::AuthorityPosition<threadpak::navigation::ReferenceFrameId>,
    ) -> threadpak::navigation::FrameVersion =
        |position| threadpak::navigation::FrameVersion(position);

    let _ = (out_of_the_role, into_the_role);
}
