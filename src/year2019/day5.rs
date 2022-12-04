use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn task1(code: &[isize]) -> isize {
    let mut computer = Computer::new(code);
    computer.write(1);
    computer.run();
    loop {
        let result = computer.read().unwrap();
        if result != 0 {
            return result;
        }
    }
}

pub fn task2(code: &[isize]) -> isize {
    let mut computer = Computer::new(code);
    computer.write(5);
    computer.run();
    computer.read().unwrap()
}
