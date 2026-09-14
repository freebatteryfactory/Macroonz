//! The documented public codec caller must execute in the ordinary library posture.

use std::path::Path;

#[test]
fn public_codec_example_executes_independent_bytes_and_refusals() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = crate::check::scratch()?;
    let output = crate::check::cargo(
        root,
        root,
        &scratch,
        "codec-example",
        &[
            "run",
            "--example",
            "codec_workflow",
            "--no-default-features",
            "--locked",
            "--offline",
            "-j1",
        ],
    )?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        output.stdout,
        b"codec: exact bytes and round trip agree; malformed bytes and zero count refuse\n"
    );
    Ok(())
}
