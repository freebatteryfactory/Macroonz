//! Band 02 — identity: the six-class calculus, the two-column law, the
//! derived-seat law, and the scope guards. Concrete identities live with their
//! owner homes and instantiate these shapes.

pub mod types;

pub use types::{
    ApplicationScope, AuthorityPosition, ByteIdentity, Commitment, CreationLaw, IdentityClass,
    IdentityRole, MintingProfile, Occurrence, OccurrenceForm, OrderComparison, TypedRef,
};
