//! The constant answers this home's rosters settle, and the contracts its three refusals stand under.
//!
//! Every table is total, so a row admitted later stops the compiler in each of them until somebody says what that row's position, sentence, delivery, and classification are.

use super::{
    ASSEMBLY_FACT, AssemblyError, AssemblyIssue, AxisCargo, CargoAxis, DeclarationError,
    DeclaredCargo, DeliveryForm, ProvedCargo, ShellError, SupportAssembly, SupportCarrier,
};
use crate::bounded::{Bounded, Capping, Overflow};
use crate::diagnostic::{
    ASSEMBLY_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused,
    RenderedMagnitude, Repair, SHELL_FAMILY, SUPPORT_DECLARATION_FAMILY,
};
use crate::identity::{encode_bytes, human_projection};
use crate::kind::{CanonicalContent, Destination, Disposition, Kind, NoQuestions, SoleRole};
use core::fmt;

impl CanonicalContent for SupportAssembly {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_bytes(self.root().as_bytes(), into);
        encode_bytes(self.expectation().as_bytes(), into);
        match self.address() {
            None => into.push(0),
            Some(address) => {
                into.push(1);
                encode_bytes(address.spelling().as_bytes(), into);
            }
        }
        encode_axis(self.declared(), encode_declared, into);
        encode_axis(self.deferred(), encode_proved, into);
        encode_axis(self.bench(), encode_proved, into);
    }
}

fn encode_axis<Material>(
    axis: &AxisCargo<Material>,
    encode: fn(&Material, &mut Vec<u8>),
    into: &mut Vec<u8>,
) {
    match axis {
        AxisCargo::Absent { because } => {
            into.push(0);
            encode_disposition(because, into);
        }
        AxisCargo::Carried(material) => {
            into.push(1);
            let mut encoded = Vec::new();
            encode(material, &mut encoded);
            encode_bytes(&encoded, into);
        }
    }
}

fn encode_disposition(disposition: &Disposition, into: &mut Vec<u8>) {
    match *disposition {
        Disposition::Generated { unit } => {
            into.push(0);
            encode_bytes(unit.as_bytes(), into);
        }
        Disposition::NotApplicable { because } => {
            into.push(1);
            encode_bytes(&because.citation_bytes(), into);
        }
        Disposition::NotRequested { because } => {
            into.push(2);
            encode_bytes(&because.citation_bytes(), into);
        }
        Disposition::UnavailableUnderProfile { profile, because } => {
            into.push(3);
            profile.encode_into(into);
            encode_bytes(&because.citation_bytes(), into);
        }
    }
}

fn encode_declared(cargo: &DeclaredCargo, into: &mut Vec<u8>) {
    encode_bytes(&cargo.matched().canonical_bytes(), into);
    encode_bytes(&cargo.stamped().canonical_bytes(), into);
}

fn encode_proved(cargo: &ProvedCargo, into: &mut Vec<u8>) {
    encode_bytes(cargo.source().as_bytes(), into);
    encode_bytes(cargo.root().as_bytes(), into);
    encode_bytes(cargo.destination().name().as_bytes(), into);
    encode_bytes(cargo.digest().as_bytes(), into);
    encode_bytes(&cargo.cargo().tree().canonical_bytes(), into);
}

impl Kind for SupportCarrier {
    const NAME: &'static str = "support-carrier";

    type Content = SupportAssembly;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl CargoAxis {
    /// The delivery this axis reads a terminal's proved cargo from, where it reads one at all.
    ///
    /// # Authority
    ///
    /// **The mapping is stated once, here, and reading it is what makes "no cargo reaches a second destination" checkable.** A caller names the delivery it read; this table says which one the axis reads from; a disagreement is the typed refusal.
    /// The declaration-site delivery is the case that costs: the ordinary build already compiles its units, so carrying them again into a consumption target compiles them twice.
    ///
    /// The DECLARED axis answers with nothing, and the absence is the honest shape rather than a missing row: that axis carries a body somebody wrote rather than a delivery somebody proved, so there is no delivery for it to have been read from.
    #[must_use]
    pub const fn reads_from(self) -> Option<Destination> {
        match self {
            Self::Declared => None,
            Self::Deferred => Some(Destination::TestCarrier),
            Self::Bench => Some(Destination::BenchCarrier),
        }
    }
}

impl DeliveryForm {
    /// The clause this form's opaque seat is written under.
    ///
    /// The stamped seat's clause is the row's own declared name, so the coupled pair is one row and two readings rather than two tables that agree until one is edited.
    #[must_use]
    pub const fn opaque(self) -> &'static str {
        match self {
            Self::Trials => "deferred",
            Self::Benches => "reporter",
        }
    }
}

