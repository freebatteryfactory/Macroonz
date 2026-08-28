//! Canonical cargo encoding for assembly.
use super::{AxisCargo, DeclaredCargo, ProvedCargo};
use crate::identity::encode_bytes;
use crate::kind::Disposition;
pub(in crate::support) fn encode_axis<M>(
    axis: &AxisCargo<M>,
    encode: fn(&M, &mut Vec<u8>),
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
pub(in crate::support) fn encode_declared(cargo: &DeclaredCargo, into: &mut Vec<u8>) {
    encode_bytes(&cargo.matched().canonical_bytes(), into);
    encode_bytes(&cargo.stamped().canonical_bytes(), into);
}
pub(in crate::support) fn encode_proved(cargo: &ProvedCargo, into: &mut Vec<u8>) {
    encode_bytes(cargo.source().as_bytes(), into);
    encode_bytes(cargo.root().as_bytes(), into);
    encode_bytes(cargo.destination().name().as_bytes(), into);
    encode_bytes(cargo.digest().as_bytes(), into);
    encode_bytes(&cargo.cargo().tree().canonical_bytes(), into);
}
