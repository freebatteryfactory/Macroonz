#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let executable = env::current_exe()?;
    let directory = executable.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "the executable has no parent")
    })?;
    let name = executable
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid executable name"))?;
    if name == "llvm-profdata" {
        println!("LLVM version {}", read(directory, "profdata-version.txt")?);
        return Ok(());
    }
    if name == "llvm-cov" {
        println!("LLVM version {}", read(directory, "cov-version.txt")?);
        return Ok(());
    }
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [flag] if flag == "-vV" => {
            println!("rustc {}", read(directory, "release.txt")?);
            println!("host: {}", read(directory, "host.txt")?);
            println!("release: {}", read(directory, "release.txt")?);
            println!("LLVM version: {}", read(directory, "rustc-llvm.txt")?);
            Ok(())
        }
        [flag, value] if flag == "--print" && value == "sysroot" => {
            println!("{}", read(directory, "sysroot.txt")?);
            Ok(())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "unsupported preflight-double command",
        )),
    }
}

fn read(directory: &Path, name: &str) -> io::Result<String> {
    fs::read_to_string(directory.join(name)).map(|text| text.trim().to_owned())
}
