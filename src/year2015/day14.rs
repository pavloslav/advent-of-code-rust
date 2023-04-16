use super::super::common::Result;
use super::Error::TaskError;

pub struct Reindeer {
    speed: usize,
    burst: usize,
    rest: usize,
}

pub fn parse_input(input: &str) -> Result<Vec<Reindeer>> {
    input
        .lines()
        .map(|line| {
            let (speed, burst, rest) = scan_fmt::scan_fmt!(line, "{*} can fly {} km/s for {} seconds, but then must rest for {} seconds.", usize, usize, usize)
            .map_err(|_|TaskError(format!("Wrong Reindeer: {line}")))?;
            Ok(Reindeer { speed, burst, rest })
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

pub fn task1(deers: &[Reindeer]) -> Result<usize> {
    deers
        .iter()
        .map(|deer| deer.distance(RACE_TIME))
        .max()
        .ok_or(TaskError("No deers to find the result!".to_string()))
}

pub fn task2(deers: &Vec<Reindeer>) -> Result<usize> {
    let mut results = vec![0; deers.len()];
    for t in 1..RACE_TIME {
        let distances: Vec<_> =
            deers.iter().map(|deer| deer.distance(t)).collect();
        let &best = distances
            .iter()
            .max()
            .ok_or(TaskError("No deers to find the result!".to_string()))?;
        for i in 0..deers.len() {
            if distances[i] == best {
                results[i] += 1;
            }
        }
    }
    results
        .into_iter()
        .max()
        .ok_or(TaskError("No deers to find the result!".to_string()))
}
