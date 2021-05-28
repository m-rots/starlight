use crate::fd::{parse_input, FunctionalDependency};
use crate::minimal_keys::minimal_keys;
use anyhow::{bail, Context};
use clap::Clap;
use std::path::PathBuf;
use tracing::info;

#[derive(Clap)]
pub struct Bcnf {
    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl Bcnf {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        is_bcnf(deps).context("The given FDs are NOT BCNF!")?;
        println!("The given FDs are BCNF!");
        Ok(())
    }
}

/// The left-hand side contains a key.
fn is_bcnf(deps: Vec<FunctionalDependency>) -> anyhow::Result<()> {
    let minimal_keys = minimal_keys(deps.clone());
    info!("Keys: {:?}", minimal_keys);

    for dep in &deps {
        match minimal_keys.iter().find(|key| key.is_subset(&dep.left)) {
            Some(key) => info!("FD {:?} contains key: {:?}", dep.left, key),
            None => bail!("{:?} does not contain any key", dep.left),
        }
    }

    Ok(())
}
