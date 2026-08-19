//! The codec home's declarative surface: the tables and trait implementations
//! this home states rather than computes.
//!
//! Four declarations stand here.
//!
//! The REFUSAL FAMILY's declared shape: an issue collection, because several
//! members may shadow the decode road's own bindings at once, and a caller
//! repairing a shape one member per attempt is a caller this home failed.
//!
//! The MEMBER CONTRACT: the exact roads a member's own type owes before a
//! rendered surface type-checks at a consumer's site, one row per wire shape. It
//! is a constant table over a closed roster rather than a sentence in a README,
//! so a reader can read the bill back and the compiler keeps the roster and the
//! bill the same length.
//!
//! The RESERVED BINDINGS: the locals the decode road declares for itself. Stated
//! once, so a caller reads which spellings are taken rather than discovering them
//! one refusal at a time, and so the binding pass and the rendering read one
//! roster.
//!
//! The DIRECTION TABLE: which of the two rendered roads each declared direction
//! covers. Stated as a constant answer over two closed rosters rather than as a
//! sentence, so "an encode-only codec renders no reader" is a value a reader can
//! read back and a match the compiler keeps exhaustive.

use super::{CodecComposition, CodecMemberShape, CodecRoad};
use crate::planning::CodecDirection;
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for CodecComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// One wire shape's bill: the roads the rendered surface calls on a member's own
/// type, spelled exactly as the emission writes them.
///
/// # Authority
///
/// **The bill is stated and never worked around.** A member the rendering could
/// not write end to end would be a member whose bytes nobody could re-read, so
/// the rendering does not degrade — it calls the roads named here and the
/// consumer's compiler answers. Where a road is absent the failure lands at the
/// consumer's site as an ordinary unresolved method, which is exactly where a
/// missing road on the consumer's own type belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberContract {
    /// The wire shape this row is about.
    pub shape: CodecMemberShape,
    /// The road the ENCODE surface calls to read the member out, or the empty
    /// spelling where the encode surface calls nothing at all.
    pub encode_road: &'static str,
    /// The road the DECODE surface calls to build the member back.
    pub decode_road: &'static str,
}

/// The complete member contract, one row per admitted wire shape, in the roster's
/// own order.
///
/// Five rows and no more, because the roster is five: a row added here without an
/// arm beside it, or an arm added without a row, is a length disagreement the
/// declaration itself carries.
pub const MEMBER_CONTRACT: [MemberContract; 5] = [
    MemberContract {
        shape: CodecMemberShape::Count,
        encode_road: "u64::from",
        decode_road: "<T as ::core::convert::TryFrom<u64>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::Bytes,
        encode_road: "<T as ::core::convert::AsRef<[u8]>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::vec::Vec<u8>>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::Text,
        encode_road: "<T as ::core::convert::AsRef<str>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::string::String>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::ClosedChoice,
        encode_road: "T::slot",
        decode_road: "T::ALL",
    },
    MemberContract {
        shape: CodecMemberShape::Nested,
        encode_road: "T::encode_canonical",
        decode_road: "T::decode_canonical",
    },
];

/// The locals the rendered decode road declares for itself.
///
/// # Authority
///
/// **A member whose spelling is one of these is refused rather than renamed.**
/// The decode road binds one local per member under the member's OWN spelling,
/// which is what makes the rendered road readable and what lets the assembly call
/// name its arguments the way the owner named its members. A member colliding
/// with one of these would shadow the rendering's own binding, and the road would
/// go on reading a value nobody meant — a defect that compiles.
///
/// Renaming the rendering's locals to something nobody would write is not the
/// repair: an unreadable rendered road is a road nobody can audit, and the
/// collision would still exist for whatever names were chosen instead.
pub const RESERVED_BINDINGS: [&str; 12] = [
    "material",
    "remaining",
    "into",
    "nested",
    "collected",
    "candidate",
    "chosen",
    "elected",
    "present",
    "carried",
    "length",
    "width",
];

/// Whether one declared direction covers one of the two rendered roads.
///
/// A constant answer over two closed rosters, so a third direction or a third
/// road admitted later stops the compiler here until somebody says which side of
/// this line it stands on.
///
/// # Nonclaims
///
/// A direction that does not cover [`CodecRoad::Decode`] delivers NO validator,
/// and that is a stated posture rather than a rendering that fell short: the
/// honest sentence "a codec that refuses on decode IS the validator" says exactly
/// as much about the codec that has no decode road.
#[must_use]
pub const fn covers(direction: CodecDirection, road: CodecRoad) -> bool {
    match road {
        CodecRoad::Encode => match direction {
            CodecDirection::Encode | CodecDirection::RoundTrip => true,
            CodecDirection::Decode => false,
        },
        CodecRoad::Decode => match direction {
            CodecDirection::Decode | CodecDirection::RoundTrip => true,
            CodecDirection::Encode => false,
        },
    }
}
