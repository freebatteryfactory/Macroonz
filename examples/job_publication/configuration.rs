use macroonz::native_publication::BakeCommand;
use serde_json::Value;

pub(super) fn read() -> Result<(BakeCommand, [u8; 2]), String> {
    let configuration = super::input::read()?;
    let values = if super::input::text(&configuration, "action")? == "recover" {
        [0, 0]
    } else {
        values(&configuration)?
    };
    let command = super::publication_configuration::command(
        &configuration,
        super::generate::LIMITS,
        include_bytes!("fixture.rs"),
        "job-publication",
    )?;
    Ok((command, values))
}

fn values(configuration: &Value) -> Result<[u8; 2], String> {
    let values = configuration
        .get("values")
        .and_then(Value::as_array)
        .ok_or("values must be two u8 integers")?;
    let [first, second] = values.as_slice() else {
        return Err("values must be two u8 integers".to_owned());
    };
    let byte = |value: &Value| {
        value
            .as_u64()
            .and_then(|number| u8::try_from(number).ok())
            .ok_or_else(|| "values must be two u8 integers".to_owned())
    };
    Ok([byte(first)?, byte(second)?])
}
