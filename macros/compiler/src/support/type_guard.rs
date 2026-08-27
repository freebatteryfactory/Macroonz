//! Declaration and schema constructors.
use super::{
    BoundPath, CrateFacing, DeclarationError, PATH_SEGMENT_LIMIT, SchemaId, SupportName, WallName,
};
use crate::bounded::{NonEmpty, NonEmptyError};
impl SchemaId {
    /// Pins one full schema address.
    #[must_use]
    pub const fn pinned(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Reads the full address.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl WallName {
    /// Declares an owner and spelling.
    ///
    /// # Errors
    /// Returns the first empty seat.
    pub fn named(namespace: &str, stem: &str) -> Result<Self, DeclarationError> {
        if namespace.is_empty() {
            return Err(DeclarationError::EmptyNamespace);
        }
        if stem.is_empty() {
            return Err(DeclarationError::EmptyStem);
        }
        Ok(Self {
            namespace: namespace.to_owned(),
            stem: stem.to_owned(),
        })
    }
    /// Reads the owner.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }
    /// Reads the spelling.
    #[must_use]
    pub fn stem(&self) -> &str {
        self.stem.as_str()
    }
}
impl BoundPath {
    /// Declares a rooted item path.
    ///
    /// # Errors
    /// Returns the identifier, empty, or magnitude refusal established by the input.
    pub fn rooted(facing: CrateFacing, segments: Vec<String>) -> Result<Self, DeclarationError> {
        for segment in &segments {
            if !rendered_name(segment.as_str()) {
                return Err(DeclarationError::SpellingNotAnIdentifier);
            }
        }
        let segments = NonEmpty::new(segments).map_err(|refusal| match refusal {
            NonEmptyError::Empty(_) => DeclarationError::PathSegmentsAbsent,
            NonEmptyError::Overflow(_) => DeclarationError::PathSegmentsUnbounded,
        })?;
        Ok(Self { facing, segments })
    }
    /// Reads the crate facing.
    #[must_use]
    pub const fn facing(&self) -> CrateFacing {
        self.facing
    }
    /// Reads the path segments.
    #[must_use]
    pub fn segments(&self) -> &NonEmpty<String, PATH_SEGMENT_LIMIT> {
        &self.segments
    }
    /// Reads the segment count.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.count()
    }
}
impl SupportName {
    /// Declares an exported address.
    ///
    /// # Errors
    /// Returns an identifier refusal for an unwritable spelling.
    pub fn declared(spelling: &str) -> Result<Self, DeclarationError> {
        if rendered_name(spelling) {
            Ok(Self(spelling.to_owned()))
        } else {
            Err(DeclarationError::SpellingNotAnIdentifier)
        }
    }
    /// Reads the spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.0.as_str()
    }
}
pub use crate::token::{rendered_identifier, rendered_name};
