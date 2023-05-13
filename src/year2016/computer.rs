use crate::*;

type Value = isize;
type Register = usize;

fn reg_num(input: &str) -> Result<Register> {
    ["a", "b", "c", "d"]
        .iter()
        .position(|&r| r == input)
        .ok_or_else(|| aoc_error!("'{input}' is not a register name"))
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
pub enum Instruction {
    Inc(RegValue),
    Dec(RegValue),
    Tgl(RegValue),
    Out(RegValue),
    Jnz(RegValue, RegValue),
    Cpy(RegValue, RegValue),
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Ok((src, dst)) =
            scan_fmt::scan_fmt!(s, "cpy {} {}", RegValue, RegValue)
        {
            Ok(Instruction::Cpy(src, dst))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "inc {}", RegValue) {
            Ok(Instruction::Inc(tgt))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "dec {}", RegValue) {
            Ok(Instruction::Dec(tgt))
        } else if let Ok((src, tgt)) =
            scan_fmt::scan_fmt!(s, "jnz {} {}", RegValue, RegValue)
        {
            Ok(Instruction::Jnz(src, tgt))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "tgl {}", RegValue) {
            Ok(Instruction::Tgl(tgt))
        } else if let Ok(tgt) = scan_fmt::scan_fmt!(s, "out {}", RegValue) {
            Ok(Instruction::Out(tgt))
        } else {
            Err(aoc_error!("Unknown instruction '{}'", s))
        }
    }
}

pub struct Computer {
    pub registers: [Value; 4],
    program: Vec<Instruction>,
    ip: usize,
    out: Option<Value>,
}

impl Computer {
    pub fn new(program: &[Instruction]) -> Computer {
        Computer {
            registers: [0; 4],
            program: program.to_vec(),
            ip: 0,
            out: None,
        }
    }
    pub fn step(&mut self) -> Result<()> {
        match &self.program[self.ip] {
            Instruction::Inc(RegValue::Register(r)) => {
                self.registers[*r] += 1;
            }
            Instruction::Dec(RegValue::Register(r)) => {
                self.registers[*r] -= 1;
            }
            Instruction::Tgl(tgt) => {
                let tgt = self.ip as Value + tgt.get(&self.registers);
                if (0..self.program.len() as isize).contains(&tgt) {
                    let tgt = tgt as usize;
                    self.program[tgt] = match &self.program[tgt] {
                        Instruction::Inc(r) => Instruction::Dec(*r),
                        Instruction::Dec(r)
                        | Instruction::Out(r)
                        | Instruction::Tgl(r) => Instruction::Inc(*r),
                        Instruction::Jnz(r1, r2) => Instruction::Cpy(*r1, *r2),
                        Instruction::Cpy(r1, r2) => Instruction::Jnz(*r1, *r2),
                    }
                }
            }
            Instruction::Cpy(src, RegValue::Register(tgt)) => {
                self.registers[*tgt] = src.get(&self.registers);
            }
            Instruction::Jnz(src, tgt) => {
                if src.get(&self.registers) != 0 {
                    self.ip = self
                        .ip
                        .checked_add_signed(tgt.get(&self.registers))
                        .ok_or_else(|| {
                            aoc_error!("Ip shouldn't be less then 0!")
                        })?;
                    return Ok(());
                }
            }
            Instruction::Out(tgt) => {
                self.out = Some(tgt.get(&self.registers));
            }
            _ => {} //skip incorrect instruction
        }
        self.ip += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<Value> {
        while self.ip < self.program.len() {
            self.step()?;
        }
        Ok(self.registers[0])
    }

    pub fn run_to_output(&mut self) -> Result<Option<Value>> {
        while self.ip < self.program.len() && self.out.is_none() {
            self.step()?;
        }
        Ok(self.out.take())
    }

    pub fn get_state(&self) -> (usize, [Value; 4]) {
        (self.ip, self.registers)
    }
}
