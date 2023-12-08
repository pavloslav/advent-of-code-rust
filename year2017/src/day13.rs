use anyhow::Context;

type Firewall = Vec<(i32, i32)>;

pub fn parse_input(input: &str) -> anyhow::Result<Firewall> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{}: {}")?))
        .collect()
}

pub fn task1(firewall: &Firewall) -> anyhow::Result<i32> {
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

pub fn task2(firewall: &Firewall) -> anyhow::Result<i32> {
    (0..)
        .find(|delay| {
            firewall
                .iter()
                .all(|(depth, range)| (delay + depth) % (2 * (range - 1)) != 0)
        })
        .context("unreachable")
}
