use super::configure::{configuration, publication, standin as body};
use super::fixture::LIMITS;
use super::prepared::outputs;
use super::stage_fixture::{DRIVER, authored, request};
use crate::compiler::configure::{bounds, host, root, tool};
use crate::compiler::refusal::standin;
use macroonz::native_publication::{
    Formatter, PreparedPublication, StagingObservationError, StagingPlan, StagingRun,
};
use macroonz::native_storage::StorageName;

#[test]
fn successful_formatting_with_invalid_rust_still_refuses_staged_compilation() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let executable = standin(
        &root,
        &host,
        "invalid-rust-formatter",
        &body(
            "let mut source = String::new(); std::io::Read::read_to_string(&mut std::io::stdin(), &mut source)?; std::io::stdout().write_all(source.replace(\"OTHER : u8 = 42\", \"OTHER : u8 = \\\"invalid\\\"\").as_bytes())?;",
        ),
    )?;
    let selected = Formatter::qualified(
        &tool(&host, &executable, &root, bounds()?)?,
        configuration(&root)?,
    )
    .map_err(|error| error.to_string())?;
    let publication = publication()?;
    let physical = outputs(&selected, &publication, &root)?;
    assert!(
        physical
            .iter()
            .all(|output| output.process().status().success())
    );
    let prepared = PreparedPublication::formatted(publication, physical, 65536)
        .map_err(|error| error.to_string())?;
    let name = StorageName::informed("invalid-format").map_err(|error| format!("{error:?}"))?;
    let stage = StagingPlan::declared(
        prepared,
        vec![authored("fixture.rs", DRIVER.as_bytes())?],
        LIMITS,
    )
    .and_then(|plan| plan.stage(&root, &name))
    .map_err(|error| error.to_string())?;
    let compiler = request(
        &host,
        &host.rustc,
        stage.path(),
        &root.join("invalid-format-artifact"),
        bounds()?,
    )?;
    let StagingRun::Refused(output) = stage
        .compile(&compiler)
        .map_err(|error| error.to_string())?
    else {
        return Err("invalid formatted Rust qualified".to_owned());
    };
    assert_eq!(output.reason(), &StagingObservationError::Compilation);
    assert!(!output.compiler().process().status().success());
    assert!(
        String::from_utf8_lossy(output.compiler().process().stderr().bytes()).contains("E0308")
    );
    Ok(())
}
