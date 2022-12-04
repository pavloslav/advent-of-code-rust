pub struct Reindeer {
    speed: usize,
    burst: usize,
    rest: usize,
}

use once_cell::sync::Lazy;

pub fn parse_input(input: &str) -> Vec<Reindeer> {
    static INPUT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        regex::Regex::new(r"\w+ can fly (?P<speed>\d+) km/s for (?P<burst>\d+) seconds, but then must rest for (?P<rest>\d+) seconds\.").unwrap()
    });
    input
        .lines()
        .filter_map(|line| {
            if let Some(captures) = INPUT_REGEX.captures(line) {
                if let (Some(speed), Some(burst), Some(rest)) = (
                    captures.name("speed"),
                    captures.name("burst"),
                    captures.name("rest"),
                ) {
                    if let (Ok(speed), Ok(burst), Ok(rest)) = (
                        speed.as_str().parse(),
                        burst.as_str().parse(),
                        rest.as_str().parse(),
                    ) {
                        return Some(Reindeer { speed, burst, rest });
                    }
                }
            }
            None
        })
        .collect()
}

const RACE_TIME: usize = 2503;

impl Reindeer {
    fn distance(&self, time: usize) -> usize {
        let repeat = self.burst + self.rest;
        (time / repeat * self.burst + std::cmp::min(time % repeat, self.burst))
            * self.speed
    }
}

pub fn task1(deers: &[Reindeer]) -> usize {
    deers
        .iter()
        .map(|deer| deer.distance(RACE_TIME))
        .max()
        .unwrap()
}

pub fn task2(deers: &Vec<Reindeer>) -> usize {
    let mut results = vec![0; deers.len()];
    for t in 1..RACE_TIME {
        let distances: Vec<_> =
            deers.iter().map(|deer| deer.distance(t)).collect();
        let &best = distances.iter().max().unwrap();
        for i in 0..deers.len() {
            if distances[i] == best {
                results[i] += 1;
            }
        }
    }
    results.into_iter().max().unwrap()
}
