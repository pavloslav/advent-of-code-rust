use super::super::common::Error::TaskError;
use super::super::common::Result;
use super::computer::Computer;

pub fn parse_input(input: &str) -> Result<Vec<isize>> {
    Computer::prepare_code(input)
}

pub fn task1(code: &[isize]) -> Result<isize> {
    let mut computer = Computer::new(code);
    computer.write(1);
    computer.run()?;
    loop {
        let result = computer.read().ok_or_else(|| {
            TaskError("Computer output is empty!".to_string())
        })?;
        if result != 0 {
            return Ok(result);
        }
    }
}

pub fn task2(code: &[isize]) -> Result<isize> {
    let mut computer = Computer::new(code);
    computer.write(5);
    computer.run()?;
    computer
        .read()
        .ok_or_else(|| TaskError("Computer output is empty!".to_string()))
}
