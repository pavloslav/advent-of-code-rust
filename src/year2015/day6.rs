use super::aoc::*;

type Data = Vec<Instruction>;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn try_new(x: &str, y: &str) -> Result<Self> {
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
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

use once_cell::sync::Lazy;

pub fn parse_input(input: &str) -> Result<Data> {
    static INPUT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r"(?P<op>turn on|turn off|toggle) (?P<left>\d+),(?P<top>\d+) through (?P<right>\d+),(?P<bottom>\d+)").unwrap()
    });
    let mut result = Vec::new();
    for line in input.lines() {
        if let Some(captures) = INPUT_REGEX.captures(line) {
            if let (
                Some(op),
                Some(left),
                Some(top),
                Some(right),
                Some(bottom),
            ) = (
                captures.name("op"),
                captures.name("left"),
                captures.name("top"),
                captures.name("right"),
                captures.name("bottom"),
            ) {
                let top_left = Point::try_new(left.as_str(), top.as_str())?;
                let bottom_right =
                    Point::try_new(right.as_str(), bottom.as_str())?;
                let command = match op.as_str() {
                    "turn on" => Command::On,
                    "turn off" => Command::Off,
                    "toggle" => Command::Toggle,
                    other => {
                        return Err(TaskError(format!(
                            "Wrong command: '{other}'"
                        )))
                    }
                };
                result.push(Instruction {
                    command,
                    top_left,
                    bottom_right,
                });
            }
        } else {
            return Err(TaskError(format!("Wrong input line: '{line}'")));
        }
    }
    Ok(result)
}

pub fn task1(instructions: &Data) -> Result<usize> {
    let mut lights = vec![vec![false; 1000]; 1000];
    for instr in instructions {
        for line in &mut lights[instr.top_left.y..instr.bottom_right.y + 1] {
            for light in &mut line[instr.top_left.x..instr.bottom_right.x + 1] {
                *light = match instr.command {
                    Command::On => true,
                    Command::Off => false,
                    Command::Toggle => !*light,
                }
            }
        }
    }
    Ok(lights
        .iter()
        .map(|line| line.iter().filter(|&&x| x).count())
        .sum())
}

pub fn task2(instructions: &Data) -> Result<usize> {
    let mut lights = vec![vec![0usize; 1000]; 1000];
    for instr in instructions {
        for line in &mut lights[instr.top_left.y..instr.bottom_right.y + 1] {
            for light in &mut line[instr.top_left.x..instr.bottom_right.x + 1] {
                *light = match instr.command {
                    Command::On => *light + 1,
                    Command::Off => light.saturating_sub(1),
                    Command::Toggle => *light + 2,
                }
            }
        }
    }
    Ok(lights.iter().map(|line| line.iter().sum::<usize>()).sum())
}
