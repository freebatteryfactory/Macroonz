//! The tables this home states rather than computes, and the contracts its kind and its refusal stand under.
//!
//! Each table is total, so a row admitted later stops the compiler in every one of them until somebody says what that row's answer is.

use super::render::{
    CANDIDATE_BINDING, CARRIED_BINDING, CHOSEN_BINDING, COLLECTED_BINDING, ELECTED_BINDING,
    INTO_BINDING, LENGTH_BINDING, MATERIAL_BINDING, NESTED_BINDING, PRESENT_BINDING,
    REMAINING_BINDING, WIDTH_BINDING,
};
use super::{
    AssemblyPosture, CodecContent, CodecDirection, CodecError, CodecIssue, CodecMemberShape,
    CodecPlacement, CodecProjection, CodecTypePath, DECODE_ROAD, DecodeRefusal, ENCODE_ROAD,
    MemberContract, ROSTER_CONSTANT, SLOT_ROAD,
};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    CODEC_DECLARATION_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass,
    Refused, Repair,
};
use crate::identity::{OwnerIdentity, encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Kind, NoQuestions, SoleRole};
use core::fmt;

impl CanonicalContent for CodecContent {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_path(self.shape.owner(), into);
        encode_bytes(self.shape.refusal().as_bytes(), into);
        let assembly = self.shape.assembly();
        encode_bytes(assembly.road().as_bytes(), into);
        match assembly.posture() {
            AssemblyPosture::Total => into.push(0),
            AssemblyPosture::Checked { refusal } => {
                into.push(1);
                encode_path(refusal, into);
            }
        }
        encode_length(self.shape.count(), into);
        for member in self.shape.members() {
            let mut encoded = Vec::new();
            encode_bytes(member.spelling().as_bytes(), &mut encoded);
            encode_path(member.held_as(), &mut encoded);
            encode_bytes(member.shape().name().as_bytes(), &mut encoded);
            encode_bytes(member.cardinality().name().as_bytes(), &mut encoded);
            encode_bytes(&encoded, into);
        }
        encode_bytes(self.direction.name().as_bytes(), into);
        match &self.placement {
            CodecPlacement::AtDeclarationSite => into.push(0),
            CodecPlacement::PublishedModule { spelling } => {
                into.push(1);
                encode_bytes(spelling.spelling().as_bytes(), into);
            }
        }
        encode_owner(self.schema.as_ref(), into);
        encode_owner(self.byte_role.as_ref(), into);
        encode_length(self.assumptions.len(), into);
        for assumption in self.assumptions.as_slice() {
            encode_bytes(&assumption.citation_bytes(), into);
        }
    }
}

fn encode_path(path: &CodecTypePath, into: &mut Vec<u8>) {
    encode_bytes(path.rooting().name().as_bytes(), into);
    encode_length(path.count(), into);
    for segment in path.segments() {
        encode_bytes(segment.as_bytes(), into);
    }
}

fn encode_owner(owner: Option<&OwnerIdentity>, into: &mut Vec<u8>) {
    match owner {
        None => into.push(0),
        Some(identity) => {
            into.push(1);
            encode_bytes(&identity.citation_bytes(), into);
        }
    }
}

impl Kind for CodecProjection {
    const NAME: &'static str = "codec-projection";

    type Content = CodecContent;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl CodecDirection {
    /// Whether this direction covers the road that writes canonical bytes.
    #[must_use]
    pub const fn writes(self) -> bool {
        match self {
            Self::Encode | Self::RoundTrip => true,
            Self::Decode => false,
        }
    }

    /// Whether this direction covers the road that reads them back.
    ///
    /// # Nonclaims
    ///
    /// A direction that does not cover it delivers no reader, and that is a stated posture rather than a rendering that fell short: "a codec that refuses on decode is the validator" says exactly as much about the codec that has no decode road.
    #[must_use]
    pub const fn reads(self) -> bool {
        match self {
            Self::Decode | Self::RoundTrip => true,
            Self::Encode => false,
        }
    }
}

impl DecodeRefusal {
    /// Whether this arm names the member the read was standing at.
    ///
    /// The two that do not are facts about the whole material and about the assembly, and a member seat on either would name a member no read was standing at.
    #[must_use]
    pub const fn carries_member(self) -> bool {
        match self {
            Self::Truncated
            | Self::LengthPastRemaining
            | Self::LengthPastAddressableWidth
            | Self::CountPastDeclaredWidth
            | Self::TextNotUtf8
            | Self::MemberNotAdmitted
            | Self::SlotNotAdmitted
            | Self::NestedMemberRefused
            | Self::PresenceNotAdmitted => true,
            Self::TrailingBytes | Self::NotAssembled => false,
        }
    }

