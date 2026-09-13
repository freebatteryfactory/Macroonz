use super::fixture::{LIMITS, debug, driven, route, topology};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::network::{
    self, DeliveryCopy, Replay, ReproducedReplay, SendOrdinal, Tick, TranscriptEntry,
    TranscriptRefusal,
};
use macroonz::presentation;
use serde_json::Value;

#[test]
fn presentation_separates_simulation_ticks_playback_exhaustion_and_same_address_join()
-> Result<(), String> {
    let fixture = driven(0)?;
    let read = network::read_simulated(
        &topology()?,
        &fixture.schedule,
        fixture.pack.encoded(),
        LIMITS,
    )
    .map_err(debug)?;
    let reproduced = network::reproduce(&read).map_err(debug)?;
    assert_eq!(reproduced, fixture.reproduction);
    let reproduction = parsed(&presentation::network_reproduction(&reproduced))?;
    assert_eq!(field(&reproduction, "/record/actions")?, 8_usize);
    assert_eq!(field(&reproduction, "/record/rows")?, 3_usize);
    assert_eq!(field(&reproduction, "/record/final_tick")?, 4_u64);
    let (mut replay, opening) = Replay::opened(&read);
    assert!(opening.is_empty());
    let incomplete = replay
        .clone()
        .exhaust()
        .err()
        .ok_or("unplayed rows exhausted")?;
    let incomplete_display = parsed(&presentation::network_replay_incomplete(&incomplete))?;
    assert_eq!(field(&incomplete_display, "/record/remaining")?, 3_usize);
    assert_eq!(
        decoded_hex(field(&incomplete_display, "/record/address")?)?,
        read.address().address().as_bytes()
    );
    assert_eq!(replay.tick(), Tick::at(0));
    while replay.remaining() != 0 {
        let _handed_out = replay.advance();
    }
    let exhausted = replay.exhaust().map_err(debug)?;
    let exhaustion = parsed(&presentation::network_replay_exhaustion(&exhausted))?;
    assert_eq!(field(&exhaustion, "/record/total")?, 3_usize);
    assert_eq!(field(&exhaustion, "/record/final_tick")?, 3_u64);
    let joined = ReproducedReplay::joined(reproduced, exhausted).map_err(debug)?;
    let joined_display = parsed(&presentation::network_reproduced_replay(&joined))?;
    assert_eq!(
        field(&joined_display, "/record/reproduction")?,
        field(&reproduction, "/record")?
    );
    assert_eq!(
        field(&joined_display, "/record/exhaustion")?,
        field(&exhaustion, "/record")?
    );
    let other = driven(1)?;
    let mismatch = ReproducedReplay::joined(other.reproduction, exhausted)
        .err()
        .ok_or("foreign addresses joined")?;
    let refused = parsed(&presentation::network_replay_join_refusal(&mismatch))?;
    assert_eq!(
        decoded_hex(field(&refused, "/record/value/reproduction")?)?,
        other.pack.address().address().as_bytes()
    );
    assert_eq!(
        decoded_hex(field(&refused, "/record/value/replay")?)?,
        read.address().address().as_bytes()
    );
    Ok(())
}

#[test]
fn presentation_live_source_retains_tick_zero_empty_payload_and_no_reproduction()
-> Result<(), String> {
    let rows = vec![TranscriptEntry::witnessed(
        route()?,
        SendOrdinal::at(u32::MAX),
        Vec::new(),
        Tick::at(0),
        Tick::at(0),
        DeliveryCopy::Duplicate,
    )];
    let written = network::recorded_live(&topology()?, rows).map_err(debug)?;
    let read =
        network::read_recorded_live(&topology()?, written.encoded(), LIMITS).map_err(debug)?;
    let shown = parsed(&presentation::network_transcript(&read))?;
    assert_eq!(field(&shown, "/record/source_claim")?, "recorded-live");
    assert_eq!(field(&shown, "/record/simulation_manifest")?, &Value::Null);
    assert_eq!(field(&shown, "/record/entries/0/ordinal")?, u32::MAX);
    assert_eq!(
        decoded_hex(field(&shown, "/record/entries/0/payload")?)?,
        Vec::<u8>::new()
    );
    assert_eq!(field(&shown, "/record/entries/0/delivered_at")?, 0_u64);
    let refusal = network::reproduce(&read)
        .err()
        .ok_or("live source reproduced")?;
    assert_eq!(refusal, TranscriptRefusal::RecordedLiveCannotReproduce);
    let refused = parsed(&presentation::network_transcript_refusal(&refusal))?;
    assert_eq!(
        field(&refused, "/record/kind")?,
        "recorded-live-cannot-reproduce"
    );
    let (replay, handed_out) = Replay::opened(&read);
    assert_eq!(handed_out.len(), 1);
    let exhaustion = replay.exhaust().map_err(debug)?;
    let exhaustion_display = parsed(&presentation::network_replay_exhaustion(&exhaustion))?;
    assert_eq!(field(&exhaustion_display, "/record/final_tick")?, 0_u64);
    assert_eq!(field(&exhaustion_display, "/record/total")?, 1_usize);
    Ok(())
}
