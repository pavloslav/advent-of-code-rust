use anyhow::Context;

enum Brackets {
    Correct(Vec<char>),
    Incorrect(char),
}

fn check_brackets(line: &str) -> anyhow::Result<Brackets> {
    let mut stack = Vec::new();
    for symbol in line.chars() {
        match symbol {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            closing => {
                if symbol != stack.pop().context("No brackets to pop")? {
                    return Ok(Brackets::Incorrect(closing));
                }
            }
        }
    }
    Ok(Brackets::Correct(stack))
}

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

pub fn task1(data: &str) -> anyhow::Result<usize> {
    data.lines()
        .map(|line| {
            Ok(match check_brackets(line)? {
                Brackets::Incorrect(')') => 3,
                Brackets::Incorrect(']') => 57,
                Brackets::Incorrect('}') => 1197,
                Brackets::Incorrect('>') => 25137,
                _ => 0,
            })
        })
        .try_fold(0, |acc, x: anyhow::Result<_>| Ok(acc + x?))
}

pub fn task2(data: &str) -> anyhow::Result<usize> {
    let mut scores: Vec<_> = Vec::new();
    for line in data.lines() {
        if let Brackets::Correct(rest) = check_brackets(line)? {
            scores.push(rest.iter().rev().try_fold(0, |acc, br| {
                Ok(5 * acc
                    + match br {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        other => {
                            anyhow::bail!("Unknown bracket '{other}'");
                        }
                    })
            })?)
        }
    }
    scores.sort();
    Ok(scores[scores.len() / 2])
}
