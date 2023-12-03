use crate::*;

pub fn parse_input(input: &str) -> AocResult<&str> {
    Ok(input.trim())
}

pub fn task1(input: &str) -> AocResult<usize> {
    Ok((0..128)
        .map(|i| {
            super::knots_hash::dense_hash(format!("{input}-{i}").bytes().map(|c| c.into()))
                .iter()
                .map(|x| x.count_ones() as usize)
                .sum::<usize>()
        })
        .sum())
}

fn find_start(map: &[Vec<bool>]) -> Option<(usize, usize)> {
    for (x, line) in map.iter().enumerate() {
        for (y, &v) in line.iter().enumerate() {
            if v {
                return Some((x, y));
            }
        }
    }
    None
}

fn floodfill(map: &mut [Vec<bool>], (x, y): (usize, usize)) {
    if map[x][y] {
        map[x][y] = false;
        if x > 0 {
            floodfill(map, (x - 1, y))
        }
        if y > 0 {
            floodfill(map, (x, y - 1))
        }
        if x < map.len() - 1 {
            floodfill(map, (x + 1, y))
        }
        if y < map.len() - 1 {
            floodfill(map, (x, y + 1))
        }
    }
}

pub fn task2(input: &str) -> AocResult<usize> {
    let mut map: Vec<Vec<bool>> = (0..128)
        .map(|i| {
            super::knots_hash::dense_hash(format!("{input}-{i}").bytes().map(|c| c.into()))
                .iter()
                .flat_map(|&x| (0..8).map(move |i| (x >> (7 - i)) & 1 == 1))
                .collect()
        })
        .collect();
    for i in 0.. {
        if let Some(pos) = find_start(&map) {
            floodfill(&mut map, pos);
        } else {
            return Ok(i);
        }
    }
    Err(aoc_error!("unreachable"))
}
