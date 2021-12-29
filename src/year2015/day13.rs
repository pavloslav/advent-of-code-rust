pub struct Table {
    scores: Vec<Vec<i32>>,
}

pub fn parse_input(input: &str) -> Table {
    let mut names = std::collections::HashMap::new();
    let mut scores = vec![];
    for line in input.lines() {
        lazy_static::lazy_static! {
            static ref INPUT_REGEX: regex::Regex = regex::Regex::new(r"(?P<first>\w+) would (?P<gain>gain|lose) (?P<score>\d+) happiness units by sitting next to (?P<second>\w+)\.").unwrap();
        }
        if let Some(captures) = INPUT_REGEX.captures(line) {
            if let Some(first) = captures.name("first") {
                if let Some(gain) = captures.name("gain") {
                    if let Some(score) = captures.name("score") {
                        if let Ok(score) = score.as_str().parse::<i32>() {
                            if let Some(second) = captures.name("second") {
                                let score = (if gain.as_str() == "gain" {
                                    1
                                } else {
                                    -1
                                }) * score;
                                let first = first.as_str().to_owned();
                                let second = second.as_str().to_owned();
                                let names_size = names.len();
                                let first =
                                    *names.entry(first).or_insert(names_size);
                                let names_size = names.len();
                                let second =
                                    *names.entry(second).or_insert(names_size);
                                if scores.len() < names.len() {
                                    scores.resize(
                                        names.len(),
                                        vec![0; names.len()],
                                    );
                                }
                                if scores[first].len() < names.len() {
                                    scores[first].resize(names.len(), 0);
                                }
                                if scores[second].len() < names.len() {
                                    scores[second].resize(names.len(), 0);
                                }
                                scores[first][second] += score;
                                scores[second][first] += score;
                            }
                        }
                    }
                }
            }
        }
    }
    Table { scores }
}

impl Table {
    fn round_score<I>(&self, it: I) -> i32
    where
        I: Iterator<Item = usize>,
    {
        let v: Vec<_> = it.collect();
        (0..v.len())
            .map(|i| self.scores[v[i]][v[(i + 1) % v.len()]])
            .sum()
    }
    fn line_score<I>(&self, it: I) -> i32
    where
        I: Iterator<Item = usize>,
    {
        let v: Vec<_> = it.collect();
        (0..v.len() - 1).map(|i| self.scores[v[i]][v[i + 1]]).sum()
    }
}

use itertools::Itertools;

pub fn task1(input: &Table) -> i32 {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.round_score(permutation.into_iter()))
        .max()
        .unwrap()
}

pub fn task2(input: &Table) -> i32 {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.line_score(permutation.into_iter()))
        .max()
        .unwrap()
}
