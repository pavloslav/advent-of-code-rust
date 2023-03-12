pub fn parse_input(input: &str) -> &str {
    input.trim()
}

pub fn task1(input: &str) -> usize {
    (0..128)
        .map(|i| {
            super::knots_hash::dense_hash(
                format!("{input}-{i}").bytes().map(|c| c.into()),
            )
            .iter()
            .map(|x| x.count_ones() as usize)
            .sum::<usize>()
        })
        .sum()
}

pub fn task2(input: &str) -> usize {
    unimplemented!();
}
