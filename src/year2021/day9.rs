pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

pub fn task1(map: &Vec<Vec<u8>>) -> usize {
    map.iter()
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
        .sum()
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

pub fn task2(map: &[Vec<u8>]) -> usize {
    let mut map = map.to_owned();
    let mut result = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            result.push(floodfill(&mut map, i as i32, j as i32));
        }
    }
    result.sort();
    result.iter().rev().take(3).fold(1, |x, acc| acc * x)
}