impl AssemblyIssue {
    /// This row's position in the declared roster, written ahead of the material it governs.
    ///
    /// Appended and never renumbered: the byte stands inside every related identity derived over an assembly refusal that carries it.
    /// Slot 1 is a hole: the published-expectation row left when its refusal became unmintable, and a hole is cheaper than a shifted preimage.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::RootsDisagree { .. } => 0,
            Self::CargoConsumedTwice { .. } => 2,
            Self::CargoReachesASecondDestination { .. } => 3,
            Self::CargoNotTheSourcesOwn { .. } => 4,
            Self::TwoFormsCarried => 5,
            Self::StampedCargoAbsent { .. } => 6,
        }
    }

    /// Which axis this issue is about, where it is about one.
    ///
    /// It answers with nothing for the issues about the assembly as a WHOLE — the pin it stands under, the terminal delivery two axes read, and the form the axes together name — because there is no single axis to name and electing one would be a stand-in nobody established.
    #[must_use]
    pub const fn axis(&self) -> Option<CargoAxis> {
        match self {
            Self::RootsDisagree { axis, .. }
            | Self::CargoReachesASecondDestination { axis, .. } => Some(*axis),
            Self::CargoConsumedTwice { .. }
            | Self::CargoNotTheSourcesOwn { .. }
            | Self::TwoFormsCarried
            | Self::StampedCargoAbsent { .. } => None,
        }
    }

    /// How what was observed differs from the expected contract, for this row alone.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::RootsDisagree { .. } | Self::CargoNotTheSourcesOwn { .. } => {
                Observed::IdentityDisagreement
            }
            Self::CargoConsumedTwice { .. }
            | Self::CargoReachesASecondDestination { .. }
            | Self::TwoFormsCarried => Observed::ContractDisagreement,
            Self::StampedCargoAbsent { .. } => Observed::SeatAbsent,
        }
    }
}

impl fmt::Display for AssemblyIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootsDisagree { axis, .. } => {
                let named = axis.name();
                write!(
                    into,
                    "the {named} axis carries cargo from a terminal planned over another declaration"
                )
            }
            Self::CargoConsumedTwice { destination, .. } => {
                let delivery = destination.name();
                write!(
                    into,
                    "two axes read one terminal's {delivery} delivery, so one proved cargo is delivered twice"
                )
            }
            Self::CargoReachesASecondDestination { axis, destination } => {
                let named = axis.name();
                let delivery = destination.name();
                write!(
                    into,
                    "the {named} axis read the {delivery} delivery, which is not the one its own row names"
                )
            }
            Self::CargoNotTheSourcesOwn { destination, .. } => {
                let delivery = destination.name();
                write!(
                    into,
                    "the cargo handed for an axis is not the cargo that terminal's {delivery} delivery proved"
                )
            }
            Self::TwoFormsCarried => into.write_str(
                "both proved axes are carried, and one carrier writes one gate invocation under one delivery form",
            ),
            Self::StampedCargoAbsent { form } => {
                let named = form.name();
                write!(
                    into,
                    "the {named} form requires stamped material and this carrier declares none"
                )
            }
        }
    }
}

impl fmt::Display for AssemblyError {
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

impl core::error::Error for AssemblyError {}

impl Refused for AssemblyError {
    const PHASE: Phase = Phase::Assembly;
    const FAMILY: Family = ASSEMBLY_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::CarrierNotAssembled
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
            .map(AssemblyIssue::canonical_bytes)
            .collect()
    }

    /// The one repair, citing this home's own declared fact.
    ///
    /// The law an assembly names is this compiler's rather than the caller's declaration, so the fact is this home's to cite.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::from_array([Repair {
            declared_by: ASSEMBLY_FACT,
            description: human_projection!(
                "one exported carrier delivers one declaration's proved cargo: every axis reads the delivery its own row names, off a terminal planned over the declaration the assembly stands on, and one carrier writes one gate invocation under one delivery form"
            ),
        }])
    }
}

impl ShellError {
    /// This row's position in the declared roster, written ahead of the material it governs.
    ///
    /// Appended and never renumbered, on [`AssemblyIssue::slot`]'s terms.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::NotOneDeclaration { .. } => 0,
            Self::TreeUnbounded { .. } => 1,
        }
    }
}

