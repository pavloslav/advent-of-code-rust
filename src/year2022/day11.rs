#[derive(Clone)]
pub enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Add(x) => item + x,
            Operation::Mult(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: std::collections::VecDeque<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut input = input.lines();
    loop {
        input.next();
        let items = input
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operator: String;
        let operand: String;
        text_io::scan!(
            input.next().unwrap().bytes() =>
            "  Operation: new = old {} {}",
            operator,
            operand
        );
        let operation = if operator == "+" {
            Operation::Add(operand.parse().unwrap())
        } else if operand == "old" {
            Operation::Square
        } else {
            Operation::Mult(operand.parse().unwrap())
        };

        let test;
        text_io::scan!(input.next().unwrap().bytes() => "  Test: divisible by {}", test);

        let if_true;
        text_io::scan!(input.next().unwrap().bytes() => "    If true: throw to monkey {}", if_true);

        let if_false;
        text_io::scan!(
            input.next().unwrap().bytes() =>
            "    If false: throw to monkey {}",
            if_false
        );

        monkeys.push(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspected: 0,
        });

        if input.next().is_none() {
            break;
        }
    }
    monkeys
}

fn simulate(monkeys: &[Monkey], time: usize, div3: bool) -> usize {
    let mut monkeys = monkeys.to_owned();
    let common: usize = monkeys.iter().map(|m| m.test).product();
    for _round in 0..time {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let item = if div3 {
                    monkeys[i].operation.apply(item) / 3
                } else {
                    monkeys[i].operation.apply(item) % common
                };
                monkeys[i].inspected += 1;
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }
    let mut inspected: Vec<_> = monkeys.iter().map(|m| m.inspected).collect();
    inspected.sort();
    inspected[inspected.len() - 1] * inspected[inspected.len() - 2]
}

pub fn task1(monkeys: &[Monkey]) -> usize {
    simulate(monkeys, 20, true)
}

pub fn task2(monkeys: &[Monkey]) -> usize {
    simulate(monkeys, 10000, false)
}
