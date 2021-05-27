use clap::Clap;
use itertools::join;
use std::collections::HashSet;
use std::path::PathBuf;

fn split_commas(s: &str) -> HashSet<String> {
    s.split(',').map(|s| s.trim().to_string()).collect()
}

#[derive(Clap)]
pub struct Cover {
    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,

    /// A comma separated set of attributes.
    ///
    /// To calculate the cover of the set attributes {A, B},
    /// you would provide this as A,B.
    #[clap(parse(from_str = split_commas))]
    attributes: HashSet<String>,
}

impl Cover {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input);

        cover(self.attributes, deps);
        Ok(())
    }
}

#[derive(Debug)]
struct FunctionalDependency {
    left: HashSet<String>,
    right: HashSet<String>,
}

impl FunctionalDependency {
    fn new<L, R, T>(left: L, right: R) -> Self
    where
        L: IntoIterator<Item = T>,
        R: IntoIterator<Item = T>,
        T: Into<String>,
    {
        Self {
            left: left.into_iter().map(|s| s.into()).collect(),
            right: right.into_iter().map(|s| s.into()).collect(),
        }
    }
}

fn cover(mut cover: HashSet<String>, mut deps: Vec<FunctionalDependency>) {
    loop {
        let mut visited = false;

        deps.retain(|dep| {
            if dep.left.is_subset(&cover) {
                println!(
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

    // Nicely formatted
    println!("\nCover: {}", join(cover, ","));
}

fn parse_input(input: &str) -> Vec<FunctionalDependency> {
    input
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once("->")?;
            let left: Vec<_> = left.split(',').map(|s| s.trim()).collect();
            let right: Vec<_> = right.split(',').map(|s| s.trim()).collect();
            let fd = FunctionalDependency::new(left, right);

            Some(fd)
        })
        .collect()
}
