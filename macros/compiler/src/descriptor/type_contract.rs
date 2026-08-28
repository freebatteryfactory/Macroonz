//! The descriptor home's stated tables: how each refusal reads for a person, what it observed, the bytes it is committed to, and the one projection a helper grammar refuses through.

use super::{
    CaptureCause, CaptureIssue, CompositionError, CompositionIssue, DESCRIPTOR_MEANING_FACT,
    DeclarationError, HelperRefusal, RENDERED_SPELLING_FACT,
};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    DECLARATION_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    Repair,
};
use crate::identity::{encode_bytes, human_projection};

impl CaptureCause {
    /// How this cause reads for a person.
    #[must_use]
    pub const fn described(self) -> &'static str {
        match self {
            Self::HelperDoubled => "the declaration carries the helper more than once",
            Self::BodyAbsent => "the helper states no body",
            Self::ClauseUnread => "a clause is not one key and one value",
            Self::ClauseUndeclared => "a clause is not one the grammar declares",
            Self::ClauseDoubled => "one clause is stated twice",
            Self::ClauseAbsent => "a required clause is absent",
            Self::ReferenceUnread => "a value is not one namespaced reference",
            Self::RosterUnread => "a value is not one bracketed roster",
            Self::GroupUnread => "a value is not one named group",
            Self::RowUnread => "a row is not one name and one clause body",
            Self::MappingUnread => "a mapping is not one fact and one claim",
            Self::PermissionUnread => "a permission is not one claim and one family roster",
            Self::PathUnread => "a value is not one declared Rust item path",
            Self::ItemUnread => "the item the helper sits on states no declared order",
            Self::OrderUnpressable => "the declared order has fewer than two members to transpose",
            Self::ChoiceUnread => "a choice is not one bare name",
            Self::ChoiceDoubled => "one name is chosen twice",
            Self::NameUnshadowed => "a chosen name is not one the shadow roster covers",
            Self::NothingChosen => "the declaration chooses no name at all",
            Self::PhraseUnread => "a fault phrase is not one this grammar reads",
            Self::EndpointUnknown => "a phrase names a node or link the declaration never declared",
            Self::NumberBeyondSeat => "an authored number outruns the width of its seat",
            Self::NameReserved => {
                "a declared name is one the language or the generated module already owns"
            }
            Self::SeparatorDangling => "a separator stands where no clause does",
        }
    }

    /// How what was observed differs from the grammar that was expected.
    ///
    /// A clause the grammar does not declare and a value whose shape it cannot read are contract disagreements: the declaration is well formed Rust and says something this grammar does not admit.
    /// A clause that is absent is a seat that was not filled, and a clause stated twice is one key answering for two values.
    #[must_use]
    pub const fn classified(self) -> Observed {
        match self {
            Self::BodyAbsent
            | Self::ClauseAbsent
            | Self::OrderUnpressable
            | Self::NothingChosen => Observed::SeatAbsent,
            Self::HelperDoubled | Self::ClauseDoubled | Self::ChoiceDoubled => {
                Observed::IdentityDisagreement
            }
            Self::ClauseUnread
            | Self::ClauseUndeclared
            | Self::ReferenceUnread
            | Self::RosterUnread
            | Self::GroupUnread
            | Self::RowUnread
            | Self::MappingUnread
            | Self::PermissionUnread
            | Self::PathUnread
            | Self::ItemUnread
            | Self::ChoiceUnread
            | Self::NameUnshadowed
            | Self::PhraseUnread
            | Self::EndpointUnknown
            | Self::NumberBeyondSeat
            | Self::NameReserved
            | Self::SeparatorDangling => Observed::ContractDisagreement,
        }
    }
}

