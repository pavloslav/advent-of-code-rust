#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
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
}

pub fn task1(way: &str) -> i32 {
    let mut position = Pos { x: 0, y: 0 };
    let mut dir = Direction::North;
    for step in way.split(", ") {
        let d = step.chars().next().unwrap();
        let l = step[1..].trim().parse::<i32>().unwrap();
        dir = match d {
            'R' => dir.right(),
            'L' => dir.left(),
            _ => panic!("wrong input!"),
        };
        match dir {
            Direction::North => position.x += l,
            Direction::West => position.y -= l,
            Direction::South => position.x -= l,
            Direction::East => position.y += l,
        }
    }
    position.manhattan_distance()
}

use std::collections::HashSet;

pub fn task2(way: &str) -> i32 {
    let mut position = Pos { x: 0, y: 0 };
    let mut dir = Direction::North;
    let mut visited: HashSet<Pos> = HashSet::new();
    for step in way.split(", ") {
        let d = step.chars().next().unwrap();
        let l = step[1..].parse::<i32>().unwrap();
        match d {
            'R' => dir = dir.right(),
            'L' => dir = dir.left(),
            _ => {
                panic!("wrong input!")
            }
        }

        for _ in 0..l {
            if visited.contains(&position) {
                return position.manhattan_distance();
            }
            visited.insert(position);
            match dir {
                Direction::North => position.x += 1,
                Direction::West => position.y -= 1,
                Direction::South => position.x -= 1,
                Direction::East => position.y += 1,
            }
        }
    }
    position.manhattan_distance()
}

pub fn parse_input(input: &str) -> &str {
    input
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_path_length() {
        assert_eq!(task1(&"R2, L3".to_string()), 5);
        assert_eq!(task1(&"R2, R2, R2".to_string()), 2);
        assert_eq!(task1(&"R5, L5, R5, R3".to_string()), 12);
    }

    #[test]
    fn test_path_to_intersection() {
        assert_eq!(task2(&"R8, R4, R4, R8".to_string()), 4);
    }
}
