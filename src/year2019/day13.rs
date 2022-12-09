use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    Computer::prepare_code(input)
}

use std::collections::HashMap;

pub fn task1(code: &[isize]) -> usize {
    let mut computer = Computer::new(code);
    computer.run();
    let mut grid = HashMap::new();
    while let (Some(x), Some(y), Some(t)) =
        (computer.read(), computer.read(), computer.read())
    {
        grid.insert((x, y), t);
    }
    grid.values().filter(|&&v| v == 2).count()
}

pub fn task2(code: &[isize]) -> usize {
    unimplemented!();
}
