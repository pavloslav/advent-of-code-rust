use super::super::common::Result;
use super::Error::TaskError;

type Presents = Vec<Vec<usize>>;

pub fn parse_input(input: &str) -> Result<Presents> {
    let presents = input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|x| {
                    x.parse().map_err(|_| TaskError(format!("Can't parse {x}")))
                })
                .collect::<Result<Vec<usize>>>()
        })
        .collect::<Result<Presents>>()?;
    if presents.is_empty() {
        Err(TaskError("No presents!".to_string()))
    } else {
        Ok(presents)
    }
}

pub fn task1(presents: &Presents) -> Result<usize> {
    Ok(presents
        .iter()
        .map(|dims| {
            let side1 = dims[0] * dims[1];
            let side2 = dims[0] * dims[2];
            let side3 = dims[1] * dims[2];
            2 * (side1 + side2 + side3)
                + [side1, side2, side3]
                    .iter()
                    .min()
                    .expect("Can't happen, because presents are non-empty")
        })
        .sum())
}

pub fn task2(presents: &Presents) -> Result<usize> {
    Ok(presents
        .iter()
        .map(|dims| {
            2 * (dims.iter().sum::<usize>() - dims.iter().max().unwrap())
                + dims[0] * dims[1] * dims[2]
        })
        .sum())
}
