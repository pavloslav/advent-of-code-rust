use crate::*;

pub fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|line| Ok(scan_fmt::scan_fmt!(line, "{}, {}", i32, i32)?))
        .collect()
}

fn dist(pt1: (i32, i32), pt2: (i32, i32)) -> i32 {
    (pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs()
}

fn closest(points: &[(i32, i32)], pt: (i32, i32)) -> Option<usize> {
    enum LookupState {
        Infinite,
        Unique(usize, i32),
        Duplicate(i32),
    }

    use std::cmp::Ordering as Ord;

    let min = points
        .iter()
        .enumerate()
        .fold(LookupState::Infinite, |acc, (i, &point)| {
            let d = dist(point, pt);
            match acc {
                LookupState::Infinite => LookupState::Unique(i, d),
                LookupState::Unique(_, old) | LookupState::Duplicate(old) => match old.cmp(&d) {
                    Ord::Greater => LookupState::Unique(i, d),
                    Ord::Equal => LookupState::Duplicate(d),
                    Ord::Less => acc,
                },
            }
        });

    match min {
        LookupState::Unique(idx, _) => Some(idx),
        _ => None,
    }
}

pub fn task1(input: &[(i32, i32)]) -> Result<usize> {
    let min_x = input
        .iter()
        .map(|(x, _)| x)
        .min()
        .ok_or(aoc_error!("Empty input!"))?
        - 1;
    let max_x = input
        .iter()
        .map(|(x, _)| x)
        .max()
        .ok_or(aoc_error!("Empty input!"))?
        + 1;
    let min_y = input
        .iter()
        .map(|(_, y)| y)
        .min()
        .ok_or(aoc_error!("Empty input!"))?
        - 1;
    let max_y = input
        .iter()
        .map(|(_, y)| y)
        .max()
        .ok_or(aoc_error!("Empty input!"))?
        + 1;
    let mut counter = vec![Some(0); input.len()];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(point) = closest(input, (x, y)) {
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    counter[point] = None;
                } else {
                    counter[point] = counter[point].map(|x| x + 1);
                }
            }
        }
    }
    counter
        .iter()
        .filter_map(|&x| x)
        .max()
        .ok_or(aoc_error!("No suitable solution"))
}

pub fn task2(input: &[(i32, i32)]) -> Result<usize> {
    let min_x = input
        .iter()
        .map(|(x, _)| x)
        .min()
        .ok_or(aoc_error!("Empty input!"))?
        - 1;
    let max_x = input
        .iter()
        .map(|(x, _)| x)
        .max()
        .ok_or(aoc_error!("Empty input!"))?
        + 1;
    let min_y = input
        .iter()
        .map(|(_, y)| y)
        .min()
        .ok_or(aoc_error!("Empty input!"))?
        - 1;
    let max_y = input
        .iter()
        .map(|(_, y)| y)
        .max()
        .ok_or(aoc_error!("Empty input!"))?
        + 1;
    let mut counter = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if input.iter().map(|&pt| dist(pt, (x, y))).sum::<i32>() < 10_000 {
                counter += 1;
            }
        }
    }
    Ok(counter)
}
