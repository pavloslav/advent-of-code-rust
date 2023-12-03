use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<u32>> {
    let mut result = vec![0];
    for calories in input.lines() {
        if let Ok(calories) = calories.parse::<u32>() {
            let idx = result.len() - 1;
            result[idx] += calories;
        } else if calories.is_empty() {
            result.push(0);
        } else {
            return Err(
                aoc_error!("Incorrect input: '{calories}'"), /* aoc_error!(("Incorrect input: '{calories}'")*/
            );
        }
    }
    result.sort();
    Ok(result)
}

pub fn task1(elves: &[u32]) -> AocResult<u32> {
    Ok(elves[elves.len() - 1])
}

pub fn task2(elves: &[u32]) -> AocResult<u32> {
    Ok(elves[elves.len() - 3..].iter().sum())
}
