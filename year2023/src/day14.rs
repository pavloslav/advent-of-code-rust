pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    Ok(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

fn north_load(input: &[Vec<u8>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let mut sum = 0;
    for x in 0..width {
        let mut count = 0;
        let mut floor = height;
        for (y, line) in input.iter().enumerate() {
            if line[x] == b'O' {
                count += 1;
            } else if line[x] == b'#' {
                sum += (floor + (floor - count + 1)) * count / 2;
                count = 0;
                floor = height - y - 1;
            }
        }
        sum += (floor + (floor - count + 1)) * count / 2;
    }
    sum
}

pub fn task1(input: &[Vec<u8>]) -> anyhow::Result<usize> {
    Ok(north_load(input))
}

#[allow(clippy::needless_range_loop)]
fn cycle_rocks(rocks: &mut Vec<Vec<u8>>) {
    {
        let height = rocks.len();
        let width = rocks[0].len();
        //NORTH
        for x in 0..width {
            let mut count = 0;
            let mut floor = 0;
            for y in 0..height {
                if rocks[y][x] == b'O' {
                    count += 1;
                    rocks[y][x] = b'.';
                } else if rocks[y][x] == b'#' {
                    for dy in 0..count {
                        rocks[floor + dy][x] = b'O';
                    }
                    count = 0;
                    floor = y + 1;
                }
            }
            for dy in 0..count {
                rocks[floor + dy][x] = b'O';
            }
        }
        //WEST
        for y in 0..height {
            let mut count = 0;
            let mut floor = 0;
            for x in 0..width {
                if rocks[y][x] == b'O' {
                    count += 1;
                    rocks[y][x] = b'.';
                } else if rocks[y][x] == b'#' {
                    for dx in 0..count {
                        rocks[y][floor + dx] = b'O';
                    }
                    count = 0;
                    floor = x + 1;
                }
            }
            for dx in 0..count {
                rocks[y][floor + dx] = b'O';
            }
        }
        //SOUTH
        for x in 0..width {
            let mut count = 0;
            let mut floor = height - 1;
            for y in 0..height {
                let y = height - y - 1;
                if rocks[y][x] == b'O' {
                    count += 1;
                    rocks[y][x] = b'.';
                } else if rocks[y][x] == b'#' {
                    for dy in 0..count {
                        rocks[floor - dy][x] = b'O';
                    }
                    count = 0;
                    floor = y.saturating_sub(1);
                }
            }
            for dy in 0..count {
                rocks[floor - dy][x] = b'O';
            }
        }
        //EAST
        for y in 0..height {
            let mut count = 0;
            let mut floor = width - 1;
            for x in 0..width {
                let x = width - x - 1;
                if rocks[y][x] == b'O' {
                    count += 1;
                    rocks[y][x] = b'.';
                } else if rocks[y][x] == b'#' {
                    for dx in 0..count {
                        rocks[y][floor - dx] = b'O';
                    }

                    count = 0;
                    floor = x.saturating_sub(1);
                }
            }
            for dx in 0..count {
                rocks[y][floor - dx] = b'O';
            }
        }
    }
}

const CYCLES: usize = 1_000_000_000;

pub fn task2(input: &[Vec<u8>]) -> anyhow::Result<usize> {
    let (lambda, mu) = common::floyd_hare_tortoise(|| input.to_vec(), cycle_rocks);
    let mut rocks = input.to_vec();
    //CYCLES = mu + n*lambda + x
    let x = mu + (CYCLES - mu) % lambda;
    for _ in 0..x {
        cycle_rocks(&mut rocks);
    }
    Ok(north_load(&rocks)) //107002 - too high
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 136);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 64);
    }

    #[test]
    fn test_cycle_rocks() {
        let mut rocks = parse_input(INPUT).unwrap().to_vec();
        cycle_rocks(&mut rocks);
        assert_eq!(
            rocks.as_ref(),
            parse_input(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            )
            .unwrap()
        );
        cycle_rocks(&mut rocks);
        assert_eq!(
            rocks.as_ref(),
            parse_input(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            )
            .unwrap()
        );
        cycle_rocks(&mut rocks);
        assert_eq!(
            rocks.as_ref(),
            parse_input(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            )
            .unwrap()
        );
    }
}
