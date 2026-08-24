//! Resolving one span handle back to the position its producer holds, and stating the position a refused read already carries.
//!
//! Nothing here invents a position.
//! A byte-offset table answers a handle it issued and refuses one it does not reach; a producer-held table answers in the semantic-origin role with the ordinal the handle already carries; and a read that refused before any table existed was born carrying the byte it sits at.
//! One file composes every [`SourceCoordinate`] the seam hands out, so what a coordinate from here means is settled in one place.

use super::{
    CoordinateRole, SourceCoordinate, SpanHandle, SpanResolutionRefusal, SpanTable, TextReadRefusal,
};

impl SpanTable {
    /// Where the token one handle names sits, in whatever coordinate role this producer speaks.
    ///
    /// [`SpanTable::ProducerHeld`] always answers, and answering is not inventing: the coordinate is in the semantic-origin role and its position is the handle's own ordinal in reading order, which is the fact the handle already carries.
    /// It states no byte, no line, and no file, because this table holds none.
    ///
    /// # Errors
    ///
    /// Returns [`SpanResolutionRefusal`] where a byte-offset table does not reach the handle.
    /// That table's whole content is one byte position per handle it issued, so answering with a semantic-origin coordinate at the handle's index would be a value indistinguishable from an honest answer under the other posture.
    pub fn coordinate_of(
        &self,
        span: SpanHandle,
    ) -> Result<SourceCoordinate, SpanResolutionRefusal> {
        match self {
            Self::ByteOffsets(offsets) => {
                let unreached = SpanResolutionRefusal {
                    handle: span,
                    reaches: offsets.len(),
                };
                let index = usize::try_from(span.index()).map_err(|_| unreached)?;
                offsets
                    .as_slice()
                    .get(index)
                    .map(|offset| SourceCoordinate {
                        role: CoordinateRole::Byte,
                        position: *offset,
                    })
                    .ok_or(unreached)
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
    /// The text route reads bytes, so every cause it establishes has a byte position and is born carrying it.
    /// Stating it in the shape [`SpanTable::coordinate_of`] answers in lets a caller report a read that never produced a capture — and therefore never produced a span table or a handle to look up in one.
    ///
    /// # Nonclaims
    ///
    /// No table is consulted, because at this point there is none: the position is the refusal's own, and the role says which text it counts into.
    #[must_use]
    pub const fn coordinate(self) -> SourceCoordinate {
        SourceCoordinate {
            role: CoordinateRole::Byte,
            position: self.at,
        }
    }
}
