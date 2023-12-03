use crate::*;

const STEPS: usize = 100;

#[rustfmt::skip]
const NEIGHBORS: [(i32, i32); 8] = [(-1,-1), (-1, 0), (-1, 1),
                                    ( 0,-1),          ( 0, 1),
                                    ( 1,-1), ( 1, 0), ( 1, 1)];

#[derive(Clone, Copy)]
pub enum Cell {
    Enabled,
    Disabled,
}

impl Cell {
    fn value(&self) -> usize {
        match self {
            Cell::Enabled => 1,
            Cell::Disabled => 0,
        }
    }
}

#[derive(Clone)]
pub struct Life {
    field: Vec<Vec<Cell>>,
}

impl Life {
    fn step(&mut self, light_corners: bool) {
        if light_corners {
            self.light_up_corners();
        }
        self.field = (0..self.field.len())
            .map(|row| {
                (0..self.field[row].len())
                    .map(|col| {
                        match NEIGHBORS
                            .iter()
                            .filter_map(|&(dx, dy)| self.read(row as i32 + dx, col as i32 + dy))
                            .sum()
                        {
                            2 => self.field[row][col],
                            3 => Cell::Enabled,
                            _ => Cell::Disabled,
                        }
                    })
                    .collect()
            })
            .collect();
        if light_corners {
            self.light_up_corners();
        }
    }

    fn read(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0
            || row >= self.field.len() as i32
            || col < 0
            || col >= self.field[row as usize].len() as i32
        {
            None
        } else {
            Some(self.field[row as usize][col as usize].value())
        }
    }

    fn count_on(&self) -> usize {
        self.field
            .iter()
            .map(|line| line.iter().map(|x| x.value()).sum::<usize>())
            .sum()
    }

    fn light_up_corners(&mut self) {
        for (y, x) in [
            (0, 0),
            (0, self.field[0].len() - 1),
            (self.field.len() - 1, 0),
            (self.field.len() - 1, self.field[0].len() - 1),
        ] {
            self.field[y][x] = Cell::Enabled;
        }
    }
}

pub fn parse_input(input: &str) -> AocResult<Life> {
    Ok(Life {
        field: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Ok(Cell::Disabled),
                        '#' => Ok(Cell::Enabled),
                        _ => Err(aoc_error!("Unexpected character: {ch}")),
                    })
                    .collect::<Result<Vec<Cell>>>()
            })
            .collect::<Result<_>>()?,
    })
}

fn task(life: &Life, steps: usize, light_corners: bool) -> usize {
    let mut life = life.clone();
    for _ in 0..steps {
        life.step(light_corners);
    }
    life.count_on()
}

pub fn task1(life: &Life) -> AocResult<usize> {
    Ok(task(life, STEPS, false))
}

pub fn task2(life: &Life) -> AocResult<usize> {
    Ok(task(life, STEPS, true))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(
            task(
                &parse_input(
                    ".#.#.#
...##.
#....#
..#...
#.#..#
####.."
                )
                .unwrap(),
                5,
                false
            ),
            4
        );
    }

    #[test]
    fn test_task2() {
        assert_eq!(
            task(
                &parse_input(
                    "##.#.#
...##.
#....#
..#...
#.#..#
####.#"
                )
                .unwrap(),
                5,
                true
            ),
            17
        );
    }
}
