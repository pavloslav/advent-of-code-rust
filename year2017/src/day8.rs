use anyhow::Context;

#[derive(Debug)]
pub enum Operation {
    Inc,
    Dec,
}

use std::str::FromStr;

impl FromStr for Operation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Operation> {
        match s {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            other => Err(anyhow::anyhow!("Parsing operation '{other}' failed")),
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
    type Err = anyhow::Error;
    fn from_str(item: &str) -> anyhow::Result<Comparison> {
        use Comparison::*;
        match item {
            ">" => Ok(Gt),
            "<" => Ok(Ls),
            ">=" => Ok(Ge),
            "<=" => Ok(Le),
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            other => Err(anyhow::anyhow!("Parsing comparison '{other}' failed")),
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

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            let (target_reg, operation, operand, check_reg, comparison, compare): (
                String,
                &str,
                i32,
                String,
                &str,
                i32,
            ) = prse::try_parse!(line, "{} {} {} if {} {} {}")?;
            Ok(Instruction {
                target_reg,
                operation: operation.parse()?,
                operand,
                check_reg,
                comparison: comparison.parse()?,
                compare,
            })
        })
        .collect()
}

pub fn task1(input: &[Instruction]) -> anyhow::Result<i32> {
    let mut registers = std::collections::HashMap::<String, i32>::new();
    for instr in input {
        if instr.comparison.exec(
            *registers.get(&instr.check_reg).unwrap_or(&0),
            instr.compare,
        ) {
            *registers.entry(instr.target_reg.clone()).or_insert(0) += match instr.operation {
                Operation::Inc => instr.operand,
                Operation::Dec => -instr.operand,
            }
        }
    }
    registers
        .values()
        .max()
        .copied()
        .context("No registers present")
}

pub fn task2(input: &[Instruction]) -> anyhow::Result<i32> {
    let mut registers = std::collections::HashMap::<String, i32>::new();
    let mut max = 0;
    for instr in input {
        if instr.comparison.exec(
            *registers.get(&instr.check_reg).unwrap_or(&0),
            instr.compare,
        ) {
            *registers.entry(instr.target_reg.clone()).or_insert(0) += match instr.operation {
                Operation::Inc => instr.operand,
                Operation::Dec => -instr.operand,
            };
            max = max.max(registers[&instr.target_reg]);
        }
    }
    Ok(max)
}
