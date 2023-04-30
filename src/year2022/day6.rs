use crate::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

use std::collections::HashSet;

fn first_different(input: &str, length: usize) -> Result<usize> {
    if length == 0 {
        return Err(task_error!("Length should be greater then 0"));
    }
    if input.len() < length {
        return Err(task_error!(
            "String length should be less then length parameter"
        ));
    }
    for i in 0..input.len() - length {
        let set: HashSet<_> = input[i..i + length].chars().collect();
        if set.len() == length {
            return Ok(i + length);
        }
    }
    Err(task_error!("Set not found!"))
}

pub fn task1(input: &str) -> Result<usize> {
    first_different(input, 4)
}

pub fn task2(input: &str) -> Result<usize> {
    first_different(input, 14)
}
