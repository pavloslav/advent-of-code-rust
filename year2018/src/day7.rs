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
        map.entry(target).or_default().insert(needed);
        map.entry(needed).or_default();
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

pub fn task2(input: &[(char, char)]) -> Result<usize> {
    let mut steps: HashSet<char> = input
        .iter()
        .flat_map(|(needed, target)| [needed, target])
        .copied()
        .collect();
    let mut map: HashMap<char, Vec<char>> = steps.iter().map(|&step| (step, vec![])).collect();
    for &(needed, target) in input {
        if let Some(targets) = map.get_mut(&needed) {
            targets.push(target);
        }
    }

    let mut times: HashMap<char, usize> = HashMap::new();
    while !steps.is_empty() {
        let mut to_remove = vec![];
        for &step in &steps {
            if let Some(min) = map[&step]
                .iter()
                .map(|target| times.get(target))
                .min()
                .map_or(Some(0), |rest| {
                    rest.map(|t| t + 61 + step as usize - 'A' as usize)
                })
            {
                times.insert(step, min);
                to_remove.push(step);
            }
        }
        for r in to_remove {
            steps.remove(&r);
        }
    }

    todo!("return the answer")
}
