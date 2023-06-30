use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<i32>> {
    input
        .trim()
        .chars()
        .map(|c| match c.to_digit(10) {
            Some(d) => Ok(d as i32),
            None => Err(aoc_error!("Wrong digit: '{c}'")),
        })
        .collect()
}

const PHASES: usize = 100;
const PATTERN: [i32; 4] = [0, 1, 0, -1];

pub fn task1(input: &[i32]) -> Result<String> {
    let mut signal = input.to_vec();
    for _ in 0..PHASES {
        signal = (1..=signal.len())
            .map(|i| {
                (signal
                    .iter()
                    .enumerate()
                    .map(|(j, d)| d * PATTERN[(j + 1) / i % PATTERN.len()] as i32)
                    .sum::<i32>()
                    % 10)
                    .abs()
            })
            .collect();
    }
    signal
        .iter()
        .take(8)
        .map(|&d| char::from_digit(d as u32, 10).ok_or_else(|| aoc_error!("Incorrect digit: {d}")))
        .collect()
}

pub fn task2(_input: &[i32]) -> Result<usize> {
    Err(aoc_error!("Todo"))
}
