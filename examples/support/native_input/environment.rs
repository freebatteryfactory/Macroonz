use serde_json::Value;

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
