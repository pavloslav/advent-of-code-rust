pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn fuel_needed(mass: i64) -> i64 {
    mass / 3 - 2
}

pub fn task1(data: &[i64]) -> i64 {
    data.iter().copied().map(fuel_needed).sum()
}

fn fuel_needed_for_fuel(mass: i64) -> i64 {
    (0..)
        .scan(mass, |m, _| {
            *m = fuel_needed(*m);
            if *m > 0 {
                Some(*m)
            } else {
                None
            }
        })
        .sum()
}

pub fn task2(data: &[i64]) -> i64 {
    data.iter().copied().map(fuel_needed_for_fuel).sum()
}
