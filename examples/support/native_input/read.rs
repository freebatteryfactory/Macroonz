use serde_json::Value;
use std::io::Read;

pub(crate) fn read() -> Result<Value, String> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .take(65_537)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > 65_536 {
        return Err("example configuration exceeds 64 KiB".to_owned());
    }
    serde_json::from_slice(&bytes).map_err(|error| error.to_string())
}

pub(crate) fn text<'a>(configuration: &'a Value, name: &str) -> Result<&'a str, String> {
    configuration
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing text configuration field {name}"))
}
