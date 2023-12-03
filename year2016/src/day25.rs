use super::computer::{Computer, Instruction};
use crate::*;
use std::collections::HashSet;

pub fn parse_input(input: &str) -> AocResult<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Instruction]) -> AocResult<isize> {
    for start in 0.. {
        let mut computer = Computer::new(input);
        let mut visited = HashSet::new();
        computer.registers[0] = start;
        let mut pattern = 0;
        while let Some(out) = computer.run_to_output()? {
            if out != pattern {
                break;
            }
            pattern = 1 - pattern;
            let state = (out, computer.get_state());
            if !visited.insert(state) {
                return Ok(start);
            }
        }
    }
    Err(aoc_error!("unreachable!"))
}

pub fn task2(input: &[Instruction]) -> AocResult<isize> {
    let mut computer = Computer::new(input);
    computer.registers[0] = 12;
    computer.run()
}
