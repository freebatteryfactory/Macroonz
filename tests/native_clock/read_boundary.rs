//! Controlled native operands observed against the unchanged private reading source.

#[path = "../../src/native_clock/read.rs"]
mod reading;

use macroonz::harness::clock::ClockReadRefusal;
use std::time::{Duration, Instant};

#[test]
fn actual_reader_supplies_ordered_nanoseconds() -> Result<(), ClockReadRefusal> {
    let first = reading::native()?;
    let second = reading::native()?;
    assert!(second >= first);
    Ok(())
}

#[test]
fn equal_forward_and_reversed_instants_remain_distinct() -> Result<(), String> {
    let earlier = Instant::now();
    let later = earlier
        .checked_add(Duration::from_nanos(17))
        .ok_or("the host cannot represent the declared nearby instant")?;
    assert!(later > earlier);
    assert_eq!(reading::offset(earlier, earlier), Ok(0));
    assert_eq!(reading::offset(earlier, later), Ok(17));
    assert_eq!(
        reading::offset(later, earlier),
        Err(ClockReadRefusal::Refused)
    );
    Ok(())
}

#[test]
fn the_nanosecond_ceiling_refuses_without_saturation_or_truncation() {
    assert_eq!(reading::nanoseconds(Duration::ZERO), Ok(0));
    assert_eq!(
        reading::nanoseconds(Duration::new(18_446_744_073, 709_551_615)),
        Ok(u64::MAX)
    );
    assert_eq!(
        reading::nanoseconds(Duration::new(18_446_744_073, 709_551_616)),
        Err(ClockReadRefusal::Refused)
    );
    assert_eq!(
        reading::nanoseconds(Duration::MAX),
        Err(ClockReadRefusal::Refused)
    );
}
