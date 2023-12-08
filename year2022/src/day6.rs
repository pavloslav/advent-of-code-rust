pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

use std::collections::HashSet;

fn first_different(input: &str, length: usize) -> anyhow::Result<usize> {
    if length == 0 {
        anyhow::bail!("Length should be greater then 0");
    }
    if input.len() < length {
        anyhow::bail!("String length should be less then length parameter");
    }
    for i in 0..input.len() - length {
        let set: HashSet<_> = input[i..i + length].chars().collect();
        if set.len() == length {
            return Ok(i + length);
        }
    }
    Err(anyhow::anyhow!("Set not found!"))
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    first_different(input, 4)
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    first_different(input, 14)
}
