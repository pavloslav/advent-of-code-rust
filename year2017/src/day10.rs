use super::knots_hash;
use crate::*;

pub fn parse_input(input: &str) -> AocResult<&str> {
    Ok(input)
}

pub fn task1(input: &str) -> AocResult<usize> {
    let numbers = input
        .trim()
        .split(',')
        .map(|x| Ok(x.parse::<usize>()?))
        .collect::<Result<Vec<usize>>>()?;
    let knots = knots_hash::knots_hash(1, knots_hash::SIZE, numbers.into_iter());
    Ok(knots[0] * knots[1])
}

pub fn task2(input: &str) -> AocResult<String> {
    use std::fmt::Write;
    Ok(
        knots_hash::dense_hash(input.trim().bytes().map(|c| c.into()))
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02x}");
                output
            }),
    )
}
