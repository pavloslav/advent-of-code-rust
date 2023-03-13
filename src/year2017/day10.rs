pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(input: &str) -> usize {
    let knots = super::knots_hash::knots_hash(
        1,
        super::knots_hash::SIZE,
        input.trim().split(',').map(|x| x.parse().unwrap()),
    );
    knots[0] * knots[1]
}

pub fn task2(input: &str) -> String {
    super::knots_hash::dense_hash(input.trim().bytes().map(|c| c.into()))
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
