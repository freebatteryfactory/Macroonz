//! The invariant nucleus for a declared compiled member roster.

use super::{
    DeclaredCompilation, DeclaredReadBack, DeclaredReadBackRoster, DeclaredReadBackRosterRefusal,
    DiagnosticAnchor, ObservedCompilation, PrimarySourceSpan, PrimarySourceSpanRefusal,
    RelativeSourcePath, RelativeSourcePathRefusal, RustcErrorCode, RustcErrorCodeRefusal,
    SourcePosition, SourcePositionRefusal,
};

impl RustcErrorCode {
    /// Inform one stable rustc diagnostic error code from its structured spelling.
    ///
    /// # Errors
    ///
    /// Refuses every spelling other than `E` followed by exactly four ASCII digits.
    pub fn informed(spelling: &str) -> Result<Self, RustcErrorCodeRefusal> {
        match spelling.as_bytes() {
            [b'E', first, second, third, fourth]
                if first.is_ascii_digit()
                    && second.is_ascii_digit()
                    && third.is_ascii_digit()
                    && fourth.is_ascii_digit() =>
            {
                Ok(Self(spelling.to_owned()))
            }
            _ => Err(RustcErrorCodeRefusal::Grammar),
        }
    }

    /// The structured error-code spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl RelativeSourcePath {
    /// Inform one slash-separated logical path relative to a declared challenge root.
    ///
    /// # Errors
    ///
    /// Refuses empty, rooted, backslash-bearing, and non-normal paths without rewriting them.
    pub fn informed(spelling: &str) -> Result<Self, RelativeSourcePathRefusal> {
        if spelling.is_empty() {
            return Err(RelativeSourcePathRefusal::Empty);
        }
        if spelling.starts_with('/') || has_windows_prefix(spelling) {
            return Err(RelativeSourcePathRefusal::Absolute);
        }
        if spelling.contains('\\') {
            return Err(RelativeSourcePathRefusal::Backslash);
        }
        for (at, segment) in spelling.split('/').enumerate() {
            if segment.is_empty() || matches!(segment, "." | "..") {
                return Err(RelativeSourcePathRefusal::NonNormalSegment { at });
            }
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The canonical slash-separated logical spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl SourcePosition {
    /// Inform one source position with one-based coordinates.
    ///
    /// # Errors
    ///
    /// Refuses a zero line before a zero column.
    pub const fn informed(line: u64, column: u64) -> Result<Self, SourcePositionRefusal> {
        if line == 0u64 {
            return Err(SourcePositionRefusal::ZeroLine);
        }
        if column == 0u64 {
            return Err(SourcePositionRefusal::ZeroColumn);
        }
        Ok(Self { line, column })
    }

    /// The one-based source line.
    #[must_use]
    pub const fn line(self) -> u64 {
        self.line
    }

    /// The one-based source column.
    #[must_use]
    pub const fn column(self) -> u64 {
        self.column
    }
}

impl PrimarySourceSpan {
    /// Inform one root-independent rustc primary span.
    ///
    /// Rustc reports both lines one-based and inclusive, the start column one-based and inclusive, and the end column one-based and exclusive.
    /// A zero-width span is admitted, while a lexicographically reversed end refuses.
    ///
    /// # Errors
    ///
    /// Refuses an end position that precedes the start.
    pub fn informed(
        source: RelativeSourcePath,
        start: SourcePosition,
        end: SourcePosition,
    ) -> Result<Self, PrimarySourceSpanRefusal> {
        if end < start {
            return Err(PrimarySourceSpanRefusal::Reversed);
        }
        Ok(Self { source, start, end })
    }

    /// The logical path relative to the declared challenge root.
    pub const fn source(&self) -> &RelativeSourcePath {
        &self.source
    }

    /// The one-based inclusive start position.
    pub const fn start(&self) -> SourcePosition {
        self.start
    }

    /// The position whose line is inclusive and whose column is exclusive.
    pub const fn end(&self) -> SourcePosition {
        self.end
    }
}

impl DiagnosticAnchor {
    /// Bind one stable rustc error code to one normalized primary source span.
    pub fn at(code: RustcErrorCode, primary: PrimarySourceSpan) -> Self {
        Self { code, primary }
    }

    /// The stable rustc error code.
    pub const fn code(&self) -> &RustcErrorCode {
        &self.code
    }

    /// The normalized primary source span.
    pub const fn primary(&self) -> &PrimarySourceSpan {
        &self.primary
    }
}

impl DeclaredCompilation {
    /// Declare that the compiler must accept the challenge.
    pub const fn compiles() -> Self {
        Self { refusal: None }
    }

    /// Declare that the compiler must refuse at this exact diagnostic anchor.
    pub fn refuses(anchor: DiagnosticAnchor) -> Self {
        Self {
            refusal: Some(anchor),
        }
    }

    /// The required refusal anchor, or nothing where acceptance is required.
    #[must_use]
    pub const fn refusal(&self) -> Option<&DiagnosticAnchor> {
        self.refusal.as_ref()
    }
}

impl ObservedCompilation {
    /// State that the compiler accepted the challenge.
    pub const fn compiled() -> Self {
        Self { refusal: None }
    }

    /// State the one exact diagnostic anchor the external host established.
    pub fn refused(anchor: DiagnosticAnchor) -> Self {
        Self {
            refusal: Some(anchor),
        }
    }

    /// The observed refusal anchor, or nothing where the compiler accepted.
    #[must_use]
    pub const fn refusal(&self) -> Option<&DiagnosticAnchor> {
        self.refusal.as_ref()
    }
}

fn has_windows_prefix(spelling: &str) -> bool {
    matches!(spelling.as_bytes(), [drive, b':', ..] if drive.is_ascii_alphabetic())
}

impl<'spec> DeclaredReadBackRoster<'spec> {
    /// Build one declared read-back roster after establishing that every member name occurs once.
    ///
    /// # Errors
    ///
    /// Refuses the second occurrence of a member name before a roster exists.
    pub fn declared(
        members: &'spec [DeclaredReadBack<'spec>],
    ) -> Result<Self, DeclaredReadBackRosterRefusal> {
        for (at, member) in members.iter().enumerate() {
            let duplicate = members
                .iter()
                .take(at)
                .any(|earlier| earlier.name == member.name);
            if duplicate {
                return Err(DeclaredReadBackRosterRefusal::DuplicateMember { at });
            }
        }
        Ok(Self { members })
    }

    /// The declared members, in declaration order.
    #[must_use]
    pub const fn members(self) -> &'spec [DeclaredReadBack<'spec>] {
        self.members
    }
}
