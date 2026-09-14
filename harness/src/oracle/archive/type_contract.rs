//! Refusal composition preserves each existing admission owner's reason.

use super::OracleArchiveRefusal;
use crate::oracle::{
    PrimarySourceSpanRefusal, RelativeSourcePathRefusal, RustcErrorCodeRefusal,
    SourcePositionRefusal,
};
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for OracleArchiveRefusal {
    fn from(refusal: ArchiveRefusal) -> Self {
        Self::Archive(refusal)
    }
}

impl From<RustcErrorCodeRefusal> for OracleArchiveRefusal {
    fn from(refusal: RustcErrorCodeRefusal) -> Self {
        Self::ErrorCode(refusal)
    }
}

impl From<RelativeSourcePathRefusal> for OracleArchiveRefusal {
    fn from(refusal: RelativeSourcePathRefusal) -> Self {
        Self::SourcePath(refusal)
    }
}

impl From<SourcePositionRefusal> for OracleArchiveRefusal {
    fn from(refusal: SourcePositionRefusal) -> Self {
        Self::SourcePosition(refusal)
    }
}

impl From<PrimarySourceSpanRefusal> for OracleArchiveRefusal {
    fn from(refusal: PrimarySourceSpanRefusal) -> Self {
        Self::PrimarySpan(refusal)
    }
}
