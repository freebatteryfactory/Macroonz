//! The invariant nucleus for a declared compiled member roster.

use super::{DeclaredReadBack, DeclaredReadBackRoster, DeclaredReadBackRosterRefusal};

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