impl DeclarationError {
    /// This refusal's position in its own roster, appended and never renumbered.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match *self {
            Self::NamespaceEmpty => 0,
            Self::StemEmpty => 1,
            Self::NotAnIdentifier => 2,
            Self::Absent { .. } => 3,
            Self::Doubled { .. } => 4,
            Self::Unbounded { .. } => 5,
            Self::NotACurve { .. } => 6,
        }
    }

    /// How what was observed differs from the vocabulary that was expected.
    #[must_use]
    pub const fn classified(&self) -> Observed {
        match *self {
            Self::NamespaceEmpty | Self::StemEmpty | Self::Absent { .. } => Observed::SeatAbsent,
            Self::NotAnIdentifier | Self::NotACurve { .. } => Observed::ContractDisagreement,
            Self::Doubled { .. } => Observed::IdentityDisagreement,
            Self::Unbounded { .. } => Observed::BoundExceeded,
        }
    }

    /// This refusal's canonical bytes: its own position, then the members it carries.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.slot()];
        match *self {
            Self::NamespaceEmpty | Self::StemEmpty | Self::NotAnIdentifier => {}
            Self::Absent { seat } | Self::Doubled { seat } => {
                encode_bytes(seat.name().as_bytes(), &mut bytes);
            }
            Self::Unbounded {
                seat,
                bound,
                observed,
            } => {
                encode_bytes(seat.name().as_bytes(), &mut bytes);
                bytes.extend_from_slice(&bound.to_be_bytes());
                bytes.extend_from_slice(&observed.to_be_bytes());
            }
            Self::NotACurve { observed } => bytes.extend_from_slice(&observed.to_be_bytes()),
        }
        bytes
    }
}

impl core::fmt::Display for DeclarationError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::NamespaceEmpty => into.write_str("a namespaced name states no owner"),
            Self::StemEmpty => into.write_str("a namespaced name states no spelling"),
            Self::NotAnIdentifier => {
                into.write_str("a rendered spelling is not one Rust identifier")
            }
            Self::Absent { seat } => {
                let seat = seat.name();
                write!(into, "the declaration states no {seat}")
            }
            Self::Doubled { seat } => {
                let seat = seat.name();
                write!(into, "one {seat} of the declaration is stated twice")
            }
            Self::Unbounded {
                seat,
                bound,
                observed,
            } => {
                let seat = seat.name();
                write!(
                    into,
                    "the declaration states {observed} {seat} rows where at most {bound} fit"
                )
            }
            Self::NotACurve { observed } => write!(
                into,
                "an input-size axis states {observed} sizes where a growth class needs at least two"
            ),
        }
    }
}

impl core::error::Error for DeclarationError {}

impl Refused for DeclarationError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = DECLARATION_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::CarrierNotDeclared
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        self.classified()
    }

    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    /// A single cause enumerates nothing: the primary cause is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::from_array([Repair {
            declared_by: RENDERED_SPELLING_FACT,
            description: human_projection!(
                "state the value in the vocabulary the grammar declares: a namespaced name states an owner and a spelling, a rendered spelling is one Rust identifier, and every roster stands inside its declared magnitude"
            ),
        }])
    }
}

impl HelperRefusal {
    /// Which class of refusal the summary line opens with.
    #[must_use]
    pub const fn class(&self) -> RefusalClass {
        match self.issue() {
            CaptureIssue::Grammar { .. } => RefusalClass::DeclarationNotRead,
            CaptureIssue::Vocabulary { .. } => RefusalClass::CarrierNotDeclared,
        }
    }

    /// How what was observed differs from the grammar that was expected.
    #[must_use]
    pub const fn classified(&self) -> Observed {
        match self.issue() {
            CaptureIssue::Grammar { cause } => cause.classified(),
            CaptureIssue::Vocabulary { refusal } => refusal.classified(),
        }
    }

    /// The established refusal, stated in full, naming the helper the author wrote.
    #[must_use]
    pub fn first(&self) -> String {
        let attribute = self.grammar().attribute;
        match self.issue() {
            CaptureIssue::Grammar { cause } => {
                let cause = cause.described();
                format!("in `{attribute}`: {cause}")
            }
            CaptureIssue::Vocabulary { refusal } => format!("in `{attribute}`: {refusal}"),
        }
    }

