//! All offer grounds remain unadmitted and keep their complete retained coordinates.

use super::proposal_fixture::{candidate, destination, kill};
use crate::presentation_formats::{field, parsed};
use crate::presentation_parity::{binding, mapped};
use macroonz::harness::descriptor::ClaimRef;
use macroonz::harness::muterprater::{
    DischargeEvidence, ObligationLane, OwedClaim, ProofDelta, ProposalDocument,
    proposal_archive::{
        ArchivedProposal, ArchivedProposalGround, ProposalArchiveLimits, read_proposal,
        retain_claim_pin, retain_mutant_kill, retain_obligation_discharge,
    },
    propose::{offer_claim_pin, offer_obligation_discharge},
};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::harness::runner::trial_identity;
use macroonz::presentation;
use serde_json::{Value, json};

const LIMITS: ProposalArchiveLimits =
    ProposalArchiveLimits::declared(ArchiveLimits::declared(65_536, 32_768), 8, 16, 8);

fn shown(record: &ArchivedProposal) -> Result<Value, String> {
    let loaded = mapped(read_proposal(record.encoded(), LIMITS))?;
    let shown = parsed(&presentation::archived_proposal(&loaded))?;
    assert_eq!(field(&shown, "/standing")?, "historical-unauthenticated");
    assert_eq!(field(&shown, "/kind")?, "proposal");
    assert_eq!(
        field(&shown, "/record/destination")?,
        &json!({
            "namespace":"review", "stem":"destination",
        })
    );
    assert_eq!(
        field(&shown, "/record/candidate/synthesis/kind")?,
        "proof-gap"
    );
    assert_eq!(
        field(&shown, "/record/candidate/roles")?,
        &json!([
            {"namespace":"offer", "stem":"control"},
            {"namespace":"offer", "stem":"witness"},
        ])
    );
    assert_eq!(
        field(&shown, "/record/candidate/tags/0/stem")?,
        "<foreign>| tag"
    );
    for absent in ["custody", "admission", "replay_permission"] {
        assert!(field(&shown, "/record")?.get(absent).is_none());
    }
    Ok(shown)
}

#[test]
fn presentation_proposal_keeps_kill_census_and_repeated_known_failures() -> Result<(), String> {
    let proposal = kill()?;
    let archived = mapped(retain_mutant_kill(&proposal, LIMITS))?;
    let shown = shown(&archived)?;
    assert_eq!(field(&shown, "/record/ground/kind")?, "mutant-killed");
    let value = field(&shown, "/record/ground/value")?;
    assert_eq!(field(value, "/report/denominator")?, &json!(2usize));
    assert_eq!(field(value, "/report/posture/kind")?, "staged");
    assert_eq!(
        field(
            value,
            "/report/census/0/disposition/value/attempt/value/kind"
        )?,
        "passed"
    );
    assert_eq!(
        field(
            value,
            "/report/census/1/disposition/value/attempt/value/kind"
        )?,
        "refused"
    );
    assert_eq!(field(value, "/activation/value/firings")?, &json!(7u32));
    assert_eq!(field(value, "/capsule/input")?, "ff000d");
    assert_eq!(field(value, "/rejection/foreign/shown")?, "<even>| refused");
    let known = field(value, "/comparison/known")?
        .as_array()
        .ok_or("known roster missing")?;
    assert_eq!(known.len(), 2);
    assert_eq!(known.first(), known.last());
    assert_ne!(known.first(), Some(field(value, "/comparison/candidate")?));
    assert_eq!(
        field(value, "/comparison/candidate")?,
        field(value, "/rejection/fingerprint")?
    );
    let ArchivedProposalGround::MutantKilled(ground) = archived.ground() else {
        return Err("wrong ground".into());
    };
    for (name, nested) in [
        ("capsule", presentation::archived_capsule(ground.capsule())),
        ("report", presentation::archived_run(ground.report())),
        (
            "trial_report",
            presentation::archived_trial(ground.trial_report()),
        ),
    ] {
        assert_eq!(
            field(&parsed(&nested)?, "/record")?,
            field(value, &format!("/{name}"))?
        );
    }
    Ok(())
}

#[test]
fn presentation_proposal_keeps_pin_evidence_separate_from_identity_and_claim() -> Result<(), String>
{
    let kill = kill()?;
    let mut displays = Vec::new();
    for (before, after) in [(3usize, 8usize), (4, 9)] {
        let pin = mapped(offer_claim_pin(
            kill.candidate().clone(),
            mapped(ClaimRef::named("independent", "pin"))?,
            kill.ground().capsule().clone(),
            mapped(ProofDelta::between(before, after))?,
            kill.destination(),
        ))?;
        let shown = shown(&mapped(retain_claim_pin(&pin, LIMITS))?)?;
        let ground = field(&shown, "/record/ground/value")?;
        assert_eq!(field(ground, "/before")?, &json!(before));
        assert_eq!(field(ground, "/after")?, &json!(after));
        assert_eq!(field(ground, "/claim/stem")?, "pin");
        assert_ne!(
            field(ground, "/claim")?,
            field(&shown, "/record/candidate/claim")?
        );
        assert_eq!(
            field(ground, "/comparison")?,
            &json!({
                "kind":"ground-carries-no-failure", "value":null,
            })
        );
        assert!(ground.get("known").is_none());
        displays.push(shown);
    }
    let [first, second] = displays.as_slice() else {
        return Err("missing pin".into());
    };
    assert_eq!(
        field(first, "/record/identity")?,
        field(second, "/record/identity")?
    );
    assert_ne!(
        field(first, "/record/archive_address")?,
        field(second, "/record/archive_address")?
    );
    Ok(())
}

#[test]
fn presentation_proposal_keeps_each_discharge_lane_and_unjoined_trial() -> Result<(), String> {
    let kill = kill()?;
    let key = kill.ground().capsule().key();
    let other = trial_identity(binding()?.row());
    for (lane, name) in [
        (ObligationLane::TestRow, "test-row"),
        (ObligationLane::FuzzSeed, "fuzz-seed"),
        (ObligationLane::ChaosScenario, "chaos-scenario"),
    ] {
        let proposal = mapped(offer_obligation_discharge(
            candidate()?.row().clone(),
            mapped(OwedClaim::declared(
                mapped(ClaimRef::named("separate", "owed"))?,
                "open <later>|\nwith independent evidence",
            ))?,
            DischargeEvidence::recorded(lane, other, key.clone()),
            &[],
            destination()?,
        ))?;
        let shown = shown(&mapped(retain_obligation_discharge(&proposal, LIMITS))?)?;
        assert_eq!(
            field(&shown, "/record/ground/kind")?,
            "obligation-discharged"
        );
        let ground = field(&shown, "/record/ground/value")?;
        assert_eq!(field(ground, "/lane")?, name);
        assert_eq!(
            field(ground, "/opening_condition")?,
            "open <later>|\nwith independent evidence"
        );
        assert_ne!(field(ground, "/trial")?, field(ground, "/key/trial")?);
        assert_eq!(field(ground, "/comparison/owed")?, field(ground, "/owed")?);
        assert_eq!(field(ground, "/comparison/recorded")?, &json!([]));
        assert!(ground.get("capsule").is_none());
    }
    Ok(())
}
