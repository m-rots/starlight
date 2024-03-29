use crate::cover::cover;
use crate::fd::{parse_input, FunctionalDependency};
use clap::Clap;
use itertools::join;
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::{debug, info, trace};

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

        let minimal_keys = minimal_keys(deps);
        for key in minimal_keys {
            println!("{{{}}}", join(key, ","));
        }

        Ok(())
    }
}

pub fn all_attributes(deps: Vec<FunctionalDependency>) -> HashSet<String> {
    deps.into_iter()
        .flat_map(|mut fd| {
            fd.left.extend(fd.right);
            fd.left
        })
        .collect()
}

pub fn right_hand_side(deps: Vec<FunctionalDependency>) -> HashSet<String> {
    deps.into_iter().flat_map(|fd| fd.right).collect()
}
pub fn minimal_keys(deps: Vec<FunctionalDependency>) -> Vec<HashSet<String>> {
    let attributes = all_attributes(deps.clone());
    info!("All attributes: {:?}", attributes);

    let right_hand: HashSet<String> = right_hand_side(deps.clone());
    info!("Right hand: {:?}", right_hand);

    // All attributes that don't appear in the right hand side.
    let candidates: HashSet<String> = attributes
        .difference(&right_hand)
        .map(|s| s.to_owned())
        .collect();

    let mut candidates: Vec<HashSet<String>> = vec![candidates];
    let mut minimal_keys: Vec<HashSet<String>> = vec![];

    loop {
        // Remove from candidates all candidates that contain a minimal key.
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
        info!("Candidates: {:?}", candidates);

        let mut new_candidates: Vec<HashSet<String>> = vec![];

        for candidate in candidates {
            let cover = cover(candidate.clone(), deps.clone());
            debug!("Cover of {:?} is: {:?}", candidate, cover);

            if cover == attributes {
                info!("Adding: {:?} to minimal keys", candidate);
                minimal_keys.push(candidate);
            } else {
                // Attributes not in cover
                for attribute in attributes.difference(&cover) {
                    let mut new_candidate = candidate.clone();
                    new_candidate.insert(attribute.to_owned());
                    trace!("Adding: {:?} to candidates", candidate);

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
    minimal_keys
}