    /// The sentence this arm is rendered with, for whoever reads the refusal in their own crate.
    #[must_use]
    pub const fn sentence(self) -> &'static str {
        match self {
            Self::Truncated => "The material ended inside this member.",
            Self::LengthPastRemaining => {
                "This member's declared length runs past the material that remains."
            }
            Self::LengthPastAddressableWidth => {
                "This member's declared length does not fit an addressable width."
            }
            Self::CountPastDeclaredWidth => {
                "This member's declared count does not fit the width the member is held at."
            }
            Self::TextNotUtf8 => "This member's framed bytes are not UTF-8.",
            Self::MemberNotAdmitted => "The member's own type refused what was read for it.",
            Self::SlotNotAdmitted => {
                "The slot read for this member names no arm of the roster it was declared over."
            }
            Self::NestedMemberRefused => {
                "The nested codec this member carries refused the framed material."
            }
            Self::PresenceNotAdmitted => {
                "This member's presence byte is neither of the two the encode road writes."
            }
            Self::TrailingBytes => {
                "Material remains after the last declared member. A canonical encoding is the \
                 whole of what a value writes, so a longer input is not this value with something \
                 after it."
            }
            Self::NotAssembled => {
                "Every member was read, and the road that assembles them refused. The refusal is \
                 the owner's own, carried exactly."
            }
        }
    }
}

/// The complete bill, one row per wire shape, in the roster's own order.
///
/// Five rows because the roster is five: a row added here without an arm beside it, or an arm without a row, is a length disagreement the declaration itself carries.
///
/// The closed-choice row is this compiler's own contract on a caller's roster — a complete roster constant and a position road answering one byte — and not an inheritance from any stamp that happens to emit one.
pub const MEMBER_CONTRACT: [MemberContract; 5] = [
    MemberContract {
        shape: CodecMemberShape::Count,
        encode_road: "u64::from",
        decode_road: "<T as ::core::convert::TryFrom<u64>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::Bytes,
        encode_road: "<T as ::core::convert::AsRef<[u8]>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::vec::Vec<u8>>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::Text,
        encode_road: "<T as ::core::convert::AsRef<str>>::as_ref",
        decode_road: "<T as ::core::convert::TryFrom<::std::string::String>>::try_from",
    },
    MemberContract {
        shape: CodecMemberShape::ClosedChoice,
        encode_road: SLOT_ROAD,
        decode_road: ROSTER_CONSTANT,
    },
    MemberContract {
        shape: CodecMemberShape::Nested,
        encode_road: ENCODE_ROAD,
        decode_road: DECODE_ROAD,
    },
];

/// The locals the rendered decode road declares for itself.
///
/// # Authority
///
/// **A member whose spelling is one of these is refused rather than renamed.**
/// The decode road binds one local per member under the member's OWN spelling, which is what makes the rendered road readable and what lets the assembly call name its arguments the way the owner named its members.
/// A member colliding with one of these would shadow the rendering's own binding, and the road would go on reading a value nobody meant — a defect that compiles.
///
/// Renaming the rendering's locals to something nobody would write is not the repair: an unreadable rendered road is a road nobody can audit, and the collision would still exist for whatever names were chosen instead.
pub const RESERVED_BINDINGS: [&str; 12] = [
    MATERIAL_BINDING,
    REMAINING_BINDING,
    INTO_BINDING,
    NESTED_BINDING,
    COLLECTED_BINDING,
    CANDIDATE_BINDING,
    CHOSEN_BINDING,
    ELECTED_BINDING,
    PRESENT_BINDING,
    CARRIED_BINDING,
    LENGTH_BINDING,
    WIDTH_BINDING,
];

