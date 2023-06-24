use crate::*;

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
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok((x, y)) = scan_fmt::scan_fmt!(
            s,
            "swap position {} with position {}",
            usize,
            usize
        ) {
            Ok(Command::SwapPosition(x, y))
        } else if let Ok((x, y)) =
            scan_fmt::scan_fmt!(s, "swap letter {} with letter {}", char, char)
        {
            Ok(Command::SwapLetter(x as u8, y as u8))
        } else if let Ok(r) =
            scan_fmt::scan_fmt!(s, "rotate left {} steps", usize)
        {
            Ok(Command::RotateLeft(r))
        } else if let Ok(r) =
            scan_fmt::scan_fmt!(s, "rotate right {} steps", usize)
        {
            Ok(Command::RotateRight(r))
        } else if let Ok(l) = scan_fmt::scan_fmt!(
            s,
            "rotate based on position of letter {}",
            char
        ) {
            Ok(Command::RotateBased(l as u8))
        } else if let Ok((x, y)) = scan_fmt::scan_fmt!(
            s,
            "reverse positions {} through {}",
            usize,
            usize
        ) {
            Ok(Command::Reverse(x, y))
        } else if let Ok((x, y)) = scan_fmt::scan_fmt!(
            s,
            "move position {} to position {}",
            usize,
            usize
        ) {
            Ok(Command::Move(x, y))
        } else {
            Err(aoc_error!("Can't parse '{s}' into command"))
        }
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Command>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Command]) -> Result<String> {
    let mut password: VecDeque<_> = PASSWORD.bytes().collect();

    for command in input {
        match command {
            Command::SwapPosition(x, y) => password.swap(*x, *y),
            Command::SwapLetter(x, y) => {
                let x =
                    password.iter().position(|c| c == x).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *x as char)
                    })?;
                let y =
                    password.iter().position(|c| c == y).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *y as char)
                    })?;
                password.swap(x, y);
            }
            Command::RotateLeft(r) => password.rotate_left(*r),
            Command::RotateRight(r) => password.rotate_right(*r),
            Command::RotateBased(b) => {
                let mut b =
                    password.iter().position(|c| c == b).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *b as char)
                    })?;
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
                    .ok_or_else(|| aoc_error!("Index {x} out of bounds"))?;
                password.insert(*y, letter);
            }
        }
    }

    Ok(password.iter().map(|&b| b as char).collect())
}

pub fn task2(input: &[Command]) -> Result<String> {
    let mut password: VecDeque<_> = SCRAMBLED.bytes().collect();

    for command in input.iter().rev() {
        match command {
            Command::SwapPosition(x, y) => password.swap(*x, *y),
            Command::SwapLetter(x, y) => {
                let x =
                    password.iter().position(|c| c == x).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *x as char)
                    })?;
                let y =
                    password.iter().position(|c| c == y).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *y as char)
                    })?;
                password.swap(x, y);
            }
            Command::RotateLeft(r) => password.rotate_right(*r),
            Command::RotateRight(r) => password.rotate_left(*r),
            Command::RotateBased(b) => {
                let mut b =
                    password.iter().position(|c| c == b).ok_or_else(|| {
                        aoc_error!("No letter {} in password", *b as char)
                    })?;
                b = match b {
                    0 => 7,
                    1 => 7,
                    2 => 2,
                    3 => 6,
                    4 => 1,
                    5 => 5,
                    6 => 0,
                    7 => 4,
                    other => {
                        return Err(aoc_error!(
                            "Letter position {other}! How?"
                        ))
                    }
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
                    .ok_or_else(|| aoc_error!("Index {x} out of bounds"))?;
                password.insert(*x, letter);
            }
        }
    }

    Ok(password.iter().map(|&b| b as char).collect())
}
