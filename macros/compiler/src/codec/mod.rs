#![doc = include_str!("README.md")]

mod bank;
mod declare;
mod encode;
mod place;
mod read;
mod render;
mod spell;
mod type_contract;
mod types;
mod write;

pub use bank::{MEMBER_CONTRACT, RESERVED_BINDINGS};
pub use render::{codec_surface, render_codec};
pub use types::{
    AssemblyPosture, CODEC_ISSUE_LIMIT, CODEC_MEMBER_LIMIT, CODEC_PATH_SEGMENT_LIMIT, Cardinality,
    CodecAssembly, CodecContent, CodecDirection, CodecError, CodecIssue, CodecMember,
    CodecMemberShape, CodecPlacement, CodecProjection, CodecShape, CodecTypePath, DECODE_ROAD,
    DecodeRefusal, ENCODE_ROAD, MemberContract, ModuleSpelling, PathRooting, ROSTER_CONSTANT,
    SLOT_ROAD, rendered_identifier, rendered_name,
};
