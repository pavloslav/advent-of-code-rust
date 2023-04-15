type Firewall = Vec<(i32, i32)>;

pub fn parse_input(input: &str) -> Firewall {
    input
        .lines()
        .map(|line| scan_fmt::scan_fmt!(line, "{}: {}", i32, i32).unwrap())
        .collect()
}

pub fn task1(firewall: &Firewall) -> i32 {
    firewall
        .iter()
        .map(|(depth, range)| {
            if depth % (2 * (range - 1)) == 0 {
                depth * range
            } else {
                0
            }
        })
        .sum()
}

pub fn task2(firewall: &Firewall) -> i32 {
    (0..)
        .find(|delay| {
            firewall
                .iter()
                .all(|(depth, range)| (delay + depth) % (2 * (range - 1)) != 0)
        })
        .unwrap()
}
