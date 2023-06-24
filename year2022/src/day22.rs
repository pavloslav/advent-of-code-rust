use crate::*;

pub enum Order {
    Walk(i16),
    Turn(i16),
}

pub struct Task {
    map: Vec<Vec<u8>>,
    path: Vec<Order>,
}

use once_cell::sync::Lazy;

pub fn parse_input(input: &str) -> Result<Task> {
    let mut lines = input.lines();
    let map = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect();
    static PATH_REGEX: Lazy<regex::Regex> =
        Lazy::new(|| regex::Regex::new(r"\d+|[RL]").unwrap());

    let path = PATH_REGEX
        .find_iter(lines.next().unwrap())
        .map(|mat| match mat.as_str() {
            "R" => Ok(Order::Turn(1)),
            "L" => Ok(Order::Turn(-1)),
            num => Ok(Order::Walk(num.parse()?)),
        })
        .collect::<Result<_>>()?;
    Ok(Task { map, path })
}

pub fn task1(input: &Task) -> Result<usize> {
    let mut dir = 0;
    let mut x = 0;
    let mut y = 0;
    for order in &input.path {
        match order {
            Order::Walk(len) => {
                for _ in 0..*len {
                    let (nx, ny) = match dir {
                        0 => (
                            if x + 1 < input.map[y].len()
                                && input.map[y][x + 1] != b' '
                            {
                                x + 1
                            } else {
                                input.map[y]
                                    .iter()
                                    .position(|&c| c != b' ')
                                    .ok_or_else(|| {
                                        aoc_error!(
                                            "Empty string, x = {x}, y = {y}, dir = {dir}!"
                                        )
                                    })?
                            },
                            y,
                        ),

                        1 => (
                            x,
                            if y < input.map.len() - 1
                                && input.map[y + 1].get(x).unwrap_or(&b' ')
                                    != &b' '
                            {
                                y + 1
                            } else {
                                input
                                    .map
                                    .iter()
                                    .position(|row| row[x] != b' ')
                                    .ok_or_else(|| {
                                        aoc_error!(
                                            "Empty string, x = {x}, y = {y}, dir = {dir}!"
                                        )
                                    })?
                            },
                        ),
                        2 => (
                            if x > 0 && input.map[y][x - 1] != b' ' {
                                x - 1
                            } else {
                                input.map[y]
                                    .iter()
                                    .rposition(|&c| c != b' ')
                                    .ok_or_else(|| {
                                        aoc_error!(
                                            "Empty string, x = {x}, y = {y}, dir = {dir}!"
                                        )
                                    })?
                            },
                            y,
                        ),
                        3 => (
                            x,
                            if y > 0
                                && input.map[y - 1].get(x).unwrap_or(&b' ')
                                    != &b' '
                            {
                                y - 1
                            } else {
                                input
                                    .map
                                    .iter()
                                    .rposition(|row| {
                                        row.len() > x && row[x] != b' '
                                    })
                                    .ok_or_else(|| {
                                        aoc_error!(
                                            "Empty string, x = {x}, y = {y}, dir = {dir}!"
                                        )
                                    })?
                            },
                        ),
                        _ => {
                            return Err(aoc_error!(
                                "Direction can't be {dir}!"
                            ))
                        }
                    };
                    if input.map[ny][nx] == b'.' {
                        (x, y) = (nx, ny);
                    }
                }
            }
            Order::Turn(d) => dir = (dir + d).rem_euclid(4),
        }
    }
    Ok(1000 * (y + 1) + 4 * (x + 1) + dir as usize)
}

pub fn task2(_input: &Task) -> Result<usize> {
    todo!();
}
