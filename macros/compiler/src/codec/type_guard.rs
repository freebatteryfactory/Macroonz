//! The codec home's invariant nucleus: every road that reaches a private field, and the alphabet they all read a spelling through.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's walls structural.
//! A shape is seated by one road, and that road refuses an empty roster, a doubled spelling, and a spelling the decode road has already taken — so a codec whose decode road could not refuse, or whose bindings would shadow one another, is a value nobody can write rather than a state a reader has to notice.

use super::super::type_contract::RESERVED_BINDINGS;
use super::{
    AssemblyPosture, CODEC_ISSUE_LIMIT, Cardinality, CodecAssembly, CodecError, CodecIssue,
    CodecMember, CodecMemberShape, CodecShape, CodecTypePath, ModuleSpelling, PathRooting,
};
use crate::bounded::{Capped, Capping, NonEmpty, NonEmptyError, Overflow};
use std::collections::BTreeSet;

impl CodecTypePath {
    /// One type path, rooted as the caller stated and spelled from the segments it named.
    ///
    /// # Errors
    ///
    /// Returns [`CodecIssue::SegmentNotAnIdentifier`] where a segment cannot name a rendered item — the root is typed as the rooting, so every segment is an item step, refused outside the alphabet or on the keyword roster — [`CodecIssue::PathSegmentsAbsent`] where no segment was supplied, and [`CodecIssue::PathSegmentsUnbounded`] where the segments outgrow the declared magnitude.
    /// The checks are dependent and in that order, so exactly one cause is true of any refused path.
    pub fn spelled(rooting: PathRooting, segments: Vec<String>) -> Result<Self, CodecError> {
        for segment in &segments {
            if !rendered_name(segment) {
                return Err(CodecError::of(CodecIssue::SegmentNotAnIdentifier {
                    segment: segment.clone(),
                }));
            }
        }
        let admitted = NonEmpty::new(segments).map_err(|refused| match refused {
            NonEmptyError::Empty(_) => CodecError::of(CodecIssue::PathSegmentsAbsent),
            NonEmptyError::Overflow(overflow) => {
                let (bound, observed) = counted(overflow);
                CodecError::of(CodecIssue::PathSegmentsUnbounded { bound, observed })
            }
        })?;
        Ok(Self {
            rooting,
            segments: admitted,
        })
    }

    /// Where this path is rooted.
    #[must_use]
    pub const fn rooting(&self) -> PathRooting {
        self.rooting
    }

    /// The segments, from the root inward; structurally at least one.
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.segments.iter().map(String::as_str)
    }

    /// How many segments the path carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.count()
    }
}

