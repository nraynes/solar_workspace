pub mod deinit;
pub mod init;
pub mod install;
pub mod new;
pub mod uninstall;
pub mod update;
pub mod upgrade;

pub use deinit::Deinit;
pub use init::Init;
pub use install::Install;
pub use new::New;
pub use uninstall::Uninstall;
pub use update::Update;
pub use upgrade::Upgrade;

use clap::Subcommand as SC;

#[derive(SC, Clone)]
pub enum Subcommand {
    /// Create a new Solar project in a new directory with a configuration.
    NEW(New),

    /// Initialize a new Solar project with a configuration in the current directory.
    INIT(Init),

    /// Updates the configuration of tools on the current project.
    UPDATE(Update),

    /// Upgrade a single tool in the solar project.
    UPGRADE(Upgrade),

    /// Install tools from the Solar framework to the project.
    INSTALL(Install),

    /// Removes Solar framework tools from the project.
    UNINSTALL(Uninstall),

    /// Deinitializes a solar project.
    DEINIT(Deinit),
}
