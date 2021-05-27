use anyhow::anyhow;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct FunctionalDependency {
    pub left: HashSet<String>,
    pub right: HashSet<String>,
}

impl FromStr for FunctionalDependency {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line
            .split_once("->")
            .ok_or_else(|| anyhow!("String {:?} lacks {:?}", line, "->"))?;

        Ok(Self {
            left: left.split(',').map(|s| s.trim().to_string()).collect(),
            right: right.split(',').map(|s| s.trim().to_string()).collect(),
        })
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<FunctionalDependency>> {
    input.lines().map(FunctionalDependency::from_str).collect()
}
