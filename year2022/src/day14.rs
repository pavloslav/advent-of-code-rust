use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<Vec<u8>>> {
    let input: Vec<Vec<(i32, i32)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| Ok(prse::try_parse!(pair, "{},{}")?))
                .collect::<AocResult<Vec<_>>>()
        })
        .collect::<AocResult<Vec<_>>>()?;
    let (max_x, max_y) = input
        .iter()
        .flat_map(|line| line.iter())
        .fold((i32::MIN, i32::MIN), |(max_x, max_y), &(x, y)| {
            (std::cmp::max(max_x, x), std::cmp::max(max_y, y))
        });
    let width = (2 * max_x) as usize;
    let height = (max_y + 1) as usize;
    let mut map = vec![vec![b' '; width]; height];
    for line in input {
        for pair in line.windows(2) {
            if pair[0].0 == pair[1].0 {
                let range = if pair[0].1 <= pair[1].1 {
                    pair[0].1..=pair[1].1
                } else {
                    pair[1].1..=pair[0].1
                };
                for y in range {
                    map[y as usize][pair[0].0 as usize] = b'#';
                }
            } else {
                let range = if pair[0].0 <= pair[1].0 {
                    pair[0].0..=pair[1].0
                } else {
                    pair[1].0..=pair[0].0
                };
                for x in range {
                    map[pair[0].1 as usize][x as usize] = b'#';
                }
            }
        }
    }
    Ok(map)
}

pub fn task1(map: &[Vec<u8>]) -> AocResult<usize> {
    let mut map = map.to_vec();
    for sandgrain in 0.. {
        assert_eq!(map[0][500], b' ');
        let (mut x, mut y) = (500, 0);
        loop {
            if y == map.len() - 1 {
                return Ok(sandgrain);
            } else if map[y + 1][x] == b' ' {
            } else if map[y + 1][x - 1] == b' ' {
                x -= 1;
            } else if map[y + 1][x + 1] == b' ' {
                x += 1;
            } else {
                map[y][x] = b'o';
                break;
            }
            y += 1;
        }
    }
    Err(aoc_error!("unreachable"))
}

pub fn task2(map: &[Vec<u8>]) -> AocResult<usize> {
    let mut map = map.to_vec();
    map.push(vec![b' '; map[0].len()]);
    for sandgrain in 0.. {
        if map[0][500] != b' ' {
            return Ok(sandgrain);
        }
        let (mut x, mut y) = (500, 0);
        loop {
            if y == map.len() - 1 {
                map[y][x] = b'o';
                break;
            } else if map[y + 1][x] == b' ' {
            } else if map[y + 1][x - 1] == b' ' {
                x -= 1;
            } else if map[y + 1][x + 1] == b' ' {
                x += 1;
            } else {
                map[y][x] = b'o';
                break;
            }
            y += 1;
        }
    }
    Err(aoc_error!("unreachable!"))
}
