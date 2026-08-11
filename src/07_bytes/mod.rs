//! Band 07 — bytes: the shared frame grammar, domain-tag register shape,
//! digest-family law, width conventions, text-form scheme, commitment roles,
//! content regions, and the bounded-reader maxima. Owner homes profile over
//! these primitives; none restates them.

pub mod types;

pub use types::{
    CapacityProfile, CommitmentRole, ContentRegionId, ContentRegionRole, DECODE_MAXIMA,
    DigestFamilyId, DomainTag, FRAME_HEADER_BYTES, FRAME_MAGIC, FRAME_TRAILER_BYTES, FrameDecode,
    FrameDigest, FrameHeader, FrameRoleId, PayloadBindingClaim, PayloadReference, TagProjection,
    TextFormDecode, WIDTH_CONVENTIONS,
};
