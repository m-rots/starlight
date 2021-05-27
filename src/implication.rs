use crate::cover::cover;
use crate::fd::{parse_input, FunctionalDependency};
use anyhow::anyhow;
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct Implication {
    /// The functional dependency to check the implication for.
    ///
    /// Should be given as "B -> A, G".
    #[clap(parse(try_from_str))]
    dependency: FunctionalDependency,

    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,

    /// Print steps?
    #[clap(short, long)]
    verbose: bool,
}

impl Implication {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        match implication(self.dependency, deps, self.verbose) {
            true => {
                println!("Yay! This functional dependency is implied by its cover :D");
                Ok(())
            }
            false => Err(anyhow!("Nooooo, it's NOT implied :(")),
        }
    }
}

fn implication(fd: FunctionalDependency, deps: Vec<FunctionalDependency>, verbose: bool) -> bool {
    let cover = cover(fd.left, deps, verbose);
    if verbose {
        println!("Cover: {:?}", cover);
    }

    fd.right.is_subset(&cover)
}
