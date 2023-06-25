use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
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
            other => return Err(aoc_error!("Units can not be '{other}'")),
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
    Ok(scores)
}

use itertools::Itertools;

fn score(scores: &[Vec<i32>], perm: &[usize]) -> i32 {
    perm.iter()
        .tuple_windows()
        .map(|(&left, &right)| scores[left][right])
        .sum()
}

pub fn task1(input: &[Vec<i32>]) -> Result<i32> {
    (0..input.len())
        .permutations(input.len())
        .map(|permutation| {
            let perm: Vec<usize> = permutation
                .iter()
                .chain(std::iter::once(&permutation[0]))
                .copied()
                .collect();
            score(input, &perm)
        })
        .max()
        .ok_or_else(|| aoc_error!("No options to consider"))
}

pub fn task2(input: &[Vec<i32>]) -> Result<i32> {
    (0..input.len())
        .permutations(input.len())
        .map(|permutation| score(input, &permutation))
        .max()
        .ok_or_else(|| aoc_error!("No options to consider"))
}