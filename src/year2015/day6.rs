//#[derive(Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: &str, y: &str) -> Self {
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

enum Command {
    On, 
    Off, 
    Toggle,
}

pub struct Instruction {
    command: Command,
    top_left: Point,
    bottom_right: Point,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    lazy_static::lazy_static! {
        static ref INPUT_REGEX: regex::Regex = regex::Regex::new(r"(?P<op>turn on|turn off|toggle) (?P<left>\d+),(?P<top>\d+) through (?P<right>\d+),(?P<bottom>\d+)").unwrap();
    }
    let mut result = Vec::new();
    for line in input.lines() {
        if let Some(captures) = INPUT_REGEX.captures(line) {
            if let Some(op) = captures.name("op") {
                if let Some(left) = captures.name("left") {
                    if let Some(top) = captures.name("top") {
                        if let Some(right) = captures.name("right") {
                            if let Some(bottom) = captures.name("bottom") {
                                let top_left = Point::new(left.as_str(), top.as_str());
                                let bottom_right = Point::new(right.as_str(), bottom.as_str());
                                let command = match op.as_str() {
                                    "turn on" => Command::On,
                                    "turn off" => Command::Off,
                                    "toggle" => Command::Toggle,
                                    _ => panic!("Wrong command!"),
                                }; 
                                result.push(Instruction { command, top_left, bottom_right });
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn task1(instructions: &Vec<Instruction>) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];
    for instr in instructions {
        for y in instr.top_left.y..instr.bottom_right.y+1 {
            for x in instr.top_left.x..instr.bottom_right.x+1 {
                lights[y][x] = match instr.command {
                    Command::On => true,
                    Command::Off => false,
                    Command::Toggle => !lights[y][x],
                }
            }
        }
    }
    lights.iter().map(|line|line.iter().filter(|&&x|x).count()).sum()
}

pub fn task2(instructions: &Vec<Instruction>) -> usize {
    let mut lights = vec![vec![0; 1000]; 1000];
    for instr in instructions {
        for y in instr.top_left.y..instr.bottom_right.y+1 {
            for x in instr.top_left.x..instr.bottom_right.x+1 {
                lights[y][x] = match instr.command {
                    Command::On => lights[y][x] + 1,
                    Command::Off => if lights[y][x] == 0 {0} else {lights[y][x]- 1},
                    Command::Toggle => lights[y][x] + 2,
                }
            }
        }
    }
    lights.iter().map(|line|line.iter().sum::<usize>()).sum()
}
