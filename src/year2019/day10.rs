pub fn parse_input(input: &str) -> Vec<(i16, i16)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i16, y as i16))
                } else {
                    None
                }
            })
        })
        .collect()
}

use num::integer::gcd;

fn normalized(coord: (i16, i16)) -> (i16, i16) {
    if coord.0 == 0 && coord.1 == 0 {
        (0, 0)
    } else if coord.0 == 0 {
        (0, if coord.1 > 0 { 1 } else { -1 })
    } else if coord.1 == 0 {
        (if coord.0 > 0 { 1 } else { -1 }, 0)
    } else {
        let gcd = gcd(coord.0, coord.1);
        (coord.0 / gcd, coord.1 / gcd)
    }
}

use std::collections::HashSet;

fn visible_from(station: (i16, i16), asteroids: &[(i16, i16)]) -> usize {
    asteroids
        .iter()
        .filter_map(|&asteroid| {
            if asteroid != station {
                Some(normalized((
                    asteroid.0 - station.0,
                    asteroid.1 - station.1,
                )))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

fn station_position(asteroids: &[(i16, i16)]) -> (i16, i16) {
    *asteroids
        .iter()
        .max_by_key(|&&station| visible_from(station, asteroids))
        .unwrap()
}

pub fn task1(asteroids: &[(i16, i16)]) -> usize {
    visible_from(station_position(asteroids), asteroids)
}

fn atan((x, y): (i16, i16)) -> f64 {
    (x as f64).atan2(y as f64)
}

pub fn task2(asteroids: &[(i16, i16)]) -> i16 {
    assert!(asteroids.len() > 200);
    let station = station_position(asteroids);
    let mut asteroids: Vec<_> = asteroids
        .iter()
        .filter_map(|&a| {
            if a != station {
                Some((a.0 - station.0, a.1 - station.1))
            } else {
                None
            }
        })
        .collect();
    asteroids.sort_by(|&a, &b| atan(b).total_cmp(&atan(a)));
    let mut sorted_asteroids: Vec<Vec<_>> = Vec::new();
    for a in asteroids {
        if !sorted_asteroids.is_empty()
            && normalized(a)
                == normalized(*sorted_asteroids.last().unwrap().last().unwrap())
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
                    return x_r * 100 + y_r;
                }
            }
        }
        level += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
        let input = parse_input(SMALL_FIELD);
        assert_eq!(station_position(&input), (3, 4));
        assert_eq!(task1(&input), 8);
    }

    #[test]
    fn test_task1() {
        let input = parse_input(BIG_FIELD);
        assert_eq!(station_position(&input), (11, 13));
        assert_eq!(task1(&input), 210);
    }

    #[test]
    fn test_task2() {
        let input = parse_input(BIG_FIELD);
        assert_eq!(task2(&input), 802);
    }
}
