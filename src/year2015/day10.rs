use itertools::Itertools;

use super::super::common::Result;
type Data<'a> = &'a str;

fn look_and_say(input: &str) -> String {
    input
        .trim()
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| format!("{}{}", group.count(), c))
        .collect()
}

pub fn parse_input(input: &str) -> Result<Data> {
    Ok(input)
}

fn task(input: &str, count: usize) -> usize {
    (0..count)
        .fold(input.to_string(), |acc, _| look_and_say(&acc))
        .len()
}

pub fn task1(input: &Data) -> Result<usize> {
    Ok(task(input, 40))
}

pub fn task2(input: &Data) -> Result<usize> {
    Ok(task(input, 50))
}
