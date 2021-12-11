enum Brackets {
    Correct(Vec<char>),
    Incorrect(char),
}

fn check_brackets(line: &str) -> Brackets {
    let mut stack = Vec::new();
    for symbol in line.chars() {
        match symbol {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            closing => {
                if symbol != stack.pop().unwrap() {
                    return Brackets::Incorrect(closing);
                }
            }
        }
    }
    Brackets::Correct(stack)
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(data: &str) -> usize {
    data.lines()
        .map(|line| match check_brackets(line) {
            Brackets::Incorrect(')') => 3,
            Brackets::Incorrect(']') => 57,
            Brackets::Incorrect('}') => 1197,
            Brackets::Incorrect('>') => 25137,
            _ => 0,
        })
        .sum()
}

pub fn task2(data: &str) -> usize {
    let mut scores: Vec<_> = data
        .lines()
        .filter_map(|line| match check_brackets(line) {
            Brackets::Correct(rest) => Some(rest.iter().rev().fold(0, |acc, br| {
                5 * acc
                    + match br {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("wrong char in ending!"),
                    }
            })),
            _ => None,
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}