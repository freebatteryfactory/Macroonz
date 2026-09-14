use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = std::io::stdout().lock();
    for key in [
        "PATH", "SystemRoot", "TEMP", "TMP", "INCLUDE", "LIB", "LIBPATH",
        "HOME", "USERPROFILE", "CARGO_HOME", "RUSTUP_HOME", "SDKROOT",
        "MACOSX_DEPLOYMENT_TARGET", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH",
    ] {
        if let Some(raw) = std::env::var_os(key) {
            let value = raw.into_string().map_err(|_| "non-Unicode tool environment")?;
            write!(output, "{key}\0{value}\0")?;
        }
    }
    Ok(())
}
