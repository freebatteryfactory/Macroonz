//! Declared enum orders and owner permissions cross deferred carriers through the facade.

mod observation;

use enum_mutation_workflow::Priority;

use std::io::Write;

macroonz::support! { enum_mutation_workflow::priority_support {} }
macroonz::support! { enum_mutation_workflow::pipeline_support { declaring: enum_mutation_workflow, } }

fn main() -> Result<(), String> {
    observation::permitted()?;
    observation::withheld()?;
    std::io::stdout()
        .write_all(b"enum: unchanged order agrees; two alternatives disagree; unpermitted point stays discoverable\n")
        .map_err(|error| error.to_string())
}
