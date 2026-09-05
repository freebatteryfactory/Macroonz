//! The codec home's authored bill and reserved decode bindings.
//!
//! The public bill and both generated roads read one rendering row, so a wire shape cannot move one without moving all three.

use super::spell::{
    CANDIDATE_BINDING, CARRIED_BINDING, CHOSEN_BINDING, COLLECTED_BINDING, ELECTED_BINDING,
    INTO_BINDING, LENGTH_BINDING, MATERIAL_BINDING, NESTED_BINDING, PRESENT_BINDING,
    REMAINING_BINDING, WIDTH_BINDING,
};
use super::types::{ReadRoad, RenderingContract, WriteRoad};
use super::{
    CodecMemberShape, DECODE_ROAD, ENCODE_ROAD, MemberContract, ROSTER_CONSTANT, SLOT_ROAD,
};

/// The complete bill, one row per wire shape, in the roster's own order.
///
/// Five rows because the roster is five: a row added here without an arm beside it, or an arm without a row, is a length disagreement the declaration itself carries.
///
/// The closed-choice row is this compiler's own contract on a caller's roster — a complete roster constant and a position road answering one byte — and not an inheritance from any stamp that happens to emit one.
pub const MEMBER_CONTRACT: [MemberContract; 5] = [
    COUNT_CONTRACT.bill,
    BYTES_CONTRACT.bill,
    TEXT_CONTRACT.bill,
    CLOSED_CHOICE_CONTRACT.bill,
    NESTED_CONTRACT.bill,
];

const COUNT_CONTRACT: RenderingContract = RenderingContract {
    bill: MemberContract {
        shape: CodecMemberShape::Count,
        encode_road: "u64::from",
        decode_road: "<T as ::core::convert::TryFrom<u64>>::try_from",
    },
    write: WriteRoad::Count,
    read: ReadRoad::Count,
};

const BYTES_CONTRACT: RenderingContract = RenderingContract {
    bill: MemberContract {
        shape: CodecMemberShape::Bytes,
        encode_road: "<T as ::core::convert::AsRef<[u8]>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::vec::Vec<u8>>>::try_from",
    },
    write: WriteRoad::Bytes,
    read: ReadRoad::Bytes,
};

const TEXT_CONTRACT: RenderingContract = RenderingContract {
    bill: MemberContract {
        shape: CodecMemberShape::Text,
        encode_road: "<T as ::core::convert::AsRef<str>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::string::String>>::try_from",
    },
    write: WriteRoad::Text,
    read: ReadRoad::Text,
};

const CLOSED_CHOICE_CONTRACT: RenderingContract = RenderingContract {
    bill: MemberContract {
        shape: CodecMemberShape::ClosedChoice,
        encode_road: SLOT_ROAD,
        decode_road: ROSTER_CONSTANT,
    },
    write: WriteRoad::ClosedChoice,
    read: ReadRoad::ClosedChoice,
};

const NESTED_CONTRACT: RenderingContract = RenderingContract {
    bill: MemberContract {
        shape: CodecMemberShape::Nested,
        encode_road: ENCODE_ROAD,
        decode_road: DECODE_ROAD,
    },
    write: WriteRoad::Nested,
    read: ReadRoad::Nested,
};

/// The authoritative row one member shape selects.
///
/// Both the public bill and generated operations read this seat, so adding or reassigning a shape cannot leave an independently selected renderer behind it.
pub(super) const fn rendering_contract(shape: CodecMemberShape) -> RenderingContract {
    match shape {
        CodecMemberShape::Count => COUNT_CONTRACT,
        CodecMemberShape::Bytes => BYTES_CONTRACT,
        CodecMemberShape::Text => TEXT_CONTRACT,
        CodecMemberShape::ClosedChoice => CLOSED_CHOICE_CONTRACT,
        CodecMemberShape::Nested => NESTED_CONTRACT,
    }
}

/// The locals the rendered decode road declares for itself.
///
/// # Authority
///
/// **A member whose spelling is one of these is refused rather than renamed.**
/// The decode road binds one local per member under the member's OWN spelling, which is what makes the rendered road readable and what lets the assembly call name its arguments the way the owner named its members.
/// A member colliding with one of these would shadow the rendering's own binding, and the road would go on reading a value nobody meant — a defect that compiles.
///
/// Renaming the rendering's locals to something nobody would write is not the repair: an unreadable rendered road is a road nobody can audit, and the collision would still exist for whatever names were chosen instead.
pub const RESERVED_BINDINGS: [&str; 12] = [
    MATERIAL_BINDING,
    REMAINING_BINDING,
    INTO_BINDING,
    NESTED_BINDING,
    COLLECTED_BINDING,
    CANDIDATE_BINDING,
    CHOSEN_BINDING,
    ELECTED_BINDING,
    PRESENT_BINDING,
    CARRIED_BINDING,
    LENGTH_BINDING,
    WIDTH_BINDING,
];
