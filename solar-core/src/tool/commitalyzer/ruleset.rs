use std::{fmt::Display, str::FromStr};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::SolarError;

#[derive(ValueEnum, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Copy)]
pub enum Ruleset {
    ConventionalCommits,
}

impl Display for Ruleset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Self::ConventionalCommits => "conventional-commits",
            }
        )
    }
}

impl FromStr for Ruleset {
    type Err = SolarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "conventional-commits" => Ok(Self::ConventionalCommits),
            _ => Err(SolarError::from(format!("{} is not a valid ruleset.", s))),
        }
    }
}
