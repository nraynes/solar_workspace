pub mod deinit;
pub mod init;
pub mod install;
pub mod new;
pub mod uninstall;
pub mod update;
pub mod update_all;
pub mod upgrade;

pub use deinit::Deinit;
use enum_dispatch::enum_dispatch;
pub use init::Init;
pub use install::Install;
pub use new::New;
pub use uninstall::Uninstall;
pub use update::Update;
pub use update_all::UpdateAll;
pub use upgrade::Upgrade;

use clap::Subcommand as SC;

#[enum_dispatch(Run)]
#[derive(SC, Clone)]
pub enum Subcommand {
    /// Create a new Solar project in a new directory with a configuration.
    New(New),

    /// Initialize a new Solar project with a configuration in the current directory.
    Init(Init),

    /// Deinitializes a solar project.
    Deinit(Deinit),

    /// Updates the configuration of tools on the current project.
    Update(Update),

    /// Updates every tool that has an upgrader found in the project.
    UpdateAll(UpdateAll),

    /// Upgrade a single tool in the solar project.
    Upgrade(Upgrade),

    /// Install tools from the Solar framework to the project.
    Install(Install),

    /// Removes Solar framework tools from the project.
    Uninstall(Uninstall),
}
