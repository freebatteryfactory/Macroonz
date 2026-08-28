//! Claim: transcript admission and reading refuse malformed material in documented priority order.
//! Subject: live row admission, address-first envelope reading, topology comparison, and format retirement.
//! Population: empty, foreign, impossible, backward, truncated, tampered, foreign-topology, and retired-version inputs.
//! Hostile control: one foreign row also violates time, proving foreign-link priority.
//! Denominator: the externally writable malformed shapes represented by the public refusal roster.
//! Evidence ceiling: finite hostile examples do not exhaust all byte strings.

use super::support::*;

/// The live writer refuses empty, foreign, impossible, and backward records at their exact clauses.
#[test]
fn the_live_write_road_refuses_incoherent_records() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    assert_eq!(
        recorded_live(&topology, Vec::new()).err(),
        Some(TranscriptRefusal::NoDelivery)
    );
    let backward = TranscriptEntry::witnessed(
        forward()?,
        SendOrdinal::at(0u32),
        b"impossible".to_vec(),
        Tick::at(99u64),
        Tick::at(0u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded_live(&topology, vec![backward]).err(),
        Some(TranscriptRefusal::DeliveryBeforeSend { at: 0usize })
    );
    let stranger = Link::between(
        NodeRef::declared(name("stranger")?),
        NodeRef::declared(name("server")?),
    );
    let foreign = TranscriptEntry::witnessed(
        stranger,
        SendOrdinal::at(0u32),
        b"lost".to_vec(),
        Tick::at(0u64),
        Tick::at(1u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded_live(&topology, vec![foreign]).err(),
        Some(TranscriptRefusal::ForeignLink { at: 0usize })
    );
    assert_eq!(
        recorded_live(
            &topology,
            vec![
                live_entry(b"late", 0u32, 5u64)?,
                live_entry(b"early", 1u32, 2u64)?
            ]
        )
        .err(),
        Some(TranscriptRefusal::DeliveryOrderBroken { at: 1usize })
    );
    Ok(())
}

/// The reader settles the address first, rejects foreign topology, and refuses the retired wire version explicitly.
#[test]
fn the_reader_refuses_tampered_foreign_and_retired_envelopes() -> Result<(), LaneFailure> {
    let (_rows, schedule, pack, _standing) = packed_run(0usize)?;
    let topology = pair_topology()?;
    let mut tampered = pack.encoded().to_vec();
    if let Some(last) = tampered.last_mut() {
        *last = last.wrapping_add(1u8);
    }
    assert!(matches!(
        read_simulated(&topology, &schedule, &tampered).err(),
        Some(TranscriptRefusal::AddressMismatch { derived: _ })
    ));
    let short = pack
        .encoded()
        .get(0usize..10usize)
        .ok_or(LaneFailure::Standing)?;
    assert_eq!(
        read_simulated(&topology, &schedule, short).err(),
        Some(TranscriptRefusal::Truncated)
    );
    let elsewhere = Topology::declared(
        vec![
            NodeRef::declared(name("alpha")?),
            NodeRef::declared(name("beta")?),
        ],
        vec![Link::between(
            NodeRef::declared(name("alpha")?),
            NodeRef::declared(name("beta")?),
        )],
    )?;
    assert_eq!(
        read_simulated(&elsewhere, &schedule, pack.encoded()).err(),
        Some(TranscriptRefusal::TopologyMismatch)
    );

    let address_width = ContentAddress::derived(TRANSCRIPT_TAG, &[])
        .as_bytes()
        .len();
    let mut retired = pack.encoded().to_vec();
    let version_end = address_width.saturating_add(4usize);
    let version = retired
        .get_mut(address_width..version_end)
        .ok_or(LaneFailure::Standing)?;
    version.copy_from_slice(&1u32.to_be_bytes());
    readdress(&mut retired)?;
    assert_eq!(
        read_simulated(&topology, &schedule, &retired).err(),
        Some(TranscriptRefusal::UnsupportedFormat { found: 1u32 })
    );
    Ok(())
}

/// A row that is both foreign and temporally impossible refuses as foreign before its timestamps are read as claims.
#[test]
fn compounded_invalid_rows_keep_admission_priority() -> Result<(), LaneFailure> {
    let topology = pair_topology()?;
    let stranger = Link::between(
        NodeRef::declared(name("stranger-priority")?),
        NodeRef::declared(name("server")?),
    );
    let row = TranscriptEntry::witnessed(
        stranger,
        SendOrdinal::at(0u32),
        b"impossible".to_vec(),
        Tick::at(9u64),
        Tick::at(0u64),
        DeliveryCopy::Original,
    );
    assert_eq!(
        recorded_live(&topology, vec![row]).err(),
        Some(TranscriptRefusal::ForeignLink { at: 0usize })
    );
    Ok(())
}
