//! Reading bytes off a source, over the one chunk grid both arms share.
//!
//! A draw hands back the width it was asked for, or it hands back nothing and says how far short the source fell.
//! Neither road reads anything but the source, the cursor, and the width.

use super::encode::chunk_material;
use super::types::{ByteDraw, ByteSourceAddress, SOURCE_CHUNK_BYTES, StreamCursor};

/// One draw against the counter-addressed stream.
///
/// The stream is unbounded, so this arm always yields the width it was asked for: byte insufficiency is the supplied arm's fact, not this one's.
pub(super) fn from_stream(
    address: ByteSourceAddress,
    cursor: StreamCursor,
    width: usize,
) -> ByteDraw {
    let mut bytes: Vec<u8> = Vec::new();
    let mut position = cursor;
    while bytes.len() < width {
        let wanted = width.saturating_sub(bytes.len());
        let available = SOURCE_CHUNK_BYTES.saturating_sub(position.within());
        let taken = wanted.min(available);
        let material = chunk_material(address, position.chunk());
        bytes.extend(material.iter().copied().skip(position.within()).take(taken));
        position = position.advanced(taken);
    }
    ByteDraw::Drawn {
        bytes,
        next: position,
    }
}

/// One draw against supplied bytes, read over the same chunk grid.
pub(super) fn from_material(material: &[u8], cursor: StreamCursor, width: usize) -> ByteDraw {
    let Some(offset) = flat_offset(cursor) else {
        return ByteDraw::Insufficient {
            requested: width,
            available: 0,
        };
    };
    let available = material.len().saturating_sub(offset);
    if available < width {
        return ByteDraw::Insufficient {
            requested: width,
            available,
        };
    }
    ByteDraw::Drawn {
        bytes: material.iter().copied().skip(offset).take(width).collect(),
        next: cursor.advanced(width),
    }
}

/// Where one cursor lands in a flat buffer, or nothing when the position is past what an index can count.
fn flat_offset(cursor: StreamCursor) -> Option<usize> {
    usize::try_from(cursor.chunk())
        .ok()
        .and_then(|chunk| chunk.checked_mul(SOURCE_CHUNK_BYTES))
        .and_then(|base| base.checked_add(cursor.within()))
}
