use anyhow::Context;
use std::collections::HashMap;

type Instructions = Vec<usize>;
type Map<'a> = HashMap<&'a str, [&'a str; 2]>;

pub fn parse_input<'a>(input: &'a str) -> anyhow::Result<(Instructions, Map<'a>)> {
    let mut input = input.lines();
    let instruction = input
        .next()
        .context("No instructions found!")?
        .chars()
        .map(|c| match c {
            'L' => Ok(0),
            'R' => Ok(1),
            other => Err(anyhow::anyhow!("Invalid direction {other}")),
        })
        .collect::<anyhow::Result<_>>()?;
    let map = input
        .skip(1)
        .map(|line| {
            let (from, left, right) = prse::try_parse!(line, "{} = ({}, {})")?;
            Ok((from, [left, right]))
        })
        .collect::<anyhow::Result<_>>()?;

    Ok((instruction, map))
}

pub fn task1(input: &(Instructions, Map)) -> anyhow::Result<usize> {
    let mut location = &"AAA";
    let mut instructions = input.0.iter().cycle();
    let map = &input.1;
    let mut steps = 0;
    while location != &"ZZZ" {
        location = &map
            .get(location)
            .with_context(|| format!("No {location} found in map!"))?
            [*instructions.next().unwrap()];
        steps += 1;
    }
    Ok(steps)
}

pub fn task2(input: &(Instructions, Map)) -> anyhow::Result<usize> {
    let map = &input.1;

    let mut total_steps = 1;
    for ghost in map.keys().filter(|loc| loc.ends_with('A')) {
        let mut steps = 0;
        let mut location = ghost;
        let mut instructions = input.0.iter().cycle();
        while !location.ends_with('Z') {
            location = &map
                .get(location)
                .with_context(|| format!("No {location} found in map!"))?
                [*instructions.next().unwrap()];
            steps += 1;
        }
        total_steps = common::lcm(total_steps, steps);
    }
    Ok(total_steps)
}
