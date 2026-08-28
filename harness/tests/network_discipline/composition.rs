//! Claim: command-shaped deliveries compose with the ordinary interleave and temporal-contract instruments.
//! Subject: per-link delivery sequences projected into strands.
//! Population: two links delivering opposing balance changes.
//! Hostile control: the explored withdraw-first order violates the declared nonnegative invariant.
//! Denominator: the one public delivery-to-command composition claimed by the network README.
//! Evidence ceiling: command-order composition does not establish instruction-level preemption behavior.

use super::support::*;

/// Per-link deliveries stand as strands, so a delivery-order bug is caught by the ordinary exploration.
#[test]
fn deliveries_stand_as_strands_for_exploration() -> Result<(), LaneFailure> {
    /// One transfer against the shared balance.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Payment {
        amount: i128,
    }
    /// The balance the two senders race over.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Balance {
        held: i128,
    }
    fn opening_balance() -> Balance {
        Balance { held: 0i128 }
    }
    fn applied_payment(state: &Balance, payment: &Payment) -> Balance {
        Balance {
            held: state.held.saturating_add(payment.amount),
        }
    }
    fn negative(state: &Balance) -> Holding {
        if state.held < 0i128 {
            Holding::Holds
        } else {
            Holding::Fails
        }
    }
    let alpha = NodeRef::declared(name("alpha")?);
    let beta = NodeRef::declared(name("beta")?);
    let hub = NodeRef::declared(name("hub")?);
    let from_alpha = Link::between(alpha, hub);
    let from_beta = Link::between(beta, hub);
    let topology = Topology::declared(vec![alpha, beta, hub], vec![from_alpha, from_beta])?;
    let hostile = NetworkSchedule::declared(
        name("drop-alpha-first")?,
        vec![LinkDiscipline::declared(
            from_alpha,
            vec![LinkFault::DropAt {
                position: SendOrdinal::at(0u32),
            }],
        )],
    )?;
    let campaign = NetworkCampaign::declared(vec![quiet_control()?, hostile])?;
    let mut sim = SimNet::declared(topology, campaign.select(name("quiet-control")?)?)?;
    sim.send(from_alpha, Payment { amount: 5i128 })?;
    sim.send(from_beta, Payment { amount: -5i128 })?;
    let mut alpha_commands = Vec::new();
    let mut beta_commands = Vec::new();
    while sim.pending() > 0usize {
        for delivery in sim.advance() {
            if delivery.link() == from_alpha {
                alpha_commands.push(*delivery.payload());
            } else {
                beta_commands.push(*delivery.payload());
            }
        }
    }
    let set = StrandSet::declared(vec![
        Strand::declared(name("from-alpha")?, alpha_commands)?,
        Strand::declared(name("from-beta")?, beta_commands)?,
    ])?;
    let contract = TransitionContract::declared(
        opening_balance,
        applied_payment,
        vec![TemporalClaim::declared(
            NEVER_NEGATIVE,
            TemporalDemand::Never(negative),
        )],
    )?;
    let reading = explored(
        &set,
        &contract,
        ExplorationBound::declared(8u32, 4u32)?,
        PopulationRef::named("lane", "delivery-orders")?,
        RootSeed::declared(11u64),
    )?;
    let ExplorationStanding::CounterexampleFound(counterexample) = reading.standing() else {
        return Err(LaneFailure::Standing);
    };
    assert_eq!(counterexample.interleaving().choices(), [1u8, 0u8]);
    assert_eq!(counterexample.finding().cause(), NEVER_NEGATIVE);
    Ok(())
}
