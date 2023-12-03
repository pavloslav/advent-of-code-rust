use crate::*;

fn value(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u8 - b'A') as u32 + 27
    } else {
        (c as u8 - b'a') as u32 + 1
    }
}

pub fn parse_input(input: &str) -> AocResult<Vec<Vec<u32>>> {
    Ok(input
        .lines()
        .map(|line| line.chars().map(value).collect())
        .collect())
}

use std::collections::HashSet;

pub fn task1(input: &[Vec<u32>]) -> AocResult<u32> {
    input
        .iter()
        .map(|backpack| {
            let (left, right) = backpack.split_at(backpack.len() / 2);
            let left: HashSet<_> = left.iter().collect();
            right
                .iter()
                .find(|&item| left.contains(&item))
                .ok_or_else(|| aoc_error!("Empty badges!"))
        })
        .try_fold(0, |acc, x: AocResult<_>| Ok(acc + x?))
}

pub fn task2(input: &[Vec<u32>]) -> AocResult<u32> {
    let mut result = 0;
    for group in input.chunks(3) {
        let mut badges: HashSet<_> = group[0].iter().collect();
        for backpack in &group[1..3] {
            badges = badges
                .intersection(&backpack.iter().collect())
                .copied()
                .collect();
        }
        result += *badges
            .iter()
            .next()
            .ok_or_else(|| aoc_error!("Empty badges!"))?;
    }
    Ok(result)
}
