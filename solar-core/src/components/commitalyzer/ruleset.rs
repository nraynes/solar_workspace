use std::str::FromStr;

use clap::ValueEnum;

use crate::solar_error::SolarError;

#[derive(ValueEnum, Clone, Eq, PartialEq, Debug, Copy, Default)]
pub enum Ruleset {
    #[default]
    ConventionalCommits,
}

impl Ruleset {
    pub fn get(&self) -> &str {
        match self {
            Self::ConventionalCommits => "conventional-commits",
        }
    }

    pub fn file_name(&self) -> String {
        format!("{}.yml", self.get())
    }
}

impl FromStr for Ruleset {
    type Err = SolarError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conventional-commits" => Ok(Self::ConventionalCommits),
            _ => Err(SolarError::from(format!(
                "{} is not a valid commitalyzer ruleset.",
                s
            ))),
        }
    }
}
