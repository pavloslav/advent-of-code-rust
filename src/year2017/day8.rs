#[derive(Debug)]
pub enum Operation {
    Inc,
    Dec,
}

use std::str::FromStr;

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Operation, <Operation as FromStr>::Err> {
        match s {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Comparison {
    Gt,
    Ls,
    Ge,
    Le,
    Eq,
    Ne,
}

impl FromStr for Comparison {
    type Err = ();
    fn from_str(
        item: &str,
    ) -> Result<Comparison, <Comparison as FromStr>::Err> {
        use Comparison::*;
        match item {
            ">" => Ok(Gt),
            "<" => Ok(Ls),
            ">=" => Ok(Ge),
            "<=" => Ok(Le),
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            _ => Err(()),
        }
    }
}

impl Comparison {
    fn exec(&self, left: i32, right: i32) -> bool {
        use Comparison::*;
        match self {
            Gt => left > right,
            Ls => left < right,
            Ge => left >= right,
            Le => left <= right,
            Eq => left == right,
            Ne => left != right,
        }
    }
}

pub struct Instruction {
    target_reg: String,
    operation: Operation,
    operand: i32,
    check_reg: String,
    comparison: Comparison,
    compare: i32,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (
                target_reg,
                operation,
                operand,
                check_reg,
                comparison,
                compare,
            ) = scan_fmt::scan_fmt!(
                line,
                "{} {} {} if {} {} {}",
                String,
                String,
                i32,
                String,
                String,
                i32
            )
            .unwrap();
            Instruction {
                target_reg,
                operation: Operation::from_str(&operation).unwrap(),
                operand,
                check_reg,
                comparison: Comparison::from_str(&comparison).unwrap(),
                compare,
            }
        })
        .collect()
}

pub fn task1(input: &[Instruction]) -> i32 {
    let mut registers = std::collections::HashMap::<String, i32>::new();
    for instr in input {
        if instr.comparison.exec(
            *registers.get(&instr.check_reg).unwrap_or(&0),
            instr.compare,
        ) {
            *registers.entry(instr.target_reg.clone()).or_insert(0) +=
                match instr.operation {
                    Operation::Inc => instr.operand,
                    Operation::Dec => -instr.operand,
                }
        }
    }
    *registers.values().max().unwrap()
}

pub fn task2(input: &[Instruction]) -> i32 {
    let mut registers = std::collections::HashMap::<String, i32>::new();
    let mut max = 0;
    for instr in input {
        if instr.comparison.exec(
            *registers.get(&instr.check_reg).unwrap_or(&0),
            instr.compare,
        ) {
            *registers.entry(instr.target_reg.clone()).or_insert(0) +=
                match instr.operation {
                    Operation::Inc => instr.operand,
                    Operation::Dec => -instr.operand,
                };
            max = std::cmp::max(max, registers[&instr.target_reg]);
        }
    }
    max
}
