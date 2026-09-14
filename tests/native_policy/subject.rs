//! An ordinary Cargo consumer selecting and observing the public native source.

fn main() {
    let name = macroonz::native_storage::StorageName::informed("run");
    assert!(name.is_ok());
    #[cfg(not(any(unix, windows)))]
    assert!(matches!(
        macroonz::native_storage::StorageRoot::open(std::path::Path::new("declared-root")),
        Err(macroonz::native_storage::StorageError::Unavailable)
    ));
    let source = macroonz::native_clock::source();
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        assert_eq!(
            source.attribution(),
            macroonz::harness::clock::ClockAttribution::Monotonic
        );
        let opened = source.begin();
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert!(opened.finish().duration().is_some());
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        assert_eq!(
            source.attribution(),
            macroonz::harness::clock::ClockAttribution::Unspecified
        );
        assert_eq!(
            source.begin().finish(),
            macroonz::harness::clock::MeasurementReading::Unavailable
        );
    }
}
