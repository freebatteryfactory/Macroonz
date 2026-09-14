#![forbid(unsafe_code)]

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let executable = std::env::current_exe()?;
    let name = executable.file_stem().and_then(|name| name.to_str()).ok_or_else(|| io::Error::other("no executable name"))?;
    let phase = match arguments.first().map(String::as_str) {
        Some("-vV") => "verbose",
        Some("--print") => "sysroot",
        Some("--version") if name == "llvm-profdata" => "profdata-version",
        Some("--version") => "cov-version",
        Some("merge") => "merge",
        Some("export") => "export",
        None => "target",
        _ => return Err(io::Error::other("unexpected coverage command")),
    };
    if phase == std::env::var("COVERAGE_TEST_PHASE").map_err(io::Error::other)? {
        match std::env::var("COVERAGE_TEST_MODE").map_err(io::Error::other)?.as_str() {
            "deadline" => std::thread::sleep(std::time::Duration::from_secs(60)),
            "stdout" => return std::io::stdout().write_all(&vec![b'x'; 131_072]),
            "stderr" => return std::io::stderr().write_all(&vec![b'x'; 131_072]),
            "failure" => std::process::exit(19),
            _ => return Err(io::Error::other("unexpected coverage mode")),
        }
    }
    match phase {
        "verbose" => println!("rustc 1.98.1\nrelease: 1.98.1\nhost: {}\nLLVM version: 22.1.8", std::env::var("COVERAGE_TEST_HOST").map_err(io::Error::other)?),
        "sysroot" => println!("{}", std::env::var("COVERAGE_TEST_ROOT").map_err(io::Error::other)?),
        "profdata-version" | "cov-version" => println!("LLVM version 22.1.8"),
        "target" => std::fs::write(std::env::var("LLVM_PROFILE_FILE").map_err(io::Error::other)?, b"test-only raw profile")?,
        "merge" => {
            let path = arguments.windows(2).find_map(|pair| match pair { [flag, path] if flag == "-o" => Some(path), _ => None }).ok_or_else(|| io::Error::other("no merged profile path"))?;
            std::fs::write(path, b"test-only merged profile")?;
        }
        "export" => return Err(io::Error::other("this negative control must fail before a coverage observation")),
        _ => return Err(io::Error::other("unknown coverage role")),
    }
    Ok(())
}
