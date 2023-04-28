use super::super::common::Error::TaskError;
use super::aoc::*;

pub struct Table {
    scores: Vec<Vec<i32>>,
}

pub fn parse_input(input: &str) -> Result<Table> {
    todo!("Make it dynamic programming");
    let mut names = std::collections::HashMap::new();
    let mut scores = vec![];
    for line in input.lines() {
        let (first, gain, score, second) = scan_fmt::scan_fmt!(
            line,
            "{} would {} {} happiness units by sitting next to {}",
            String,
            String,
            i32,
            String
        )?;
        let score = match gain.as_str() {
            "gain" => score,
            "lose" => -score,
            other => return Err(TaskError(format!("Units can not '{other}'"))),
        };

        let names_size = names.len();
        let first = *names.entry(first).or_insert(names_size);
        let names_size = names.len();
        let second = *names.entry(second).or_insert(names_size);
        if scores.len() < names.len() {
            scores.resize(names.len(), vec![0; names.len()]);
            for line in scores.iter_mut() {
                line.resize(names.len(), 0);
            }
        }
        scores[first][second] += score;
        scores[second][first] += score;
    }
    Ok(Table { scores })
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

pub fn task1(input: &Table) -> Result<i32> {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.round_score(permutation.into_iter()))
        .max()
        .ok_or_else(|| TaskError("No options to consider".to_string()))
}

pub fn task2(input: &Table) -> Result<i32> {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.line_score(permutation.into_iter()))
        .max()
        .ok_or_else(|| TaskError("No options to consider".to_string()))
}