impl ModuleSpelling {
    /// One published module's spelling.
    ///
    /// # Errors
    ///
    /// Returns [`CodecIssue::ModuleSpellingNotAnIdentifier`] where the spelling cannot name a rendered item: not one Rust identifier, or a keyword the language already took.
    pub fn spelled(spelling: &str) -> Result<Self, CodecError> {
        if rendered_name(spelling) {
            Ok(Self {
                spelling: spelling.to_owned(),
            })
        } else {
            Err(CodecError::of(CodecIssue::ModuleSpellingNotAnIdentifier {
                spelling: spelling.to_owned(),
            }))
        }
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl CodecMember {
    /// Declare one member of a shape.
    ///
    /// # Errors
    ///
    /// Returns [`CodecIssue::MemberSpellingAbsent`] where the member states no spelling, and [`CodecIssue::MemberSpellingNotAnIdentifier`] where the spelling cannot name a rendered item — not one Rust identifier, or a keyword the language already took.
    /// The two are dependent — there is no alphabet to read until there are characters — so exactly one is ever established.
    pub fn declared(
        spelling: &str,
        held_as: CodecTypePath,
        shape: CodecMemberShape,
        cardinality: Cardinality,
    ) -> Result<Self, CodecError> {
        if spelling.is_empty() {
            return Err(CodecError::of(CodecIssue::MemberSpellingAbsent));
        }
        if !rendered_name(spelling) {
            return Err(CodecError::of(CodecIssue::MemberSpellingNotAnIdentifier {
                spelling: spelling.to_owned(),
            }));
        }
        Ok(Self {
            spelling: spelling.to_owned(),
            held_as,
            shape,
            cardinality,
        })
    }

    /// What the owner calls this member.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }

    /// The type ONE OCCURRENCE of this member is held at, never the collection or the option a cardinality wraps it in.
    #[must_use]
    pub const fn held_as(&self) -> &CodecTypePath {
        &self.held_as
    }

    /// How this member is written.
    #[must_use]
    pub const fn shape(&self) -> CodecMemberShape {
        self.shape
    }

    /// How many of this member there are.
    #[must_use]
    pub const fn cardinality(&self) -> Cardinality {
        self.cardinality
    }
}

impl CodecAssembly {
    /// The assembly road, under the posture the caller stated.
    ///
    /// # Errors
    ///
    /// Returns [`CodecIssue::AssemblyRoadAbsent`] where the road states no spelling, and [`CodecIssue::AssemblyRoadNotAnIdentifier`] where the spelling cannot name a rendered item — not one Rust identifier, or a keyword the language already took.
    pub fn stated(road: &str, posture: AssemblyPosture) -> Result<Self, CodecError> {
        if road.is_empty() {
            return Err(CodecError::of(CodecIssue::AssemblyRoadAbsent));
        }
        if !rendered_name(road) {
            return Err(CodecError::of(CodecIssue::AssemblyRoadNotAnIdentifier {
                spelling: road.to_owned(),
            }));
        }
        Ok(Self {
            road: road.to_owned(),
            posture,
        })
    }

    /// The associated road the decode surface calls.
    #[must_use]
    pub fn road(&self) -> &str {
        self.road.as_str()
    }

    /// The posture that road stands under.
    #[must_use]
    pub const fn posture(&self) -> &AssemblyPosture {
        &self.posture
    }
}

impl CodecShape {
    /// Declare one complete shape.
    ///
    /// # Errors
    ///
    /// Returns [`CodecIssue::RefusalSpellingNotAnIdentifier`] where the rendered refusal's spelling cannot name a rendered item — not one Rust identifier, or a keyword the language already took — then whatever the pass over the offered members established — [`CodecIssue::MemberSpellingDoubled`] and [`CodecIssue::MemberShadowsBinding`], which co-establish — then [`CodecIssue::MembersAbsent`] where no member was supplied and [`CodecIssue::MembersUnbounded`] where the members outgrow the declared magnitude.
    pub fn declared(
        owner: CodecTypePath,
        refusal: &str,
        assembly: CodecAssembly,
        members: Vec<CodecMember>,
    ) -> Result<Self, CodecError> {
        if !rendered_name(refusal) {
            return Err(CodecError::of(CodecIssue::RefusalSpellingNotAnIdentifier {
                spelling: refusal.to_owned(),
            }));
        }
        if let Some((first, rest)) = established(member_issues(&members)) {
            return Err(CodecError::over(first, rest));
        }
        let admitted = NonEmpty::new(members).map_err(|refused| match refused {
            NonEmptyError::Empty(_) => CodecError::of(CodecIssue::MembersAbsent),
            NonEmptyError::Overflow(overflow) => {
                let (bound, observed) = counted(overflow);
                CodecError::of(CodecIssue::MembersUnbounded { bound, observed })
            }
        })?;
        Ok(Self {
            owner,
            refusal: refusal.to_owned(),
            assembly,
            members: admitted,
        })
    }

    /// The type the codec is written for.
    #[must_use]
    pub const fn owner(&self) -> &CodecTypePath {
        &self.owner
    }

    /// The spelling the rendered decode refusal is declared under.
    #[must_use]
    pub fn refusal(&self) -> &str {
        self.refusal.as_str()
    }

    /// The road the decoded members are assembled by.
    #[must_use]
    pub const fn assembly(&self) -> &CodecAssembly {
        &self.assembly
    }

    /// The members, in the order the shape declares them.
    ///
    /// # Ordering
    ///
    /// This order IS meaning: it is the order the encode road writes and the decode road reads, so the same members supplied in another order are a different byte string for the same value — which is exactly what a canonical encoding may not have two of.
    pub fn members(&self) -> impl Iterator<Item = &CodecMember> {
        self.members.iter()
    }

    /// How many members the shape declares; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.members.count()
    }
}

impl CodecError {
    /// The refusal one established issue makes.
    pub fn of(issue: CodecIssue) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }

    /// The refusal a pass whose checks co-establish makes.
    ///
    /// The caller arrives holding every issue its pass established, so the posture the body writes is about the REPORT and never about the pass: where the issues fit it carries all of them, and where they do not it carries what fits and counts the rest.
    pub fn over(first: CodecIssue, rest: Vec<CodecIssue>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }

    /// The first issue the pass established, which every refusal has.
    #[must_use]
    pub fn first_issue(&self) -> &CodecIssue {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the pass established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<CodecIssue, CODEC_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether this refusal carries every issue its pass established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

pub use crate::token::{rendered_identifier, rendered_name};

/// The pass over one shape's offered members: what their spellings say about each other and about the locals the decode road declares for itself.
///
/// Every member is asked and every collision is reported, because a caller repairing a shape one member per attempt is a caller this home failed.
/// A spelling doubled three times establishes one issue, not two: the fact is that the spelling is shared, and it is stated once.
fn member_issues(members: &[CodecMember]) -> Vec<CodecIssue> {
    let mut issues: Vec<CodecIssue> = Vec::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut reported: BTreeSet<&str> = BTreeSet::new();
    for member in members {
        let spelling = member.spelling();
        if !seen.insert(spelling) && reported.insert(spelling) {
            issues.push(CodecIssue::MemberSpellingDoubled {
                spelling: spelling.to_owned(),
            });
        }
        for binding in RESERVED_BINDINGS {
            if spelling == binding {
                issues.push(CodecIssue::MemberShadowsBinding {
                    spelling: spelling.to_owned(),
                    binding,
                });
            }
        }
    }
    issues
}

/// One pass's established issues as the pair a refusal is built from, or nothing where the pass established none.
fn established(issues: Vec<CodecIssue>) -> Option<(CodecIssue, Vec<CodecIssue>)> {
    let mut walk = issues.into_iter();
    let first = walk.next()?;
    Some((first, walk.collect()))
}

/// The two counts an overflow already carries, at the width a refusal states them.
fn counted(overflow: Overflow) -> (u64, u64) {
    (
        u64::try_from(overflow.capacity).unwrap_or(u64::MAX),
        u64::try_from(overflow.offered).unwrap_or(u64::MAX),
    )
}
