# Version: v1.0.0

## BREAKING CHANGES

- refactor: changed the command structure
    
    BREAKING CHANGE: The new command structure separates tools and collections of tools as configurations.

## Features

- feat: added commitalyzer tool business logic
- feat: added configuration structure
- feat: added logic for multiple tools
    
    Added business logic to cargo_deny, github_workflows, licenses, pre_commit, and vhooks tools.
- feat: added logic for multiple tools
    
    Added business logic to cargo_deny, github_workflows, licenses, pre_commit, and vhooks tools.
- feat: added semver release tool business logic
- feat: added vhooks tool logic

## Patches

- fix: fix typo in release yml
- fix: fixed bugs with path resolving
- fix: fixed problem with github workflow types
- fix: fixed tests and licenses tool to pass
- fix: fixed upgrade of vhooks

## Maintenance Items

- chore: added all available licenses
- chore: added test for new config
- chore: change branch in release
- chore: changed matching targets to semver to be only arm macos
    
    Changed to be only arm macos because the runner used in both release workflows use macos.
- chore: checking hooks
- chore: currently working on refactor to allow for project pre configurations
- chore: fix cargo clippy errors
- chore: fix clippy mistakes
- chore: fix dependency vulnerabilities
- chore: fix random whitespace
- chore: fix target directory in release workflow
- chore: hopefully fixed release flow
- chore: refactored some things and added more tests
- chore: removed unused imports
- chore: updated semver release bins

