//! Compile and execute the lawful Job fixture and compare an independently anchored refusal.

mod checks;
mod configuration;
mod execute;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::harness::oracle::DeclaredCompilation;
use std::io::Write;

fn main() -> Result<(), String> {
    let settings = configuration::read()?;
    let lawful = execute::compile(&settings.lawful)?;
    checks::conforms(&lawful, &DeclaredCompilation::compiles())?;
    execute::lawful_assertion(&lawful)?;
    let hostile = execute::compile(&settings.hostile)?;
    let expected = DeclaredCompilation::refuses(checks::anchor("E0308", 26, 29)?);
    checks::conforms(&hostile, &expected)?;
    checks::disagreements(&lawful, &hostile)?;
    writeln!(std::io::stdout(), "lawful: compiled and executed\nwrong-state: E0308 at fixtures/wrong-state.rs:3:26..29\ncontrols: wrong code, shifted span and lawful twin disagree")
        .map_err(|error| error.to_string())
}
