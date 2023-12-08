use super::computer::Computer;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<isize>> {
    Computer::prepare_code(input)
}

pub fn task1(code: &[isize]) -> anyhow::Result<isize> {
    let mut computer = Computer::new(code);
    computer.write(1);
    computer.run()?.read()
}

pub fn task2(code: &[isize]) -> anyhow::Result<isize> {
    let mut computer = Computer::new(code);
    computer.write(2);
    computer.run()?.read()
}
