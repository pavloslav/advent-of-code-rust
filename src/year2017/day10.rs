use super::super::common::Result;
use super::knots_hash;
use super::Error::TaskError;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

pub fn task1(input: &str) -> Result<usize> {
    let numbers = input
        .trim()
        .split(',')
        .map(|x| {
            x.parse()
                .map_err(|e| TaskError(format!("Parsing failed: {e}")))
        })
        .collect::<Result<Vec<usize>>>()?;
    let knots =
        knots_hash::knots_hash(1, knots_hash::SIZE, numbers.into_iter());
    Ok(knots[0] * knots[1])
}

pub fn task2(input: &str) -> Result<String> {
    Ok(
        knots_hash::dense_hash(input.trim().bytes().map(|c| c.into()))
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect(),
    )
}
