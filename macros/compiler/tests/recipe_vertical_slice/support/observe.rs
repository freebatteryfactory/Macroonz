//! Callable bake, refusal, cargo, and emission observations shared across claims.

use super::fixtures::DOOR;
use macroonz_compiler::Diagnostic;
use macroonz_compiler::recipe::{HarnessPosture, ProjectorReplacement, RecipeBake};
use macroonz_compiler::{Destination, Door, GeneratedTree, TextCapture};

/// Bake one recipe source through the callable road under the harness-available posture.
pub(crate) fn bake(source: &str) -> Result<RecipeBake, ()> {
    bake_under(source, HarnessPosture::Available)
}

/// Bake one recipe source through the callable road under the given harness posture.
pub(crate) fn bake_under(source: &str, harness: HarnessPosture) -> Result<RecipeBake, ()> {
    bake_at(source, harness, &DOOR)
}

/// Bake one recipe source through the callable road under the given harness posture and a lane-owned door.
pub(crate) fn bake_at(
    source: &str,
    harness: HarnessPosture,
    door: &Door,
) -> Result<RecipeBake, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), harness, door).map_err(|_| ())
}

/// The diagnostic one recipe source earns under the given harness posture and a lane-owned door.
pub(crate) fn refusal_at(
    source: &str,
    harness: HarnessPosture,
    door: &Door,
) -> Result<Diagnostic, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), harness, door)
        .err()
        .ok_or(())
}

/// Bake one recipe source with caller-owned projectors replacing the standard ones for the named roles.
pub(crate) fn bake_with(
    source: &str,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<RecipeBake, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        replacements,
    )
    .map_err(|_| ())
}

/// The diagnostic one recipe source earns when caller-owned projectors are offered for the named roles.
pub(crate) fn bake_with_refusal(
    source: &str,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<Diagnostic, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        replacements,
    )
    .err()
    .ok_or(())
}

/// The diagnostic one recipe source earns under the harness-available posture.
pub(crate) fn refusal(source: &str) -> Result<Diagnostic, ()> {
    refusal_under(source, HarnessPosture::Available)
}

/// The diagnostic one recipe source earns under the given harness posture.
pub(crate) fn refusal_under(source: &str, harness: HarnessPosture) -> Result<Diagnostic, ()> {
    refusal_at(source, harness, &DOOR)
}

pub(crate) fn refusal_summary(source: &str) -> Result<String, ()> {
    refusal(source).map(|refusal| refusal.summary().to_owned())
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
