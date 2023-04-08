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
    Set(RegName, Operand),
    Sub(RegName, Operand),
    Mul(RegName, Operand),
    Jnz(Operand, Operand),
}

impl std::str::FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Instruction, ()> {
        let mut s = s.split_whitespace();
        match s.next() {
            Some("set") => Ok(Instruction::Set(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("sub") => Ok(Instruction::Sub(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("mul") => Ok(Instruction::Mul(
                s.next().ok_or(())?.chars().next().ok_or(())?,
                s.next().ok_or(())?.parse()?,
            )),
            Some("jnz") => Ok(Instruction::Jnz(
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

struct Computer {
    program: Vec<Instruction>,
    registers: Registers,
    ip: RegValue,
    mul_counter: usize,
}

impl Computer {
    fn step(&mut self) -> bool {
        if !(0..self.program.len() as RegValue).contains(&self.ip) {
            return false;
        }
        match self.program[self.ip as usize] {
            Instruction::Set(tgt, val) => {
                self.registers.insert(tgt, val.get(&self.registers));
            }
            Instruction::Sub(tgt, val) => {
                *self.registers.entry(tgt).or_insert(0) -=
                    val.get(&self.registers)
            }
            Instruction::Mul(tgt, val) => {
                *self.registers.entry(tgt).or_insert(0) *=
                    val.get(&self.registers);
                self.mul_counter += 1;
            }
            Instruction::Jnz(check, offset) => {
                if check.get(&self.registers) != 0 {
                    self.ip += offset.get(&self.registers);
                    return true;
                }
            }
        }
        self.ip += 1;
        true
    }
    fn new(program: &[Instruction]) -> Computer {
        Computer {
            program: program.to_vec(),
            registers: Registers::new(),
            ip: 0,
            mul_counter: 0,
        }
    }
}

pub fn task1(input: &[Instruction]) -> usize {
    let mut computer = Computer::new(input);
    while computer.step() {}
    computer.mul_counter
}

pub fn task2(input: &[Instruction]) -> RegValue {
    let mut computer = Computer::new(input);
    computer.registers.insert('a', 1);
    while computer.step() {}
    computer.registers[&'h']
}
