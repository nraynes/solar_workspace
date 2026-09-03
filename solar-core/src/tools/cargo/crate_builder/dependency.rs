use std::path::Path;

use derive_new::new;
use rust_terminal::Terminal;

use crate::solar_error::SolarError;

#[derive(new)]
pub struct Dependency {
    name: String,
    features: Vec<String>,
}

impl<const N: usize> From<(&str, [&str; N])> for Dependency {
    fn from(value: (&str, [&str; N])) -> Self {
        Self::new(value.0.into(), Vec::from(value.1.map(|s| s.to_string())))
    }
}

impl Dependency {
    pub fn add_to(&self, path: &Path) -> Result<(), SolarError> {
        let mut args = Vec::from(["add", &self.name]);
        if !self.features.is_empty() {
            args.push("--features");
            args.extend::<Vec<&str>>(self.features.iter().map(|s| s.as_str()).collect());
        }
        Terminal::command()
            .piped()
            .current_dir(path)
            .run("cargo", args)?;
        Ok(())
    }
}
