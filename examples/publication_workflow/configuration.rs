use macroonz::native_publication::BakeCommand;
use serde_json::Value;

pub(super) fn read() -> Result<(BakeCommand, u64), String> {
    let configuration = super::input::read()?;
    let value = if super::input::text(&configuration, "action")? == "recover" {
        0
    } else {
        configuration
            .get("value")
            .and_then(Value::as_u64)
            .ok_or("value must be a u64")?
    };
    let fixture = b"mod value;\nuse std::io::Write;\nfn main() -> std::io::Result<()> { writeln!(std::io::stdout(), \"{}\", value::VALUE) }\n";
    let command = super::publication_configuration::command(
        &configuration,
        super::generate::LIMITS,
        fixture,
        "published-value",
    )?;
    Ok((command, value))
}
