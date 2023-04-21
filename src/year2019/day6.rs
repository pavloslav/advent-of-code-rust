use super::super::common::Error::TaskError;
use super::super::common::Result;

pub fn parse_input(input: &str) -> Result<Vec<[String; 2]>> {
    input
        .lines()
        .map(|line| {
            line.split(')')
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| {
                    TaskError("Wrong number of elements in input!".to_string())
                })
        })
        .collect()
}

use std::collections::HashMap;

const CENTER_OF_MASS: &str = "COM";
const YOU: &str = "YOU";
const SANTA: &str = "SAN";

pub fn task1(input: &[[String; 2]]) -> Result<usize> {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    for [center, satelite] in input {
        orbits.entry(center).or_default().push(satelite);
    }
    let mut to_visit = vec![CENTER_OF_MASS];
    let mut sum = 0;
    for level in 0.. {
        let mut next_level = vec![];
        sum += level * to_visit.len();
        for object in to_visit {
            if orbits.contains_key(&object) {
                next_level.extend_from_slice(&orbits[&object]);
            }
        }
        if next_level.is_empty() {
            return Ok(sum);
        }
        to_visit = next_level;
    }
    unreachable!()
}

use std::collections::HashSet;

pub fn task2(input: &[[String; 2]]) -> Result<usize> {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    let mut back_orbits = HashMap::<&str, &str>::new();
    let mut visited = HashSet::<&str>::new();
    for [center, satelite] in input {
        orbits.entry(center).or_default().push(satelite);
        back_orbits.insert(satelite, center);
    }
    let mut to_visit = vec![&YOU];
    for transferes in 0.. {
        let mut next_level = vec![];
        for object in to_visit {
            visited.insert(object);
            if orbits.contains_key(object) {
                for next in &orbits[object] {
                    if next == &SANTA {
                        return Ok(transferes - 1);
                    }
                    if !visited.contains(next) {
                        next_level.push(next);
                    }
                }
            }
            if back_orbits.contains_key(object) {
                if back_orbits[object] == SANTA {
                    return Ok(transferes - 1);
                }
                next_level.push(&back_orbits[object]);
            }
        }
        to_visit = next_level;
    }
    unreachable!()
}
