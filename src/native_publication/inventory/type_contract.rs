use super::InventoryError;
use std::fmt;

impl fmt::Display for InventoryError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        into.write_str(match self {
            Self::Path => "publication path is non-normal, reserved or nonportable",
            Self::PathCollision => "publication destinations alias or collide with a parent file",
            Self::Binding => "publication bindings differ from the sealed unit roster",
            Self::Stamp => "stamp definition or record differs from its sealed publication unit",
            Self::Landing => "landing bindings differ from the stamp's complete site roster",
            Self::FileBound => "publication exceeds its complete file allowance",
            Self::ByteBound => "publication exceeds its aggregate source-byte allowance",
        })
    }
}

impl std::error::Error for InventoryError {}
