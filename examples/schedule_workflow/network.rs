//! Execute the authored traffic and compare deliveries with independent expectations.

use crate::types::Move;
use crate::world::baked::network as declared;
use macroonz::harness::network::{
    Delivery, DeliveryCopy, Link, NetworkCampaign, NetworkCampaignRefusal, NetworkSelectionRefusal,
    SimNet, Tick,
};

pub(super) fn run() -> Result<Vec<Delivery<Move>>, String> {
    let quiet = declared::quiet().map_err(|error| format!("quiet schedule: {error:?}"))?;
    let pressure = declared::pressure().map_err(|error| format!("pressure schedule: {error:?}"))?;
    assert_eq!(
        NetworkCampaign::declared(vec![quiet.clone()]),
        Err(NetworkCampaignRefusal::NoFaultDeclared)
    );
    let quiet_name = quiet.name();
    let pressure_name = pressure.name();
    let campaign = NetworkCampaign::declared(vec![quiet, pressure])
        .map_err(|error| format!("campaign: {error:?}"))?;
    let topology = declared::topology().map_err(|error| format!("topology: {error:?}"))?;
    let [deposits, withdrawals] = topology.links() else {
        return Err("the declaration must supply the two addressed links".to_owned());
    };
    let quiet_selection = campaign
        .select(quiet_name)
        .map_err(|error| format!("quiet selection: {error:?}"))?;
    let mut quiet_sim = SimNet::declared(topology.clone(), quiet_selection)
        .map_err(|error| format!("quiet simulation: {error:?}"))?;
    let _deposit = quiet_sim
        .send(*deposits, Move::Deposit(5u64))
        .map_err(|error| format!("quiet deposit: {error:?}"))?;
    let _withdrawal = quiet_sim
        .send(*withdrawals, Move::Withdraw(5u64))
        .map_err(|error| format!("quiet withdrawal: {error:?}"))?;
    let deliveries = quiet_sim.advance();
    assert_eq!(
        observed(&deliveries),
        [
            (
                *deposits,
                Move::Deposit(5u64),
                0u32,
                0u64,
                1u64,
                DeliveryCopy::Original
            ),
            (
                *withdrawals,
                Move::Withdraw(5u64),
                0u32,
                0u64,
                1u64,
                DeliveryCopy::Original
            ),
        ]
    );
    assert_eq!(quiet_sim.pending(), 0usize);
    let pressure_selection = campaign
        .select(pressure_name)
        .map_err(|error| format!("pressure selection: {error:?}"))?;
    let stressed = SimNet::declared(topology.clone(), pressure_selection)
        .map_err(|error| format!("pressure simulation: {error:?}"))?;
    pressure_run(stressed, *deposits, *withdrawals)?;
    let missing =
        macroonz::harness::descriptor::NamespacedName::named("schedule-example", "missing")
            .map_err(|error| format!("missing selection name: {error:?}"))?;
    assert_eq!(
        campaign.select(missing),
        Err(NetworkSelectionRefusal::ScheduleAbsent(missing))
    );
    Ok(deliveries)
}

fn pressure_run(mut sim: SimNet<Move>, deposits: Link, withdrawals: Link) -> Result<(), String> {
    use macroonz::harness::network::SendFate;

    let delayed = sim
        .send(deposits, Move::Deposit(5u64))
        .map_err(|error| format!("delayed deposit: {error:?}"))?;
    assert_eq!(
        delayed.fate(),
        SendFate::Scheduled {
            copies: 2u32,
            due: Tick::at(3u64)
        }
    );
    let partitioned = sim
        .send(withdrawals, Move::Withdraw(5u64))
        .map_err(|error| format!("partitioned withdrawal: {error:?}"))?;
    assert_eq!(partitioned.fate(), SendFate::DroppedByPartition);
    assert!(sim.advance().is_empty());
    let dropped = sim
        .send(deposits, Move::Deposit(99u64))
        .map_err(|error| format!("dropped deposit: {error:?}"))?;
    assert_eq!(dropped.fate(), SendFate::DroppedByDiscipline);
    assert!(sim.advance().is_empty());
    let healed = sim
        .send(withdrawals, Move::Withdraw(5u64))
        .map_err(|error| format!("healed withdrawal: {error:?}"))?;
    assert_eq!(
        healed.fate(),
        SendFate::Scheduled {
            copies: 1u32,
            due: Tick::at(3u64)
        }
    );
    assert_eq!(
        observed(&sim.advance()),
        [
            (
                deposits,
                Move::Deposit(5u64),
                0u32,
                0u64,
                3u64,
                DeliveryCopy::Original
            ),
            (
                deposits,
                Move::Deposit(5u64),
                0u32,
                0u64,
                3u64,
                DeliveryCopy::Duplicate
            ),
            (
                withdrawals,
                Move::Withdraw(5u64),
                1u32,
                2u64,
                3u64,
                DeliveryCopy::Original
            ),
        ]
    );
    assert_eq!(sim.pending(), 0usize);
    let census = sim.census();
    assert_eq!(
        (
            census.sends(),
            census.scheduled_deliveries(),
            census.delivered()
        ),
        (4u64, 3u64, 3u64)
    );
    assert_eq!(
        (
            census.dropped_by_discipline(),
            census.dropped_by_partition()
        ),
        (1u64, 1u64)
    );
    Ok(())
}

fn observed(deliveries: &[Delivery<Move>]) -> Vec<(Link, Move, u32, u64, u64, DeliveryCopy)> {
    deliveries
        .iter()
        .map(|delivery| {
            (
                delivery.link(),
                *delivery.payload(),
                delivery.ordinal().ordinal(),
                delivery.sent_at().ordinal(),
                delivery.delivered_at().ordinal(),
                delivery.copy(),
            )
        })
        .collect()
}
