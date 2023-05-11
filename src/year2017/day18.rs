use crate::*;

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
    type Err = Error;
    fn from_str(s: &str) -> Result<Operand> {
        if let Ok(value) = s.parse() {
            Ok(Operand::Val(value))
        } else {
            match s.chars().next() {
                Some(c) if s.len() == 1 => Ok(Operand::Reg(c)),
                _ => Err(aoc_error!("Can't parse operand '{s}'")),
            }
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
    type Err = Error;
    fn from_str(s: &str) -> Result<Instruction> {
        use once_cell::sync::Lazy;
        static INPUT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
            regex::Regex::new(
                r"^(snd (?P<snd>.+))|(set (?P<set_x>.+) (?P<set_y>.+))|(add (?P<add_x>.+) (?P<add_y>.+))|(mul (?P<mul_x>.+) (?P<mul_y>.+))|(mod (?P<mod_x>.+) (?P<mod_y>.+))|(rcv (?P<rcv>.+))|(jgz (?P<jgz_x>.+) (?P<jgz_y>.+))$",
            )
            .unwrap()
        });
        INPUT_REGEX
            .captures(s)
            .ok_or_else(|| aoc_error!("Can't parse instruction: '{s}'"))
            .and_then(|cap| {
                if let Some(x) = cap.name("snd") {
                    Ok(Instruction::Snd(x.as_str().parse()?))
                } else if let (Some(x), Some(y)) =
                    (cap.name("set_x"), cap.name("set_y"))
                {
                    Ok(Instruction::Set(
                        x.as_str().parse()?,
                        y.as_str().parse()?,
                    ))
                } else if let (Some(x), Some(y)) =
                    (cap.name("add_x"), cap.name("add_y"))
                {
                    Ok(Instruction::Add(
                        x.as_str().parse()?,
                        y.as_str().parse()?,
                    ))
                } else if let (Some(x), Some(y)) =
                    (cap.name("mul_x"), cap.name("mul_y"))
                {
                    Ok(Instruction::Mul(
                        x.as_str().parse()?,
                        y.as_str().parse()?,
                    ))
                } else if let (Some(x), Some(y)) =
                    (cap.name("mod_x"), cap.name("mod_y"))
                {
                    Ok(Instruction::Mod(
                        x.as_str().parse()?,
                        y.as_str().parse()?,
                    ))
                } else if let Some(x) = cap.name("rcv") {
                    Ok(Instruction::Rcv(x.as_str().parse()?))
                } else if let (Some(x), Some(y)) =
                    (cap.name("jgz_x"), cap.name("jgz_y"))
                {
                    Ok(Instruction::Jgz(
                        x.as_str().parse()?,
                        y.as_str().parse()?,
                    ))
                } else {
                    Err(aoc_error!("Unknown instruction: '{s}'"))
                }
            })
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(|line| line.parse()).collect()
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
    fn step(&mut self) -> Result<bool> {
        let instr = self.program.get(self.ip as usize).ok_or_else(|| {
            aoc_error!("Wrong IP {}, stopping execution", self.ip)
        })?;
        match instr {
            Instruction::Snd(val) => {
                self.output.borrow_mut().push_back(val.get(&self.registers));
                self.output_counter += 1;
            }
            Instruction::Set(tgt, val) => {
                self.registers.insert(*tgt, val.get(&self.registers));
            }
            Instruction::Add(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) +=
                    val.get(&self.registers)
            }
            Instruction::Mul(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) *=
                    val.get(&self.registers)
            }
            Instruction::Mod(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) %=
                    val.get(&self.registers)
            }
            Instruction::Rcv(reg) => match self.kind {
                ComputerKind::SoundRecover => {
                    if let Some(&x) = self.registers.get(reg) {
                        if x != 0 {
                            return Ok(false);
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
                        self.registers.insert(*reg, data);
                    } else {
                        return Ok(false);
                    }
                }
            },
            Instruction::Jgz(check, offset) => {
                if check.get(&self.registers) > 0 {
                    self.ip += offset.get(&self.registers);
                    return Ok(true);
                }
            }
        }
        self.ip += 1;
        Ok(true)
    }
    fn last_sound(&self) -> Result<RegValue> {
        self.output
            .borrow_mut()
            .back()
            .copied()
            .ok_or_else(|| aoc_error!("No last sound played"))
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

pub fn task1(input: &[Instruction]) -> Result<RegValue> {
    let mut computer = Computer::new(input, ComputerKind::SoundRecover);
    while computer.step()? {}
    computer.last_sound()
}

pub fn task2(input: &[Instruction]) -> Result<usize> {
    let mut computer0 = Computer::new(input, ComputerKind::SendRecieve);
    computer0.registers.insert('p', 0);
    let mut computer1 = Computer::new(input, ComputerKind::SendRecieve);
    computer1.registers.insert('p', 1);

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