impl fmt::Display for ShellError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotOneDeclaration { .. } => into.write_str(
                "the carrier's own plan stands over a declaration other than the one this assembly composed",
            ),
            Self::TreeUnbounded { bound, observed } => write!(
                into,
                "the composed carrier passed {}: {observed} offered where {bound} are declared",
                RenderedMagnitude::GeneratedTokens.described()
            ),
        }
    }
}

impl core::error::Error for ShellError {}

impl From<Overflow> for ShellError {
    /// The refusal a composition that outgrew the per-level token magnitude makes.
    ///
    /// The one overflow this road meets, so `?` carries a composition helper's answer straight out of the rendering.
    fn from(overflow: Overflow) -> Self {
        Self::TreeUnbounded {
            bound: overflow.capacity,
            observed: overflow.offered,
        }
    }
}

impl Refused for ShellError {
    const PHASE: Phase = Phase::Assembly;
    const FAMILY: Family = SHELL_FAMILY;

    fn class(&self) -> RefusalClass {
        match self {
            Self::NotOneDeclaration { .. } => RefusalClass::CarrierNotAssembled,
            Self::TreeUnbounded { .. } => RefusalClass::MagnitudeNotHeld,
        }
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        match self {
            Self::NotOneDeclaration { .. } => Observed::IdentityDisagreement,
            Self::TreeUnbounded { .. } => Observed::BoundExceeded,
        }
    }

    /// One cause, always.
    ///
    /// The two rows are dependent and in order, so the road refuses at the first and there is no body behind it for a line to count.
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    /// A single cause enumerates nothing: the primary cause is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    /// The pairing repair is this home's own law; the magnitude is not.
    ///
    /// A token tree that outgrew its bound is a fact about how much a caller asked to be carried, so no fact this home declares repairs it.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        match self {
            Self::NotOneDeclaration { .. } => Bounded::from_array([Repair {
                declared_by: ASSEMBLY_FACT,
                description: human_projection!(
                    "a carrier is rendered from the plan that declares it and from the assembly composed for that same declaration, so a plan and an assembly naming two declarations are refused rather than rendered into one exported name"
                ),
            }]),
            Self::TreeUnbounded { .. } => Bounded::empty(),
        }
    }
}

impl DeclarationError {
    /// This row's position in the declared roster, written ahead of nothing.
    ///
    /// Appended and never renumbered, on [`AssemblyIssue::slot`]'s terms.
    #[must_use]
    pub const fn slot(self) -> u8 {
        match self {
            Self::EmptyNamespace => 0,
            Self::EmptyStem => 1,
            Self::SpellingNotAnIdentifier => 2,
            Self::PathSegmentsAbsent => 3,
            Self::PathSegmentsUnbounded => 4,
        }
    }
}

impl fmt::Display for DeclarationError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.write_str(match self {
            Self::EmptyNamespace => "a name states no owner",
            Self::EmptyStem => "a name states no spelling",
            Self::SpellingNotAnIdentifier => "a rendered spelling is not one Rust identifier",
            Self::PathSegmentsAbsent => {
                "a rendered path names no segment past the crate it is rooted at"
            }
            Self::PathSegmentsUnbounded => {
                "a rendered path carries more segments than the declared magnitude"
            }
        })
    }
}

impl core::error::Error for DeclarationError {}

impl Refused for DeclarationError {
    const PHASE: Phase = Phase::Capture;
    const FAMILY: Family = SUPPORT_DECLARATION_FAMILY;

    fn class(&self) -> RefusalClass {
        RefusalClass::CarrierNotDeclared
    }

    fn first(&self) -> String {
        self.to_string()
    }

    fn observed(&self) -> Observed {
        match self {
            Self::EmptyNamespace | Self::EmptyStem | Self::PathSegmentsAbsent => {
                Observed::SeatAbsent
            }
            Self::SpellingNotAnIdentifier => Observed::ContractDisagreement,
            Self::PathSegmentsUnbounded => Observed::BoundExceeded,
        }
    }

    /// One cause, always.
    ///
    /// The checks are dependent and in a declared order, so exactly one is true of any refused declaration and nothing stands behind it.
    fn body(&self) -> LineBody {
        LineBody::SingleCause
    }

    /// A single cause enumerates nothing: the primary cause is the summary's own subject, never a member of its related set.
    fn related(&self) -> Vec<Vec<u8>> {
        Vec::new()
    }

    /// This home declares no repair for it.
    ///
    /// Every row is about what the caller's own declaration states, so the repair is that declaration; a sentence composed here would be this compiler citing a fact nobody declared.
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::empty()
    }
}
