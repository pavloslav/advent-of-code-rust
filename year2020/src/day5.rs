use crate::*;

fn seat_id(code: &str) -> AocResult<usize> {
    code.chars()
        .map(|c| match c {
            'F' | 'L' => Ok(0),
            'B' | 'R' => Ok(1),
            other => Err(aoc_error!("Incorrect zone/seat symbol '{other}'")),
        })
        .try_fold(0, |acc, n| Ok(acc * 2 + n?))
}

pub fn parse_input(input: &str) -> AocResult<Vec<usize>> {
    input.lines().map(seat_id).collect()
}

pub fn task1(s: &[usize]) -> AocResult<usize> {
    s.iter()
        .max()
        .copied()
        .ok_or_else(|| aoc_error!("Empty input!"))
}

pub fn task2(s: &[usize]) -> AocResult<usize> {
    let (min, max, sum) = s.iter().fold((1024, 0, 0), |(min, max, sum), &id| {
        (min.min(id), max.max(id), sum + id)
    });
    Ok((max - min + 1) * (max + min) / 2 - sum)
}
