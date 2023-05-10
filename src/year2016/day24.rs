use crate::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const FREE: u8 = b'.';
const WALL: u8 = b'#';
const ZERO: u8 = b'0';

pub fn parse_input(input: &str) -> Result<Vec<Vec<u8>>> {
    Ok(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

fn find_locations(map: &[Vec<u8>]) -> HashMap<u8, (i32, i32)> {
    let mut locations = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != WALL && cell != FREE {
                locations.insert(cell, (x as i32, y as i32));
            }
        }
    }
    locations
}

fn find_distances(
    map: &[Vec<u8>],
    locations: &HashMap<u8, (i32, i32)>,
) -> HashMap<(u8, u8), i32> {
    let mut distances = HashMap::new();
    for (&location, &(x, y)) in locations {
        let mut to_visit = HashSet::from([(x, y)]);
        let mut visited_map = map.to_vec();
        let mut step = 1;
        while !to_visit.is_empty() {
            let mut new_visit = HashSet::new();
            for (x, y) in to_visit {
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    let cell = visited_map[ny as usize][nx as usize];
                    if cell != WALL {
                        new_visit.insert((nx, ny));
                        if cell != FREE {
                            distances.entry((location, cell)).or_insert(step);
                            distances.entry((cell, location)).or_insert(step);
                        }
                    }
                    visited_map[y as usize][x as usize] = WALL;
                }
            }
            to_visit = new_visit;
            step += 1;
        }
    }
    distances
}

pub fn task1(map: &[Vec<u8>]) -> Result<i32> {
    let locations = find_locations(map);
    let distances = find_distances(map, &locations);
    let mut best = i32::MAX;
    for perms in locations
        .keys()
        .filter(|&&loc| loc != ZERO)
        .copied()
        .permutations(locations.len() - 1)
    {
        best = std::iter::once(ZERO)
            .chain(perms.iter().copied())
            .tuple_windows()
            .map(|(a, b)| distances[&(a, b)])
            .sum::<i32>()
            .min(best);
    }
    Ok(best)
}

pub fn task2(map: &[Vec<u8>]) -> Result<i32> {
    let locations = find_locations(map);
    let distances = find_distances(map, &locations);
    let mut best = i32::MAX;
    for perms in locations
        .keys()
        .filter(|&&loc| loc != ZERO)
        .copied()
        .permutations(locations.len() - 1)
    {
        best = std::iter::once(ZERO)
            .chain(perms.iter().copied())
            .collect::<Vec<u8>>()
            .iter()
            .circular_tuple_windows()
            .map(|(&a, &b)| distances[&(a, b)])
            .sum::<i32>()
            .min(best);
    }
    Ok(best)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let parsed = parse_input(input).unwrap();
        assert_eq!(task1(&parsed).unwrap(), 14);
    }
}
