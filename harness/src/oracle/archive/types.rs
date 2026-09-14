//! Method-specific historical verdicts without a current conclusion or execution mint.

use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::oracle::{
    ByteDifference, CompilationDisagreement, CompiledDisagreement, PrimarySourceSpanRefusal,
    RelativeSourcePathRefusal, RustcErrorCodeRefusal, SourcePositionRefusal,
};
use crate::report::archive::{AddressClaim, ArchiveRefusal};

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_verdict;

/// The domain of bounded historical oracle verdict envelopes.
pub const ORACLE_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-oracle-verdict",
    IdentityProfileVersion::declared(1),
);

/// The oracle method an independent reader expects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchivedMethod {
    /// Exact golden-vector byte comparison.
    Vector,
    /// Independently composed identity transcript.
    Transcript,
    /// Structural syntax comparison.
    Structural,
    /// Caller-stated compiler read-back comparison.
    Compiled,
    /// Exact compilation code and primary-span comparison.
    Compilation,
}

/// An owned historical oracle record whose method and fields obey their declared grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedOracle {
    encoded: Vec<u8>,
    address: ContentAddress,
    verdict: ArchivedVerdict,
}

/// The historical disposition and complete payload of one particular oracle method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedVerdict {
    /// The source stated exact vector agreement without retaining successful-case material.
    VectorAgrees,
    /// The retained buffers disagree at their actual first difference.
    VectorDisagrees(ArchivedVectorDisagreement),
    /// The source stated transcript agreement without retaining its derivation.
    TranscriptAgrees,
    /// The retained identity claims differ without their original preimages.
    TranscriptDisagrees(ArchivedTranscriptDisagreement),
    /// The source stated structural conformity.
    StructuralConforms,
    /// The source stated this structural disagreement.
    StructuralDeviates(ArchivedStructuralDisagreement),
    /// The source stated that the artifact was unparsable.
    StructuralUnparsable,
    /// The source stated compiler read-back conformity.
    CompiledConforms,
    /// The source stated this compiler read-back disagreement.
    CompiledDeviates(CompiledDisagreement),
    /// The source stated exact compilation conformity.
    CompilationConforms,
    /// The source stated this exact compilation disagreement.
    CompilationDeviates(CompilationDisagreement),
}

/// Two complete historical byte buffers and their owner-derived first difference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedVectorDisagreement {
    expected: Vec<u8>,
    produced: Vec<u8>,
    difference: ByteDifference,
}

/// Two unequal historical identity claims without derivation authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchivedTranscriptDisagreement {
    rederived: AddressClaim,
    published: AddressClaim,
}

/// A caller-stated structural disagreement with portable historical positions and counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedStructuralDisagreement {
    /// The source stated that an unexpected item occurred.
    UnexpectedItem,
    /// The source retained these declared and observed cardinalities.
    OutputCardinality {
        /// The declared historical count.
        declared: u64,
        /// The observed historical count.
        read: u64,
    },
    /// The source stated a repeated implementation at this position.
    DuplicateImplementation {
        /// The historical implementation position.
        at: u64,
    },
    /// The source stated a different target at this position.
    ImplementationTarget {
        /// The historical implementation position.
        at: u64,
    },
    /// The source stated a different trait path at this position.
    TraitPath {
        /// The historical implementation position.
        at: u64,
    },
    /// The source stated a different implementation posture at this position.
    ImplPosture {
        /// The historical implementation position.
        at: u64,
    },
    /// The source stated this unapproved attribute spelling.
    MeaningBearingAttribute {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated attribute text.
        attribute: String,
    },
    /// The source stated this unexpected member.
    UnexpectedImplMember {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated member text.
        member: String,
    },
    /// The source stated this repeated member.
    DuplicateMember {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated member text.
        member: String,
    },
    /// The source stated this missing member.
    MissingImplMember {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated member text.
        member: String,
    },
    /// The source stated that this member value was unread.
    MemberValueUnread {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated member text.
        member: String,
    },
    /// The source stated that this member value differed.
    MemberValue {
        /// The historical implementation position.
        at: u64,
        /// The exact caller-stated member text.
        member: String,
    },
}

/// Why a bounded historical oracle record was not admitted.
#[must_use = "a refusal states why historical oracle data was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleArchiveRefusal {
    /// Shared envelope, framing or resource admission refused.
    Archive(ArchiveRefusal),
    /// Buffers claimed as a vector disagreement are equal.
    EqualVectorBuffers,
    /// Identities claimed as a transcript disagreement are equal.
    EqualTranscriptClaims,
    /// A retained error code violates its existing owner grammar.
    ErrorCode(RustcErrorCodeRefusal),
    /// A retained source path violates its existing owner grammar.
    SourcePath(RelativeSourcePathRefusal),
    /// A retained source position violates its existing owner boundary.
    SourcePosition(SourcePositionRefusal),
    /// A retained source span violates its existing owner boundary.
    PrimarySpan(PrimarySourceSpanRefusal),
}
