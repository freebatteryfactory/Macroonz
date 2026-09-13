use super::command_fixture::{generate, preparation};
use super::destination_fixture::snapshot;
use super::install_fixture::installed;
use super::stage_fixture::authored;
use super::visibility_fixture::{DRIVER, publication};
use crate::compiler::configure::{bounds, host, root};
use crate::compiler::real::read_count;
use macroonz::compiler::stamp::OPAQUE_REACH_REFUSAL;
use macroonz::native_publication::{BakeCause, BakeCommand, BakeOutput, StagingRun, bake};
use std::time::Duration;

#[test]
fn formatted_publication_preserves_each_reach_and_refuses_opaque_or_widened_use_before_installation()
-> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    let mut preparation = preparation(&source, &work, &host)?;
    preparation.tools.runs = 9;
    preparation.tools.time = Duration::from_secs(585);
    preparation.tools.capture_bytes = 18_874_368;
    let mut first = None;
    for _repeat in 0usize..2 {
        let mut command = generate(preparation.clone(), &source, &output, &host, bounds()?)?;
        replace_driver(&mut command, DRIVER)?;
        let BakeOutput::Generated(compiled) =
            bake(command, publication).map_err(|error| error.to_string())?
        else {
            return Err("visibility generation lost compilation".to_owned());
        };
        read_count(compiled.compiler(), &host, &source)?;
        installed(&output, compiled.prepared())?;
        let material = compiled
            .prepared()
            .files()
            .map(|file| {
                (
                    file.path().clone(),
                    file.canonical_digest(),
                    file.bytes().to_vec(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(material.len(), 7);
        if let Some(first) = &first {
            assert_eq!(&material, first);
        } else {
            first = Some(material);
        }
    }
    let before = snapshot(&output)?;
    for (addition, expected) in [
        (
            "fn forbidden_private() -> u8 { private_site::VALUE }",
            "constant import `VALUE` is private",
        ),
        (
            "fn forbidden_module() -> u8 { module_site::VALUE }",
            "constant import `VALUE` is private",
        ),
        (
            "fn forbidden_parent() -> u8 { outer::parent_site::VALUE }",
            "constant import `VALUE` is private",
        ),
        (
            "pub use crate_site::VALUE as EXPORTED;",
            "cannot be re-exported outside",
        ),
        (
            "macro_rules! forward { ($reach:vis $name:ident) => { crate::published_reach! { $reach $name } }; } mod opaque { forward!(pub refused); }",
            OPAQUE_REACH_REFUSAL,
        ),
    ] {
        let mut command = generate(preparation.clone(), &source, &output, &host, bounds()?)?;
        replace_driver(&mut command, &format!("{DRIVER}\n{addition}\n"))?;
        let failure = bake(command, publication)
            .err()
            .ok_or("invalid reach compiled")?;
        let BakeCause::Compilation(StagingRun::Refused(refused)) = failure.cause() else {
            return Err(format!("wrong visibility refusal: {failure}"));
        };
        let stderr = String::from_utf8_lossy(refused.compiler().process().stderr().bytes());
        assert!(stderr.contains(expected), "{stderr}");
        assert_eq!(snapshot(&output)?, before);
    }
    Ok(())
}

fn replace_driver(command: &mut BakeCommand, source: &str) -> Result<(), String> {
    let BakeCommand::Generate(selection) = command else {
        return Err("generation command absent".to_owned());
    };
    selection.authored = vec![authored("fixture.rs", source.as_bytes())?];
    Ok(())
}
