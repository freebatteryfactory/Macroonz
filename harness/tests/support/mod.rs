//! Package-local construction shared by independent harness integration lanes.

mod exploration_lane_failure;

pub(crate) use exploration_lane_failure::declare_exploration_lane_failure;
