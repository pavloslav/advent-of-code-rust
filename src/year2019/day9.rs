use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    Computer::prepare_code(input)
}

pub fn task1(code: &[isize]) -> isize {
    let mut computer = Computer::new(code);
    computer.write(1);
    computer.run();
    computer.read().unwrap()
}

pub fn task2(code: &[isize]) -> isize {
    let mut computer = Computer::new(code);
    computer.write(2);
    computer.run();
    computer.read().unwrap()
}
