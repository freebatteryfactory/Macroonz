//! Generate declared structural alternatives for the existing public compiler workflow.

mod codec;
mod declaration;
mod transition;
#[path = "../support/native_input/read.rs"]
mod input;

use macroonz::compiler::GeneratedToken;
use std::io::Write;

fn main() -> Result<(), String> {
    let configuration = input::read()?;
    let family = input::text(&configuration, "family")?;
    let selection = input::text(&configuration, "selection")?;
    let (surface, observer) = match family {
        "codec" => (codec::surface()?, codec::OBSERVER),
        "transition" => (transition::targets()?, transition::OBSERVER),
        "effect" => (transition::effects()?, transition::OBSERVER),
        _ => return Err("family must be codec, transition or effect".to_owned()),
    };
    let tokens = if selection == "unchanged" {
        surface.site().production()
    } else {
        let index = selection
            .parse::<usize>()
            .map_err(|_| "selection must be unchanged or a zero-based alternative index")?;
        surface
            .site()
            .alternatives()
            .get(index)
            .ok_or_else(|| {
                format!(
                    "selection {index} is absent; this surface has {} alternatives",
                    surface.site().alternatives().len()
                )
            })?
            .meaning()
    };
    let [GeneratedToken::Text(source)] = tokens else {
        return Err("the structural producer did not return one source literal".to_owned());
    };
    writeln!(std::io::stdout(), "{source}\n{observer}").map_err(|error| error.to_string())
}
