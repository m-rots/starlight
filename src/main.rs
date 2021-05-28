use bcnf::Bcnf;
use clap::{AppSettings, Clap};
use cover::Cover;
use determinants::Determinants;
use implication::Implication;
use minimal_keys::MinimalKeys;
use std::collections::HashSet;
use thirdnf::ThirdNF;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

mod bcnf;
mod cover;
mod determinants;
mod fd;
mod implication;
mod minimal_keys;
mod thirdnf;

#[derive(Clap)]
#[clap(setting = AppSettings::VersionlessSubcommands)]
enum SubCommand {
    /// Check whether a given set of functional dependencies is in 3NF.
    #[clap(name = "3nf")]
    ThirdNF(ThirdNF),

    /// Check whether a given set of functional dependencies is in BCNF.
    Bcnf(Bcnf),

    /// Calculate the cover of a set of attributes.
    Cover(Cover),

    /// Calculate all the determinants for a given set of functional dependencies.
    Determinants(Determinants),

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

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();

    match opt.cmd {
        SubCommand::Bcnf(cmd) => cmd.run(),
        SubCommand::Cover(cmd) => cmd.run(),
        SubCommand::Determinants(cmd) => cmd.run(),
        SubCommand::Implication(cmd) => cmd.run(),
        SubCommand::MinimalKeys(cmd) => cmd.run(),
        SubCommand::ThirdNF(cmd) => cmd.run(),
    }
}

pub fn split_commas(s: &str) -> HashSet<String> {
    s.split(',').map(|s| s.trim().to_string()).collect()
}
