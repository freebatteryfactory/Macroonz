//! The steps every road here takes the same way: projecting a refusal into a diagnostic, reading a carrier address, materializing a rendered tree, and reading proved cargo off a terminal.

use crate::bounded::Overflow;
use crate::closure::PartitionCargo;
use crate::diagnostic::{Diagnostic, Placement, Refused};
use crate::expansion::Expansion;
use crate::kind::{Destination, Kind};
use crate::render::RenderError;
use crate::request::Door;
use crate::support::{AssemblyError, CargoAxis, DeferredCargo, ProvedCargo, SupportName};
use crate::token::{GeneratedToken, GeneratedTree, SpanHandle, SpanTable};

/// One helper refusal as the diagnostic a person reads, at the token the refusal names.
///
/// The producer holds the compiler's spans, so the placement says so and the host resolves the handle.
pub(super) fn helper_refused<E: Refused>(refusal: &E, at: SpanHandle, door: &Door) -> Diagnostic {
    Diagnostic::refused(
        refusal,
        door,
        &Placement::AtToken {
            token: at,
            spans: &SpanTable::ProducerHeld,
        },
    )
}

/// One refusal about the declaration as a whole, as the diagnostic a person reads.
pub(super) fn whole<E: Refused>(refusal: &E, door: &Door) -> Diagnostic {
    Diagnostic::refused(refusal, door, &Placement::WholeDeclaration)
}

/// The carrier address one authored spelling declares, in the carrier home's own vocabulary.
///
/// The descriptor grammar admitted the spelling as an identifier already, and the carrier home admits it again on its own terms, because the value crossing homes is a spelling and each home answers for what it renders.
pub(super) fn support_address(spelling: &str, door: &Door) -> Result<SupportName, Diagnostic> {
    SupportName::declared(spelling).map_err(|refusal| whole(&refusal, door))
}

/// One rendered token sequence as the tree a unit is written from, in the render step's own refusal vocabulary.
pub(super) fn unit_tree(
    tokens: Result<Vec<GeneratedToken>, Overflow>,
) -> Result<GeneratedTree, RenderError> {
    let composed = tokens.map_err(overflown)?;
    GeneratedTree::assembled(composed).map_err(overflown)
}

/// One token-magnitude overflow in the render step's own words.
pub(super) const fn overflown(overflow: Overflow) -> RenderError {
    RenderError::TokensUnbounded {
        bound: overflow.capacity,
        observed: overflow.offered,
    }
}

/// One axis's proved cargo, read off the terminal that proved it.
///
/// The tree is read off the delivery first because the promotion road takes the cargo it must compare, and a delivery that carries nothing is the promotion road's own refusal spoken here in its words.
pub(super) fn proved_off<K: Kind>(
    expansion: &Expansion<K>,
    axis: CargoAxis,
    destination: Destination,
    door: &Door,
) -> Result<ProvedCargo, Diagnostic> {
    let cargo = match expansion.emission().joined(destination) {
        Some(PartitionCargo::Carried(proved)) => DeferredCargo::deferred(proved.tree().clone()),
        Some(PartitionCargo::NothingPlanned) | None => {
            return Err(whole(
                &AssemblyError::of(crate::support::AssemblyIssue::CargoNotTheSourcesOwn {
                    source: expansion.identity(),
                    destination,
                }),
                door,
            ));
        }
    };
    ProvedCargo::carried(expansion, axis, destination, cargo)
        .map_err(|refusal| whole(&refusal, door))
}
