use crate::cover::cover;
use crate::fd::{parse_input, FunctionalDependency};
use clap::Clap;
use colored::Colorize;
use itertools::join;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Clap)]
pub struct MinimalKeys {
    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl MinimalKeys {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        minimal_keys(deps);
        Ok(())
    }
}

fn all_attributes(deps: Vec<FunctionalDependency>) -> HashSet<String> {
    deps.into_iter()
        .flat_map(|mut fd| {
            fd.left.extend(fd.right);
            fd.left
        })
        .collect()
}

fn minimal_keys(deps: Vec<FunctionalDependency>) {
    let attributes = all_attributes(deps.clone());

    println!("Attributes: {:?}", attributes);

    let right_hand: HashSet<String> = deps.clone().into_iter().flat_map(|fd| fd.right).collect();
    println!("Right hand: {:?}", right_hand);

    // All attributes that don't appear in the right hand side.
    let candidates: HashSet<String> = attributes
        .difference(&right_hand)
        .map(|s| s.to_owned())
        .collect();

    let mut candidates: Vec<HashSet<String>> = vec![candidates];
    let mut minimal_keys: Vec<HashSet<String>> = vec![];

    loop {
        // Remove from candidates all candidates that contain a key in results.
        candidates.retain(|candidate| {
            for minimal_key in &minimal_keys {
                // if a key is a subset of the candidate,
                // then a more minimal version of the candidate already exists.
                if minimal_key.is_subset(candidate) {
                    return false;
                }
            }

            true
        });

        // Smallest first
        candidates.sort_by_key(|k| k.len());
        println!("\n{}", format!("Candidates: {:?}", candidates).bold());

        let mut new_candidates: Vec<HashSet<String>> = vec![];

        for candidate in candidates {
            let cover = cover(candidate.clone(), deps.clone(), false);
            println!("\nCover of {:?} is: {:?}", candidate, cover);

            if cover == attributes {
                println!(
                    "{}",
                    format!("| Adding: {:?} to minimal keys", candidate).green()
                );
                minimal_keys.push(candidate);
            } else {
                // Attributes not in cover
                for attribute in attributes.difference(&cover) {
                    let mut new_candidate = candidate.clone();
                    new_candidate.insert(attribute.to_owned());
                    println!(
                        "{}",
                        format!("| Adding: {:?} to candidates", new_candidate).dimmed()
                    );

                    new_candidates.push(new_candidate);
                }
            }
        }

        candidates = new_candidates;

        if candidates.is_empty() {
            break;
        }
    }

    minimal_keys.dedup();

    println!("\nMinimal keys:");
    for key in minimal_keys {
        println!("{{{}}}", join(key, ","));
    }
}
