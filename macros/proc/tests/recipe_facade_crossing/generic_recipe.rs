//! Independent generic recipe behaviors through one renamed facade package.

use super::support::{observe_generic_crossing, observe_generic_refusals, observed_in_scratch};

#[test]
fn each_generic_shape_crosses_the_renamed_facade_without_the_dense_fixture() -> Result<(), String> {
    observed_in_scratch(observe_generic_crossing)
}

#[test]
fn malformed_generic_shapes_refuse_in_external_package_context() -> Result<(), String> {
    observed_in_scratch(observe_generic_refusals)
}
