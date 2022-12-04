use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn task1(input: &[isize]) -> isize {
    let mut computer = Computer::new(input);
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run();
    computer.memory[0]
}

pub fn task2(input: &[isize]) -> isize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new(input);
            computer.memory[1] = noun;
            computer.memory[2] = verb;
            computer.run();
            if computer.memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}
