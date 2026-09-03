use clap::Parser;
use derive_new::new;
use yaml_serde::Value;

use crate::components::github_workflows::yaml::Yaml;

#[derive(Parser, Debug, Clone, PartialEq, Default, new)]
pub struct CargoAnyGeneralTest {
    #[arg(short, long, default_value = "CI/CD Test")]
    name: String,

    #[arg(short, long, default_value = "master")]
    default_branch: String,
}

impl CargoAnyGeneralTest {
    pub fn build_yaml(&self) -> Value {
        Yaml::root(|y| {
            [
                (y.string("name"), y.string(&self.name)),
                (
                    y.string("permissions"),
                    y.mapping(|y| {
                        [
                            (y.string("contents"), y.string("write")),
                            (y.string("id-token"), y.string("write")),
                        ]
                    }),
                ),
                (
                    y.string("on"),
                    y.mapping(|y| {
                        [
                            (
                                y.string("push"),
                                y.mapping(|y| {
                                    [(
                                        y.string("branches"),
                                        y.sequence(|y| [(y.string(&self.default_branch))]),
                                    )]
                                }),
                            ),
                            (y.string("workflow_dispatch"), y.empty_mapping()),
                        ]
                    }),
                ),
                (
                    y.string("jobs"),
                    y.mapping(|y| {
                        [(
                            y.string("release"),
                            y.mapping(|y| {
                                [
                                    (y.string("runs-on"), y.string("ubuntu-latest")),
                                    (
                                        y.string("steps"),
                                        y.sequence(|y| {
                                            [
                                                y.mapping(|y| {
                                                    [(
                                                        y.string("uses"),
                                                        y.string("actions/checkout@v6"),
                                                    )]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (
                                                            y.string("name"),
                                                            y.string("Install Tools"),
                                                        ),
                                                        (
                                                            y.string("run"),
                                                            y.string("cargo install cargo-audit"),
                                                        ),
                                                    ]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (y.string("name"), y.string("Build")),
                                                        (y.string("run"), y.string("cargo build")),
                                                    ]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (
                                                            y.string("name"),
                                                            y.string("Run Static Analysis"),
                                                        ),
                                                        (y.string("run"), y.string("cargo check")),
                                                    ]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (y.string("name"), y.string("Run Linter")),
                                                        (
                                                            y.string("run"),
                                                            y.string("cargo clippy -- -D warnings"),
                                                        ),
                                                    ]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (
                                                            y.string("name"),
                                                            y.string("Run Dependency Audit"),
                                                        ),
                                                        (y.string("run"), y.string("cargo audit")),
                                                    ]
                                                }),
                                                y.mapping(|y| {
                                                    [
                                                        (y.string("name"), y.string("Run Tests")),
                                                        (y.string("run"), y.string("cargo test")),
                                                    ]
                                                }),
                                            ]
                                        }),
                                    ),
                                ]
                            }),
                        )]
                    }),
                ),
            ]
        })
    }
}
