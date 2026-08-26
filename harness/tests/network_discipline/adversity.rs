//! Claim: declared link faults change only the named sends and deterministic replay of a drive remains exact.
//! Subject: the public topology, campaign, selection, send, advance, and census roads.
//! Population: duplicate, drop, partition, delay, retry, and quiet-control schedules over a two-node protocol.
//! Reversal: naive versus deduplicating servers, no retry versus retry, outage versus quiet control, and two identical drives.
//! Denominator: every fault arm and every send fate exposed by the public simulation vocabulary.
//! Evidence ceiling: these finite drives do not characterize arbitrary distributed protocols.

use super::support::*;

/// A duplicated request double-applies on a naive server, the control stays calm, and a deduplicating server survives the same schedule.
#[test]
fn a_duplicate_breaks_at_most_once_and_deduplication_restores_it() -> Result<(), LaneFailure> {
    let duplicate = NetworkSchedule::declared(
        name("duplicate-the-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DuplicateAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, duplicate])?;
    let contract = once_contract()?;
    let stressed = served(
        campaign.select(name("duplicate-the-request")?)?,
        ServerKind::Naive,
    )?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &stressed) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), AT_MOST_ONCE);
    let calm = served(campaign.select(name("quiet-control")?)?, ServerKind::Naive)?;
    assert_eq!(calm.len(), 1usize);
    assert_eq!(
        holds_over_history(&contract, &calm),
        TrialConclusion::Passed
    );
    let hardened = served(
        campaign.select(name("duplicate-the-request")?)?,
        ServerKind::Deduplicating,
    )?;
    assert_eq!(
        holds_over_history(&contract, &hardened),
        TrialConclusion::Passed
    );
    Ok(())
}

/// A dropped request starves the reply claim, and one retry recovers it under the same schedule.
#[test]
fn a_drop_starves_the_reply_and_a_retry_recovers_it() -> Result<(), LaneFailure> {
    let drop_first = NetworkSchedule::declared(
        name("drop-the-first-request")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, drop_first])?;
    let contract = reply_contract()?;
    let (starved, _starved_sim) = driven(
        campaign.select(name("drop-the-first-request")?)?,
        0u32,
        2u64,
    )?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &starved) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), REPLY_ARRIVES);
    let (recovered, _recovered_sim) = driven(
        campaign.select(name("drop-the-first-request")?)?,
        1u32,
        2u64,
    )?;
    assert_eq!(
        holds_over_history(&contract, &recovered),
        TrialConclusion::Passed
    );
    let (control, _control_sim) = driven(campaign.select(name("quiet-control")?)?, 0u32, 2u64)?;
    assert_eq!(
        holds_over_history(&contract, &control),
        TrialConclusion::Passed
    );
    Ok(())
}

/// Retries cross a healed partition, and the census counts exactly what the open interval took.
#[test]
fn a_partition_heals_and_retries_cross_it() -> Result<(), LaneFailure> {
    let parted = NetworkSchedule::declared(
        name("partition-then-heal")?,
        vec![LinkDiscipline::declared(
            forward()?,
            vec![LinkFault::Partition {
                opens: Tick::at(0u64),
                heals: Tick::at(3u64),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, parted])?;
    let contract = reply_contract()?;
    let (outage, _outage_sim) = driven(campaign.select(name("partition-then-heal")?)?, 0u32, 2u64)?;
    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &outage) else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(finding.cause(), REPLY_ARRIVES);
    let (healed, sim) = driven(campaign.select(name("partition-then-heal")?)?, 2u32, 2u64)?;
    assert_eq!(
        holds_over_history(&contract, &healed),
        TrialConclusion::Passed
    );
    let census = sim.census();
    assert_eq!(census.sends(), 4u64);
    assert_eq!(census.dropped_by_partition(), 2u64);
    assert_eq!(census.dropped_by_discipline(), 0u64);
    assert_eq!(census.scheduled_deliveries(), 2u64);
    assert_eq!(census.delivered(), 2u64);
    Ok(())
}

/// A held send crosses a later one — reordering as latency — and two identically driven sims agree delivery for delivery.
#[test]
fn a_delayed_send_crosses_a_later_one_deterministically() -> Result<(), LaneFailure> {
    fn crossing() -> Result<Vec<Delivery<Message>>, LaneFailure> {
        let hold = NetworkSchedule::declared(
            name("hold-the-first")?,
            vec![LinkDiscipline::declared(
                forward()?,
                vec![LinkFault::DelayAt {
                    position: SendOrdinal::at(0u32),
                    ticks: TickSpan::declared(2u32)?,
                }],
            )],
        )?;
        let campaign = NetworkCampaign::declared(vec![quiet_control()?, hold])?;
        let mut sim =
            SimNet::declared(pair_topology()?, campaign.select(name("hold-the-first")?)?)?;
        sim.send(forward()?, Message::Request { id: 1u64 })?;
        sim.send(forward()?, Message::Request { id: 2u64 })?;
        let mut deliveries = Vec::new();
        while sim.pending() > 0usize {
            deliveries.extend(sim.advance());
        }
        Ok(deliveries)
    }
    let first = crossing()?;
    let second = crossing()?;
    assert_eq!(first, second);
    assert_eq!(first.len(), 2usize);
    let crossed = first.first().ok_or(LaneFailure::Standing)?;
    assert_eq!(crossed.ordinal(), SendOrdinal::at(1u32));
    assert_eq!(crossed.delivered_at(), Tick::at(1u64));
    let held = first.get(1usize).ok_or(LaneFailure::Standing)?;
    assert_eq!(held.ordinal(), SendOrdinal::at(0u32));
    assert_eq!(held.delivered_at(), Tick::at(3u64));
    Ok(())
}
