use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn task1(code: &[isize]) -> isize {
    let mut computer = Computer::new_with_input(code, &[1]);
    while !computer.is_halted() {
        computer.step();
    }
    *computer.output.last().unwrap()
}

pub fn task2(code: &[isize]) -> isize {
    let mut computer = Computer::new_with_input(code, &[5]);
    while !computer.is_halted() {
        computer.step();
    }
    *computer.output.last().unwrap()
}
