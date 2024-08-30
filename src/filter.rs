use std::collections::HashMap;

pub fn filter_by_key_value(
    logs: &[HashMap<String, String>],
    key: &str,
    value: &str
) -> Result<Vec<HashMap<String, String>>, String> {
    Ok(
        logs.iter()
            .filter(
                |log|
                log.get(key)
                    .map_or(
                        false,
                        |v|
                        v == value
                    )
            )
            .cloned()
            .collect::<Vec<HashMap<String, String>>>()
    )

}