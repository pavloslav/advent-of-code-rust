use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<(i16, i16)>> {
    Ok(input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| Some((x as i16, y as i16)).filter(|_| c == '#'))
        })
        .collect())
}

fn normalized((x, y): (i16, i16)) -> (i16, i16) {
    let gcd = common::gcd(x, y);
    if gcd != 0 {
        (x / gcd, y / gcd)
    } else {
        (x.signum(), y.signum())
    }
}

use std::collections::HashSet;

fn visible_from(station: (i16, i16), asteroids: &[(i16, i16)]) -> usize {
    asteroids
        .iter()
        .filter_map(|&asteroid| {
            if asteroid != station {
                Some(normalized((asteroid.0 - station.0, asteroid.1 - station.1)))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

fn station_position(asteroids: &[(i16, i16)]) -> anyhow::Result<(i16, i16)> {
    asteroids
        .iter()
        .max_by_key(|&&station| visible_from(station, asteroids))
        .copied()
        .context("Asteroids shouldn't be empty!")
}

pub fn task1(asteroids: &[(i16, i16)]) -> anyhow::Result<usize> {
    Ok(visible_from(station_position(asteroids)?, asteroids))
}

fn atan((x, y): (i16, i16)) -> f64 {
    (x as f64).atan2(y as f64)
}

const ASTEROID_NEEDED: usize = 200;

pub fn task2(asteroids: &[(i16, i16)]) -> anyhow::Result<i16> {
    if asteroids.len() <= ASTEROID_NEEDED {
        anyhow::bail!("There should be at least {ASTEROID_NEEDED} asteroids!");
    }
    let station = station_position(asteroids)?;
    let mut asteroids: Vec<_> = asteroids
        .iter()
        .map(|&a| (a.0 - station.0, a.1 - station.1))
        .filter(|&asteroid| asteroid != (0, 0))
        .collect();
    asteroids.sort_by(|&a, &b| atan(b).total_cmp(&atan(a)));
    let mut sorted_asteroids: Vec<Vec<_>> = Vec::new();
    /* REFACTOR */
    for a in asteroids {
        if !sorted_asteroids.is_empty()
            && normalized(a) == normalized(*sorted_asteroids.last().unwrap().last().unwrap())
        {
            sorted_asteroids.last_mut().unwrap().push(a);
        } else {
            sorted_asteroids.push(vec![a]);
        }
    }
    for stack in &mut sorted_asteroids {
        stack.sort_by_key(|(x, y)| x * x + y * y);
    }
    let mut count = 200;
    let mut level = 0;
    loop {
        for stack in &sorted_asteroids {
            if stack.len() > level {
                count -= 1;
                if count == 0 {
                    let x_r = stack[level].0 + station.0;
                    let y_r = stack[level].1 + station.1;
                    return Ok(x_r * 100 + y_r);
                }
            }
        }
        level += 1;
    }
}

#[cfg(test)]
mod test {
    use super::{parse_input, station_position, task1, task2};
    const BIG_FIELD: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    const SMALL_FIELD: &str = ".#..#
.....
#####
....#
...##";

    #[test]
    fn test_task1_small() {
        let input = parse_input(SMALL_FIELD).unwrap();
        assert_eq!(station_position(&input).unwrap(), (3, 4));
        assert_eq!(task1(&input).unwrap(), 8);
    }

    #[test]
    fn test_task1() {
        let input = parse_input(BIG_FIELD).unwrap();
        assert_eq!(station_position(&input).unwrap(), (11, 13));
        assert_eq!(task1(&input).unwrap(), 210);
    }

    #[test]
    #[ignore]
    fn test_task2() {
        let input = parse_input(BIG_FIELD).unwrap();
        assert_eq!(task2(&input).unwrap(), 802);
    }
}
