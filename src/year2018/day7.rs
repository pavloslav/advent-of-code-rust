use crate::*;
use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Result<Vec<(char, char)>> {
    input
        .lines()
        .map(|line| {
            Ok(scan_fmt::scan_fmt!(
                line,
                "Step {} must be finished before step {} can begin.",
                char,
                char
            )?)
        })
        .collect()
}

pub fn task1(input: &[(char, char)]) -> Result<String> {
    let mut map: HashMap<char, HashSet<char>> = HashMap::new();
    for &(needed, target) in input {
        map.entry(target).or_insert(HashSet::new()).insert(needed);
        map.entry(needed).or_insert(HashSet::new());
    }
    let mut steps = String::new();
    while !map.is_empty() {
        let next_step = map
            .iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(&k, _)| k)
            .min()
            .ok_or(aoc_error!("No suitable action found!"))?;
        steps.push(next_step);
        for v in map.values_mut() {
            v.remove(&next_step);
        }
        map.remove(&next_step);
    }
    Ok(steps)
}

pub fn task2(_input: &[(char, char)]) -> Result<usize> {
    Err(aoc_error!("Todo"))
}
