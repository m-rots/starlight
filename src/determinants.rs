use crate::cover::cover;
use crate::fd::{parse_input, FunctionalDependency};
use crate::minimal_keys::{all_attributes, right_hand_side};
use crate::split_commas;
use clap::Clap;
use itertools::join;
use std::collections::HashSet;
use std::path::PathBuf;
use tracing::{debug, info, trace};

#[derive(Clap)]
pub struct Determinants {
    /// A comma separated set of attributes.
    ///
    /// To calculate the determinants of the set attributes {A, B},
    /// you would provide this as A,B.
    #[clap(parse(from_str = split_commas))]
    attributes: HashSet<String>,

    /// Path to the file containing the functional dependencies.
    #[clap(short, long)]
    file: PathBuf,
}

impl Determinants {
    pub fn run(self) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(&self.file)?;
        let deps = parse_input(&input)?;

        let determinants = determinants(self.attributes, deps);
        for determinant in determinants {
            println!("{{{}}}", join(determinant, ","));
        }

        Ok(())
    }
}

fn determinants(
    attributes: HashSet<String>,
    deps: Vec<FunctionalDependency>,
) -> Vec<HashSet<String>> {
    let all_attributes = all_attributes(deps.clone());
    info!("All attributes: {:?}", all_attributes);

    let right_hand: HashSet<String> = right_hand_side(deps.clone());
    info!("Right hand: {:?}", right_hand);

    // All attributes that don't appear in the right hand side.
    let candidates: HashSet<String> = attributes
        .difference(&right_hand)
        .map(|s| s.to_owned())
        .collect();

    let mut candidates: Vec<HashSet<String>> = vec![candidates];
    let mut determinants: Vec<HashSet<String>> = vec![];

    loop {
        // Remove all candidates that contain a determinant.
        candidates.retain(|candidate| {
            for determinant in &determinants {
                if determinant.is_subset(candidate) {
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

            // The candidate a superset would be cheating!
            if cover.is_superset(&attributes) && !candidate.is_superset(&attributes) {
                info!("Adding: {:?} to determinants", candidate);
                determinants.push(candidate);
            } else {
                // Attributes not in cover
                for attribute in all_attributes.difference(&cover) {
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

    // dedup does not always work :(
    determinants.dedup();
    determinants
}
