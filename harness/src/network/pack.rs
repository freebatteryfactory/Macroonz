//! The transcript roads: writing a content-addressed pack of deliveries, reading one that arrived from somewhere else, and playing one back.
//!
//! The addressed body is exactly this, with no separators and no padding — `u32be(n)` is an integer in four big-endian bytes, `u64be(n)` in eight, and `bytes(x)` is `u64be(x.len())` followed by `x`:
//!
//! ```text
//! u32be(TRANSCRIPT_FORMAT_VERSION)
//! u32be(provenance slot)
//! u64be(node count)
//! bytes(node namespace), bytes(node stem)                     repeated, in topology order
//! u64be(link count)
//! bytes(from namespace), bytes(from stem),
//! bytes(to namespace), bytes(to stem)                         repeated, in topology order
//! u64be(entry count)
//! bytes(from namespace), bytes(from stem),
//! bytes(to namespace), bytes(to stem),
//! u32be(send ordinal), bytes(payload),
//! u64be(sent-at tick), u64be(delivered-at tick),
//! u32be(copy slot)                                            repeated, in delivery order
//! ```
//!
//! The address is that body under [`TRANSCRIPT_TAG`], and the complete envelope is the address followed by the body it addresses.
//! An address never covers itself.

use super::{
    Delivery, DeliveryCopy, Link, NodeRef, Replay, SendOrdinal, TRANSCRIPT_FORMAT_VERSION,
    TRANSCRIPT_TAG, Tick, Topology, TranscriptAddress, TranscriptEntry, TranscriptPack,
    TranscriptProvenance, TranscriptRefusal,
};
use crate::identity::ContentAddress;
use crate::report::{encode_bytes, encode_length};

/// Write one content-addressed transcript in delivery order.
///
/// The address is derived here rather than accepted from anyone, and every row is judged against the topology before a byte is written.
///
/// # Errors
///
/// Refuses a transcript with no delivery, then the first entry on a link the topology never declared, then the first entry stamped earlier than the entry before it.
pub fn recorded(
    provenance: TranscriptProvenance,
    topology: &Topology,
    entries: Vec<TranscriptEntry>,
) -> Result<TranscriptPack, TranscriptRefusal> {
    lawful_entries(topology, &entries)?;
    let body = encode_body(provenance, topology, &entries);
    let address = TranscriptAddress::derived(ContentAddress::derived(TRANSCRIPT_TAG, &body));
    let claim = address.address();
    let mut encoded = Vec::with_capacity(claim.as_bytes().len().saturating_add(body.len()));
    encoded.extend_from_slice(claim.as_bytes());
    encoded.extend_from_slice(&body);
    Ok(TranscriptPack::assembled(
        provenance, address, entries, encoded,
    ))
}

/// Read one content-addressed transcript envelope for the topology the caller expects.
///
/// The leading claim is settled before a single member of the body is interpreted, and the caller hands in a topology already built, so foreign bytes never mint a name.
///
/// # Errors
///
/// Refuses, in reading order: truncated address material, a claim the body does not derive, an unsupported format, an unknown provenance, a topology that is not the expected one, a length this platform cannot index, a truncated member, an unknown copy slot, trailing bytes, then everything the write road refuses of the rows themselves.
pub fn read(expected: &Topology, encoded: &[u8]) -> Result<TranscriptPack, TranscriptRefusal> {
    let (address, body) = addressed_body(encoded)?;
    let (provenance, entries) = read_body(expected, body)?;
    lawful_entries(expected, &entries)?;
    Ok(TranscriptPack::assembled(
        provenance,
        address,
        entries,
        encoded.to_vec(),
    ))
}

impl Replay {
    /// Open one admitted pack for playback, at tick zero.
    #[must_use]
    pub fn opened(pack: &TranscriptPack) -> Self {
        Self {
            entries: pack.entries().to_vec(),
            at: 0usize,
            tick: Tick::at(0u64),
        }
    }

    /// The current logical tick.
    #[must_use]
    pub const fn tick(&self) -> Tick {
        self.tick
    }

    /// How many recorded deliveries have not yet been played.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.entries.len().saturating_sub(self.at)
    }

    /// Advance one tick and hand back every recorded delivery due by it, stamps included.
    ///
    /// The deliveries are exactly the record's, in the record's order — a replay invents nothing and reorders nothing.
    #[must_use]
    pub fn advance(&mut self) -> Vec<Delivery<Vec<u8>>> {
        self.tick = self.tick.next();
        let now = self.tick;
        let mut played = Vec::new();
        while let Some(entry) = self.entries.get(self.at) {
            if entry.delivered_at() > now {
                break;
            }
            played.push(Delivery::delivered(
                entry.link(),
                entry.ordinal(),
                entry.payload().to_vec(),
                entry.sent_at(),
                entry.delivered_at(),
                entry.copy(),
            ));
            self.at = self.at.saturating_add(1usize);
        }
        played
    }
}

