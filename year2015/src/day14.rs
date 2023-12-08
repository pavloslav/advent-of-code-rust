pub struct Reindeer {
    speed: usize,
    burst: usize,
    rest: usize,
}

pub fn parse_input(input: &str) -> anyhow::Result<Vec<Reindeer>> {
    input
        .lines()
        .map(|line| {
            let (_, speed, burst, rest): (&str, usize, usize, usize) = prse::try_parse!(
                line,
                "{} can fly {} km/s for {} seconds, but then must rest for {} seconds."
            )?;
            Ok(Reindeer { speed, burst, rest })
        })
        .collect()
}

const RACE_TIME: usize = 2503;

impl Reindeer {
    fn distance(&self, time: usize) -> usize {
        let repeat = self.burst + self.rest;
        (time / repeat * self.burst + std::cmp::min(time % repeat, self.burst)) * self.speed
    }
}

pub fn task1(deers: &[Reindeer]) -> anyhow::Result<usize> {
    deers
        .iter()
        .map(|deer| deer.distance(RACE_TIME))
        .max()
        .ok_or(anyhow::anyhow!("No deers to find the result!"))
}

pub fn task2(deers: &[Reindeer]) -> anyhow::Result<usize> {
    let mut results = vec![0; deers.len()];
    for t in 1..RACE_TIME {
        let distances: Vec<_> = deers.iter().map(|deer| deer.distance(t)).collect();
        let &best = distances
            .iter()
            .max()
            .ok_or(anyhow::anyhow!("No deers to find the result!"))?;
        for i in 0..deers.len() {
            if distances[i] == best {
                results[i] += 1;
            }
        }
    }
    results
        .into_iter()
        .max()
        .ok_or(anyhow::anyhow!("No deers to find the result!"))
}
