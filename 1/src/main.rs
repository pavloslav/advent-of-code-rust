extern crate aoc;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West  => Direction::South,
            Direction::South => Direction::East,
            Direction::East  => Direction::North
        }
    }
    fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West  => Direction::North,
            Direction::South => Direction::West,
            Direction::East  => Direction::South
        }
    }    
}
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Pos {
    x:i32,
    y:i32,
}

impl Pos{
    fn manhattan_distance(&self) -> i32 {
        self.x.abs()+self.y.abs()
    }
}

fn path_length(way:&str)->i32{
    let mut position = Pos{x:0,y:0};
    let mut dir = Direction::North;
    for step in way.split(", ") {
        if cfg!(debug_assertions) {
            print!("Got {}", step);
        }
        let d = step.chars().next().unwrap();
        let l = step[1..].parse::<i32>().unwrap();
        match d {
            'R' =>{dir=dir.right()},
            'L' =>{dir=dir.left() },
            _   => {panic!("wrong input!")}
        }
        match dir {
            Direction::North => position.x += l,
            Direction::West => position.y -= l,
            Direction::South => position.x -= l,
            Direction::East => position.y += l,
        }
        if cfg!(debug_assertions) {
            println!(", stepped {} to {:?}, now in {:?}, distance is {}", l, dir, position, position.manhattan_distance() );
        }
    } 
    position.manhattan_distance()
}

use std::collections::HashSet;

fn path_to_intersection(way:&str)->i32{
    let mut position = Pos{x:0,y:0};
    let mut dir = Direction::North;
    let mut visited: HashSet<Pos> = HashSet::new();
    for step in way.split(", ") {
        if cfg!(debug_assertions) {
            print!("Got {}", step);
        }
        let d = step.chars().next().unwrap();
        let l = step[1..].parse::<i32>().unwrap();
        match d {
            'R' =>{dir=dir.right()},
            'L' =>{dir=dir.left() },
            _   => {panic!("wrong input!")}
        }

        for _ in 0..l {
            if cfg!(debug_assertions) {
                println!(" stepping to {:?}",position);
            }
            if visited.contains(&position) {
                return position.manhattan_distance();
            }
            visited.insert(position);
            match dir {
                Direction::North => position.x += 1,
                Direction::West  => position.y -= 1,
                Direction::South => position.x -= 1,
                Direction::East  => position.y += 1,
            }
        }
        if cfg!(debug_assertions) {
            println!(", stepped {} to {:?}, now in {:?}, distance is {}", l, dir, position, position.manhattan_distance() );
        }
    } 
    position.manhattan_distance()    
}

fn main() {
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    println!("Answer is {}", path_length(&input));
    println!("Second answer is {}", path_to_intersection(&input));
}

#[test]
fn test_path_length() {
    assert_eq!(path_length(&"R2, L3".to_string()),5);
    assert_eq!(path_length(&"R2, R2, R2".to_string()),2);
    assert_eq!(path_length(&"R5, L5, R5, R3".to_string()),12);
}

#[test]
fn test_path_to_intersection() {
    assert_eq!(path_to_intersection(&"R8, R4, R4, R8".to_string()),4);
}