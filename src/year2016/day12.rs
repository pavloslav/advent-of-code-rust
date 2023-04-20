use super::super::common::Error;
use super::super::common::Error::TaskError;
use super::super::common::Result;

pub struct Register {
    reg: usize,
}

impl std::str::FromStr for Register {
    type Err = Error;
    fn from_str(reg: &str) -> Result<Register> {
        let reg = ["a", "b", "c", "d"]
            .iter()
            .position(|&r| r == reg)
            .ok_or_else(|| {
                TaskError(format!("'{reg}' is not a register name"))
            })?;
        Ok(Register { reg })
    }
}

pub enum Operand {
    Register(Register),
    Value(i32),
}

impl std::str::FromStr for Operand {
    type Err = Error;
    fn from_str(input: &str) -> Result<Operand> {
        match input.parse::<i32>() {
            Ok(value) => Ok(Operand::Value(value)),
            _ => Ok(Operand::Register(input.parse::<Register>()?)),
        }
    }
}

impl Operand {
    fn get(&self, regs: &[i32; 4]) -> i32 {
        match self {
            Operand::Value(x) => *x,
            Operand::Register(r) => regs[r.reg],
        }
    }
}

pub enum Instruction {
    Cpy { from: Operand, reg: usize },
    Inc(usize),
    Dec(usize),
    Jnz { value: Operand, shift: Operand },
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| {
            regex::Regex::new(
    r"^(cpy (?P<cpy_x>.+) (?P<cpy_y>.+))|(inc (?P<inc_x>.+))|(dec (?P<dec_x>.+))|(jnz (?P<jnz_x>.+) (?P<jnz_y>.+))$"
    ).unwrap()
        });

    input
        .lines()
        .map(|line| {
            if let Some(cap) = INPUT_REGEX.captures(line) {
                if let (Some(x), Some(y)) =
                    (cap.name("cpy_x"), cap.name("cpy_y"))
                {
                    Ok(Instruction::Cpy {
                        from: x.as_str().parse()?,
                        reg: y.as_str().parse()?,
                    })
                } else if let Some(x) = cap.name("inc_x") {
                    Ok(Instruction::Inc(x.as_str().parse()?))
                } else if let Some(x) = cap.name("dec_x") {
                    Ok(Instruction::Dec(x.as_str().parse()?))
                } else if let (Some(x), Some(y)) =
                    (cap.name("jnz_x"), cap.name("jnz_y"))
                {
                    Ok(Instruction::Jnz {
                        value: x.as_str().parse()?,
                        shift: y.as_str().parse()?,
                    })
                } else {
                    Err(TaskError(format!("Can't find parts in '{line}'")))
                }
            } else {
                Err(TaskError(format!("Can't match '{line}'")))
            }
        })
        .collect::<Result<Vec<_>>>()
}

fn run(program: &[Instruction], regs: [i32; 4]) -> i32 {
    let mut registers = regs;
    let mut ip = 0;
    while ip < program.len() {
        match &program[ip] {
            Instruction::Cpy { from, reg } => {
                registers[*reg] = from.get(&registers)
            }
            Instruction::Inc(reg) => registers[*reg] += 1,
            Instruction::Dec(reg) => registers[*reg] -= 1,
            Instruction::Jnz { value, shift } => {
                if value.get(&registers) != 0 {
                    ip = ip
                        .checked_add_signed(shift.get(&registers) as isize)
                        .unwrap();
                    continue;
                }
            }
        }
        ip += 1;
    }
    registers[0]
}

pub fn task1(input: &[Instruction]) -> Result<i32> {
    Ok(run(input, [0; 4]))
}

pub fn task2(input: &[Instruction]) -> Result<i32> {
    Ok(run(input, [0, 0, 1, 0]))
}
