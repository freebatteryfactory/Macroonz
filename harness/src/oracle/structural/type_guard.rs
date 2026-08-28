//! The invariant nucleus for complete paths and declared member rosters.

use super::{
    DeclaredMember, DeclaredMemberRoster, DeclaredMemberRosterRefusal, StructuralPath,
    StructuralPathRefusal, StructuralPathRoot, StructuralPathSegment,
};

impl StructuralPathSegment {
    /// This segment's spelling, without a path separator.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl StructuralPath {
    /// Build one relative path from its segments.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, an empty segment, or a segment containing `::` before a path exists.
    pub fn relative(segments: &[&str]) -> Result<Self, StructuralPathRefusal> {
        Self::with_root(StructuralPathRoot::Relative, segments)
    }

    /// Build one absolute path from its segments.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, an empty segment, or a segment containing `::` before a path exists.
    pub fn absolute(segments: &[&str]) -> Result<Self, StructuralPathRefusal> {
        Self::with_root(StructuralPathRoot::Absolute, segments)
    }

    /// Whether the path carries a leading separator or begins in its surrounding scope.
    #[must_use]
    pub const fn root(&self) -> StructuralPathRoot {
        self.root
    }

    /// The path's indivisible segments, in order.
    #[must_use]
    pub fn segments(&self) -> &[StructuralPathSegment] {
        &self.segments
    }

    /// The complete path spelling, including a leading `::` where the path is absolute.
    #[must_use]
    pub fn spelling(&self) -> String {
        let mut spelled = String::new();
        if self.root == StructuralPathRoot::Absolute {
            spelled.push_str("::");
        }
        for (position, segment) in self.segments.iter().enumerate() {
            if position > 0 {
                spelled.push_str("::");
            }
            spelled.push_str(segment.spelling());
        }
        spelled
    }

    /// Build one path after every segment has been established as indivisible.
    fn with_root(
        root: StructuralPathRoot,
        segments: &[&str],
    ) -> Result<Self, StructuralPathRefusal> {
        if segments.is_empty() {
            return Err(StructuralPathRefusal::NoSegments);
        }
        let mut informed: Vec<StructuralPathSegment> = Vec::new();
        for (at, segment) in segments.iter().enumerate() {
            if segment.is_empty() {
                return Err(StructuralPathRefusal::EmptySegment { at });
            }
            if segment.contains("::") {
                return Err(StructuralPathRefusal::EmbeddedSeparator { at });
            }
            informed.push(StructuralPathSegment((*segment).to_owned()));
        }
        Ok(Self {
            root,
            segments: informed,
        })
    }
}

impl<'spec> DeclaredMemberRoster<'spec> {
    /// Build one declared member roster after establishing that every name occurs once.
    ///
    /// # Errors
    ///
    /// Refuses the second occurrence of a member name before a roster exists.
    pub fn declared(
        members: &'spec [DeclaredMember<'spec>],
    ) -> Result<Self, DeclaredMemberRosterRefusal> {
        for (at, member) in members.iter().enumerate() {
            let duplicate = members
                .iter()
                .take(at)
                .any(|earlier| earlier.name == member.name);
            if duplicate {
                return Err(DeclaredMemberRosterRefusal::DuplicateMember { at });
            }
        }
        Ok(Self { members })
    }

    /// The declared members, in declaration order.
    #[must_use]
    pub const fn members(self) -> &'spec [DeclaredMember<'spec>] {
        self.members
    }
}
