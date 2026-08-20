#![doc = include_str!("README.md")]

mod plan;
mod render;
mod type_contract;
mod types;

pub use plan::codec_plan;
pub use render::{
    CANDIDATE_BINDING, CARRIED_BINDING, CHOSEN_BINDING, COLLECTED_BINDING, COUNT_PAST_WIDTH_ARM,
    DECODE_REFUSAL_ARMS, DECODE_ROAD, ELECTED_BINDING, ENCODE_ROAD, INTO_PARAMETER, LENGTH_BINDING,
    LENGTH_PAST_REMAINING_ARM, LENGTH_PAST_WIDTH_ARM, MATERIAL_PARAMETER, MEMBER_NOT_ADMITTED_ARM,
    MEMBER_SEAT, MODULE_PRELUDE_ROOT, NESTED_BINDING, NESTED_REFUSED_ARM, NOT_ASSEMBLED_ARM,
    PRESENCE_NOT_ADMITTED_ARM, PRESENT_BINDING, REMAINING_BINDING, ROSTER_CONSTANT,
    SLOT_NOT_ADMITTED_ARM, SLOT_ROAD, TEXT_NOT_UTF8_ARM, TRAILING_BYTES_ARM, TRUNCATED_ARM,
    WIDTH_BINDING, absent, appended, associated, attribute, borrowed_self_member, bound,
    bound_mutable, byte_sink, byte_slice, byte_width, call, codec_surface, decode_member,
    decode_road, derive_attribute, doc_attribute, empty_vector, encode_member, encode_road,
    framed_length, framing_read, framing_width, generics, group, language_path, left, mapped,
    member_refusal, member_variant, published_module, qualified, read_occurrence, read_one_byte,
    reassigned, refusal_conversion, refusal_declaration, self_member, sole_refusal, statement,
    taken, type_path, unbounded, write_member,
};
pub use type_contract::{MEMBER_CONTRACT, MemberContract, RESERVED_BINDINGS, covers};
pub use types::{
    AssemblyPosture, CodecAssembly, CodecComposition, CodecDeclarationRefusal, CodecMember,
    CodecMemberLimit, CodecMemberShape, CodecPathSegmentLimit, CodecPlacement, CodecPlan,
    CodecRoad, CodecShape, CodecSurface, CodecSurfaceIssue, CodecSurfaceIssueLimit, CodecTypePath,
    DecodeRefusalArm, ModuleSpelling, PathRooting, is_codec_identifier,
};
