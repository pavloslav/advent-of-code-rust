type Yell = u64;

pub enum Monkey {
    Value(Yell),
    Operation(String, char, String),
}

use std::collections::HashMap;

impl Monkey {
    fn new(input: &str) -> Monkey {
        if let Ok(value) = input.parse() {
            Monkey::Value(value)
        } else {
            let (left, op, right): (String, char, String);
            text_io::scan!(input.bytes()=>"{} {} {}",left, op, right);
            Monkey::Operation(left, op, right)
        }
    }
    fn yell(&self, monkeys: &HashMap<String, Monkey>) -> Yell {
        match self {
            Monkey::Value(yell) => *yell,
            Monkey::Operation(left, op, right) => {
                let left = monkeys[left].yell(monkeys);
                let right = monkeys[right].yell(monkeys);
                match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => unimplemented!(),
                }
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
                    let to_left = match op {
                        '+' => value - right,
                        '-' => value + right,
                        '*' => value / right,
                        '/' => value * right,
                        _ => unimplemented!(),
                    };
                    monkeys[left].find_humn(monkeys, to_left)
                } else {
                    let left = monkeys[left].yell(monkeys);
                    let to_right = match op {
                        '+' => value - left,
                        '-' => left - value,
                        '*' => value / left,
                        '/' => left / value,
                        _ => unimplemented!(),
                    };
                    monkeys[right].find_humn(monkeys, to_right)
                }
            }
            Monkey::Value(_) => value, //this means it's humn
        }
    }
}

pub fn parse_input(input: &str) -> HashMap<String, Monkey> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let yell = parts.next().unwrap();
            (name.to_owned(), Monkey::new(yell))
        })
        .collect()
}

pub fn task1(map: &HashMap<String, Monkey>) -> Yell {
    map[&"root".to_owned()].yell(map)
}

pub fn task2(map: &HashMap<String, Monkey>) -> Yell {
    if let Monkey::Operation(left, _, right) = &map["root"] {
        if map[left].has_humn(map) {
            map[left].find_humn(map, map[right].yell(map))
        } else {
            map[right].find_humn(map, map[left].yell(map))
        }
    } else {
        unreachable!()
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
        assert_eq!(task1(&parse_input(INPUT)), 152);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input(INPUT)), 301);
    }
}
