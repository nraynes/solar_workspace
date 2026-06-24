mod config;
mod global;
mod solar_error;
mod subcommand;
pub mod tool;

pub use config::Config;
pub use global::{Global, SOLARCONFIGNAME};
pub use solar_error::SolarError;
pub use subcommand::{
    Subcommand,
    init::{Init, solar_init},
    install::Install,
    new::{New, solar_new},
    update::{Update, solar_update},
    upgrade::Upgrade,
};
pub use tool::{
    Action, CargoDeny, Commitalyzer, GithubWorkflows, LICENSES_DIR, Licenses, PreCommit,
    SemverRelease, Tool, ToolTrait, Vhooks,
};

/// Method that provides a sorted collection. Takes ownership of the collection, mutates it in place,
/// then returns it as an owned value. Useful if you just want a one-liner to get a sorted collection rather
/// than a separate line to sort the collection after its instantiated, especially if you don't want the
/// collection to be mutable after sorting.
pub fn sorted<C, U>(mut collection: C) -> C
where
    C: AsMut<[U]>,
    U: Ord,
{
    collection.as_mut().sort();
    collection
}
