//! The compiler-resolved declaration, observation, and verdict vocabulary.

#[path = "type_guard.rs"]
mod guard;

/// One value a compiled artifact handed back, as the reader that ran it observed it.
///
/// This is a value and not syntax, so it carries no constructor path: by the time a compiler hands a constant back, the path it was built through has been resolved away.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObservedValue {
    /// One word a typed value states — the variant a constant reads back as.
    Word(String),
    /// One text value.
    Text(String),
    /// One whole number.
    Count(u64),
    /// One truth value.
    Truth(bool),
    /// Several values, in the order the compiled artifact hands them back.
    Series(Vec<ObservedValue>),
}

/// One member a compiled artifact handed back: the name the reader asked for, and the value it got.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObservedMember {
    /// The member's name.
    pub name: String,
    /// The value it read back as.
    pub value: ObservedValue,
}

/// What a caller reports observing after presenting one artifact to a compiler.
///
/// Holding one does not establish that a compiler ran; the effectful challenge that constructs it owns that provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledObservation {
    /// The compiler refused the artifact, so nothing was read back.
    RefusedByCompiler,
    /// The artifact compiled, and these are the members the caller read back, in read order.
    ReadBack(Vec<ObservedMember>),
}

/// One stable rustc diagnostic error code.
#[must_use = "an informed rustc error code is the structured compiler class one diagnostic anchor compares"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RustcErrorCode(String);

/// Why one rustc error code was refused.
#[must_use = "a refusal is the reason a rustc error code was not informed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustcErrorCodeRefusal {
    /// The spelling is not `E` followed by exactly four ASCII digits.
    Grammar,
}

/// One canonical logical path relative to a declared compiler-challenge root.
#[must_use = "a relative source path is the root-independent source coordinate one diagnostic anchor compares"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelativeSourcePath(String);

/// Why one relative source path was refused.
#[must_use = "a refusal is the reason a relative source path was not informed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeSourcePathRefusal {
    /// The path carries no segment.
    Empty,
    /// The path is rooted rather than relative to the declared challenge root.
    Absolute,
    /// The path carries a backslash instead of the canonical logical separator.
    Backslash,
    /// One segment is empty, current, or parent traversal.
    NonNormalSegment {
        /// The segment's position in the offered path.
        at: usize,
    },
}

/// One source position with one-based coordinates.
#[must_use = "a source position names one one-based line and column"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourcePosition {
    line: u64,
    column: u64,
}

/// Why one source position was refused.
#[must_use = "a refusal is the reason a source position was not informed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourcePositionRefusal {
    /// Source lines are one-based.
    ZeroLine,
    /// Source columns are one-based.
    ZeroColumn,
}

/// One rustc primary source span under a root-independent logical path.
#[must_use = "a primary source span is the structured location one diagnostic anchor compares"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimarySourceSpan {
    source: RelativeSourcePath,
    start: SourcePosition,
    end: SourcePosition,
}

/// Why one primary source span was refused.
#[must_use = "a refusal is the reason a primary source span was not informed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimarySourceSpanRefusal {
    /// The end position precedes the start position.
    Reversed,
}

/// One authoritative compiler-diagnostic anchor.
///
/// The pair is comparison material rather than a claim that rustc emitted no other diagnostic.
#[must_use = "a diagnostic anchor is the exact code and primary span one compilation contract compares"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticAnchor {
    code: RustcErrorCode,
    primary: PrimarySourceSpan,
}

/// What one exact compilation contract declares.
///
/// This type is additive beside [`DeclaredBehavior`], whose existing coarse refusal and read-back meanings remain unchanged.
#[must_use = "a declared compilation states whether acceptance or one exact diagnostic refusal is required"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredCompilation {
    refusal: Option<DiagnosticAnchor>,
}

/// What one compiler host reports observing for an exact compilation contract.
///
/// Holding one does not establish that a compiler ran; the effectful host that constructs it owns that provenance and ambiguity detection.
#[must_use = "an observed compilation carries either acceptance or one host-established diagnostic anchor"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObservedCompilation {
    refusal: Option<DiagnosticAnchor>,
}

/// Which exact compilation declaration and observation disagree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationDisagreement {
    /// The compiler accepted where one exact refusal was declared.
    AcceptedWhereRefusalDeclared,
    /// The compiler refused where acceptance was declared.
    RefusedWhereAcceptanceDeclared {
        /// The diagnostic anchor the host established.
        observed: DiagnosticAnchor,
    },
    /// The stable rustc error code differs.
    ErrorCode {
        /// The declared code.
        expected: RustcErrorCode,
        /// The observed code.
        observed: RustcErrorCode,
    },
    /// The root-independent primary span differs.
    PrimarySpan {
        /// The declared span.
        expected: PrimarySourceSpan,
        /// The observed span.
        observed: PrimarySourceSpan,
    },
}

/// What one exact compilation comparison concluded.
#[must_use = "a verdict is what the exact compilation comparison concluded"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompilationVerdict {
    /// The supplied observation agrees with the declared compilation.
    Conforms,
    /// The supplied observation and declared compilation disagree, about this.
    Deviates(CompilationDisagreement),
}

/// One member a caller states a compiled artifact will hand back.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeclaredReadBack<'spec> {
    /// The member's name.
    pub name: &'spec str,
    /// The value it must read back as.
    pub value: ObservedValue,
}

/// A duplicate-free roster of members a caller states a compiled artifact will hand back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredReadBackRoster<'spec> {
    members: &'spec [DeclaredReadBack<'spec>],
}

/// Why a declared compiled read-back roster was refused.
#[must_use = "a refusal is the reason a declared read-back roster was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclaredReadBackRosterRefusal {
    /// One member name appears more than once.
    DuplicateMember {
        /// The second member's position in the offered roster.
        at: usize,
    },
}

/// What a caller states a compiler will do with one artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclaredBehavior<'spec> {
    /// The compiler must refuse this artifact.
    RefusedByCompiler,
    /// The artifact must compile and hand back exactly these members.
    ReadsBack(DeclaredReadBackRoster<'spec>),
}

/// Which supplied observation and declared behavior disagree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledDisagreement {
    /// The caller reported acceptance where the declaration requires refusal.
    AcceptedWhereRefusalDeclared,
    /// The caller reported refusal where the declaration requires acceptance.
    RefusedWhereAcceptanceDeclared,
    /// The supplied read-back carries a member the declaration did not name.
    UnexpectedMember {
        /// The member's name.
        member: String,
    },
    /// The supplied read-back carries one member more than once.
    DuplicateMember {
        /// The member's name.
        member: String,
    },
    /// The supplied read-back omits a member the declaration names.
    MissingMember {
        /// The member's name.
        member: String,
    },
    /// A supplied member value differs from the declaration.
    MemberValue {
        /// The member's name.
        member: String,
    },
}

/// What one compiled-observation comparison concluded.
#[must_use = "a verdict is what the compiled-observation comparison concluded"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompiledVerdict {
    /// The supplied observation agrees with the declared behavior.
    Conforms,
    /// The supplied observation and declared behavior disagree, about this.
    Deviates(CompiledDisagreement),
}
