use super::super::common::Error::TaskError;
use super::aoc::*;

pub fn parse_input(input: &str) -> Result<Vec<Vec<u8>>> {
    Ok(input.lines().map(|line| line.bytes().collect()).collect())
}

fn value(f: u8) -> u8 {
    if f == b'S' {
        b'a'
    } else if f == b'E' {
        b'z'
    } else {
        f
    }
}

fn can_move(
    map: &[Vec<u8>],
    path: &[Vec<usize>],
    field: (i32, i32),
    neighbor: (i32, i32),
) -> bool {
    if neighbor.0 < 0
        || neighbor.0 >= map.len().try_into().unwrap()
        || neighbor.1 < 0
        || neighbor.1 >= map[0].len().try_into().unwrap()
        || path[neighbor.0 as usize][neighbor.1 as usize] != 0
    {
        false
    } else {
        let field = value(map[field.0 as usize][field.1 as usize]);
        let neighbor = value(map[neighbor.0 as usize][neighbor.1 as usize]);
        neighbor <= field + 1
    }
}

fn shortest_way(start: (i32, i32), map: &[Vec<u8>]) -> Option<usize> {
    let mut path = vec![vec![0; map[0].len()]; map.len()];
    let mut to_check = vec![start];
    for step in 1.. {
        let mut new_check = vec![];
        for field in to_check {
            for shift in &[(1i32, 0i32), (0, 1), (-1, 0), (0, -1)] {
                let neighbor = (field.0 + shift.0, field.1 + shift.1);
                if can_move(map, &path, field, neighbor) {
                    if map[neighbor.0 as usize][neighbor.1 as usize] == b'E' {
                        return Some(step);
                    }
                    path[neighbor.0 as usize][neighbor.1 as usize] = step;
                    new_check.push(neighbor);
                }
            }
        }
        if new_check.is_empty() {
            return None;
        }
        to_check = new_check;
    }
    unreachable!()
}

pub fn task1(map: &[Vec<u8>]) -> Result<usize> {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter()
                .position(|&symbol| symbol == b'S')
                .map(|j| (i as i32, j as i32))
        })
        .ok_or_else(|| TaskError("Empty input!".to_string()))?;
    shortest_way(start, map)
        .ok_or_else(|| TaskError("Way not found!".to_string()))
}

pub fn task2(map: &[Vec<u8>]) -> Result<usize> {
    map.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, &symbol)| {
                    if symbol == b'S' || symbol == b'a' {
                        shortest_way((i as i32, j as i32), map)
                    } else {
                        None
                    }
                })
                .min()
        })
        .min()
        .ok_or_else(|| TaskError("Empty table!".to_string()))?
        .ok_or_else(|| TaskError("Empty line!".to_string()))
}
