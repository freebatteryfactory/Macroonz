//! The network declaration home's stated contracts: what the kind is, and how its capture refusal composes into a diagnostic.

use super::{
    DisciplineRow, FaultRow, NetworkCaptureError, NetworkDeclaration, NetworkModule, ScheduleRow,
};
use crate::diagnostic::NETWORK_HELPER_FAMILY;
use crate::identity::{encode_bytes, encode_length};
use crate::kind::{CanonicalContent, Kind, NoQuestions, SoleRole};

impl CanonicalContent for NetworkDeclaration {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        encode_length(self.harness().segments().count(), into);
        for segment in self.harness().segments() {
            encode_bytes(segment.as_bytes(), into);
        }
        encode_bytes(self.module().as_bytes(), into);
        encode_bytes(self.namespace().as_bytes(), into);
        encode_length(self.nodes().len(), into);
        for node in self.nodes() {
            encode_bytes(node.as_bytes(), into);
        }
        encode_length(self.links().len(), into);
        for link in self.links() {
            let mut encoded = Vec::new();
            encode_bytes(link.name().as_bytes(), &mut encoded);
            encode_bytes(link.from().as_bytes(), &mut encoded);
            encode_bytes(link.to().as_bytes(), &mut encoded);
            encode_bytes(&encoded, into);
        }
        encode_length(self.schedules().len(), into);
        for schedule in self.schedules() {
            encode_schedule(schedule, into);
        }
    }
}

fn encode_schedule(schedule: &ScheduleRow, into: &mut Vec<u8>) {
    let mut encoded = Vec::new();
    encode_bytes(schedule.name().as_bytes(), &mut encoded);
    encode_length(schedule.disciplines().len(), &mut encoded);
    for discipline in schedule.disciplines() {
        let mut member = Vec::new();
        encode_discipline(discipline, &mut member);
        encode_bytes(&member, &mut encoded);
    }
    encode_bytes(&encoded, into);
}

fn encode_discipline(discipline: &DisciplineRow, into: &mut Vec<u8>) {
    encode_bytes(discipline.link().name().as_bytes(), into);
    encode_length(discipline.faults().len(), into);
    for fault in discipline.faults() {
        encode_fault(fault, into);
    }
}

fn encode_fault(fault: &FaultRow, into: &mut Vec<u8>) {
    match *fault {
        FaultRow::Drop { at } => {
            into.push(0);
            into.extend_from_slice(&at.to_be_bytes());
        }
        FaultRow::Delay { at, by } => {
            into.push(1);
            into.extend_from_slice(&at.to_be_bytes());
            into.extend_from_slice(&by.to_be_bytes());
        }
        FaultRow::Duplicate { at } => {
            into.push(2);
            into.extend_from_slice(&at.to_be_bytes());
        }
        FaultRow::Partition { from, until } => {
            into.push(3);
            into.extend_from_slice(&from.to_be_bytes());
            into.extend_from_slice(&until.to_be_bytes());
        }
    }
}

impl Kind for NetworkModule {
    const NAME: &'static str = "network-module";
    type Content = NetworkDeclaration;
    type Role = SoleRole;
    type Question = NoQuestions;
}

crate::descriptor::impl_helper_capture_contract!(
    NetworkCaptureError,
    NETWORK_HELPER_FAMILY,
    canonical
);
