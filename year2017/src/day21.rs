use crate::*;

type RuleBook = std::collections::HashMap<String, String>;

pub fn parse_input(input: &str) -> AocResult<Vec<RuleBook>> {
    let mut rule_books = vec![RuleBook::new(); 2];
    for line in input.lines() {
        let (left, right): (&str, &str) = prse::try_parse!(line, "{} => {}")?;
        let pat = left.replace('/', "").as_bytes().to_vec();
        let ench = right.replace('/', "");
        let size = (pat.len() as f64).sqrt().round() as i32; //2 or 3
        if (size + 1).pow(2) as usize != ench.len() {
            return Err(aoc_error!(
                "invalid pattern size: left {}, right {}",
                pat.len(),
                ench.len()
            ));
        };
        for (x, h) in [(0, 1), (size - 1, -1)] {
            for (y, v) in [(0, 1), (size - 1, -1)] {
                //coord = size*y+x
                rule_books[(size - 2) as usize].insert(
                    (0..size * size)
                        .map(|ij| {
                            pat[(size * (y + (ij / size) * v) + x + (ij % size) * h) as usize]
                                as char
                        })
                        .collect(),
                    ench.to_string(),
                );
                rule_books[(size - 2) as usize].insert(
                    (0..size * size)
                        .map(|ij| {
                            pat[(size * (y + (ij % size) * v) + x + (ij / size) * h) as usize]
                                as char
                        })
                        .collect(),
                    ench.to_string(),
                );
            }
        }
    }
    Ok(rule_books)
}

const START_PATTERN: &str = ".#.
..#
###";

fn iterate_and_count(rule_books: &[RuleBook], count: usize) -> AocResult<usize> {
    let mut pattern: Vec<String> = START_PATTERN.lines().map(|line| line.to_owned()).collect();
    for _step in 0..count {
        for size in 2..=3 {
            if pattern.len() % size == 0 {
                let mut new_pattern = vec![String::new(); pattern.len() / size * (size + 1)];
                for y_sq in 0..pattern.len() / size {
                    for x_sq in 0..pattern.len() / size {
                        let square: String = (0..size)
                            .map(|i| &pattern[y_sq * size + i][x_sq * size..(x_sq + 1) * size])
                            .collect();
                        let output = &rule_books[size - 2][&square];

                        for i in 0..=size {
                            new_pattern[y_sq * (size + 1) + i]
                                .push_str(&output[(size + 1) * i..(size + 1) * (i + 1)]);
                        }
                    }
                }
                pattern = new_pattern;
                break;
            }
        }
    }
    Ok(pattern
        .iter()
        .map(|line| line.chars().filter(|&c| c == '#').count())
        .sum())
}

pub fn task1(rule_books: &[RuleBook]) -> AocResult<usize> {
    iterate_and_count(rule_books, 5)
}

pub fn task2(rule_books: &[RuleBook]) -> AocResult<usize> {
    iterate_and_count(rule_books, 18)
}
