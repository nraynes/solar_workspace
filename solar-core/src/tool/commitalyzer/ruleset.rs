use std::{fmt::Display, str::FromStr};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::SolarError;

#[derive(ValueEnum, Clone, Serialize, Eq, PartialEq, Debug, Copy)]
pub enum Ruleset {
    ConventionalCommits,
}

impl<'de> Deserialize<'de> for Ruleset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
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
