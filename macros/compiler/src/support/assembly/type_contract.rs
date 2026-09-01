//! Assembly identity and diagnostic contracts.
use super::{ASSEMBLY_FACT, AssemblyError, AssemblyIssue, SupportAssembly};
use crate::bounded::{Bounded, Capping};
use crate::diagnostic::{
    ASSEMBLY_FAMILY, Family, LineBody, Observed, Phase, REPAIR_LIMIT, RefusalClass, Refused, Repair,
};
use crate::identity::{encode_bytes, human_projection};
use crate::kind::CanonicalContent;
use crate::support::cargo::{CargoAxis, encode_axis, encode_declared, encode_proved};
use core::fmt;
impl CanonicalContent for SupportAssembly {
    /// Appends the root, expectation, address, and axes, followed by the optional helper identity and declaring-binding extension.
    /// Harness-only assemblies retain their established bytes; only a required declaring binding appends its tagged extension.
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
        if let Some(helper) = self.helper() {
            into.push(1);
            encode_bytes(helper.as_bytes(), into);
        }
        if self.declaring_binding() == crate::support::DeclaringBinding::Required {
            into.push(2);
        }
    }
}
impl AssemblyIssue {
    /// Reads the stable canonical slot.
    #[must_use]
    pub const fn slot(&self) -> u8 {
        match self {
            Self::RootsDisagree { .. } => 0,
            Self::DeclaredAxisRequiresStampedCargo => 7,
            Self::CargoConsumedTwice { .. } => 2,
            Self::CargoReachesASecondDestination { .. } => 3,
            Self::CargoNotTheSourcesOwn { .. } => 4,
            Self::TwoFormsCarried => 5,
            Self::StampedCargoAbsent { .. } => 6,
        }
    }
    /// Reads the affected axis where one exists.
    #[must_use]
    pub const fn axis(&self) -> Option<CargoAxis> {
        match self {
            Self::RootsDisagree { axis, .. }
            | Self::CargoReachesASecondDestination { axis, .. } => Some(*axis),
            Self::DeclaredAxisRequiresStampedCargo => Some(CargoAxis::Declared),
            Self::CargoConsumedTwice { .. }
            | Self::CargoNotTheSourcesOwn { .. }
            | Self::TwoFormsCarried
            | Self::StampedCargoAbsent { .. } => None,
        }
    }
    /// Reads the observation classification.
    #[must_use]
    pub const fn observed(&self) -> Observed {
        match self {
            Self::RootsDisagree { .. } | Self::CargoNotTheSourcesOwn { .. } => {
                Observed::IdentityDisagreement
            }
            Self::DeclaredAxisRequiresStampedCargo
            | Self::CargoConsumedTwice { .. }
            | Self::CargoReachesASecondDestination { .. }
            | Self::TwoFormsCarried => Observed::ContractDisagreement,
            Self::StampedCargoAbsent { .. } => Observed::SeatAbsent,
        }
    }
}
impl fmt::Display for AssemblyIssue {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
        Self::RootsDisagree { axis, .. } => write!(into, "the {} axis carries cargo from a terminal planned over another declaration", axis.name()),
        Self::DeclaredAxisRequiresStampedCargo => into.write_str("the declared axis requires stamped declaration cargo and cannot carry opaque deferred cargo"),
        Self::CargoConsumedTwice { destination, .. } => write!(into, "two axes read one terminal's {} delivery, so one proved cargo is delivered twice", destination.name()),
        Self::CargoReachesASecondDestination { axis, destination } => write!(into, "the {} axis read the {} delivery, which is not the one its own row names", axis.name(), destination.name()),
        Self::CargoNotTheSourcesOwn { destination, .. } => write!(into, "the cargo handed for an axis is not the cargo that terminal's {} delivery proved", destination.name()),
        Self::TwoFormsCarried => into.write_str("both proved axes are carried, and one carrier writes one gate invocation under one delivery form"),
        Self::StampedCargoAbsent { form } => write!(into, "the {} form requires stamped material and this carrier declares none", form.name()),
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
    fn related(&self) -> Vec<Vec<u8>> {
        self.issues()
            .iter()
            .skip(1)
            .map(AssemblyIssue::canonical_bytes)
            .collect()
    }
    fn repairs(&self) -> Bounded<Repair, REPAIR_LIMIT> {
        Bounded::from_array([Repair {
            declared_by: ASSEMBLY_FACT,
            description: human_projection!(
                "one exported carrier delivers one declaration's proved cargo: every axis reads the delivery its own row names, off a terminal planned over the declaration the assembly stands on, and one carrier writes one gate invocation under one delivery form"
            ),
        }])
    }
}
