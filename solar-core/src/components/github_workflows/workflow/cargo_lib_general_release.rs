use clap::Parser;
use derive_new::new;
use yaml_serde::Value;

use crate::components::github_workflows::yaml::Yaml;

#[derive(Parser, Debug, Clone, PartialEq, Default, new)]
pub struct CargoLibGeneralRelease {
    #[arg(short, long, default_value = "CI/CD Release")]
    name: String,

    #[arg(short, long, default_value = "master")]
    default_branch: String,
}

impl CargoLibGeneralRelease {
    pub fn build_yaml(&self) -> Value {
        Yaml::root(|y| {
            [
            (y.string("name"), y.string(&self.name)),
            (y.string("permissions"), y.mapping(|y| [
                (y.string("contents"), y.string("write")),
                (y.string("id-token"), y.string("write")),
            ])),
            (y.string("on"), y.mapping(|y| [
                (y.string("push"), y.mapping(|y| [
                    (y.string("branches"), y.sequence(|y| [
                        (y.string(&self.default_branch))
                    ])),
                ])),
                (y.string("workflow_dispatch"), y.empty_mapping()),
            ])),
            (y.string("jobs"), y.mapping(|y| [
                (y.string("release"), y.mapping(|y| [
                    (y.string("runs-on"), y.string("macos-latest")),
                    (y.string("steps"), y.sequence(|y| [
                        y.mapping(|y| [
                            (y.string("uses"), y.string("actions/checkout@v6")),
                            (y.string("with"), y.mapping(|y| [
                                (y.string("fetch-depth"), y.number(0)),
                                (y.string("fetch-tags"), y.bool(true)),
                            ])),
                        ]),
                        y.mapping(|y| [
                            (y.string("name"), y.string("Update Release")),
                            (y.string("env"), y.mapping(|y| [
                                (y.string("CARGO_REGISTRY_TOKEN"), y.string("${{ secrets.CARGO_REGISTRY_TOKEN }}")),
                            ])),
                            (y.string("run"), y.string("./.release/semver-release")),
                        ]),
                    ])),
                ])),
            ])),
        ]
        })
    }
}
