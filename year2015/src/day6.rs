use crate::*;

struct Point {
    x: usize,
    y: usize,
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

impl std::str::FromStr for Instruction {
    type Err = AocError;
    fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("turn ") {
            s = &s[5..];
        }
        let (op, left, top, right, bottom): (&str, usize, usize, usize, usize) =
            prse::try_parse!(s, "{} {},{} through {},{}")?;
        let command = match op {
            "on" => Command::On,
            "off" => Command::Off,
            "toggle" => Command::Toggle,
            other => return Err(aoc_error!("Wrong command: '{other}'")),
        };
        Ok(Instruction {
            command,
            top_left: Point { x: left, y: top },
            bottom_right: Point {
                x: right,
                y: bottom,
            },
        })
    }
}

pub fn parse_input(input: &str) -> AocResult<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(instructions: &[Instruction]) -> AocResult<usize> {
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

pub fn task2(instructions: &[Instruction]) -> AocResult<usize> {
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
