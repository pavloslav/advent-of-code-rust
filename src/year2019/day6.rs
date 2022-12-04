pub fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(')');
            (
                line.next().unwrap().to_string(),
                line.next().unwrap().to_string(),
            )
        })
        .collect()
}

use std::collections::HashMap;

pub fn task1(input: &[(String, String)]) -> usize {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    for (center, satelite) in input {
        orbits.entry(center).or_default().push(satelite);
    }
    let mut to_visit = vec!["COM"];
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
            return sum;
        }
        to_visit = next_level;
    }
    unreachable!()
}

use std::collections::HashSet;

pub fn task2(input: &[(String, String)]) -> usize {
    let mut orbits = HashMap::<&str, Vec<&str>>::new();
    let mut back_orbits = HashMap::<&str, &str>::new();
    let mut visited = HashSet::<&str>::new();
    for (center, satelite) in input {
        orbits.entry(center).or_default().push(satelite);
        back_orbits.insert(satelite, center);
    }
    let mut to_visit = vec![&"YOU"];
    for transferes in 0.. {
        let mut next_level = vec![];
        for object in to_visit {
            visited.insert(object);
            if orbits.contains_key(object) {
                for next in &orbits[*object] {
                    if next == &"SAN" {
                        return transferes - 1;
                    }
                    if !visited.contains(next) {
                        next_level.push(next);
                    }
                }
            }
            if back_orbits.contains_key(object) {
                if back_orbits[*object] == "SAN" {
                    return transferes - 1;
                }
                next_level.push(&back_orbits[*object]);
            }
        }
        to_visit = next_level;
    }
    unreachable!()
}
