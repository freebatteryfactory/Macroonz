//! Retained topology, schedule, action and delivery vocabulary.

use crate::harness::network::{
    DeliveryCopy, Link, LinkFault, NetworkSchedule, SimulationAction, Topology,
    TranscriptSourceClaim,
};
use crate::presentation::{
    context,
    value::{array, hex, object, tagged},
};
use serde_json::Value;

pub(super) fn link(value: Link) -> Value {
    object([
        ("from", context::declared_name(value.from().name())),
        ("to", context::declared_name(value.to().name())),
    ])
}

pub(super) fn topology(value: &Topology) -> Value {
    object([
        (
            "nodes",
            array(
                value
                    .nodes()
                    .iter()
                    .map(|node| context::declared_name(node.name())),
            ),
        ),
        ("links", array(value.links().iter().copied().map(link))),
    ])
}

pub(super) fn schedule(value: &NetworkSchedule) -> Value {
    object([
        ("name", context::declared_name(value.name())),
        (
            "disciplines",
            array(value.disciplines().iter().map(|discipline| {
                object([
                    ("link", link(discipline.link())),
                    (
                        "faults",
                        array(discipline.faults().iter().copied().map(fault)),
                    ),
                ])
            })),
        ),
    ])
}

fn fault(value: LinkFault) -> Value {
    match value {
        LinkFault::DropAt { position } => tagged("drop-at", position.ordinal().into()),
        LinkFault::DelayAt { position, ticks } => tagged(
            "delay-at",
            object([
                ("position", position.ordinal().into()),
                ("ticks", ticks.ticks().into()),
            ]),
        ),
        LinkFault::DuplicateAt { position } => tagged("duplicate-at", position.ordinal().into()),
        LinkFault::Partition { opens, heals } => tagged(
            "partition",
            object([
                ("opens", opens.ordinal().into()),
                ("heals", heals.ordinal().into()),
            ]),
        ),
    }
}

pub(super) fn action(value: &SimulationAction) -> Value {
    match value {
        SimulationAction::Send {
            link: route,
            payload,
        } => tagged(
            "send",
            object([("link", link(*route)), ("payload", hex(payload))]),
        ),
        SimulationAction::Advance => tagged("advance", Value::Null),
    }
}

pub(super) fn copy(value: DeliveryCopy) -> &'static str {
    match value {
        DeliveryCopy::Original => "original",
        DeliveryCopy::Duplicate => "duplicate",
    }
}

pub(super) fn source(value: TranscriptSourceClaim) -> &'static str {
    match value {
        TranscriptSourceClaim::Simulated => "simulated",
        TranscriptSourceClaim::RecordedLive => "recorded-live",
    }
}
