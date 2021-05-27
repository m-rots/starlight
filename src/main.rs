use clap::{AppSettings, Clap};
use cover::Cover;
use implication::Implication;
use minimal_keys::MinimalKeys;

mod cover;
mod fd;
mod implication;
mod minimal_keys;

#[derive(Clap)]
#[clap(setting = AppSettings::VersionlessSubcommands)]
enum SubCommand {
    /// Calculate the cover of a set of attributes.
    Cover(Cover),

    /// Check whether a Functional Dependency (FD) is implied by a set of FDs.
    Implication(Implication),

    /// Calculate all the minimal keys for a given set of functional dependencies.
    MinimalKeys(MinimalKeys),
}

#[derive(Clap)]
struct Opt {
    #[clap(subcommand)]
    cmd: SubCommand,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    match opt.cmd {
        SubCommand::Cover(cmd) => cmd.run(),
        SubCommand::Implication(cmd) => cmd.run(),
        SubCommand::MinimalKeys(cmd) => cmd.run(),
    }
}
