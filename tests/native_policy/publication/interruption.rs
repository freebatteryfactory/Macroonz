use super::install_fixture::{destination, finish, installed, subject};
use crate::compiler::configure::root;
use macroonz::native_publication::DestinationError;
use macroonz::native_storage::StorageError;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

#[test]
#[ignore = "Invoked by the publication interruption control with its destination on stdin."]
fn interrupted_installer() -> Result<(), String> {
    let mut input = std::io::stdin().lock();
    let mut line = String::new();
    input
        .read_line(&mut line)
        .map_err(|error| error.to_string())?;
    let destination = destination(Path::new(line.trim_end()))?;
    let mut installation = destination
        .recover()
        .map_err(|error| error.to_string())?
        .ok_or("intent absent")?;
    assert!(
        installation
            .write_next()
            .map_err(|error| error.to_string())?
            .is_some()
    );
    let mut output = std::io::stdout().lock();
    writeln!(output, "publication-ready").map_err(|error| error.to_string())?;
    output.flush().map_err(|error| error.to_string())?;
    line.clear();
    input
        .read_line(&mut line)
        .map_err(|error| error.to_string())?;
    drop(installation);
    Err("the parent must terminate this installer".to_owned())
}

#[test]
fn killed_installer_leaves_recoverable_output_and_releases_exclusive_custody() -> Result<(), String>
{
    let source = root()?;
    let compiled = subject(&source, "subject", [7, 9, 42], "generated/other.rs")?;
    let output = root()?;
    let destination = destination(&output)?;
    drop(
        destination
            .begin(&compiled)
            .map_err(|error| error.to_string())?,
    );
    let mut child = Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
        .args([
            "--exact",
            "publication::interruption::interrupted_installer",
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| error.to_string())?;
    let exchange = (|| {
        let mut input = child.stdin.take().ok_or("child stdin absent")?;
        writeln!(input, "{}", output.display()).map_err(|error| error.to_string())?;
        input.flush().map_err(|error| error.to_string())?;
        let reader = child.stdout.take().ok_or("child stdout absent")?;
        let (sender, receiver) = std::sync::mpsc::channel();
        let reader = std::thread::spawn(move || {
            let ready = std::io::BufReader::new(reader)
                .lines()
                .take(16)
                .any(|line| line.is_ok_and(|line| line.contains("publication-ready")));
            let _sent = sender.send(ready);
        });
        let ready = receiver.recv_timeout(Duration::from_secs(30));
        let held = destination.check(compiled.prepared());
        Ok::<_, String>((input, reader, ready, held))
    })();
    let killed = child.kill();
    let status = child.wait().map_err(|error| error.to_string())?;
    let (input, reader, ready, held) = exchange?;
    drop(input);
    reader.join().map_err(|_panic| "reader panicked")?;
    killed.map_err(|error| error.to_string())?;
    assert!(ready.map_err(|error| error.to_string())?);
    assert!(!status.success());
    assert!(matches!(
        held,
        Err(DestinationError::Storage(StorageError::Busy))
    ));
    assert!(output.join("generated/alpha.rs").is_file());
    assert!(!output.join("generated/beta.rs").exists());
    drop(destination);
    let fresh = super::install_fixture::destination(&output)?;
    finish(
        fresh
            .recover()
            .map_err(|error| error.to_string())?
            .ok_or("recovery absent")?,
    )?;
    installed(&output, compiled.prepared())
}
