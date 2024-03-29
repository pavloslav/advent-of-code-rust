#[derive(Debug, Clone, Copy)]
struct Direction {
    dir: i8,
}

impl Direction {
    fn rotate(&mut self, rot: i8) {
        self.dir = (self.dir + rot).rem_euclid(4);
    }
    const NORTH: Direction = Direction { dir: 0 };
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
    fn walk(&mut self, dir: Direction, len: i32) {
        match dir.dir {
            0 => self.y += len,
            1 => self.x += len,
            2 => self.y -= len,
            3 => self.x -= len,
            other => unreachable!("Direction can't be {other}!"),
        }
    }
}

pub struct Command {
    rot: i8,
    len: i32,
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> anyhow::Result<Command> {
        let rot = match input.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => anyhow::bail!("wrong rotation"),
        };
        Ok(Command {
            rot,
            len: input[1..].parse()?,
        })
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    input.split(", ").map(|step| step.parse()).collect()
}

pub fn task1(way: &[Command]) -> anyhow::Result<i32> {
    let mut position = Pos { x: 0, y: 0 };
    let mut dir = Direction::NORTH;
    for step in way {
        dir.rotate(step.rot);
        position.walk(dir, step.len);
    }
    Ok(position.manhattan_distance())
}

use std::collections::HashSet;

pub fn task2(way: &[Command]) -> anyhow::Result<i32> {
    let mut position = Pos { x: 0, y: 0 };
    let mut dir = Direction::NORTH;
    let mut visited: HashSet<Pos> = HashSet::new();
    for step in way {
        dir.rotate(step.rot);
        for _ in 0..step.len {
            if visited.contains(&position) {
                return Ok(position.manhattan_distance());
            }
            visited.insert(position);
            position.walk(dir, 1);
        }
    }
    Ok(position.manhattan_distance())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_path_length() {
        assert_eq!(task1(&parse_input("R2, L3").unwrap()).unwrap(), 5);
        assert_eq!(task1(&parse_input("R2, R2, R2").unwrap()).unwrap(), 2);
        assert_eq!(task1(&parse_input("R5, L5, R5, R3").unwrap()).unwrap(), 12);
    }

    #[test]
    fn test_path_to_intersection() {
        assert_eq!(task2(&parse_input("R8, R4, R4, R8").unwrap()).unwrap(), 4);
    }
}
