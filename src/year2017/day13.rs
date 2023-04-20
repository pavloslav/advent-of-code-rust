use super::super::common::Error::TaskError;
use super::super::common::Result;

type Firewall = Vec<(i32, i32)>;

pub fn parse_input(input: &str) -> Result<Firewall> {
    input
        .lines()
        .map(|line| Ok(scan_fmt::scan_fmt!(line, "{}: {}", i32, i32)?))
        .collect()
}

pub fn task1(firewall: &Firewall) -> Result<i32> {
    Ok(firewall
        .iter()
        .map(|(depth, range)| {
            if depth % (2 * (range - 1)) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum())
}

pub fn task2(firewall: &Firewall) -> Result<i32> {
    (0..)
        .find(|delay| {
            firewall
                .iter()
                .all(|(depth, range)| (delay + depth) % (2 * (range - 1)) != 0)
        })
        .ok_or_else(|| TaskError("This shouldn't be".to_string()))
}
