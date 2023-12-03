use super::computer::Computer;
use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<isize>> {
    Computer::prepare_code(input)
}

pub fn task1(code: &[isize]) -> AocResult<isize> {
    let mut computer = Computer::new(code);
    computer.write(1);
    computer.run()?.read()
}

pub fn task2(code: &[isize]) -> AocResult<isize> {
    let mut computer = Computer::new(code);
    computer.write(2);
    computer.run()?.read()
}
