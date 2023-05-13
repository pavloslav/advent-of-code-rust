use super::computer::*;
use crate::*;
use std::rc::Rc;

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn task1(input: &[Instruction]) -> Result<RegValue> {
    let mut computer = Computer::new(input, ComputerKind::SoundRecover);
    while computer.step()? {}
    computer.last_sound()
}

pub fn task2(input: &[Instruction]) -> Result<usize> {
    let mut computer0 = Computer::new(input, ComputerKind::SendRecieve);
    computer0.set_register('p', 0);
    let mut computer1 = Computer::new(input, ComputerKind::SendRecieve);
    computer1.set_register('p', 1);

    computer0.input = Rc::downgrade(&computer1.output);
    computer1.input = Rc::downgrade(&computer0.output);

    while computer0.step()? || computer1.step()? {}
    Ok(computer1.output_counter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let src = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(task1(&parse_input(src).unwrap()).unwrap(), 4);
    }
}
