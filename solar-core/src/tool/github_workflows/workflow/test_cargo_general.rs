use crate::tool::github_workflows::{
    parameters::Parameters, workflow::WorkflowDetails, workflow_file::WorkflowFile,
};

pub struct TestCargoGeneral {}

impl WorkflowDetails for TestCargoGeneral {
    fn new() -> Self {
        Self {}
    }

    fn file(&self) -> WorkflowFile {
        WorkflowFile::Test
    }

    fn get(&self, _: &Parameters) -> String {
        String::from(
            "name: CI/CD Test

on:
  pull_request:
    branches: [ \"main\" ]
  workflow_dispatch:

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4
      
      - name: Install Tools
        run: cargo install cargo-audit

      - name: Install Dependencies
        run: cargo build

      - name: Run Static Analysis
        run: cargo check

      - name: Run Linter
        run: cargo clippy -- -D warnings

      - name: Run Dependency Audit
        run: cargo audit

      - name: Run Tests
        run: cargo test
",
        )
    }
}
