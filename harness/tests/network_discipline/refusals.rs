//! Claim: malformed simulation declarations refuse at their strongest guard in documented priority order.
//! Subject: topology, span, schedule, campaign, selection, sim opening, and send construction.
//! Population: every public simulation refusal arm.
//! Hostile control: compounded invalid inputs distinguish first-failure priority from set membership.
//! Denominator: all public simulation guard refusals.
//! Evidence ceiling: compile-time privacy is observed separately by the compile-refusal target.

use super::support::*;

/// A malformed topology refuses at exactly the clause it breaks.
#[test]
fn a_malformed_topology_refuses_at_its_clause() -> Result<(), LaneFailure> {
    let a = client()?;
    let b = server()?;
    let stranger = NodeRef::declared(name("stranger")?);
    let wire = Link::between(a, b);
    assert_eq!(
        Topology::declared(Vec::new(), vec![wire]),
        Err(TopologyRefusal::NoNode)
    );
    assert_eq!(
        Topology::declared(vec![a, a], vec![wire]),
        Err(TopologyRefusal::DuplicateNode(a))
    );
    assert_eq!(
        Topology::declared(vec![a, b], Vec::new()),
        Err(TopologyRefusal::NoLink)
    );
    assert_eq!(
        Topology::declared(vec![a, b], vec![wire, wire]),
        Err(TopologyRefusal::DuplicateLink(wire))
    );
    assert_eq!(
        Topology::declared(vec![a, b], vec![Link::between(a, stranger)]),
        Err(TopologyRefusal::LinkForeignNode { node: stranger })
    );
    Ok(())
}

/// A schedule or campaign that declares pressure incoherently refuses at its clause.
#[test]
fn an_incoherent_discipline_refuses_at_its_clause() -> Result<(), LaneFailure> {
    assert_eq!(TickSpan::declared(0u32), Err(TickSpanRefusal::ZeroTicks));
    let wire = forward()?;
    let drop_fault = LinkFault::DropAt {
        position: SendOrdinal::at(0u32),
    };
    assert_eq!(
        NetworkSchedule::declared(
            name("twice")?,
            vec![
                LinkDiscipline::declared(wire, vec![drop_fault]),
                LinkDiscipline::declared(wire, vec![drop_fault]),
            ],
        ),
        Err(NetworkScheduleRefusal::DuplicateDiscipline(wire))
    );
    assert_eq!(
        NetworkSchedule::declared(
            name("hollow")?,
            vec![LinkDiscipline::declared(wire, Vec::new())],
        ),
        Err(NetworkScheduleRefusal::EmptyDiscipline(wire))
    );
    assert_eq!(
        NetworkSchedule::declared(
            name("never-open")?,
            vec![LinkDiscipline::declared(
                wire,
                vec![LinkFault::Partition {
                    opens: Tick::at(5u64),
                    heals: Tick::at(5u64),
                }],
            )],
        ),
        Err(NetworkScheduleRefusal::EmptyPartition { link: wire })
    );
    assert_eq!(
        NetworkCampaign::declared(Vec::new()),
        Err(NetworkCampaignRefusal::NoSchedule)
    );
    let twin = name("twin")?;
    assert_eq!(
        NetworkCampaign::declared(vec![
            NetworkSchedule::declared(twin, Vec::new())?,
            NetworkSchedule::declared(twin, Vec::new())?,
        ]),
        Err(NetworkCampaignRefusal::DuplicateSchedule(twin))
    );
    assert_eq!(
        NetworkCampaign::declared(vec![quiet_control()?]),
        Err(NetworkCampaignRefusal::NoFaultDeclared)
    );
    let lawful = NetworkCampaign::declared(vec![
        quiet_control()?,
        NetworkSchedule::declared(
            name("real")?,
            vec![LinkDiscipline::declared(wire, vec![drop_fault])],
        )?,
    ])?;
    assert_eq!(
        lawful.select(name("absent")?).err(),
        Some(NetworkSelectionRefusal::ScheduleAbsent(name("absent")?))
    );
    Ok(())
}

/// A sim refuses a schedule outside its topology, and a send outside its links.
#[test]
fn a_sim_refuses_foreign_disciplines_and_undeclared_links() -> Result<(), LaneFailure> {
    let stranger_link = Link::between(server()?, server()?);
    let foreign = NetworkSchedule::declared(
        name("foreign")?,
        vec![LinkDiscipline::declared(
            stranger_link,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, foreign])?;
    assert_eq!(
        SimNet::<Message>::declared(pair_topology()?, campaign.select(name("foreign")?)?).err(),
        Some(SimNetRefusal::DisciplineForeignLink {
            link: stranger_link
        })
    );
    let mut sim = SimNet::declared(pair_topology()?, campaign.select(name("quiet-control")?)?)?;
    assert_eq!(
        sim.send(stranger_link, Message::Request { id: 1u64 }).err(),
        Some(SendRefusal::LinkUndeclared(stranger_link))
    );
    Ok(())
}

/// Compounded invalid declarations refuse in the documented first-failure order.
#[test]
fn compounded_invalid_declarations_keep_refusal_priority() -> Result<(), LaneFailure> {
    let a = client()?;
    let b = server()?;
    let wire = Link::between(a, b);
    assert_eq!(
        Topology::declared(vec![a, a], Vec::new()),
        Err(TopologyRefusal::DuplicateNode(a))
    );

    let drop_fault = LinkFault::DropAt {
        position: SendOrdinal::at(0u32),
    };
    assert_eq!(
        NetworkSchedule::declared(
            name("priority")?,
            vec![
                LinkDiscipline::declared(wire, vec![drop_fault]),
                LinkDiscipline::declared(wire, Vec::new()),
            ],
        ),
        Err(NetworkScheduleRefusal::DuplicateDiscipline(wire))
    );

    let twin = name("twin-priority")?;
    assert_eq!(
        NetworkCampaign::declared(vec![
            NetworkSchedule::declared(twin, Vec::new())?,
            NetworkSchedule::declared(twin, Vec::new())?,
        ]),
        Err(NetworkCampaignRefusal::DuplicateSchedule(twin))
    );
    Ok(())
}
