use crate::tool::github_workflows::{
    parameters::Parameters, workflow::WorkflowDetails, workflow_file::WorkflowFile,
};

pub struct ReleaseCargoLibGeneral {}

impl WorkflowDetails for ReleaseCargoLibGeneral {
    fn new() -> Self {
        Self {}
    }

    fn file(&self) -> WorkflowFile {
        WorkflowFile::Release
    }

    fn get(&self, _: &Parameters) -> String {
        String::from(
            "name: CI/CD Release

permissions:
  contents: write
  id-token: write

on:
  push:
    branches: [ \"main\" ]
  workflow_dispatch:

jobs:
  release:
    runs-on: macos-latest

    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
          fetch-tags: true

      - name: Update Release
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: ./.release/semver-release
",
        )
    }
}
