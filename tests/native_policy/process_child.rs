//! Controlled child behaviors supplied through stdin rather than ambient arguments.

use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

#[test]
#[ignore = "Invoked as a controlled subprocess by the public process controls."]
fn subject() -> Result<(), String> {
    let mut reader = std::io::stdin().lock();
    let mut mode = String::new();
    let mut root = String::new();
    reader
        .read_line(&mut mode)
        .map_err(|error| error.to_string())?;
    reader
        .read_line(&mut root)
        .map_err(|error| error.to_string())?;
    match mode.trim_end() {
        "finite" => {
            std::io::stderr()
                .write_all(&vec![0x81; 300_000])
                .map_err(|error| error.to_string())?;
            std::io::stdout()
                .write_all(&vec![0x80; 400_000])
                .map_err(|error| error.to_string())?;
            Ok(())
        }
        "flood-out" => flood(std::io::stdout().lock()),
        "flood-err" => flood(std::io::stderr().lock()),
        "sleep" => {
            super::process::mark_ready()?;
            std::thread::sleep(Duration::from_secs(20));
            Err("deadline was not enforced".to_owned())
        }
        "fail" => Err("declared-child-failure".to_owned()),
        "environment" => super::process_environment::inspect(Path::new(root.trim_end())),
        "parent-exit" | "parent-sleep" => parent(Path::new(root.trim_end()), mode.trim_end()),
        "grandchild" => grandchild(Path::new(root.trim_end())),
        unknown => Err(format!("unknown controlled mode: {unknown}")),
    }
}

fn flood(mut pipe: impl Write) -> Result<(), String> {
    let bytes = [0x82u8; 8192];
    for _chunk in 0u32..100_000 {
        pipe.write_all(&bytes).map_err(|error| error.to_string())?;
    }
    Err("the output limit was not enforced".to_owned())
}

fn parent(root: &Path, mode: &str) -> Result<(), String> {
    let mut command = Command::new(std::env::current_exe().map_err(|error| error.to_string())?);
    command
        .args(super::process::arguments("process_child::subject"))
        .stdin(Stdio::from(super::process::input(root, "grandchild")?))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    let mut child = command.spawn().map_err(|error| error.to_string())?;
    let pipe = child.stdout.take().ok_or("missing grandchild stdout")?;
    super::process::ready(std::io::BufReader::new(pipe))?;
    super::process::mark_ready()?;
    if mode == "parent-sleep" {
        std::thread::sleep(Duration::from_secs(20));
    }
    drop(child);
    Ok(())
}

fn grandchild(root: &Path) -> Result<(), String> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join("held.lock"))
        .map_err(|error| error.to_string())?;
    file.try_lock().map_err(|error| error.to_string())?;
    std::fs::write(root.join("grandchild.pid"), std::process::id().to_string())
        .map_err(|error| error.to_string())?;
    super::process::mark_ready()?;
    std::thread::sleep(Duration::from_secs(20));
    drop(file);
    Err("grandchild was not terminated".to_owned())
}