    /// This refusal's canonical bytes: which reading refused, the helper it was reading, and that reading's own material.
    ///
    /// The reading rides ahead of the material, so a malformed clause and a doubled role can never derive one related identity inside one family.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        encode_bytes(self.grammar().attribute.as_bytes(), &mut bytes);
        match self.issue() {
            CaptureIssue::Grammar { cause } => {
                bytes.push(0);
                encode_bytes(cause.name().as_bytes(), &mut bytes);
            }
            CaptureIssue::Vocabulary { refusal } => {
                bytes.push(1);
                encode_bytes(&refusal.canonical_bytes(), &mut bytes);
            }
        }
        bytes
    }

    /// The owner-declared repair that applies to the reading that refused.
    #[must_use]
    pub fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        match self.issue() {
            CaptureIssue::Grammar { .. } => Bounded::from_array([Repair {
                declared_by: DESCRIPTOR_MEANING_FACT,
                description: human_projection!(
                    "write the clause the helper's grammar declares: a descriptor declaration states descriptor meaning, and every fact about the producer's own act is composed inside the rendering"
                ),
            }]),
            CaptureIssue::Vocabulary { .. } => Bounded::from_array([Repair {
                declared_by: RENDERED_SPELLING_FACT,
                description: human_projection!(
                    "state the value in the vocabulary the grammar declares: a namespaced name states an owner and a spelling, and a rendered spelling is one Rust identifier"
                ),
            }]),
        }
    }
}

impl core::fmt::Display for HelperRefusal {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        into.write_str(&self.first())
    }
}

impl core::error::Error for HelperRefusal {}

impl CompositionIssue {
    /// This issue's position in its own roster, appended and never renumbered.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::ProviderDoubled { .. } => 0,
            Self::Declaration { .. } => 1,
        }
    }

    /// Which class of refusal this issue belongs to.
    #[must_use]
    pub fn class(&self) -> RefusalClass {
        match self {
            Self::ProviderDoubled { .. } => RefusalClass::CarrierNotDeclared,
            Self::Declaration { refusal } => <DeclarationError as Refused>::class(refusal),
        }
    }

    /// How what was observed differs from the declared composition contract.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::ProviderDoubled { .. } => Observed::IdentityDisagreement,
            Self::Declaration { refusal } => refusal.classified(),
        }
    }

    /// This issue's canonical bytes: its own position, then the complete identity or declaration refusal it carries.
    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.slot()];
        match self {
            Self::ProviderDoubled { provider } => {
                encode_bytes(&provider.citation_bytes(), &mut bytes);
            }
            Self::Declaration { refusal } => {
                encode_bytes(&refusal.canonical_bytes(), &mut bytes);
            }
        }
        bytes
    }
}

impl core::fmt::Display for CompositionIssue {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::ProviderDoubled { provider } => {
                let subject = provider.subject;
                write!(into, "the provider under `{subject}` is declared twice")
            }
            Self::Declaration { refusal } => write!(into, "{refusal}"),
        }
    }
}

impl core::fmt::Display for CompositionError {
    fn fmt(&self, into: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let first = self.first_issue();
        write!(into, "{first}")?;
        let further = self.issues().count().saturating_sub(1);
        if further > 0 {
            write!(into, ", and {further} further issues")?;
        }
        match self.capping() {
            Capping::Complete => Ok(()),
            Capping::Truncated { omitted } => write!(
                into,
                "; {omitted} of them do not fit the declared issue bound"
            ),
        }
    }
}

impl core::error::Error for CompositionError {}

impl Refused for CompositionError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = DECLARATION_FAMILY;

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
            .map(CompositionIssue::canonical_bytes)
            .collect()
    }

    /// A shared declaration refusal retains its shared repair, while a doubled provider cites no new owner fact.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        match self.first_issue() {
            CompositionIssue::ProviderDoubled { .. } => Bounded::empty(),
            CompositionIssue::Declaration { refusal } => {
                <DeclarationError as Refused>::repairs(refusal)
            }
        }
    }
}
