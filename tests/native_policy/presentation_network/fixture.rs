use super::types::Driven;
use macroonz::harness::descriptor::NamespacedName;
use macroonz::harness::network::{
    self, Link, LinkDiscipline, LinkFault, NetworkCampaign, NetworkSchedule, NodeRef, SendOrdinal,
    SimNet, Tick, TickSpan, Topology, TranscriptLimits,
};
use macroonz::harness::report::archive::ArchiveLimits;

pub(super) const PAYLOAD: &[u8] = &[0, 255, b'<', b'&', b'>'];
pub(super) const LIMITS: TranscriptLimits =
    TranscriptLimits::declared(ArchiveLimits::declared(16_384, 1024), 64, 64);

pub(super) fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

pub(super) fn name(stem: &'static str) -> Result<NamespacedName, String> {
    NamespacedName::named("display.network", stem).map_err(debug)
}

pub(super) fn route() -> Result<Link, String> {
    Ok(Link::between(
        NodeRef::declared(name("sender")?),
        NodeRef::declared(name("receiver")?),
    ))
}

pub(super) fn topology() -> Result<Topology, String> {
    let link = route()?;
    Topology::declared(
        vec![link.to(), link.from(), NodeRef::declared(name("unused")?)],
        vec![Link::between(link.to(), link.from()), link],
    )
    .map_err(debug)
}

pub(super) fn driven(extra_advances: usize) -> Result<Driven, String> {
    let link = route()?;
    let schedule = NetworkSchedule::declared(
        name("mixed")?,
        vec![LinkDiscipline::declared(
            link,
            vec![
                LinkFault::DelayAt {
                    position: SendOrdinal::at(1),
                    ticks: TickSpan::declared(2).map_err(debug)?,
                },
                LinkFault::DuplicateAt {
                    position: SendOrdinal::at(1),
                },
                LinkFault::DropAt {
                    position: SendOrdinal::at(0),
                },
                LinkFault::Partition {
                    opens: Tick::at(1),
                    heals: Tick::at(2),
                },
            ],
        )],
    )
    .map_err(debug)?;
    let campaign = NetworkCampaign::declared(vec![schedule.clone()]).map_err(debug)?;
    let mut sim = SimNet::declared(topology()?, campaign.select(name("mixed")?).map_err(debug)?)
        .map_err(debug)?;
    sim.send(link, b"lost".to_vec()).map_err(debug)?;
    sim.send(link, PAYLOAD.to_vec()).map_err(debug)?;
    assert!(sim.advance().is_empty());
    sim.send(link, b"partitioned".to_vec()).map_err(debug)?;
    assert!(sim.advance().is_empty());
    sim.send(Link::between(link.to(), link.from()), Vec::new())
        .map_err(debug)?;
    assert_eq!(sim.advance().len(), 3);
    assert!(sim.advance().is_empty());
    for _ in 0..extra_advances {
        assert!(sim.advance().is_empty());
    }
    assert_eq!(sim.census().sends(), 4);
    assert_eq!(sim.census().dropped_by_discipline(), 1);
    assert_eq!(sim.census().dropped_by_partition(), 1);
    let (pack, reproduction) = network::simulated(&sim, Vec::clone).map_err(debug)?;
    Ok(Driven {
        pack,
        schedule,
        reproduction,
    })
}
