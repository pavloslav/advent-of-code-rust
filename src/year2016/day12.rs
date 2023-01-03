pub enum Operand {
    Register(usize),
    Value(i32),
}

fn reg_to_n(reg: &str) -> Option<usize> {
    ["a", "b", "c", "d"].iter().position(|r| r == &reg)
}

impl Operand {
    fn new(s: &str) -> Operand {
        reg_to_n(s)
            .map(Operand::Register)
            .unwrap_or_else(|| Operand::Value(s.parse().unwrap()))
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

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            match parts.next() {
                Some("cpy") => Instruction::Cpy {
                    from: Operand::new(parts.next().unwrap()),
                    reg: reg_to_n(parts.next().unwrap()).unwrap(),
                },
                Some("inc") => {
                    Instruction::Inc(reg_to_n(parts.next().unwrap()).unwrap())
                }
                Some("dec") => {
                    Instruction::Dec(reg_to_n(parts.next().unwrap()).unwrap())
                }
                Some("jnz") => Instruction::Jnz {
                    value: Operand::new(parts.next().unwrap()),
                    shift: Operand::new(parts.next().unwrap()),
                },
                _ => unimplemented!("Wrong instruction in {}", l),
            }
        })
        .collect()
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

pub fn task1(input: &[Instruction]) -> i32 {
    run(input, [0; 4])
}

pub fn task2(input: &[Instruction]) -> i32 {
    run(input, [0, 0, 1, 0])
}
