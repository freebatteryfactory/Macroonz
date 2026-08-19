//! Resolving one span handle back to the position its producer holds, and
//! stating the position a refused read already carries.
//!
//! The services never invent a position.
//! A byte-offset table answers a handle it issued and refuses one it does not
//! reach; a producer-held table answers in the semantic-origin role with the
//! ordinal the handle already carries.
//! Those are the only two answers, and neither is a guess.
//!
//! A read that refused before any table existed is the third position this seam
//! states, and it is the least invented of the three: the refusal was born
//! carrying the byte it sits at, and this file only puts that byte into the
//! coordinate shape the other two answers wear.
//! One file composes every [`SourceCoordinate`] the seam hands out, so what a
//! coordinate from here means is settled in one place.

use super::{SpanHandle, SpanResolutionRefusal, SpanTable, TextReadRefusal};
use threadpak::declaration::{CoordinateRole, SourceCoordinate};

impl SpanTable {
    /// Where the token one handle names sits, in whatever coordinate role this
    /// producer speaks.
    ///
    /// [`SpanTable::ProducerHeld`] always answers, and answering is not
    /// inventing: the coordinate it returns is in the semantic-origin role and
    /// its position is the handle's own ordinal in reading order, which is the
    /// fact the handle already carries.
    /// It states no byte, no line, and no file, because this table holds none —
    /// the producer that kept the compiler's spans does that mapping on its own
    /// side.
    ///
    /// # Errors
    ///
    /// Returns [`SpanResolutionRefusal`] when a byte-offset table does not
    /// reach the handle.
    /// That table's whole content is one byte position per handle it issued, so
    /// a handle it never issued has no position in it, and answering with a
    /// semantic-origin coordinate at the handle's index would be a value
    /// indistinguishable from an honest answer under the other posture.
    pub fn coordinate_of(
        &self,
        span: SpanHandle,
    ) -> Result<SourceCoordinate, SpanResolutionRefusal> {
        match self {
            Self::ByteOffsets(offsets) => {
                let index = usize::try_from(span.index()).unwrap_or(usize::MAX);
                offsets
                    .iter()
                    .nth(index)
                    .map(|offset| SourceCoordinate {
                        role: CoordinateRole::Byte,
                        position: *offset,
                    })
                    .ok_or(SpanResolutionRefusal {
                        handle: span,
                        reaches: u32::try_from(offsets.len()).unwrap_or(u32::MAX),
                    })
            }
            Self::ProducerHeld => Ok(SourceCoordinate {
                role: CoordinateRole::SemanticOrigin,
                position: u64::from(span.index()),
            }),
        }
    }
}

impl TextReadRefusal {
    /// Where this refusal sits, as a typed coordinate in the byte role.
    ///
    /// The text route reads bytes, so every cause it establishes has a byte
    /// position and is born carrying it. This states that position in the same
    /// typed shape [`SpanTable::coordinate_of`] answers in, so a caller
    /// reporting a read that never produced a capture — and therefore never
    /// produced a span table or a handle to look up in one — points at a real
    /// byte of the text it was handed.
    ///
    /// # Nonclaims
    ///
    /// No table is consulted, because at this point there is none: the position
    /// is the refusal's own, and the role says which text it counts into.
    /// It is not a span handle wearing a coordinate's shape, and nothing here
    /// turns it into one.
    #[must_use]
    pub const fn coordinate(self) -> SourceCoordinate {
        SourceCoordinate {
            role: CoordinateRole::Byte,
            position: self.at,
        }
    }
}
