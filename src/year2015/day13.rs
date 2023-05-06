use crate::*;

pub struct Table {
    scores: Vec<Vec<i32>>,
}

pub fn parse_input(input: &str) -> Result<Table> {
    let mut names = std::collections::HashMap::new();
    let mut scores = vec![];
    for line in input.lines() {
        let (first, gain, score, second) = scan_fmt::scan_fmt!(
            line,
            "{} would {/gain|lose/} {} happiness units by sitting next to {}.",
            String,
            String,
            i32,
            String
        )?;
        let score = match gain.as_str() {
            "gain" => score,
            "lose" => -score,
            other => return Err(task_error!("Units can not be '{other}'")),
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
    println!("{}", scores.len());
    Ok(Table { scores })
}

impl Table {
    fn round_score(&self, perm: &[usize]) -> i32 {
        (0..perm.len())
            .map(|i| {
                self.scores[perm[i]]
                    [perm[if i + 1 == perm.len() { 0 } else { i + 1 }]]
            })
            .sum()
    }
    fn line_score(&self, perm: &[usize]) -> i32 {
        perm.windows(2)
            .map(|pair| {
                if let &[left, right] = pair {
                    self.scores[left][right]
                } else {
                    unreachable!("windows(2) should always return 2 items")
                }
            })
            .sum()
    }
}

use itertools::Itertools;

pub fn task1(input: &Table) -> Result<i32> {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.round_score(&permutation))
        .max()
        .ok_or_else(|| task_error!("No options to consider"))
}

pub fn task2(input: &Table) -> Result<i32> {
    (0..input.scores.len())
        .permutations(input.scores.len())
        .map(|permutation| input.line_score(&permutation))
        .max()
        .ok_or_else(|| task_error!("No options to consider"))
}
