//! Complete transcript material and distinct execution standing through the root facade.

mod fixture;
mod refusal;
mod standing;
mod types;

use crate::presentation_formats::{decoded_hex, field, parsed};
use fixture::{LIMITS, PAYLOAD, debug, driven, topology};
use macroonz::harness::network;
use macroonz::presentation;
use serde_json::{Value, json};

#[test]
fn presentation_keeps_dropped_actions_empty_advances_and_complete_delivery_lineage()
-> Result<(), String> {
    let fixture = driven(0)?;
    let admitted = network::read_simulated(
        &topology()?,
        &fixture.schedule,
        fixture.pack.encoded(),
        LIMITS,
    )
    .map_err(debug)?;
    let shown = parsed(&presentation::network_transcript(&admitted))?;
    assert_eq!(field(&shown, "/owner")?, "macroonz-harness/network");
    assert_eq!(field(&shown, "/record/source_claim")?, "simulated");
    assert_eq!(
        decoded_hex(field(&shown, "/record/encoded")?)?,
        fixture.pack.encoded()
    );
    assert_eq!(
        decoded_hex(field(&shown, "/record/address")?)?,
        fixture.pack.address().address().as_bytes()
    );
    assert_eq!(
        field(&shown, "/record/topology/nodes")?,
        &json!([
            {"namespace":"display.network","stem":"receiver"},
            {"namespace":"display.network","stem":"sender"},
            {"namespace":"display.network","stem":"unused"}
        ])
    );
    let manifest = field(&shown, "/record/simulation_manifest")?;
    assert_eq!(
        field(manifest, "/schedule/disciplines/0/faults")?,
        &json!([
            {"kind":"delay-at","value":{"position":1_u32,"ticks":2_u32}},
            {"kind":"duplicate-at","value":1_u32},
            {"kind":"drop-at","value":0_u32},
            {"kind":"partition","value":{"opens":1_u64,"heals":2_u64}}
        ])
    );
    let actions = field(manifest, "/actions")?
        .as_array()
        .ok_or("missing actions")?;
    let kinds = actions
        .iter()
        .map(|value| field(value, "/kind"))
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(
        kinds,
        vec![
            "send", "send", "advance", "send", "advance", "send", "advance", "advance"
        ]
    );
    assert_eq!(
        decoded_hex(field(manifest, "/actions/0/value/payload")?)?,
        b"lost"
    );
    assert_eq!(
        decoded_hex(field(manifest, "/actions/3/value/payload")?)?,
        b"partitioned"
    );
    assert_eq!(field(manifest, "/actions/7/value")?, &Value::Null);
    let rows = field(&shown, "/record/entries")?
        .as_array()
        .ok_or("missing deliveries")?;
    assert_eq!(rows.len(), 3);
    for (row, (ordinal, payload, copy, sent)) in rows.iter().zip([
        (1_u32, PAYLOAD, "original", 0_u64),
        (1_u32, PAYLOAD, "duplicate", 0_u64),
        (0_u32, &[][..], "original", 2_u64),
    ]) {
        assert_eq!(field(row, "/ordinal")?, ordinal);
        assert_eq!(decoded_hex(field(row, "/payload")?)?, payload);
        assert_eq!(field(row, "/copy")?, copy);
        assert_eq!(field(row, "/sent_at")?, sent);
        assert_eq!(field(row, "/delivered_at")?, 3_u64);
    }
    assert!(shown.pointer("/record/reproduction").is_none());
    assert!(shown.pointer("/record/census").is_none());
    let extended = driven(1)?;
    assert_eq!(fixture.pack.entries(), extended.pack.entries());
    let later = parsed(&presentation::network_transcript(&extended.pack))?;
    assert_ne!(
        field(&shown, "/record/address")?,
        field(&later, "/record/address")?
    );
    assert_eq!(
        field(&later, "/record/simulation_manifest/actions")?
            .as_array()
            .ok_or("missing later actions")?
            .len(),
        9
    );
    Ok(())
}
