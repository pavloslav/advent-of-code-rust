use anyhow::Context;

#[derive(Debug, Clone)]
pub enum Operation {
    Jmp(i64),
    Acc(i64),
    Nop(i64),
}

type Program = Vec<Operation>;

struct Computer {
    instruction: usize,
    accumulator: i64,
    program: Program,
}

impl std::str::FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(line: &str) -> anyhow::Result<Operation> {
        let (operation, value) = prse::try_parse!(line, "{} {}")?;
        match operation {
            "nop" => Ok(Operation::Nop(value)),
            "acc" => Ok(Operation::Acc(value)),
            "jmp" => Ok(Operation::Jmp(value)),
            _ => anyhow::bail!("Can't parse operation '{line}'"),
        }
    }
}

impl Computer {
    fn with_program(program: Program) -> Computer {
        Computer {
            instruction: 0,
            accumulator: 0,
            program,
        }
    }
    fn tick(&mut self) -> anyhow::Result<()> {
        match self.program[self.instruction] {
            Operation::Jmp(offset) => {
                self.instruction = if offset.is_negative() {
                    self.instruction.checked_sub(offset.unsigned_abs() as usize)
                } else {
                    self.instruction.checked_add(offset as usize)
                }
                .context("Wrong instruction address")?;
            }
            Operation::Acc(increment) => {
                self.accumulator += increment;
                self.instruction += 1;
            }
            Operation::Nop(_) => self.instruction += 1,
        }
        Ok(())
    }
    fn exited(&self) -> bool {
        self.instruction == self.program.len()
    }
    fn not_working(&self) -> bool {
        self.instruction >= self.program.len()
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Program> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(program: &Program) -> anyhow::Result<i64> {
    let mut computer = Computer::with_program(program.clone());
    let mut visited = vec![false; computer.program.len()];
    while !visited[computer.instruction] {
        visited[computer.instruction] = true;
        computer.tick()?;
    }
    Ok(computer.accumulator)
}

pub fn task2(program: &Program) -> anyhow::Result<i64> {
    let mut computer = Computer::with_program(program.clone());
    for i in 0..computer.program.len() {
        let mut visited = vec![false; computer.program.len()];
        let save = computer.program[i].clone();
        match computer.program[i] {
            Operation::Jmp(1) => continue,
            Operation::Nop(1) => continue,
            Operation::Jmp(offset) => computer.program[i] = Operation::Nop(offset),
            Operation::Nop(offset) => computer.program[i] = Operation::Jmp(offset),
            _ => continue,
        }
        computer.instruction = 0;
        computer.accumulator = 0;
        while !computer.not_working() && !visited[computer.instruction] {
            visited[computer.instruction] = true;
            computer.tick()?;
        }
        if computer.exited() {
            break;
        }
        computer.program[i] = save;
    }
    if computer.exited() {
        Ok(computer.accumulator)
    } else {
        anyhow::bail!("Failed to calculate")
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, task2};
    #[test]
    fn test_task2() {
        let input1 = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(task2(&parse_input(input1).unwrap()).unwrap(), 8);
    }
}
