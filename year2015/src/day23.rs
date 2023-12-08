use anyhow::Context;

const REG_A: usize = 0;
const REG_B: usize = 1;

type Register = usize;

fn reg_num(input: &str) -> anyhow::Result<Register> {
    match input {
        "a" => Ok(REG_A),
        "b" => Ok(REG_B),
        other => Err(anyhow::anyhow!("Wrong register '{other}'")),
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

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Ok((op, reg, value)) = prse::try_parse!(s, "{} {}, {}") {
            match op {
                "jie" => Ok(Instruction::Jie(reg_num(reg)?, value)),
                "jio" => Ok(Instruction::Jio(reg_num(reg)?, value)),
                other => Err(anyhow::anyhow!("Unknown 2-operand instruction '{other}'")),
            }
        } else if let Ok((op, reg)) = prse::try_parse!(s, "{} {}") {
            let reg: &str = reg; //hint for prse::try_parse
            match op {
                "hlf" => Ok(Instruction::Hlf(reg_num(reg)?)),
                "tpl" => Ok(Instruction::Tpl(reg_num(reg)?)),
                "inc" => Ok(Instruction::Inc(reg_num(reg)?)),
                "jmp" => Ok(Instruction::Jmp(reg.parse()?)),
                other => Err(anyhow::anyhow!("Unknown 1-operand instruction '{other}'")),
            }
        } else {
            Err(anyhow::anyhow!("Incorrect instruction '{s}'"))
        }
    }
}

struct Computer {
    regs: [usize; 2],
    ip: usize,
    program: Vec<Instruction>,
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input.lines().map(|s| s.parse()).collect()
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

    fn get_reg(&mut self, reg: Register) -> anyhow::Result<&mut usize> {
        self.regs
            .get_mut(reg)
            .with_context(|| format!("Incorrect register {reg}"))
    }

    fn step(&mut self) -> anyhow::Result<bool> {
        let instr = self
            .program
            .get(self.ip)
            .with_context(|| format!("Incorrect ip: {}", self.ip))?
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

    fn run(&mut self) -> anyhow::Result<()> {
        while self.step()? {}
        Ok(())
    }
}

pub fn task1(input: &[Instruction]) -> anyhow::Result<usize> {
    let mut computer = Computer::new(0, input);
    computer.run()?;
    Ok(computer.regs[1])
}

pub fn task2(input: &[Instruction]) -> anyhow::Result<usize> {
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