impl CodecIssue {
    /// This row's position in the declared roster, written ahead of the issue's own material.
    ///
    /// Appended and never renumbered: the byte stands inside every identity derived over a refusal that names it.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::PathSegmentsAbsent => 0,
            Self::SegmentNotAnIdentifier { .. } => 1,
            Self::PathSegmentsUnbounded { .. } => 2,
            Self::MemberSpellingAbsent => 3,
            Self::MemberSpellingNotAnIdentifier { .. } => 4,
            Self::MemberSpellingDoubled { .. } => 5,
            Self::MemberShadowsBinding { .. } => 6,
            Self::AssemblyRoadAbsent => 7,
            Self::AssemblyRoadNotAnIdentifier { .. } => 8,
            Self::RefusalSpellingNotAnIdentifier { .. } => 9,
            Self::ModuleSpellingNotAnIdentifier { .. } => 10,
            Self::MembersAbsent => 11,
            Self::MembersUnbounded { .. } => 12,
        }
    }

    /// How what this issue observed differs from the contract that was expected.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::PathSegmentsAbsent
            | Self::MemberSpellingAbsent
            | Self::AssemblyRoadAbsent
            | Self::MembersAbsent => Observed::SeatAbsent,
            Self::PathSegmentsUnbounded { .. } | Self::MembersUnbounded { .. } => {
                Observed::BoundExceeded
            }
            Self::SegmentNotAnIdentifier { .. }
            | Self::MemberSpellingNotAnIdentifier { .. }
            | Self::MemberSpellingDoubled { .. }
            | Self::MemberShadowsBinding { .. }
            | Self::AssemblyRoadNotAnIdentifier { .. }
            | Self::RefusalSpellingNotAnIdentifier { .. }
            | Self::ModuleSpellingNotAnIdentifier { .. } => Observed::ContractDisagreement,
        }
    }

    /// Which class of refusal a summary line opens with where this issue is the first established.
    #[must_use]
    pub const fn class(&self) -> RefusalClass {
        match self {
            Self::PathSegmentsUnbounded { .. } | Self::MembersUnbounded { .. } => {
                RefusalClass::MagnitudeNotHeld
            }
            Self::PathSegmentsAbsent
            | Self::SegmentNotAnIdentifier { .. }
            | Self::MemberSpellingAbsent
            | Self::MemberSpellingNotAnIdentifier { .. }
            | Self::MemberSpellingDoubled { .. }
            | Self::MemberShadowsBinding { .. }
            | Self::AssemblyRoadAbsent
            | Self::AssemblyRoadNotAnIdentifier { .. }
            | Self::RefusalSpellingNotAnIdentifier { .. }
            | Self::ModuleSpellingNotAnIdentifier { .. }
            | Self::MembersAbsent => RefusalClass::DeclarationNotRead,
        }
    }
}

impl fmt::Display for CodecIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PathSegmentsAbsent => into.write_str("a rendered type path names no segment"),
            Self::SegmentNotAnIdentifier { segment } => {
                write!(into, "the path segment {segment} is not one Rust identifier")
            }
            Self::PathSegmentsUnbounded { bound, observed } => write!(
                into,
                "a rendered type path names {observed} segments where {bound} are declared"
            ),
            Self::MemberSpellingAbsent => into.write_str("a codec member states no spelling"),
            Self::MemberSpellingNotAnIdentifier { spelling } => {
                write!(into, "the member spelling {spelling} is not one Rust identifier")
            }
            Self::MemberSpellingDoubled { spelling } => write!(
                into,
                "two members of one shape are both spelled {spelling}, so the decode road would bind one local twice"
            ),
            Self::MemberShadowsBinding { spelling, binding } => write!(
                into,
                "the member {spelling} is spelled like {binding}, which the decode road binds for itself"
            ),
            Self::AssemblyRoadAbsent => into.write_str("a codec assembly road states no spelling"),
            Self::AssemblyRoadNotAnIdentifier { spelling } => {
                write!(into, "the assembly road {spelling} is not one Rust identifier")
            }
            Self::RefusalSpellingNotAnIdentifier { spelling } => write!(
                into,
                "the rendered decode refusal {spelling} is not one Rust identifier"
            ),
            Self::ModuleSpellingNotAnIdentifier { spelling } => write!(
                into,
                "the published module {spelling} is not one Rust identifier"
            ),
            Self::MembersAbsent => into.write_str(
                "a codec shape declares no member, so its decode road could refuse for one reason and admit every other input",
            ),
            Self::MembersUnbounded { bound, observed } => write!(
                into,
                "a codec shape declares {observed} members where {bound} are declared"
            ),
        }
    }
}

impl fmt::Display for CodecError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(into, "{}", self.first_issue())?;
        let further = self.issues().count().saturating_sub(1);
        if further > 0 {
            write!(into, ", and {further} further issues")?;
        }
        if let Capping::Truncated { omitted } = self.capping() {
            write!(into, ", {omitted} of them not carried")?;
        }
        Ok(())
    }
}

impl core::error::Error for CodecError {}

impl Refused for CodecError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = CODEC_DECLARATION_FAMILY;

    fn class(&self) -> RefusalClass {
        self.first_issue().class()
    }

    fn first(&self) -> String {
        self.first_issue().to_string()
    }

    fn observed(&self) -> Observed {
        self.first_issue().observed()
    }

    fn body(&self) -> LineBody {
        let further = self.issues().count().saturating_sub(1);
        let capping = self.capping();
        if further == 0 && capping == Capping::Complete {
            LineBody::SingleCause
        } else {
            LineBody::Body { further, capping }
        }
    }

    /// The issues established beyond the primary cause; the primary is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        self.issues()
            .iter()
            .skip(1)
            .map(CodecIssue::canonical_bytes)
            .collect()
    }

    /// This home declares no repair of its own.
    ///
    /// Every issue is about what the caller's own declaration states, so the repair is that declaration; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}
