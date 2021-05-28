use crate::cover::cover;
use crate::fd::{parse_input, FunctionalDependency};
use clap::Clap;
use std::path::PathBuf;

#[derive(Clap)]
pub struct Cononical {
    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl Cononical {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        let deps = cononical(deps);
        for dep in &deps {
            println!("{}", dep);
        }

        Ok(())
    }
}

pub fn singular_right_hand(deps: Vec<FunctionalDependency>) -> Vec<FunctionalDependency> {
    let mut new_deps: Vec<FunctionalDependency> = vec![];

    for dep in deps {
        for right in dep.right {
            let dep = FunctionalDependency {
                left: dep.left.clone(),
                right: [right].iter().cloned().collect(),
            };

            new_deps.push(dep);
        }
    }

    new_deps
}

pub fn minimise_left_hand(deps: Vec<FunctionalDependency>) -> Vec<FunctionalDependency> {
    let mut new_deps: Vec<FunctionalDependency> = vec![];

    for dep in deps.clone() {
        if dep.left.len() == 1 {
            new_deps.push(dep);
            continue;
        }

        // alpha is left-hand
        let mut count = 0;

        for alpha in &dep.left {
            let mut left = dep.left.clone();
            left.remove(alpha);

            let cover = cover(left.clone(), deps.clone());

            if dep.right.is_subset(&cover) {
                count += 1;

                new_deps.push(FunctionalDependency {
                    left,
                    right: dep.right.clone(),
                });
            }
        }

        if count == 0 {
            new_deps.push(dep);
        }
    }

    new_deps
}

fn remove_implied(mut deps: Vec<FunctionalDependency>) -> Vec<FunctionalDependency> {
    loop {
        let new_deps = deps.clone().into_iter().find_map(|dep| {
            let deps: Vec<FunctionalDependency> =
                deps.clone().into_iter().filter(|d| d != &dep).collect();

            let cover = cover(dep.left.clone(), deps.clone());

            if dep.right.is_subset(&cover) {
                Some(deps)
            } else {
                None
            }
        });

        match new_deps {
            Some(new_deps) => deps = new_deps,
            None => return deps,
        }
    }
}

fn cononical(deps: Vec<FunctionalDependency>) -> Vec<FunctionalDependency> {
    let deps = singular_right_hand(deps);
    let deps = minimise_left_hand(deps);
    remove_implied(deps)
}
