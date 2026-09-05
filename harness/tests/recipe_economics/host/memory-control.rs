//! Disposable process-memory instrument control, not a Macroonz workload or allocator observer.
//! The external observer measures the process; this child only allocates, touches, and consumes declared bytes.
#![forbid(unsafe_code)]

mod resident;

use std::hint::black_box;

const MAX_BYTES: usize = 128 * 1024 * 1024;
const FILL: u8 = 0xa5;

fn main() -> Result<(), String> {
    let mut arguments = std::env::args().skip(1);
    let declared = arguments
        .next()
        .ok_or("one byte-count argument is required")?;
    if declared == "lifecycle" {
        let mode = arguments.next().ok_or("one lifecycle mode is required")?;
        if arguments.next().is_some() {
            return Err("unexpected lifecycle argument".to_owned());
        }
        return lifecycle(&mode);
    }
    let held = match arguments.next().as_deref() {
        None => false,
        Some("hold-for-observer") => true,
        Some(_) => return Err("unexpected control argument".to_owned()),
    };
    if arguments.next().is_some() {
        return Err("unexpected control argument".to_owned());
    }
    let bytes = declared
        .parse::<usize>()
        .map_err(|error| error.to_string())?;
    if bytes > MAX_BYTES {
        return Err("control allocation exceeds its declared 128 MiB ceiling".to_owned());
    }
    let mut material = Vec::<u8>::new();
    material
        .try_reserve_exact(bytes)
        .map_err(|error| error.to_string())?;
    material.resize(bytes, black_box(FILL));
    let observed = resident::checksum(&material)?;
    let expected = u64::try_from(bytes)
        .map_err(|error| error.to_string())?
        .checked_mul(u64::from(FILL))
        .ok_or("expected checksum overflow")?;
    if black_box(observed) != expected || material.len() != bytes {
        return Err("the declared allocation was not consumed intact".to_owned());
    }
    println!("memory-control requested-bytes={bytes} consumed-checksum={observed}");
    if held {
        resident::wait_for_release(&material, expected)?;
    }
    drop(black_box(material));
    Ok(())
}

fn lifecycle(mode: &str) -> Result<(), String> {
    use std::io::Write;

    match mode {
        "missing" => return Ok(()),
        "malformed" => {
            println!("not the declared readiness");
            return Ok(());
        }
        "stall-ready" => {
            std::thread::sleep(std::time::Duration::from_secs(30));
            return Err("observer failed to enforce readiness deadline".to_owned());
        }
        "ready" | "fail" | "stderr" | "tail" | "stall-release" => {}
        _ => return Err("unknown lifecycle mode".to_owned()),
    }
    println!("memory-lifecycle ready");
    std::io::stdout()
        .flush()
        .map_err(|error| error.to_string())?;
    if mode == "stall-release" {
        std::thread::sleep(std::time::Duration::from_secs(30));
        return Err("observer failed to enforce release deadline".to_owned());
    }
    resident::wait_for_release(&[], 0)?;
    match mode {
        "fail" => return Err("deliberate lifecycle failure".to_owned()),
        "stderr" => eprintln!("deliberate stderr"),
        "tail" => println!("deliberate extra output"),
        "ready" => {}
        _ => return Err("unreachable lifecycle continuation".to_owned()),
    }
    Ok(())
}
