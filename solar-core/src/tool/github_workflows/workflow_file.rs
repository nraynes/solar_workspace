use std::fmt::Display;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum WorkflowFile {
    Release,
    Test,
}

impl Display for WorkflowFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Release => "release",
                Self::Test => "test",
            }
        )
    }
}

impl WorkflowFile {
    pub fn name(&self) -> String {
        format!("{}.yml", self)
    }
}
