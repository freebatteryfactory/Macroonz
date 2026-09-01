//! Callable and wrapper hosts observed over the same recipe material.

use super::{COMPANION_RECIPE, COMPLETE_RECIPE, DOOR, bake, cargo_bytes, emitted_bytes};
use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{CanonicalContent, Destination, TextCapture};

#[test]
fn the_callable_and_wrapper_hosts_emit_one_canonical_projection() -> Result<(), ()> {
    let callable = bake(COMPLETE_RECIPE)?;
    let wrapped_source =
        format!("{{ macroonz }} __macroonz_test_carrier_available {{ {COMPLETE_RECIPE} }}");
    let wrapped_capture = TextCapture::read(&wrapped_source).map_err(|_| ())?;
    let wrapped =
        macroonz_compiler::recipe::bake_wrapped(wrapped_capture.input(), &DOOR).map_err(|_| ())?;

    for destination in [
        Destination::DeclarationSite,
        Destination::TestCarrier,
        Destination::BenchCarrier,
    ] {
        assert_eq!(
            cargo_bytes(callable.projection(), destination),
            cargo_bytes(wrapped.projection(), destination),
            "the two hosts disagreed under {}",
            destination.name()
        );
    }
    assert_eq!(
        callable
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        wrapped
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    Ok(())
}

#[test]
fn the_wrapper_carries_the_no_harness_posture_without_changing_the_recipe() -> Result<(), ()> {
    let callable_capture = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let callable = macroonz_compiler::recipe::bake(
        callable_capture.input(),
        HarnessPosture::Unavailable,
        &DOOR,
    )
    .map_err(|_| ())?;
    let wrapped_source =
        format!("{{ macroonz }} __macroonz_test_carrier_unavailable {{ {COMPANION_RECIPE} }}");
    let wrapped_capture = TextCapture::read(&wrapped_source).map_err(|_| ())?;
    let wrapped =
        macroonz_compiler::recipe::bake_wrapped(wrapped_capture.input(), &DOOR).map_err(|_| ())?;

    assert_eq!(
        callable.projection().identity(),
        wrapped.projection().identity()
    );
    assert_eq!(emitted_bytes(&callable), emitted_bytes(&wrapped));
    Ok(())
}
