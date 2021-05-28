use anyhow::anyhow;
use itertools::join;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
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

impl fmt::Display for FunctionalDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", join(&self.left, ","), join(&self.right, ","))
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<FunctionalDependency>> {
    input.lines().map(FunctionalDependency::from_str).collect()
}
