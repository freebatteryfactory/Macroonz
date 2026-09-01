//! Callable bake, refusal, cargo, and emission observations shared across claims.

use super::fixtures::DOOR;
use macroonz_compiler::recipe::{HarnessPosture, RecipeBake};
use macroonz_compiler::{Destination, GeneratedTree, TextCapture};

pub(crate) fn bake(source: &str) -> Result<RecipeBake, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR).map_err(|_| ())
}

pub(crate) fn refusal_summary(source: &str) -> Result<String, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .err()
        .map(|refusal| refusal.summary().to_owned())
        .ok_or(())
}

pub(crate) fn cargo_bytes(
    expansion: &macroonz_compiler::Expansion<macroonz_compiler::recipe::RecipeProjection>,
    destination: Destination,
) -> Option<Vec<u8>> {
    expansion
        .emission()
        .joined(destination)
        .and_then(macroonz_compiler::PartitionCargo::tokens)
        .map(GeneratedTree::canonical_bytes)
}

pub(crate) fn emitted_bytes(bake: &RecipeBake) -> Option<Vec<u8>> {
    bake.emit().tokens().map(GeneratedTree::canonical_bytes)
}
