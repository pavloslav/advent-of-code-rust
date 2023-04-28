use super::super::common::Error::TaskError;
use super::aoc::*;

fn seat_id(code: &str) -> Result<usize> {
    code.chars()
        .map(|c| match c {
            'F' | 'L' => Ok(0),
            'B' | 'R' => Ok(1),
            other => {
                Err(TaskError(format!("Incorrect zone/seat symbol '{other}'")))
            }
        })
        .try_fold(0, |acc, n| Ok(acc * 2 + n?))
}

pub fn parse_input(input: &str) -> Result<Vec<usize>> {
    input.lines().map(seat_id).collect()
}

pub fn task1(s: &[usize]) -> Result<usize> {
    s.iter()
        .max()
        .copied()
        .ok_or_else(|| TaskError("Empty input!".to_string()))
}

pub fn task2(s: &[usize]) -> Result<usize> {
    let (min, max, sum) =
        s.iter().fold((1024, 0, 0), |(min, max, sum), &id| {
            (min.min(id), max.max(id), sum + id)
        });
    Ok((max - min + 1) * (max + min) / 2 - sum)
}
