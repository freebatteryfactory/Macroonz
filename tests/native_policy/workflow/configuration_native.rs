//! Native defaults retain their existing owners' byte custody and request refusals.

use super::{fixture, mapped, v1};
use macroonz::harness::report::archive::{ArchiveLimits, RunArchiveLimits};
use macroonz::native_process::{ProcessError, ProcessLimits, ProcessTool, ResourceControl};
use macroonz::native_storage::{StorageLimits, StorageName};
use macroonz::workflow::{RetentionLimits, StoredRun};
use std::time::Duration;

#[test]
fn default_tool_binds_the_same_explicit_request_and_refuses_missing_host_facts()
-> Result<(), String> {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let executable = directory.join("caller-selected-executable");
    let environment = vec![("EXPLICIT".to_owned(), "value".to_owned())];
    let limits = mapped(ProcessLimits::informed(
        Duration::from_secs(600),
        Duration::from_secs(5),
        16_777_216,
        4_194_304,
    ))?;
    let high = mapped(v1::process_tool(
        executable.clone(),
        directory.to_path_buf(),
        environment.clone(),
        &[],
    ))?;
    let low = mapped(ProcessTool::informed(
        executable.clone(),
        directory.to_path_buf(),
        environment,
        limits,
        &[],
    ))?;
    let arguments = vec!["literal argument with spaces".to_owned()];
    let high = mapped(high.invocation(arguments.clone()))?;
    let low = mapped(low.invocation(arguments))?;
    assert_eq!(high.executable(), low.executable());
    assert_eq!(high.directory(), low.directory());
    assert_eq!(high.environment(), low.environment());
    assert_eq!(high.arguments(), low.arguments());
    assert_eq!(high.limits().execution(), low.limits().execution());
    assert_eq!(high.limits().cleanup(), low.limits().cleanup());
    assert_eq!(high.limits().stdout(), low.limits().stdout());
    assert_eq!(high.limits().stderr(), low.limits().stderr());
    assert!(matches!(
        v1::process_tool("relative".into(), directory.to_path_buf(), Vec::new(), &[]),
        Err(ProcessError::InvalidRequest(_))
    ));
    assert!(matches!(
        v1::process_tool(executable.clone(), "relative".into(), Vec::new(), &[]),
        Err(ProcessError::InvalidRequest(_))
    ));
    assert!(matches!(
        v1::process_tool(
            executable,
            directory.to_path_buf(),
            Vec::new(),
            &[ResourceControl::DenyNetwork]
        ),
        Err(ProcessError::Unsupported(ResourceControl::DenyNetwork))
    ));
    Ok(())
}

#[test]
fn default_and_explicit_retention_have_identical_canonical_bytes_and_bounds() -> Result<(), String>
{
    super::super::retention::observe(|root, _path| {
        let original = fixture::run(&[7, 1, 9])?;
        let capsule = fixture::capsule(&original)?;
        let high_name = mapped(StorageName::informed("defaults"))?;
        let low_name = mapped(StorageName::informed("explicit"))?;
        let explicit = RetentionLimits {
            input: macroonz::harness::input::InputLimits::declared(2_097_152, 1_048_576),
            archive: RunArchiveLimits::declared(
                ArchiveLimits::declared(16_777_216, 2_097_152),
                4096,
            ),
            storage: StorageLimits {
                artifacts: 258,
                bytes: 67_108_864,
            },
        };
        let defaults = v1::retention_limits();
        assert_eq!(defaults.input, explicit.input);
        assert_eq!(defaults.archive, explicit.archive);
        assert_eq!(defaults.storage, explicit.storage);
        mapped(original.retain(root, &high_name, std::slice::from_ref(&capsule), defaults))?;
        mapped(original.retain(root, &low_name, &[capsule], explicit))?;
        fixture::reset();
        let profile = fixture::decoder()?.profile();
        let high = mapped(StoredRun::load(root, &high_name, profile, defaults))?;
        let low = mapped(StoredRun::load(root, &low_name, profile, explicit))?;
        assert_eq!(high.report(), low.report());
        assert_eq!(high.input(), low.input());
        assert_eq!(high.capsule(0), low.capsule(0));
        assert_eq!(fixture::observations(), (0, 0, 0));
        Ok(())
    })
}
