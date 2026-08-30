use clap::ValueEnum;
use serde::{Deserialize, Serialize};

mod cargo_basic;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug, ValueEnum, Eq)]
pub enum Script {
    #[default]
    CargoBasic,
}

impl Script {
    pub fn content(&self) -> &str {
        match self {
            Self::CargoBasic => cargo_basic::CONTENT,
        }
    }
}
