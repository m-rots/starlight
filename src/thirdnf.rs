use crate::cononical::singular_right_hand;
use crate::fd::{parse_input, FunctionalDependency};
use crate::minimal_keys::minimal_keys;
use anyhow::{bail, Context};
use clap::Clap;
use std::path::PathBuf;
use tracing::info;

#[derive(Clap)]
pub struct ThirdNF {
    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl ThirdNF {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        is_3nf(deps).context("The given FDs are NOT 3NF!")?;
        println!("The given FDs are 3NF!");
        Ok(())
    }
}

/// The left-hand side contains a key
/// or the right-hand side is an attribute of a minimal key.
fn is_3nf(deps: Vec<FunctionalDependency>) -> anyhow::Result<()> {
    let minimal_keys = minimal_keys(deps.clone());
    let deps = singular_right_hand(deps);
    info!("Keys: {:?}", minimal_keys);

    for dep in &deps {
        match minimal_keys.iter().find(|key| key.is_subset(&dep.left)) {
            Some(key) => info!("FD {:?} contains key: {:?}", dep.left, key),
            None => match minimal_keys.iter().find(|key| dep.right.is_subset(key)) {
                Some(key) => info!("FD {:?} is an attribute of {:?}", dep.right, key),
                None => bail!(
                    "{:?} does not contain any key and {:?} is not an attribute of any key",
                    dep.left,
                    dep.right
                ),
            },
        }
    }

    Ok(())
}
