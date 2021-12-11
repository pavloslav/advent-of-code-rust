pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(data: &str) -> i64
{
    data.lines()
        .map(|line|line.parse::<i64>().unwrap()/3-2)
        .sum()
}