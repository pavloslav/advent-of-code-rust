use super::super::common::Result;
use super::Error::TaskError;

pub enum Operand {
    Register(usize),
    Value(i32),
}

fn reg_to_n(reg: &str) -> Option<usize> {
    ["a", "b", "c", "d"].iter().position(|r| r == &reg)
}

impl Operand {
    fn new(s: &str) -> Result<Operand> {
        if let Some(reg) = reg_to_n(s) {
            Ok(Operand::Register(reg))
        } else {
            Ok(Operand::Value(s.parse().map_err(|_| {
                TaskError(format!("Can't parse value {s}"))
            })?))
        }
    }

    fn get(&self, regs: &[i32; 4]) -> i32 {
        match self {
            Operand::Value(x) => *x,
            Operand::Register(r) => regs[*r],
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
                        from: Operand::new(x.as_str())?,
                        reg: reg_to_n(y.as_str()).ok_or_else(|| {
                            TaskError(format!(
                                "Cpy target '{}' not found",
                                y.as_str()
                            ))
                        })?,
                    })
                } else if let Some(x) = cap.name("inc_x") {
                    Ok(Instruction::Inc(reg_to_n(x.as_str()).ok_or_else(
                        || {
                            TaskError(format!(
                                "Inc target '{}' not found",
                                x.as_str()
                            ))
                        },
                    )?))
                } else if let Some(x) = cap.name("dec_x") {
                    Ok(Instruction::Dec(reg_to_n(x.as_str()).ok_or_else(
                        || {
                            TaskError(format!(
                                "Decc target '{}' not found",
                                x.as_str()
                            ))
                        },
                    )?))
                } else if let (Some(x), Some(y)) =
                    (cap.name("jnz_x"), cap.name("jnz_y"))
                {
                    Ok(Instruction::Jnz {
                        value: Operand::new(x.as_str())?,
                        shift: Operand::new(y.as_str())?,
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
