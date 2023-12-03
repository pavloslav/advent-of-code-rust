use super::computer::Computer;
use crate::*;

#[derive(PartialEq)]
pub enum Cell {
    Scaffold,
    Empty,
    Up,
    Right,
    Down,
    Left,
}

pub fn parse_input(input: &str) -> AocResult<Vec<Vec<Cell>>> {
    let mut robot = Computer::new(&Computer::prepare_code(input)?);
    robot.run()?;
    let mut map = vec![];
    let mut line = vec![];
    while let Ok(data) = robot.read() {
        use Cell::*;
        match u8::try_from(data).map_err(|_| aoc_error!("Invalid code '{data}'"))? {
            b'.' => line.push(Empty),
            b'#' => line.push(Scaffold),
            b'^' => line.push(Up),
            b'>' => line.push(Right),
            b'v' => line.push(Down),
            b'<' => line.push(Left),
            b'\n' => {
                let mut tail = vec![];
                tail.append(&mut line);
                map.push(tail);
            }
            _ => return Err(aoc_error!("Invalid code '{data}'")),
        }
    }
    Ok(map)
}

fn no_neighbors(map: &[Vec<Cell>], x: usize, y: usize, check: Cell) -> bool {
    (x == 0 || map[y][x - 1] != check)
        && (y == 0 || map[y - 1][x] != check)
        && (x + 1 == map[y].len() || map[y][x + 1] != check)
        && (y + 1 == map.len() || map[y + 1][x] != check)
}

pub fn task1(input: &[Vec<Cell>]) -> AocResult<usize> {
    Ok(input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, cell)| {
                    if cell != &Cell::Empty && no_neighbors(input, x, y, Cell::Empty) {
                        x * y
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum())
}

pub fn task2(_input: &[Vec<Cell>]) -> AocResult<usize> {
    Err(aoc_error!("Todo"))
}
