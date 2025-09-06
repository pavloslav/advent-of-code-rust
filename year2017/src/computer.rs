use anyhow::Context;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type RegName = char;
pub type RegValue = i64;
pub type Registers = std::collections::HashMap<RegName, RegValue>;

#[derive(Clone, Copy)]
pub enum Operand {
    Reg(RegName),
    Val(RegValue),
}

impl prse::Parse<'_> for Operand {
    fn from_str(s: &str) -> Result<Operand, prse::ParseError> {
        if let Ok(value) = s.parse() {
            Ok(Operand::Val(value))
        } else {
            match s.chars().next() {
                Some(c) if s.len() == 1 => Ok(Operand::Reg(c)),
                _ => Err(prse::ParseError::Other(format!(
                    "Can't parse operand '{s}'"
                ))),
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
    Sub(RegName, Operand),
    Jnz(Operand, Operand),
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Instruction> {
        if let Ok(val) = prse::try_parse!(s, "snd {}") {
            Ok(Instruction::Snd(val))
        } else if let Ok((x, y)) = prse::try_parse!(s, "set {} {}") {
            Ok(Instruction::Set(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "add {} {}") {
            Ok(Instruction::Add(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "mul {} {}") {
            Ok(Instruction::Mul(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "mod {} {}") {
            Ok(Instruction::Mod(x, y))
        } else if let Ok(x) = prse::try_parse!(s, "rcv {}") {
            Ok(Instruction::Rcv(x))
        } else if let Ok((x, y)) = prse::try_parse!(s, "jgz {} {}") {
            Ok(Instruction::Jgz(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "sub {} {}") {
            Ok(Instruction::Sub(x, y))
        } else if let Ok((x, y)) = prse::try_parse!(s, "jnz {} {}") {
            Ok(Instruction::Jnz(x, y))
        } else {
            anyhow::bail!("incorrect input: '{s}'")
        }
    }
}
type Buffer = std::collections::VecDeque<RegValue>;

pub enum ComputerKind {
    SoundRecover,
    SendRecieve,
}

pub struct Computer {
    program: Vec<Instruction>,
    registers: Registers,
    ip: RegValue,
    pub input: std::rc::Weak<RefCell<Buffer>>,
    pub output: Rc<RefCell<Buffer>>,
    pub output_counter: usize,
    kind: ComputerKind,
    pub mul_counter: usize,
}

impl Computer {
    pub fn step(&mut self) -> anyhow::Result<bool> {
        let instr = self
            .program
            .get(self.ip as usize)
            .with_context(|| format!("Wrong IP {}, stopping execution", self.ip))?;
        match instr {
            Instruction::Snd(val) => {
                self.output.borrow_mut().push_back(val.get(&self.registers));
                self.output_counter += 1;
            }
            Instruction::Set(tgt, val) => {
                self.registers.insert(*tgt, val.get(&self.registers));
            }
            Instruction::Add(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) += val.get(&self.registers)
            }
            Instruction::Sub(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) -= val.get(&self.registers)
            }
            Instruction::Mul(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) *= val.get(&self.registers);
                self.mul_counter += 1;
            }
            Instruction::Mod(tgt, val) => {
                *self.registers.entry(*tgt).or_insert(0) %= val.get(&self.registers)
            }
            Instruction::Rcv(reg) => match self.kind {
                ComputerKind::SoundRecover => {
                    if let Some(&x) = self.registers.get(reg)
                        && x != 0
                    {
                        return Ok(false);
                    }
                }
                ComputerKind::SendRecieve => {
                    if let Some(data) = self.input.upgrade().and_then(|strong| {
                        strong
                            .try_borrow_mut()
                            .ok()
                            .and_then(|mut buffer| buffer.pop_front())
                    }) {
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
            Instruction::Jnz(check, offset) => {
                if check.get(&self.registers) != 0 {
                    self.ip += offset.get(&self.registers);
                    return Ok(true);
                }
            }
        }
        self.ip += 1;
        Ok(true)
    }

    pub fn last_sound(&self) -> anyhow::Result<RegValue> {
        self.output
            .borrow_mut()
            .back()
            .copied()
            .context("No last sound played")
    }

    pub fn new(program: &[Instruction], kind: ComputerKind) -> Computer {
        Computer {
            program: program.to_vec(),
            registers: Registers::new(),
            ip: 0,
            input: Weak::new(),
            output: Rc::new(RefCell::new(Buffer::new())),
            kind,
            output_counter: 0,
            mul_counter: 0,
        }
    }

    pub fn set_register(&mut self, reg: RegName, val: RegValue) {
        self.registers.insert(reg, val);
    }
    pub fn get_register(&mut self, reg: RegName) -> anyhow::Result<RegValue> {
        self.registers
            .get(&reg)
            .copied()
            .with_context(|| format!("Register {reg} not found"))
    }
}
