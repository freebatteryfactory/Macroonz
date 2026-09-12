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

pub(crate) fn environment(configuration: &Value) -> Result<Vec<(String, String)>, String> {
    let entries = configuration
        .get("environment")
        .and_then(Value::as_array)
        .ok_or("environment must be an explicit array of key/value pairs")?;
    entries
        .iter()
        .map(|entry| {
            let pair = entry
                .as_array()
                .ok_or("environment entry must be an array")?;
            let [Value::String(key), Value::String(value)] = pair.as_slice() else {
                return Err("environment entry must contain exactly two strings".to_owned());
            };
            Ok((key.clone(), value.clone()))
        })
        .collect()
}