/// Whether every row belongs to the topology and the stamps never step backward.
fn lawful_entries(
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Result<(), TranscriptRefusal> {
    if entries.is_empty() {
        return Err(TranscriptRefusal::NoDelivery);
    }
    let mut latest = Tick::at(0u64);
    for (at, entry) in entries.iter().enumerate() {
        if !topology.links().contains(&entry.link()) {
            return Err(TranscriptRefusal::ForeignLink { at });
        }
        if entry.delivered_at() < latest {
            return Err(TranscriptRefusal::DeliveryOrderBroken { at });
        }
        latest = entry.delivered_at();
    }
    Ok(())
}

/// The complete address preimage of one transcript.
fn encode_body(
    provenance: TranscriptProvenance,
    topology: &Topology,
    entries: &[TranscriptEntry],
) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&TRANSCRIPT_FORMAT_VERSION.to_be_bytes());
    body.extend_from_slice(&provenance_slot(provenance).to_be_bytes());
    encode_length(topology.nodes().len(), &mut body);
    for node in topology.nodes() {
        encode_node(*node, &mut body);
    }
    encode_length(topology.links().len(), &mut body);
    for link in topology.links() {
        encode_link(*link, &mut body);
    }
    encode_length(entries.len(), &mut body);
    for entry in entries {
        encode_link(entry.link(), &mut body);
        body.extend_from_slice(&entry.ordinal().ordinal().to_be_bytes());
        encode_bytes(entry.payload(), &mut body);
        body.extend_from_slice(&entry.sent_at().ordinal().to_be_bytes());
        body.extend_from_slice(&entry.delivered_at().ordinal().to_be_bytes());
        body.extend_from_slice(&copy_slot(entry.copy()).to_be_bytes());
    }
    body
}

/// Append one node's two name parts.
fn encode_node(node: NodeRef, into: &mut Vec<u8>) {
    let name = node.name();
    encode_bytes(name.namespace().written().as_bytes(), into);
    encode_bytes(name.stem().written().as_bytes(), into);
}

/// Append one link's four name parts.
fn encode_link(link: Link, into: &mut Vec<u8>) {
    encode_node(link.from(), into);
    encode_node(link.to(), into);
}

/// The slot a provenance is written at.
///
/// A slot rather than the Rust spelling, so renaming a variant leaves every address derived under it with its name.
const fn provenance_slot(provenance: TranscriptProvenance) -> u32 {
    match provenance {
        TranscriptProvenance::Simulated => 0u32,
        TranscriptProvenance::RecordedLive => 1u32,
    }
}

/// The provenance a slot spells, where the reader knows one.
const fn provenance_of(slot: u32) -> Result<TranscriptProvenance, TranscriptRefusal> {
    match slot {
        0u32 => Ok(TranscriptProvenance::Simulated),
        1u32 => Ok(TranscriptProvenance::RecordedLive),
        found => Err(TranscriptRefusal::UnknownProvenance { found }),
    }
}

/// The slot a delivery copy is written at.
const fn copy_slot(copy: DeliveryCopy) -> u32 {
    match copy {
        DeliveryCopy::Original => 0u32,
        DeliveryCopy::Duplicate => 1u32,
    }
}

/// The delivery copy a slot spells, where the reader knows one.
const fn copy_of(slot: u32) -> Result<DeliveryCopy, TranscriptRefusal> {
    match slot {
        0u32 => Ok(DeliveryCopy::Original),
        1u32 => Ok(DeliveryCopy::Duplicate),
        found => Err(TranscriptRefusal::UnknownCopy { found }),
    }
}

/// Split the envelope at its address claim and keep the body only if the body derives that claim.
fn addressed_body(encoded: &[u8]) -> Result<(TranscriptAddress, &[u8]), TranscriptRefusal> {
    let width = ContentAddress::derived(TRANSCRIPT_TAG, &[])
        .as_bytes()
        .len();
    let Some((claimed, body)) = encoded.split_at_checked(width) else {
        return Err(TranscriptRefusal::Truncated);
    };
    let address = TranscriptAddress::derived(ContentAddress::derived(TRANSCRIPT_TAG, body));
    if claimed != address.address().as_bytes() {
        return Err(TranscriptRefusal::AddressMismatch { derived: address });
    }
    Ok((address, body))
}

/// Read every member the body declares, in the order the format declares them.
fn read_body(
    expected: &Topology,
    body: &[u8],
) -> Result<(TranscriptProvenance, Vec<TranscriptEntry>), TranscriptRefusal> {
    let mut reader = BodyReader::over(body);
    let found = reader.u32()?;
    if found != TRANSCRIPT_FORMAT_VERSION {
        return Err(TranscriptRefusal::UnsupportedFormat { found });
    }
    let provenance = provenance_of(reader.u32()?)?;
    read_topology(expected, &mut reader)?;
    let entries = read_entries(expected, &mut reader)?;
    let trailing = reader.remaining();
    if trailing != 0usize {
        return Err(TranscriptRefusal::TrailingBytes { count: trailing });
    }
    Ok((provenance, entries))
}

