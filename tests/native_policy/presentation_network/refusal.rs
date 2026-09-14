use super::fixture::{LIMITS, PAYLOAD, debug, driven, topology};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::identity::ContentAddress;
use macroonz::harness::network::{self, TRANSCRIPT_TAG, TranscriptRefusal};
use macroonz::presentation;
use serde_json::json;

#[test]
fn presentation_readable_altered_rows_do_not_impersonate_simulation_reproduction()
-> Result<(), String> {
    let fixture = driven(0)?;
    let mut changed = fixture.pack.encoded().to_vec();
    let at = changed
        .windows(PAYLOAD.len())
        .rposition(|bytes| bytes == PAYLOAD)
        .ok_or("missing payload")?;
    *changed.get_mut(at).ok_or("missing altered byte")? = 9_u8;
    let width = fixture.pack.address().address().as_bytes().len();
    let derived =
        ContentAddress::derived(TRANSCRIPT_TAG, changed.get(width..).ok_or("missing body")?);
    changed
        .get_mut(..width)
        .ok_or("missing claim")?
        .copy_from_slice(derived.as_bytes());
    let admitted = network::read_simulated(&topology()?, &fixture.schedule, &changed, LIMITS)
        .map_err(debug)?;
    let shown = parsed(&presentation::network_transcript(&admitted))?;
    assert_eq!(decoded_hex(field(&shown, "/record/encoded")?)?, changed);
    assert_eq!(
        decoded_hex(field(&shown, "/record/entries/1/payload")?)?,
        [9_u8, 255, b'<', b'&', b'>']
    );
    let refusal = network::reproduce(&admitted)
        .err()
        .ok_or("altered row reproduced")?;
    assert_eq!(refusal, TranscriptRefusal::SimulationRowsDiverge { at: 1 });
    let refused = parsed(&presentation::network_transcript_refusal(&refusal))?;
    assert_eq!(
        field(&refused, "/record")?,
        &json!({"kind":"simulation-rows-diverge","value":1_usize})
    );
    assert!(shown.pointer("/record/reproduction").is_none());
    let source_refusal = network::read_recorded_live(&topology()?, fixture.pack.encoded(), LIMITS)
        .err()
        .ok_or("source relabeled")?;
    let source_display = parsed(&presentation::network_transcript_refusal(&source_refusal))?;
    assert_eq!(
        field(&source_display, "/record/value")?,
        &json!({"expected":"recorded-live","found":"simulated"})
    );
    Ok(())
}
