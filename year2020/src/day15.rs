use crate::*;

use std::collections::HashMap;

fn memory_game(input: &[usize], turn: usize) -> usize {
    let mut numbers: HashMap<_, _> = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, &n)| (n, i + 1))
        .collect();
    let mut last_spoken = input[input.len() - 1];
    for i in input.len()..turn {
        let new = numbers.get(&last_spoken).map(|n| i - n).unwrap_or(0);
        numbers.insert(last_spoken, i);
        last_spoken = new;
    }
    last_spoken
}

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    input.trim().split(',').map(|s| Ok(s.parse()?)).collect()
}

const NUMBER1: usize = 2020;
const NUMBER2: usize = 30_000_000;

pub fn task1(input: &[usize]) -> Result<usize> {
    Ok(memory_game(input, NUMBER1))
}

pub fn task2(input: &[usize]) -> Result<usize> {
    Ok(memory_game(input, NUMBER2))
}