/// Compare the encoded topology section against the one the caller expects.
fn read_topology(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<(), TranscriptRefusal> {
    let nodes = reader.count()?;
    if nodes != expected.nodes().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for node in expected.nodes() {
        read_expected_node(*node, reader)?;
    }
    let links = reader.count()?;
    if links != expected.links().len() {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    for link in expected.links() {
        read_expected_link(*link, reader)?;
    }
    Ok(())
}

/// Read one node's two name parts and demand they spell this node.
fn read_expected_node(node: NodeRef, reader: &mut BodyReader<'_>) -> Result<(), TranscriptRefusal> {
    let name = node.name();
    let namespace = reader.bytes()?;
    let stem = reader.bytes()?;
    if namespace != name.namespace().written().as_bytes()
        || stem != name.stem().written().as_bytes()
    {
        return Err(TranscriptRefusal::TopologyMismatch);
    }
    Ok(())
}

/// Read one link's four name parts and demand they spell this link.
fn read_expected_link(link: Link, reader: &mut BodyReader<'_>) -> Result<(), TranscriptRefusal> {
    read_expected_node(link.from(), reader)?;
    read_expected_node(link.to(), reader)
}

/// Read the entry roster, resolving each row's link against the expected topology.
fn read_entries(
    expected: &Topology,
    reader: &mut BodyReader<'_>,
) -> Result<Vec<TranscriptEntry>, TranscriptRefusal> {
    let count = reader.count()?;
    let mut entries = Vec::new();
    for at in 0..count {
        let link = read_link(expected, at, reader)?;
        let ordinal = SendOrdinal::at(reader.u32()?);
        let payload = reader.bytes()?.to_vec();
        let sent_at = Tick::at(reader.u64()?);
        let delivered_at = Tick::at(reader.u64()?);
        let copy = copy_of(reader.u32()?)?;
        entries.push(TranscriptEntry::witnessed(
            link,
            ordinal,
            payload,
            sent_at,
            delivered_at,
            copy,
        ));
    }
    Ok(entries)
}

/// Read one entry's link by matching its four name parts against the expected topology's links.
fn read_link(
    expected: &Topology,
    at: usize,
    reader: &mut BodyReader<'_>,
) -> Result<Link, TranscriptRefusal> {
    let from_namespace = reader.bytes()?.to_vec();
    let from_stem = reader.bytes()?.to_vec();
    let to_namespace = reader.bytes()?.to_vec();
    let to_stem = reader.bytes()?.to_vec();
    expected
        .links()
        .iter()
        .find(|link| {
            spells(link.from(), &from_namespace, &from_stem)
                && spells(link.to(), &to_namespace, &to_stem)
        })
        .copied()
        .ok_or(TranscriptRefusal::ForeignLink { at })
}

/// Whether this node's name is spelled by these two byte strings.
fn spells(node: NodeRef, namespace: &[u8], stem: &[u8]) -> bool {
    let name = node.name();
    name.namespace().written().as_bytes() == namespace && name.stem().written().as_bytes() == stem
}

/// A cursor over one body, which never indexes and never trusts a declared width.
struct BodyReader<'body> {
    body: &'body [u8],
    at: usize,
}

impl<'body> BodyReader<'body> {
    /// Open at the first body byte.
    const fn over(body: &'body [u8]) -> Self {
        Self { body, at: 0 }
    }

    /// Read one fixed-width 32-bit integer.
    fn u32(&mut self) -> Result<u32, TranscriptRefusal> {
        self.fixed::<4>().map(u32::from_be_bytes)
    }

    /// Read one fixed-width 64-bit integer.
    fn u64(&mut self) -> Result<u64, TranscriptRefusal> {
        self.fixed::<8>().map(u64::from_be_bytes)
    }

    /// Read one declared count, refused where the platform cannot index it.
    fn count(&mut self) -> Result<usize, TranscriptRefusal> {
        let declared = self.u64()?;
        usize::try_from(declared)
            .map_err(|_beyond_platform| TranscriptRefusal::LengthOutsidePlatform { declared })
    }

    /// Read one length-prefixed byte string.
    fn bytes(&mut self) -> Result<&'body [u8], TranscriptRefusal> {
        let length = self.count()?;
        self.take(length)
    }

    /// Read one fixed-width byte array.
    fn fixed<const WIDTH: usize>(&mut self) -> Result<[u8; WIDTH], TranscriptRefusal> {
        let bytes = self.take(WIDTH)?;
        <[u8; WIDTH]>::try_from(bytes).map_err(|_unexpected_width| TranscriptRefusal::Truncated)
    }

    /// Advance over exactly this many bytes, or refuse the envelope as truncated.
    fn take(&mut self, width: usize) -> Result<&'body [u8], TranscriptRefusal> {
        let Some(end) = self.at.checked_add(width) else {
            return Err(TranscriptRefusal::Truncated);
        };
        let Some(bytes) = self.body.get(self.at..end) else {
            return Err(TranscriptRefusal::Truncated);
        };
        self.at = end;
        Ok(bytes)
    }

    /// How many bytes are still unread.
    const fn remaining(&self) -> usize {
        self.body.len().saturating_sub(self.at)
    }
}
