use crate::tool::github_workflows::{WorkflowTrait, workflow_trait::HasConstructor};

pub struct LibRelease {}

impl HasConstructor for LibRelease {
    fn new() -> Self {
        Self {}
    }
}

impl WorkflowTrait for LibRelease {
    fn get(&self) -> std::string::String {
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
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: ./.release/semver-release
",
        )
    }
}
