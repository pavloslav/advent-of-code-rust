use super::computer::Computer;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<isize>> {
    Computer::prepare_code(input)
}

pub fn task1(input: &[isize]) -> anyhow::Result<isize> {
    let mut computer = Computer::new(input);
    computer.memory.insert(1, 12);
    computer.memory.insert(2, 2);
    computer.run()?;
    Ok(computer.memory[&0])
}

const NEEDED: isize = 19690720;

pub fn task2(input: &[isize]) -> anyhow::Result<isize> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut computer = Computer::new(input);
            computer.memory.insert(1, noun);
            computer.memory.insert(2, verb);
            computer.run()?;
            if computer.memory[&0] == NEEDED {
                return Ok(100 * noun + verb);
            }
        }
    }
    anyhow::bail!("Answer not found!")
}
