//! Cross-cutting declaration and schema vocabulary.
use crate::bounded::NonEmpty;
#[path = "type_guard.rs"]
mod guard;
/// The maximum rendered path depth after its crate root.
pub const PATH_SEGMENT_LIMIT: usize = 8;
/// A generated-support schema identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId([u8; 32]);
/// The compiler's pinned generated-support schema expectation.
pub const EXPECTED_SCHEMA_ID: SchemaId = SchemaId::pinned([
    185, 251, 251, 45, 168, 146, 85, 42, 248, 177, 196, 48, 117, 229, 207, 5, 84, 120, 104, 25,
    150, 41, 202, 2, 243, 73, 31, 148, 241, 22, 122, 34,
]);
crate::roster! {
    /// The crate at which a rendered path is rooted.
    pub enum CrateFacing {
        /// The declaring crate.
        Declaring = "declaring",
        /// The harness crate.
        Harness = "harness",
    }
}
crate::roster! {
    /// The coupled gate-seat form.
    pub enum DeliveryForm {
        /// Trial table and deferred cargo.
        Trials = "trials",
        /// Benchmark table and reporter cargo.
        Benches = "benches",
    }
}
/// A support-declaration refusal.
#[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarationError {
    /// A name has no owner.
    EmptyNamespace,
    /// A name has no spelling.
    EmptyStem,
    /// A spelling is not one Rust identifier.
    SpellingNotAnIdentifier,
    /// A path has no segment after its root.
    PathSegmentsAbsent,
    /// A path exceeds its segment bound.
    PathSegmentsUnbounded,
}
/// An owner namespace and spelling.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallName {
    namespace: String,
    stem: String,
}
/// A rendered path rooted at a declared crate facing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundPath {
    facing: CrateFacing,
    segments: NonEmpty<String, PATH_SEGMENT_LIMIT>,
}
/// The public alias a consumer invokes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportName(String);
/// The rendered-identifier admission roads.
pub use guard::{rendered_identifier, rendered_name};
