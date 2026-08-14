//! Band 11's cross-frame reversal: two frames' versions have no order to ask
//! for.
//!
//! `FrameVersion` is the Class-C guard that RIDES its authority position —
//! versions of one frame compare, versions of different frames refuse with the
//! scope-guard family body. A frame is a VALUE inside that position and not a
//! type parameter, so two frames' versions are ONE Rust type and the compiler is
//! never the thing that tells them apart. What the compiler does instead is
//! refuse every direct comparison, which leaves exactly one road to an ordering:
//! `try_cmp_same_scope`, which reads both scopes and answers
//! `OrderComparison::NotSameScope` when they differ.
//!
//! That refusal is the whole load the stamp's derive set carries. It emits
//! `Debug, Clone, PartialEq, Eq, Hash` and nothing else. Restore `PartialOrd`
//! and `left < right` answers a cross-frame question with a `bool`; restore
//! `Ord` and `left.cmp(right)` answers it with an `Ordering`. Neither answer is
//! ever refused, because neither one looked at a scope — and the law that drives
//! `try_cmp_same_scope` through both outcomes goes on passing while it happens,
//! which is why this file and not that law is what stands between the claim and
//! a silent cross-frame order.
//!
//! Both traits are attempted, each on its own road, and neither refusal depends
//! on the other. The two arguments are two frames' versions: the fixture cannot
//! say so in the types, because the claim is precisely that the types do not
//! carry it, and it does not need to — a comparison refused for every pair is
//! refused for this one.
//!
//! No value is constructed and none could be: `AuthorityPosition::assigned` is
//! the authority-side mint, so the signatures alone are the proof.

fn main() {
    let ordered: fn(
        &threadpak::navigation::FrameVersion,
        &threadpak::navigation::FrameVersion,
    ) -> bool = |one_frames_version, another_frames_version| {
        one_frames_version < another_frames_version
    };

    let ranked: fn(
        &threadpak::navigation::FrameVersion,
        &threadpak::navigation::FrameVersion,
    ) -> ::core::cmp::Ordering = |one_frames_version, another_frames_version| {
        one_frames_version.cmp(another_frames_version)
    };

    let _ = (ordered, ranked);
}
