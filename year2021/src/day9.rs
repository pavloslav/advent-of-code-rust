use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| {
                    Ok(digit
                        .to_digit(10)
                        .with_context(|| format!("Not a digit: '{digit}'"))?
                        as u8)
                })
                .collect::<anyhow::Result<_>>()
        })
        .collect()
}

pub fn task1(map: &[Vec<u8>]) -> anyhow::Result<usize> {
    Ok(map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, &height)| {
                    if (i == 0 || height < map[i - 1][j])
                        && (j == 0 || height < map[i][j - 1])
                        && (i == map.len() - 1 || height < map[i + 1][j])
                        && (j == line.len() - 1 || height < map[i][j + 1])
                    {
                        Some(height as usize + 1)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum())
}

fn floodfill(map: &mut Vec<Vec<u8>>, i: i32, j: i32) -> usize {
    if i < 0
        || j < 0
        || i >= map.len() as i32
        || j >= map[i as usize].len() as i32
        || map[i as usize][j as usize] == 9
    {
        0
    } else {
        map[i as usize][j as usize] = 9;
        1 + floodfill(map, i - 1, j)
            + floodfill(map, i + 1, j)
            + floodfill(map, i, j - 1)
            + floodfill(map, i, j + 1)
    }
}

pub fn task2(map: &[Vec<u8>]) -> anyhow::Result<usize> {
    let mut map = map.to_owned();
    let mut result = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            result.push(floodfill(&mut map, i as i32, j as i32));
        }
    }
    result.sort();
    Ok(result.iter().rev().take(3).fold(1, |x, acc| acc * x))
}
