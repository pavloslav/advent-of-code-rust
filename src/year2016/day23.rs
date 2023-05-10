use crate::*;
use super::computer::{Instruction, Computer};

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|line|line.parse()).collect()
}

pub fn task1(input: &[Instruction]) -> Result<isize> {
    let mut computer = Computer::new(input);
    computer.registers[0] = 7;
    computer.run()
}

pub fn task2(input: &[Instruction]) -> Result<isize> {
    let mut computer = Computer::new(input);
    computer.registers[0] = 12;
    computer.run()
}
