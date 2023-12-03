use super::computer::{Computer, Instruction};
use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Instruction]) -> AocResult<isize> {
    let mut computer = Computer::new(input);
    computer.run()
}

pub fn task2(input: &[Instruction]) -> AocResult<isize> {
    let mut computer = Computer::new(input);
    computer.registers[2] = 1;
    computer.run()
}
