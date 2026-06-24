use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug, Getters)]
pub struct Parameters {
    project_name: String,
}

impl Parameters {
    pub fn new(project_name: String) -> Self {
        Self { project_name }
    }
}
