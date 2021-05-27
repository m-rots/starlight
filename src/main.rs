use clap::{AppSettings, Clap};
use cover::Cover;

mod cover;

#[derive(Clap)]
#[clap(setting = AppSettings::VersionlessSubcommands)]
enum SubCommand {
    /// Calculate the cover of a set of attributes.
    Cover(Cover),
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
    }
}
