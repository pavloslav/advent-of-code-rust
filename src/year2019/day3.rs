use super::super::common::Error;
use super::super::common::Error::TaskError;
use super::aoc::*;

pub struct Step {
    direction: (i32, i32),
    length: i32,
}

impl std::str::FromStr for Step {
    type Err = Error;
    fn from_str(input: &str) -> Result<Step> {
        Ok(Step {
            direction: match input.chars().next() {
                Some('R') => (1, 0),
                Some('L') => (-1, 0),
                Some('U') => (0, 1),
                Some('D') => (0, -1),
                Some(other) => Err(TaskError(format!(
                    "Unable to parse direction '{other}'"
                )))?,
                None => {
                    Err(TaskError("Unable to parse emtpy string".to_string()))?
                }
            },
            length: input[1..].parse()?,
        })
    }
}

pub fn parse_input(input: &str) -> Result<[Vec<Step>; 2]> {
    input
        .lines()
        .map(|line| line.split(',').map(|step| step.parse()).collect())
        .collect::<Result<Vec<_>>>()?
        .try_into()
        .map_err(|_| TaskError("Wrong size".to_string()))
}

use std::collections::HashSet;

fn get_set(steps: &[Step]) -> HashSet<(i32, i32)> {
    let (mut x, mut y) = (0, 0);
    steps
        .iter()
        .flat_map(|step| {
            let (old_x, old_y) = (x, y);
            let shift = step.direction;
            x += step.length * shift.0;
            y += step.length * shift.1;
            (1..=step.length)
                .map(move |i| (old_x + i * shift.0, old_y + i * shift.1))
        })
        .collect()
}

pub fn task1(input: &[Vec<Step>; 2]) -> Result<usize> {
    let way1 = get_set(&input[0]);
    let way2 = get_set(&input[1]);
    way1.intersection(&way2)
        .map(|(x, y)| (x.abs() + y.abs()) as usize)
        .min()
        .ok_or_else(|| TaskError("Way is empty!".to_string()))
}

use std::collections::HashMap;

fn get_map(steps: &[Step]) -> HashMap<(i32, i32), usize> {
    let (mut x, mut y, mut index) = (0, 0, 0);
    steps
        .iter()
        .flat_map(|step| {
            let (old_x, old_y, old_index) = (x, y, index);
            let shift = step.direction;
            x += step.length * shift.0;
            y += step.length * shift.1;
            index += step.length as usize;
            (1..=step.length).map(move |i| {
                (
                    (old_x + i * shift.0, old_y + i * shift.1),
                    old_index + i as usize,
                )
            })
        })
        .collect()
}

pub fn task2(input: &[Vec<Step>; 2]) -> Result<usize> {
    let way1 = get_map(&input[0]);
    let way2 = get_map(&input[1]);
    way1.iter()
        .filter(|(key, _)| way2.contains_key(key))
        .map(|(_, &length)| length)
        .min()
        .ok_or_else(|| TaskError("Way is empty!".to_string()))
}
