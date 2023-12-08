use anyhow::Context;
use std::collections::VecDeque;

const PASSWORD: &str = "abcdefgh";
const SCRAMBLED: &str = "fbgdceah";

#[derive(Debug)]
pub enum Command {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok((x, y)) = prse::try_parse!(s, "swap position {} with position {}") {
            Ok(Command::SwapPosition(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "swap letter {} with letter {}") {
            let (x, y): (char, char) = (x, y); //prse::try_parse hint
            Ok(Command::SwapLetter(x as u8, y as u8))
        } else if let Ok(r) = prse::try_parse!(s, "rotate left {} steps") {
            Ok(Command::RotateLeft(r))
        } else if let Ok(r) = prse::try_parse!(s, "rotate right {} steps") {
            Ok(Command::RotateRight(r))
        } else if let Ok(l) = prse::try_parse!(s, "rotate based on position of letter {}") {
            let l: char = l; //prse::try_parse hint
            Ok(Command::RotateBased(l as u8))
        } else if let Ok((x, y)) = prse::try_parse!(s, "reverse positions {} through {}") {
            Ok(Command::Reverse(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "move position {} to position {}") {
            Ok(Command::Move(x, y))
        } else {
            Err(anyhow::anyhow!("Can't parse '{s}' into command"))
        }
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Command>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Command]) -> anyhow::Result<String> {
    let mut password: VecDeque<_> = PASSWORD.bytes().collect();

    for command in input {
        match command {
            Command::SwapPosition(x, y) => password.swap(*x, *y),
            Command::SwapLetter(x, y) => {
                let x = password
                    .iter()
                    .position(|c| c == x)
                    .with_context(|| format!("No letter {} in password", *x as char))?;
                let y = password
                    .iter()
                    .position(|c| c == y)
                    .with_context(|| format!("No letter {} in password", *y as char))?;
                password.swap(x, y);
            }
            Command::RotateLeft(r) => password.rotate_left(*r),
            Command::RotateRight(r) => password.rotate_right(*r),
            Command::RotateBased(b) => {
                let mut b = password
                    .iter()
                    .position(|c| c == b)
                    .with_context(|| format!("No letter {} in password", *b as char))?;
                b += if b >= 4 { 2 } else { 1 };
                password.rotate_right(b % password.len());
            }
            Command::Reverse(x, y) => {
                for i in 0..(y - x + 1) / 2 {
                    password.swap(x + i, y - i);
                }
            }
            Command::Move(x, y) => {
                let letter = password
                    .remove(*x)
                    .with_context(|| format!("Index {x} out of bounds"))?;
                password.insert(*y, letter);
            }
        }
    }

    Ok(password.iter().map(|&b| b as char).collect())
}

pub fn task2(input: &[Command]) -> anyhow::Result<String> {
    let mut password: VecDeque<_> = SCRAMBLED.bytes().collect();

    for command in input.iter().rev() {
        match command {
            Command::SwapPosition(x, y) => password.swap(*x, *y),
            Command::SwapLetter(x, y) => {
                let x = password
                    .iter()
                    .position(|c| c == x)
                    .with_context(|| format!("No letter {} in password", *x as char))?;
                let y = password
                    .iter()
                    .position(|c| c == y)
                    .with_context(|| format!("No letter {} in password", *y as char))?;
                password.swap(x, y);
            }
            Command::RotateLeft(r) => password.rotate_right(*r),
            Command::RotateRight(r) => password.rotate_left(*r),
            Command::RotateBased(b) => {
                let mut b = password
                    .iter()
                    .position(|c| c == b)
                    .with_context(|| format!("No letter {} in password", *b as char))?;
                b = match b {
                    0 => 7,
                    1 => 7,
                    2 => 2,
                    3 => 6,
                    4 => 1,
                    5 => 5,
                    6 => 0,
                    7 => 4,
                    other => anyhow::bail!("Letter position {other}! How?"),
                };
                password.rotate_right(b);
            }
            Command::Reverse(x, y) => {
                for i in 0..(y - x + 1) / 2 {
                    password.swap(x + i, y - i);
                }
            }
            Command::Move(x, y) => {
                let letter = password
                    .remove(*y)
                    .with_context(|| format!("Index {x} out of bounds"))?;
                password.insert(*x, letter);
            }
        }
    }

    Ok(password.iter().map(|&b| b as char).collect())
}
