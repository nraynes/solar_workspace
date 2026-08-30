use clap::Parser;
use derive_new::new;
use yaml_serde::Value;

use crate::components::github_workflows::yaml::Yaml;

#[derive(Parser, Debug, Clone, PartialEq, Default, new)]
pub struct CargoBinGeneralRelease {
    #[arg(short, long, default_value = "CI/CD Release")]
    name: String,

    #[arg(short, long, default_value = "master")]
    default_branch: String,
}

impl CargoBinGeneralRelease {
    pub fn build_yaml(&self, project_name: &str) -> Value {
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
                    (y.string("env"), y.mapping(|y| [
                        (y.string("BIN_NAME"), y.string(project_name))
                    ])),
                    (y.string("steps"), y.sequence(|y| [
                        y.mapping(|y| [
                            (y.string("uses"), y.string("actions/checkout@v6")),
                            (y.string("with"), y.mapping(|y| [
                                (y.string("fetch-depth"), y.number(0)),
                                (y.string("fetch-tags"), y.bool(true)),
                            ])),
                        ]),
                        y.mapping(|y| [
                            (y.string("name"), y.string("Build Linux Release")),
                            (y.string("run"), y.string("rustup target add x86_64-unknown-linux-gnu\nbrew install SergioBenitez/osxct/x86_64-unknown-linux-gnu\ncargo build --release --target x86_64-unknown-linux-gnu --config 'target.x86_64-unknown-linux-gnu.linker = \"x86_64-unknown-linux-gnu-gcc\"'\nmkdir -p ./bin/linux\ncp ./target/x86_64-unknown-linux-gnu/release/$BIN_NAME ./bin/linux/$BIN_NAME")),
                        ]),
                        y.mapping(|y| [
                            (y.string("name"), y.string("Build Arm MacOS Release")),
                            (y.string("run"), y.string("rustup target add aarch64-apple-darwin\ncargo build --release --target aarch64-apple-darwin\nmkdir -p ./bin/arm-macos\ncp ./target/aarch64-apple-darwin/release/$BIN_NAME ./bin/arm-macos/$BIN_NAME")),
                        ]),
                        y.mapping(|y| [
                            (y.string("name"), y.string("Build Intel MacOS Release")),
                            (y.string("run"), y.string("rustup target add x86_64-apple-darwin\ncargo build --release --target x86_64-apple-darwin\nmkdir -p ./bin/intel-macos\ncp ./target/x86_64-apple-darwin/release/$BIN_NAME ./bin/intel-macos/$BIN_NAME")),
                        ]),
                        y.mapping(|y| [
                            (y.string("name"), y.string("Build Windows Release")),
                            (y.string("run"), y.string("rustup target add x86_64-pc-windows-gnu\nbrew install min gw-w64\ncargo build --release --target x86_64-pc-windows-gnu --config 'target.x86_64-pc-windows-gnu.linker = \"x86_64-w64-mingw32-gcc\"'\nmkdir -p ./bin/windows\ncp ./target/x86_64-pc-windows-gnu/release/$BIN_NAME.exe ./bin/windows/$BIN_NAME.exe")),
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
