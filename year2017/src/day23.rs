use super::computer::*;
use crate::*;

pub fn parse_input(input: &str) -> AocResult<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Instruction]) -> AocResult<usize> {
    let mut computer = Computer::new(input, ComputerKind::SoundRecover);
    while let Ok(step) = computer.step() {
        if !step {
            break;
        }
    }
    Ok(computer.mul_counter)
}

pub fn task2(input: &[Instruction]) -> AocResult<RegValue> {
    let mut computer = Computer::new(input, ComputerKind::SoundRecover);
    computer.set_register('a', 1);
    for _ in 0..9 {
        computer.step()?;
    }
    let start = computer.get_register('b')?;
    let end = computer.get_register('c')?;
    if let Instruction::Sub(_, Operand::Val(step)) = input[30] {
        let step = -step;
        let diff = (end - start) / step;
        let mut h = 0;
        for i in 0..=diff {
            let num = start + i * step;
            for d in 2..num {
                if num % d == 0 {
                    h += 1;
                    break;
                }
            }
        }
        Ok(h)
    } else {
        Err(aoc_error!("Step was on line 30"))
    }
}
