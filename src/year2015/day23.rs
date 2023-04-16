use super::super::common::Result;
use super::Error::TaskError;

const REG_A: usize = 0;
const REG_B: usize = 1;

type Register = usize;

fn reg_num(input: &str) -> Result<Register> {
    match input {
        "a" => Ok(REG_A),
        "b" => Ok(REG_B),
        other => Err(TaskError(format!("Wrong register '{other}'"))),
    }
}

#[derive(Clone)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl Instruction {
    fn new(input: &str) -> Result<Instruction> {
        static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| {
                regex::Regex::new(
                    r"(hlf (?P<hlf>\w))|(tpl (?P<tpl>\w))|(inc (?P<inc>\w))|(jmp (?P<jmp>[+-]?\d+))|(jie (?P<jie_reg>\w), (?P<jie_off>[+-]?\d+))|(jio (?P<jio_reg>\w), (?P<jio_off>[+-]?\d+))",
                )
                .unwrap()
            });
        INPUT_REGEX
            .captures(input)
            .ok_or_else(|| {
                TaskError(format!("Can't find any instructions in '{input}'"))
            })
            .and_then(|captures| {
                Ok(if let Some(hlf) = captures.name("hlf") {
                    Instruction::Hlf(reg_num(hlf.as_str())?)
                } else if let Some(tpl) = captures.name("tpl") {
                    Instruction::Tpl(reg_num(tpl.as_str())?)
                } else if let Some(inc) = captures.name("inc") {
                    Instruction::Inc(reg_num(inc.as_str())?)
                } else if let Some(jmp) = captures.name("jmp") {
                    Instruction::Jmp(jmp.as_str().parse().map_err(|_| {
                        TaskError(format!(
                            "Can't parse jump by '{}'",
                            jmp.as_str()
                        ))
                    })?)
                } else if let (Some(jie_reg), Some(jie_off)) =
                    (captures.name("jie_reg"), captures.name("jie_off"))
                {
                    Instruction::Jie(
                        reg_num(jie_reg.as_str())?,
                        jie_off.as_str().parse().map_err(|_| {
                            TaskError(format!(
                                "Can't parse jie by '{}'",
                                jie_off.as_str()
                            ))
                        })?,
                    )
                } else if let (Some(jio_reg), Some(jio_off)) =
                    (captures.name("jio_reg"), captures.name("jio_off"))
                {
                    Instruction::Jio(
                        reg_num(jio_reg.as_str())?,
                        jio_off.as_str().parse().map_err(|_| {
                            TaskError(format!(
                                "Can't parse jio by '{}'",
                                jio_off.as_str()
                            ))
                        })?,
                    )
                } else {
                    return Err(TaskError(format!(
                        "Unknown instruction: {input}"
                    )));
                })
            })
    }
}

struct Computer {
    regs: [usize; 2],
    ip: usize,
    program: Vec<Instruction>,
}

pub fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.lines().map(Instruction::new).collect()
}

impl Computer {
    fn new(a: usize, program: &[Instruction]) -> Computer {
        Computer {
            regs: [a, 0],
            ip: 0,
            program: program.to_vec(),
        }
    }
    fn jmp(&mut self, offset: isize) -> bool {
        match self.ip.checked_add_signed(offset) {
            Some(addr) if addr < self.program.len() => {
                self.ip = addr;
                true
            }
            _ => false,
        }
    }

    fn get_reg(&mut self, reg: Register) -> Result<&mut usize> {
        self.regs
            .get_mut(reg)
            .ok_or_else(|| TaskError(format!("Incorrect register {reg}")))
    }

    fn step(&mut self) -> Result<bool> {
        let instr = self
            .program
            .get(self.ip)
            .ok_or_else(|| TaskError(format!("Incorrect ip: {}", self.ip)))?
            .clone();
        match instr {
            Instruction::Hlf(tgt) => {
                *self.get_reg(tgt)? /= 2;
            }
            Instruction::Tpl(tgt) => {
                *self.get_reg(tgt)? *= 3;
            }
            Instruction::Inc(tgt) => {
                *self.get_reg(tgt)? += 1;
            }
            Instruction::Jmp(offset) => {
                return Ok(self.jmp(offset));
            }
            Instruction::Jie(tgt, offset) => {
                if *self.get_reg(tgt)? % 2 == 0 {
                    return Ok(self.jmp(offset));
                }
            }
            Instruction::Jio(tgt, offset) => {
                if *self.get_reg(tgt)? == 1 {
                    return Ok(self.jmp(offset));
                }
            }
        }
        self.ip += 1;
        Ok(self.ip < self.program.len())
    }

    fn run(&mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }
}

pub fn task1(input: &[Instruction]) -> Result<usize> {
    let mut computer = Computer::new(0, input);
    computer.run()?;
    Ok(computer.regs[1])
}

pub fn task2(input: &[Instruction]) -> Result<usize> {
    let mut computer = Computer::new(1, input);
    computer.run()?;
    Ok(computer.regs[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computer() {
        let input = "inc a
jio a, +2
tpl a
inc a";
        let mut computer = Computer::new(0, &parse_input(input).unwrap());
        if let Err(e) = computer.run() {
            panic!("Error '{e:?}' happened")
        };
        assert_eq!(computer.regs, [2, 0]);
    }
}
