use crate::*;

type Value = isize;
type Register = usize;

fn reg_num(input: &str) -> Result<Register> {
    ["a", "b", "c", "d"]
        .iter()
        .position(|&r| r == input)
        .ok_or_else(|| task_error!("'{input}' is not a register name"))
}

#[derive(Clone, Copy)]
pub enum RegValue {
    Register(Register),
    Value(Value),
}

impl std::str::FromStr for RegValue {
    type Err = Error;
    fn from_str(s: &str) -> Result<RegValue> {
        s.parse()
            .map(RegValue::Value)
            .or_else(|_| Ok(RegValue::Register(reg_num(s)?)))
    }
}

impl RegValue {
    fn get(&self, regs: &[Value; 4]) -> Value {
        match self {
            RegValue::Value(value) => *value,
            RegValue::Register(reg) => regs[*reg],
        }
    }
}

#[derive(Clone)]
pub enum OneArgument {
    Inc,
    Dec,
    Tgl,
}

#[derive(Clone)]
pub enum TwoArgument {
    Jnz,
    Cpy,
}

#[derive(Clone)]
pub enum Instruction {
    OneArgument(OneArgument, RegValue),
    TwoArgument(TwoArgument, RegValue, RegValue),
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok((src, dst)) =
            scan_fmt::scan_fmt!(s, "cpy {} {}", RegValue, RegValue)
        {
            if matches!(dst, RegValue::Register(_)) {
                Ok(Instruction::TwoArgument(TwoArgument::Cpy, src, dst))
            } else {
                Err(task_error!("Wrong cpy second argument"))
            }
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "inc {}", RegValue) {
            Ok(Instruction::OneArgument(OneArgument::Inc, tgt))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "dec {}", RegValue) {
            Ok(Instruction::OneArgument(OneArgument::Dec, tgt))
        } else if let Ok((src, tgt)) =
            scan_fmt::scan_fmt!(s, "jnz {} {}", RegValue, RegValue)
        {
            Ok(Instruction::TwoArgument(TwoArgument::Jnz, src, tgt))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "tgl {}", RegValue) {
            Ok(Instruction::OneArgument(OneArgument::Tgl, tgt))
        } else {
            Err(task_error!("Unknown instruction '{}'", s))
        }
    }
}

pub struct Computer {
    pub registers: [Value; 4],
    program: Vec<Instruction>,
    ip: usize,
}

impl Computer {
    pub fn new(program: &[Instruction]) -> Computer {
        Computer {
            registers: [0; 4],
            program: program.to_vec(),
            ip: 0,
        }
    }
    pub fn run(&mut self) -> Result<Value> {
        while self.ip < self.program.len() {
            match &self.program[self.ip] {
                Instruction::OneArgument(
                    OneArgument::Inc,
                    RegValue::Register(r),
                ) => {
                    self.registers[*r] += 1;
                }
                Instruction::OneArgument(
                    OneArgument::Dec,
                    RegValue::Register(r),
                ) => {
                    self.registers[*r] -= 1;
                }
                Instruction::OneArgument(OneArgument::Tgl, tgt) => {
                    let tgt = self.ip as Value + tgt.get(&self.registers);
                    if (0..self.program.len() as isize).contains(&tgt) {
                        let tgt = tgt as usize;
                        self.program[tgt] = match &self.program[tgt] {
                            Instruction::OneArgument(OneArgument::Inc, r) => {
                                Instruction::OneArgument(OneArgument::Dec, *r)
                            }
                            Instruction::OneArgument(_, r) => {
                                Instruction::OneArgument(OneArgument::Inc, *r)
                            }
                            Instruction::TwoArgument(
                                TwoArgument::Jnz,
                                r1,
                                r2,
                            ) => Instruction::TwoArgument(
                                TwoArgument::Cpy,
                                *r1,
                                *r2,
                            ),
                            Instruction::TwoArgument(_, r1, r2) => {
                                Instruction::TwoArgument(
                                    TwoArgument::Jnz,
                                    *r1,
                                    *r2,
                                )
                            }
                        }
                    }
                }
                Instruction::TwoArgument(
                    TwoArgument::Cpy,
                    src,
                    RegValue::Register(tgt),
                ) => {
                    self.registers[*tgt] = src.get(&self.registers);
                }
                Instruction::TwoArgument(TwoArgument::Jnz, src, tgt) => {
                    if src.get(&self.registers) != 0 {
                        self.ip = self
                            .ip
                            .checked_add_signed(tgt.get(&self.registers))
                            .ok_or_else(|| {
                                task_error!("Ip shouldn't be less then 0!")
                            })?;
                        continue;
                    }
                }
                _ => {} //skip incorrect instruction
            }
            self.ip += 1;
        }
        Ok(self.registers[0])
    }
}
