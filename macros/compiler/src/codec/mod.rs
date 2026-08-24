#![doc = include_str!("README.md")]

mod encode;
mod render;
mod type_contract;
mod types;

pub use render::{codec_surface, render_codec};
pub use type_contract::{MEMBER_CONTRACT, RESERVED_BINDINGS};
pub use types::{
    AssemblyPosture, CODEC_ISSUE_LIMIT, CODEC_MEMBER_LIMIT, CODEC_PATH_SEGMENT_LIMIT, Cardinality,
    CodecAssembly, CodecContent, CodecDirection, CodecError, CodecIssue, CodecMember,
    CodecMemberShape, CodecPlacement, CodecProjection, CodecShape, CodecTypePath, DECODE_ROAD,
    DecodeRefusal, ENCODE_ROAD, MemberContract, ModuleSpelling, PathRooting, ROSTER_CONSTANT,
    SLOT_ROAD, is_identifier,
};
