//! Safe disposable child for deadline and bounded-stream observation controls.
#![forbid(unsafe_code)]

use std::io::Write;
use std::time::Duration;

fn main() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let [mode, amount] = arguments.as_slice() else {
        return Err("mode and amount required".to_owned());
    };
    let amount = amount.parse::<usize>().map_err(|error| error.to_string())?;
    if amount > 65536 {
        return Err("control magnitude exceeds 65536".to_owned());
    }
    match mode.as_str() {
        "stdout" => std::io::stdout().write_all(&vec![b'x'; amount]),
        "stderr" => std::io::stderr().write_all(&vec![b'e'; amount]),
        "both" => {
            for _ in 0..amount {
                std::io::stdout()
                    .write_all(b"x")
                    .map_err(|error| error.to_string())?;
                std::io::stderr()
                    .write_all(b"e")
                    .map_err(|error| error.to_string())?;
            }
            Ok(())
        }
        "stall" => {
            println!("ready");
            std::io::stdout()
                .flush()
                .map_err(|error| error.to_string())?;
            std::thread::sleep(Duration::from_millis(
                u64::try_from(amount).map_err(|error| error.to_string())?,
            ));
            Ok(())
        }
        "inherit" => {
            let child = std::process::Command::new(
                std::env::current_exe().map_err(|error| error.to_string())?,
            )
            .args(["stall", &amount.to_string()])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .map_err(|error| error.to_string())?;
            println!("descendant={}", child.id());
            Ok(())
        }
        "fail" => return Err("declared child failure".to_owned()),
        _ => return Err("unknown control mode".to_owned()),
    }
    .map_err(|error| error.to_string())
}
