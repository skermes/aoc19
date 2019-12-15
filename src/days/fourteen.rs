use regex::{Regex, Error as RegexError};
use thiserror::Error;
use itertools::Itertools;
use std::collections::HashMap;

use crate::problem::Problem;

#[derive(Debug, Error)]
enum ReactionError {
    #[error("Error parsing reaction line: {0}")]
    ParseError(#[from] RegexError)
}

#[derive(Debug, Clone)]
struct Reactant {
    chemical: String,
    amount: usize
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Reactant>,
    output: Reactant
}

fn read_line(line: &str) -> Result<Reaction, ReactionError> {
    let pattern = Regex::new(r"(?P<amount>\d+) (?P<chemical>[A-Z]+)")?;
    let reactants = pattern.captures_iter(line)
        .map(|cap| Reactant {
            chemical: cap[2].to_string(),
            amount: usize::from_str_radix(&cap[1], 10).unwrap()
        })
        .collect_vec();

    // Not thrilled about cloning the vector here but I'll deal with it later
    Ok(Reaction {
        inputs: reactants.iter().cloned().take(reactants.len() - 1).collect(),
        output: reactants[reactants.len() - 1].clone()
    })
}

fn reaction_map(reactions: &[Reaction]) -> HashMap<&String, &Reaction> {
    reactions.iter()
        .map(|r| (&r.output.chemical, r))
        .collect()
}

pub struct DayFourteen {}

impl Problem for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let reactions = input.split("\n")
            .map(|line| read_line(line.trim()).unwrap())
            .collect_vec();
        let reactions = reaction_map(&reactions);

        println!("{:#?}", reactions);

        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
