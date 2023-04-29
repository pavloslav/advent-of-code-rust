use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Valve {
    name: String,
    rate: usize,
    ways: Vec<String>,
}

type Directions = HashMap<String, Valve>;

pub fn parse_input(input: &str) -> Directions {
    static REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)").unwrap()
    });
    input
        .lines()
        .map(|line| {
            let matches = REGEX.captures(line).unwrap();
            let name = matches.get(1).unwrap().as_str().to_string();
            let rate = matches.get(2).unwrap().as_str().parse().unwrap();
            let ways = matches
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            (name.clone(), Valve { name, rate, ways })
        })
        .collect()
}

struct Searcher {
    map: Directions,
    opened: HashSet<String>,
}

impl Searcher {
    fn max_pressure(&mut self, location: &str, time: usize) -> usize {
        if time == 0 {
            0
        } else {
            let mut result = 0;
            let ways = self.map[location].ways.clone();
            let mut best = ways
                .iter()
                .cloned()
                .map(|tgt| self.max_pressure(&tgt, time - 1))
                .max()
                .unwrap_or(0);
            if time > 1 && !self.opened.contains(location) {
                self.opened.insert(location.to_string());
                result = (time - 1) * self.map[location].rate;
                best = best.max(
                    ways.iter()
                        .map(|tgt| self.max_pressure(tgt, time - 2))
                        .max()
                        .unwrap_or(0),
                );
                self.opened.remove(location);
            }
            result + best
        }
    }
}

pub fn task1(directions: &Directions) -> usize {
    todo!();
    let mut searcher = Searcher {
        map: directions.clone(),
        opened: HashSet::new(),
    };
    searcher.max_pressure("AA", 30)
}

pub fn task2(_directions: &Directions) -> usize {
    todo!();
}
