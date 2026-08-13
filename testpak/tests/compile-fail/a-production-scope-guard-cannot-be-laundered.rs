//! The production scope guards' laundering reversal: the nine guards the machine
//! actually ships seal their position, not merely the demo roles a stamp is
//! rehearsed on.
//!
//! `a-stamped-representation-cannot-be-laundered.rs` proves the stamped SHAPE
//! emits one road in and none out, on two roles this fixture's own module
//! stamps. That leaves one thing unproven, and it is the thing that matters: a
//! production guard is only sealed if it IS that shape. This fixture asks the
//! question of a guard the machine declares — `FrameVersion`, band 11's version
//! of a reference frame — from outside the crate that declares it, where no
//! module-privacy help exists in either direction.
//!
//! Both halves of the laundering are attempted, each on its own road. Taking the
//! position OUT of the role is `version.0`; putting one back IN under the role
//! is the tuple form `FrameVersion(position)`. Each refuses on its own, and
//! neither refusal depends on the other.
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
