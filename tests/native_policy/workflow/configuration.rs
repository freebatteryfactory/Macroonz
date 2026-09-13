//! Versioned defaults agree with explicitly assembled execution and retained bytes.

use super::fixture::{self, mapped};
use macroonz::configuration::v1;
use macroonz::harness::input::{self, InputLimits, InputRefusal};
use macroonz::harness::runner;

#[test]
fn composed_execution_preserves_independent_results_and_explicit_invocation() -> Result<(), String>
{
    for (payload, cases) in [(&[1u8, 2][..], 1u32), (&[2, 3][..], 1), (&[1, 2][..], 0)] {
        let table = fixture::table()?;
        let selection = fixture::selection()?;
        let decoder = fixture::decoder()?;
        fixture::reset();
        let high = mapped(v1::run(
            &table.view(),
            &selection,
            &decoder,
            payload,
            fixture::invocation(cases),
        ))?;
        assert_eq!(fixture::observations(), (1, u32::from(cases != 0), 0));
        let envelope = mapped(input::pack(
            decoder.profile(),
            payload,
            InputLimits::declared(2_097_152, 1_048_576),
        ))?;
        let bound = mapped(decoder.decode(envelope.clone()))?;
        let low = runner::run_all(
            &table.view(),
            &selection,
            &fixture::invocation(cases).with_input(bound),
        );
        assert_eq!(high.report(), &low);
        assert_eq!(high.input(), &envelope);
    }
    Ok(())
}

#[test]
fn explicit_input_override_refuses_before_decoding_or_running() -> Result<(), String> {
    let table = fixture::table()?;
    let selection = fixture::selection()?;
    let decoder = fixture::decoder()?;
    assert_eq!(
        v1::input_limits(),
        InputLimits::declared(2_097_152, 1_048_576)
    );
    fixture::reset();
    let refused = macroonz::workflow::run(
        &table.view(),
        &selection,
        &decoder,
        &[2, 3],
        fixture::invocation(1),
        InputLimits::declared(v1::input_limits().envelope(), 1),
    );
    assert!(matches!(refused, Err(InputRefusal::PayloadTooLarge)));
    assert_eq!(fixture::observations(), (0, 0, 0));
    Ok(())
}

#[cfg(all(feature = "native-tooling", any(unix, windows)))]
#[path = "configuration_native.rs"]
mod native;
