//! The generated fault arms shared by direct descriptor projections.

/// The shared name-refusal arm.
const NAME_REFUSAL: (&str, &[&str], &str) = (
    "Name",
    &["descriptor", "NameRefusal"],
    "A declared name was refused by the name vocabulary.",
);

/// Every fault arm emitted for a concurrency declaration.
pub(crate) const CONCURRENCY_FAULT_ARMS: [(&str, &[&str], &str); 3] = [
    NAME_REFUSAL,
    (
        "Bound",
        &["interleave", "ExplorationBoundRefusal"],
        "The declared bound was refused by its own guard.",
    ),
    (
        "Exploration",
        &["interleave", "ExplorationRefusal"],
        "The exploration itself refused to run.",
    ),
];

/// Every fault arm emitted for a network declaration.
pub(crate) const NETWORK_FAULT_ARMS: [(&str, &[&str], &str); 4] = [
    NAME_REFUSAL,
    (
        "Topology",
        &["network", "TopologyRefusal"],
        "The declared topology was refused by its own guard.",
    ),
    (
        "Span",
        &["network", "TickSpanRefusal"],
        "A declared delay span was refused by its own guard.",
    ),
    (
        "Schedule",
        &["network", "NetworkScheduleRefusal"],
        "A declared schedule was refused by its own guard.",
    ),
];
