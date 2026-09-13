use macroonz::harness::network::{NetworkSchedule, SimulationReproduction, TranscriptPack};

pub(super) struct Driven {
    pub(super) pack: TranscriptPack,
    pub(super) schedule: NetworkSchedule,
    pub(super) reproduction: SimulationReproduction,
}
