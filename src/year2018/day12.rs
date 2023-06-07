use crate::*;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Result<(Vec<u8>, HashSet<Vec<u8>>)> {
    let mut input = input.lines();
    let initial = input
        .next()
        .ok_or(aoc_error!("No initial state!"))?
        .bytes()
        .collect();
    let rules = input
        .skip(1)
        .filter_map(|line| {
            let (from, to) =
                scan_fmt::scan_fmt!(line, "{} => {}", String, String)?;
            if to == "#" {
                Some(from.as_ref().to_vec())
            } else {
                None
            }
        })
        .collect();
    if rules.iter().any(|rule| rule.len() != 5) {
        Err(aoc_error!("All rules must be of length 5!"))
    } else if rules.iter().any(|rule| rule.as_slice() == [b'.'; 5]) {
        Err(aoc_error!("Empty rule leads to chaos!"))
    } else {
        Ok((initial, rules))
    }
}

pub fn task1((initial, rules): &(Vec<u8>, HashSet<Vec<u8>>)) -> Result<usize> {
    let mut state: Vec<u8> = initial.iter().copied().collect();
    let mut starting_idx = 0;
    for _ in 0..20 {
        let mut new_state = Vec::new();
        for i in -1..state.len() + 2 {}
    }

    Err(aoc_error!("Todo"))
}

pub fn task2(input: &str) -> Result<usize> {
    Err(aoc_error!("Todo"))
}
