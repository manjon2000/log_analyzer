use std::collections::HashMap;
use regex::Regex;

pub fn parse_log
(
    log: &str,
    pattern: String,
    fields: &[String]
) -> Result<HashMap<String, String>, String> {
    let mut mapping_results = HashMap::new();
    let serialize_log = Regex::new(
        &pattern.to_string()
    );
    if let Ok(result_regex) = serialize_log {
        if let Some(serialize) = result_regex.captures(&log) {
            for element in fields.iter() {
                if let Some(field) = serialize.name(element.as_str()) {
                    mapping_results.insert(
                        element.clone(),
                        field.as_str().to_string()
                    );
                }
            }
        }
    }

    Ok(mapping_results)
}