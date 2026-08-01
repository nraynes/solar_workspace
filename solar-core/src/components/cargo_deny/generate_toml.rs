use std::collections::HashMap;

use toml::{Value, map::Map};

use crate::solar_error::SolarError;

pub fn generate_toml(allow_licenses: &Vec<String>) -> Result<String, SolarError> {
    let mut toml_config: Map<String, Value> = Map::new();

    // Build allow key value pair.
    let mut allow_list = HashMap::new();
    allow_list.insert("allow", allow_licenses.clone());

    // Build licenses section.
    let licenses_section = Value::from(allow_list);
    toml_config.insert("licenses".to_string(), licenses_section);

    Ok(toml::to_string_pretty(&toml_config)?)
}
