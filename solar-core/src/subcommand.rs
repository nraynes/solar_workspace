pub mod init;
pub mod install;
pub mod new;
pub mod remove;
pub mod upgrade;

pub use init::Init;
pub use install::Install;
pub use new::New;
pub use remove::Remove;
pub use upgrade::Upgrade;

use clap::Subcommand as SC;

#[derive(SC, Clone)]
pub enum Subcommand {
    /// Create a new Solar project in a new directory.
    NEW(New),

    /// Initialize a new Solar project.
    INIT(Init),

    /// Upgrade the versions of the tools in the Solar project.
    UPGRADE(Upgrade),

    /// Install tools from the Solar framework to the project.
    INSTALL(Install),

    /// Removes Solar framework tools from the project.
    REMOVE(Remove),
}
