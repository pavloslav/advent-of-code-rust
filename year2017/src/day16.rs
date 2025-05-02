pub enum Dance {
    Spin(usize),
    SwapPlace(usize, usize),
    SwapDancer(char, char),
}

use core::str::FromStr;
impl FromStr for Dance {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Dance> {
        Ok(match s.chars().next() {
            Some('s') => Dance::Spin(s[1..].parse()?),
            Some('x') => {
                let (first, second) = prse::try_parse!(&s[1..], "{}/{}")?;
                Dance::SwapPlace(first, second)
            }
            Some('p') => {
                let (first, second) = prse::try_parse!(&s[1..], "{}/{}")?;
                Dance::SwapDancer(first, second)
            }
            _ => anyhow::bail!("Unknown command {s}"),
        })
    }
}

type Line = std::collections::VecDeque<char>;

impl Dance {
    fn step(&self, line: &mut Line) {
        match self {
            Dance::Spin(spin) => line.rotate_right(*spin),
            Dance::SwapPlace(a, b) => line.swap(*a, *b),
            Dance::SwapDancer(a, b) => {
                for e in line {
                    if e == a {
                        *e = *b
                    } else if e == b {
                        *e = *a
                    }
                }
            }
        }
    }
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Dance>> {
    input.trim().split(',').map(|d| d.parse()).collect()
}

fn make_dance(start: impl Iterator<Item = char>, moves: &[Dance]) -> String {
    let mut line: Line = start.collect();
    for dance in moves {
        dance.step(&mut line);
    }
    line.iter().collect()
}

pub fn task1(input: &[Dance]) -> anyhow::Result<String> {
    Ok(make_dance('a'..='p', input))
}

const DANCES_COUNT: usize = 1_000_000_000;

pub fn task2(input: &[Dance]) -> anyhow::Result<String> {
    let gen_func = || -> String { ('a'..='p').collect() };
    let step = |s: &mut String| *s = make_dance(s.chars(), input);

    let (lam, mu) = common::floyd_hare_tortoise(gen_func, step);
    let index = mu + (DANCES_COUNT - mu) % lam;
    let mut s = gen_func();
    for _ in 0..index {
        step(&mut s);
    }
    Ok(s)
}
