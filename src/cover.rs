use crate::fd::{parse_input, FunctionalDependency};
use crate::split_commas;
use clap::Clap;
use itertools::join;
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::debug;

#[derive(Clap)]
pub struct Cover {
    /// A comma separated set of attributes.
    ///
    /// To calculate the cover of the set attributes {A, B},
    /// you would provide this as A,B.
    #[clap(parse(from_str = split_commas))]
    attributes: HashSet<String>,

    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl Cover {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        let cover = cover(self.attributes, deps);
        println!("{}", join(cover, ","));

        Ok(())
    }
}

pub fn cover(mut cover: HashSet<String>, mut deps: Vec<FunctionalDependency>) -> HashSet<String> {
    loop {
        let mut visited = false;

        deps.retain(|dep| {
            if dep.left.is_subset(&cover) {
                debug!(
                    "Added {:?} to cover as {:?} is a subset of {:?}",
                    dep.right, dep.left, cover
                );

                visited = true;
                cover.extend(dep.right.clone());
                false
            } else {
                true
            }
        });

        if !visited {
            break;
        }
    }

    cover
}
