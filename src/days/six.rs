use thiserror::Error;
use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

#[derive(Debug, Error)]
pub enum OrbitError {
    #[error("Object {0} has two parents")]
    TwoParents(String)
}

fn orbit_line(line: &str) -> (&str, &str) {
    let tokens: Vec<&str> = line.split(")").collect();
    // Assume there are always two elements because I can look at the input.
    (tokens[0].trim(), tokens[1].trim())
}

fn orbit_graph(input: &str) -> Result<HashMap<&str, &str>, OrbitError> {
    let mut graph = HashMap::new();
    for line in input.split_whitespace() {
        let (orbitee, orbiter) = orbit_line(line);

        if graph.contains_key(orbiter) {
            return Err(OrbitError::TwoParents(orbiter.to_string()));
        }

        graph.insert(orbiter, orbitee);
    }
    Ok(graph)
}

const ROOT_OBJECT: &str = "COM";

fn count_orbits_memoized<'a>(graph: &std::collections::HashMap<&str, &'a str>,
                         orbiter: &'a str,
                         memo: &mut HashMap<&'a str, usize>) -> usize {
    if orbiter == ROOT_OBJECT {
        return 1;
    }

    match memo.get(orbiter) {
        Some(orbits) => *orbits,
        None => {
            // We build this graph, we know everything but root is in there.
            let parent = graph.get(orbiter).unwrap();
            let parent_orbit_count = count_orbits_memoized(
                graph,
                parent,
                memo
            );
            memo.insert(orbiter, parent_orbit_count + 1);
            parent_orbit_count + 1
        }
    }
}

fn count_total_orbits(graph: &HashMap<&str, &str>) -> usize {
    let mut memo: HashMap<&str, usize> = HashMap::new();
    let mut total_orbits = 0;
    for (_, orbiter) in graph {
        total_orbits += count_orbits_memoized(
            graph,
            orbiter,
            &mut memo
        );
    }
    total_orbits
}

fn orbital_parents<'a>(graph: &HashMap<&str, &'a str>,
                       orbiter: &str) -> HashSet<&'a str> {
    if orbiter == ROOT_OBJECT {
        HashSet::new()
    } else {
        // Everthing but the root has a parent.
        let parent = graph.get(orbiter).unwrap();
        let mut parents = orbital_parents(graph, parent);
        parents.insert(parent);
        parents
    }
}

pub struct DaySix {}

impl Problem for DaySix {
    fn part_one(&self, input: &str) -> String {
        let graph = orbit_graph(input).unwrap();
        let total_orbits = count_total_orbits(&graph);

        format!("{}", total_orbits)
    }

    fn part_two(&self, input: &str) -> String {
        let graph = orbit_graph(input).unwrap();

        let my_parent = graph.get("YOU").unwrap();
        let santa_parent = graph.get("SAN").unwrap();

        let my_parent_ancestors = orbital_parents(&graph, my_parent);
        let santas_parent_ancestors = orbital_parents(&graph,santa_parent);
        let shared_ancenstor_count = my_parent_ancestors
            .intersection(&santas_parent_ancestors)
            .collect::<Vec<&&str>>()
            .len();

        // You can get from one node to another by walking all the way back
        // to the root, then all the way back to the node you want (that's
        // my_ancestors + santa_ancestors), but the distance between the
        // lowest shared ancestor and the root is wasted twice there.  -1
        // because we don't count COM as an orbit but it's in all the ancestor
        // sets.
        let traversal_distance = my_parent_ancestors.len()
                               + santas_parent_ancestors.len()
                               - (shared_ancenstor_count - 1) * 2;

        format!("{}", traversal_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
