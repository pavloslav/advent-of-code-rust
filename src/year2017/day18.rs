use std::cell::RefCell;
use std::rc::{Rc, Weak};

type RegName = char;
type RegValue = i64;
type Registers = std::collections::HashMap<RegName, RegValue>;

#[derive(Clone, Copy)]
pub enum Operand {
    Reg(RegName),
    Val(RegValue),
}

impl std::str::FromStr for Operand {
    type Err = ();
    fn from_str(s: &str) -> Result<Operand, ()> {
        if let Ok(value) = s.parse() {
            Ok(Operand::Val(value))
        } else if s.len() == 1 {
            Ok(Operand::Reg(s.chars().next().unwrap()))
        } else {
            Err(())
        }
    }
}

impl Operand {
    fn get(&self, regs: &Registers) -> RegValue {
        match self {
            Operand::Reg(reg) => *regs.get(reg).unwrap_or(&0),
            Operand::Val(val) => *val,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Snd(Operand),
    Set(RegName, Operand),
    Add(RegName, Operand),
    Mul(RegName, Operand),
    Mod(RegName, Operand),
    Rcv(RegName),
    Jgz(Operand, Operand),
}

impl std::str::FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Instruction, ()> {
        let mut s = s.split_whitespace();
        match s.next() {
            Some("snd") => Ok(Instruction::Snd(s.next().ok_or(())?.parse()?)),
            Some("set") => Ok(Instruction::Set(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("add") => Ok(Instruction::Add(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("mul") => Ok(Instruction::Mul(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("mod") => Ok(Instruction::Mod(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("rcv") => Ok(Instruction::Rcv(
                s.next().and_then(|s| s.chars().next()).ok_or(())?,
            )),
            Some("jgz") => Ok(Instruction::Jgz(
                s.next().ok_or(())?.parse()?,
                s.next().ok_or(())?.parse()?,
            )),
            _ => Err(()),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

type Buffer = std::collections::VecDeque<RegValue>;

enum ComputerKind {
    SoundRecover,
    SendRecieve,
}

struct Computer {
    program: Vec<Instruction>,
    registers: Registers,
    ip: RegValue,
    input: std::rc::Weak<RefCell<Buffer>>,
    output: Rc<RefCell<Buffer>>,
    output_counter: usize,
    kind: ComputerKind,
}

impl Computer {
    fn step(&mut self) -> bool {
        if !(0..self.program.len() as RegValue).contains(&self.ip) {
            panic!();
        }
        match self.program[self.ip as usize] {
            Instruction::Snd(val) => {
                self.output.borrow_mut().push_back(val.get(&self.registers));
                self.output_counter += 1;
            }
            Instruction::Set(tgt, val) => {
                self.registers.insert(tgt, val.get(&self.registers));
            }
            Instruction::Add(tgt, val) => {
                *self.registers.entry(tgt).or_insert(0) +=
                    val.get(&self.registers)
            }
            Instruction::Mul(tgt, val) => {
                *self.registers.entry(tgt).or_insert(0) *=
                    val.get(&self.registers)
            }
            Instruction::Mod(tgt, val) => {
                *self.registers.entry(tgt).or_insert(0) %=
                    val.get(&self.registers)
            }
            Instruction::Rcv(reg) => match self.kind {
                ComputerKind::SoundRecover => {
                    if let Some(&x) = self.registers.get(&reg) {
                        if x != 0 {
                            return false;
                        }
                    }
                }
                ComputerKind::SendRecieve => {
                    if let Some(data) =
                        self.input.upgrade().and_then(|strong| {
                            strong
                                .try_borrow_mut()
                                .ok()
                                .and_then(|mut buffer| buffer.pop_front())
                        })
                    {
                        self.registers.insert(reg, data);
                    } else {
                        return false;
                    }
                }
            },
            Instruction::Jgz(check, offset) => {
                if check.get(&self.registers) > 0 {
                    self.ip += offset.get(&self.registers);
                    return true;
                }
            }
        }
        self.ip += 1;
        true
    }
    fn last_sound(&self) -> Option<RegValue> {
        self.output.borrow_mut().back().copied()
    }
    fn new(program: &[Instruction], kind: ComputerKind) -> Computer {
        Computer {
            program: program.to_vec(),
            registers: Registers::new(),
            ip: 0,
            input: Weak::new(),
            output: Rc::new(RefCell::new(Buffer::new())),
            kind,
            output_counter: 0,
        }
    }
}

pub fn task1(input: &[Instruction]) -> RegValue {
    let mut computer = Computer::new(input, ComputerKind::SoundRecover);
    while computer.step() {}
    computer.last_sound().unwrap()
}

pub fn task2(input: &[Instruction]) -> usize {
    let mut computer0 = Computer::new(input, ComputerKind::SendRecieve);
    computer0.registers.insert('p', 0);
    let mut computer1 = Computer::new(input, ComputerKind::SendRecieve);
    computer1.registers.insert('p', 1);

    computer0.input = Rc::downgrade(&computer1.output);
    computer1.input = Rc::downgrade(&computer0.output);

    while computer0.step() || computer1.step() {}
    computer1.output_counter
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
        assert_eq!(task1(&parse_input(src)), 4);
    }
}
