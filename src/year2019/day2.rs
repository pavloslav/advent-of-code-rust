use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    Computer::prepare_code(input)
}

pub fn task1(input: &[isize]) -> isize {
    let mut computer = Computer::new(input);
    *computer.memory.entry(1).or_default() = 12;
    *computer.memory.entry(2).or_default() = 2;
    computer.run();
    computer.memory[&0]
}

pub fn task2(input: &[isize]) -> isize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new(input);
            *computer.memory.entry(1).or_default() = noun;
            *computer.memory.entry(2).or_default() = verb;
            computer.run();
            if computer.memory[&0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}
