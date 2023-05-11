use crate::*;

type Yell = u64;

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn apply(&self, left: Yell, right: Yell) -> Yell {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }
    fn apply_inv_left(&self, value: Yell, other: Yell) -> Yell {
        match self {
            Operation::Add => value - other,
            Operation::Sub => value + other,
            Operation::Mul => value / other,
            Operation::Div => value * other,
        }
    }

    fn apply_inv_right(&self, value: Yell, other: Yell) -> Yell {
        match self {
            Operation::Add => value - other,
            Operation::Sub => other - value,
            Operation::Mul => value / other,
            Operation::Div => other / value,
        }
    }
}

pub enum Monkey {
    Value(Yell),
    Operation(String, Operation, String),
}

use std::collections::HashMap;

impl Monkey {
    fn yell(&self, monkeys: &HashMap<String, Monkey>) -> Yell {
        match self {
            Monkey::Value(yell) => *yell,
            Monkey::Operation(left, op, right) => {
                let left = monkeys[left].yell(monkeys);
                let right = monkeys[right].yell(monkeys);
                op.apply(left, right)
            }
        }
    }
    fn has_humn(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        match self {
            Monkey::Value(_) => false,
            Monkey::Operation(left, _, right) => {
                left == "humn"
                    || right == "humn"
                    || monkeys[left].has_humn(monkeys)
                    || monkeys[right].has_humn(monkeys)
            }
        }
    }
    fn find_humn(
        &self,
        monkeys: &HashMap<String, Monkey>,
        value: Yell,
    ) -> Yell {
        match self {
            Monkey::Operation(left, op, right) => {
                if left == "humn" || monkeys[left].has_humn(monkeys) {
                    let right = monkeys[right].yell(monkeys);
                    let to_left = op.apply_inv_left(value, right);
                    monkeys[left].find_humn(monkeys, to_left)
                } else {
                    let left = monkeys[left].yell(monkeys);
                    let to_right = op.apply_inv_right(value, left);
                    monkeys[right].find_humn(monkeys, to_right)
                }
            }
            Monkey::Value(_) => value, //this means it's humn
        }
    }
}

pub fn parse_input(input: &str) -> Result<HashMap<String, Monkey>> {
    input
        .lines()
        .map(|line| {
            if let Ok((name, yell)) =
                scan_fmt::scan_fmt!(line, "{}: {} {} {}", String, Yell)
            {
                Ok((name, Monkey::Value(yell)))
            } else if let Ok((name, left, op, right)) = scan_fmt::scan_fmt!(
                line,
                "{}: {} {} {}",
                String,
                String,
                char,
                String
            ) {
                let op = match op {
                    '+' => Operation::Add,
                    '-' => Operation::Sub,
                    '*' => Operation::Mul,
                    '/' => Operation::Div,
                    other => {
                        return Err(aoc_error!("Unknown operation '{other}'"));
                    }
                };
                Ok((name, Monkey::Operation(left, op, right)))
            } else {
                Err(aoc_error!("Unknown monkey format: '{line}'"))
            }
        })
        .collect()
}

pub fn task1(map: &HashMap<String, Monkey>) -> Result<Yell> {
    Ok(map[&"root".to_owned()].yell(map))
}

pub fn task2(map: &HashMap<String, Monkey>) -> Result<Yell> {
    if let Monkey::Operation(left, _, right) = &map["root"] {
        if map[left].has_humn(map) {
            Ok(map[left].find_humn(map, map[right].yell(map)))
        } else {
            Ok(map[right].find_humn(map, map[left].yell(map)))
        }
    } else {
        Err(aoc_error!("No root monkey!"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(INPUT).unwrap()).unwrap(), 152);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(INPUT).unwrap()).unwrap(), 301);
    }
}
