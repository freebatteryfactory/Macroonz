//! The compiled read-back declaration, observation, and verdict vocabulary.

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
