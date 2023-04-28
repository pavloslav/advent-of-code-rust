use super::super::common::Error::TaskError;
use super::aoc::*;

type Connections = std::collections::HashMap<usize, Vec<usize>>;

pub fn parse_input(input: &str) -> Result<Connections> {
    use once_cell::sync::Lazy;
    static INPUT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r"^(?P<index>\d+) <-> (?P<connected>.*)$").unwrap()
    });
    input
        .lines()
        .map(|line| {
            INPUT_REGEX
                .captures(line)
                .ok_or_else(|| TaskError("Can't parse input".to_string()))
                .map(|captures| {
                    if let (Some(index), Some(connected)) =
                        (captures.name("index"), captures.name("connected"))
                    {
                        Ok((
                            index.as_str().parse()?,
                            connected
                                .as_str()
                                .split(", ")
                                .map(|s| Ok(s.parse()?))
                                .collect::<Result<_>>()?,
                        ))
                    } else {
                        Err(TaskError(
                            "Can't find all elements in line".to_string(),
                        ))
                    }
                })?
        })
        .collect()
}

fn floodfill(
    connections: &Connections,
    start: usize,
    value: i32,
    field: &mut [i32],
) {
    let mut to_fill = connections[&start].clone();
    while let Some(tgt) = to_fill.pop() {
        if field[tgt] != value {
            field[tgt] = value;
            to_fill.extend(&connections[&tgt]);
        }
    }
}

pub fn task1(input: &Connections) -> Result<usize> {
    let mut zero_connected = vec![-1; input.len()];
    floodfill(input, 0, 0, &mut zero_connected);
    Ok(zero_connected.iter().filter(|&&x| x == 0).count())
}

pub fn task2(input: &Connections) -> Result<i32> {
    let mut connect: Vec<i32> = vec![-1; input.len()];
    let mut group_index = 0;
    while let Some((first, _x)) =
        connect.iter().enumerate().find(|&(_, &x)| x == -1)
    {
        floodfill(input, first, group_index, &mut connect);
        group_index += 1;
    }
    Ok(group_index)
}
