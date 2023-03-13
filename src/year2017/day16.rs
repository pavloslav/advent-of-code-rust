use core::str::FromStr;

pub enum Dance {
    Spin(usize),
    SwapPlace(usize, usize),
    SwapDancer(char, char),
}

impl FromStr for Dance {
    type Err = ();
    fn from_str(s: &str) -> Result<Dance, <Dance as FromStr>::Err> {
        match s.chars().next() {
            Some('s') => s[1..].parse().map(|x| Dance::Spin(x)).or(Err(())),
            Some('x') => {
                let first;
                let second;
                text_io::scan!(s[1..].bytes()=>"{}/{}", first, second);
                Ok(Dance::SwapPlace(first, second))
            }
            Some('p') => {
                let first;
                let second;
                text_io::scan!(s[1..].bytes()=>"{}/{}", first, second);
                Ok(Dance::SwapDancer(first, second))
            }
            _ => Err(()),
        }
    }
}

type Line = std::collections::VecDeque<char>;

impl Dance {
    fn step(&self, line: &mut Line) {
        match self {
            &Dance::Spin(spin) => line.rotate_right(spin),
            &Dance::SwapPlace(a, b) => line.swap(a, b),
            &Dance::SwapDancer(a, b) => {
                for e in line {
                    if *e == a {
                        *e = b;
                    } else if *e == b {
                        *e = a;
                    }
                }
            }
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Dance> {
    input
        .trim()
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect()
}

fn make_dance(start: impl Iterator<Item = char>, moves: &[Dance]) -> String {
    let mut line: Line = start.collect();
    for dance in moves {
        dance.step(&mut line);
    }
    line.iter().collect()
}

pub fn task1(input: &[Dance]) -> String {
    make_dance('a'..='p', input)
}

const DANCES_COUNT: usize = 1_000_000_000;

pub fn task2(input: &[Dance]) -> String {
    let gen = || -> String { ('a'..='p').collect() };
    let step = |s: &mut String| *s = make_dance(s.chars(), input);

    let (lam, mu) = super::super::common::floyd_hare_tortoise(gen, step);
    let index = mu + (DANCES_COUNT - mu) % lam;
    let mut s = gen();
    for _ in 0..index {
        step(&mut s);
    }
    s
}
