const SIZE: usize = 300;
const SQUARE_SIZE: usize = 3;

fn power_level(x: usize, y: usize, serial: usize) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + serial) * rack_id) / 100 % 10) as i32 - 5
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    let serial = input.trim().parse()?;
    Ok((1..=SIZE)
        .map(|y| (1..=SIZE).map(|x| power_level(x, y, serial)).collect())
        .collect())
}

use itertools::Itertools;

fn best(map: &[Vec<i32>], dial: usize) -> (usize, usize, i32) {
    let (mut best_x, mut best_y, mut best_power) = (0, 0, 0);
    for x in 1..=(SIZE + 1 - dial) {
        for y in 1..=(SIZE + 1 - dial) {
            let power = (0..dial)
                .cartesian_product(0..dial)
                .map(|(dx, dy)| map[y + dy - 1][x + dx - 1])
                .sum();
            if power > best_power {
                best_x = x;
                best_y = y;
                best_power = power;
            }
        }
    }
    (best_x, best_y, best_power)
}

pub fn task1(map: &[Vec<i32>]) -> anyhow::Result<String> {
    let (best_x, best_y, _) = best(map, SQUARE_SIZE);
    Ok(format!("{best_x},{best_y}"))
}

pub fn task2(map: &[Vec<i32>]) -> anyhow::Result<String> {
    let (dial, (x, y, _)) = (1..=SIZE)
        .map(|dial| (dial, best(map, dial)))
        .max_by_key(|(_, (_, _, p))| *p)
        .ok_or(anyhow::anyhow!("unreachable"))?;
    Ok(format!("{x},{y},{dial}"))
}
