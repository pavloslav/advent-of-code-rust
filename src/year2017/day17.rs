pub fn parse_input(input: &str) -> usize {
    input.trim().parse().unwrap()
}

pub fn task1(input: &usize) -> usize {
    const SIZE: usize = 2018;
    let mut buffer = Vec::with_capacity(SIZE);
    buffer.push(0);
    let mut position = 0;
    for i in 1..SIZE {
        position = (position + *input + 1) % i;
        buffer.insert(position, i);
    }

    buffer[(position + 1) % buffer.len()]
}

pub fn task2(input: &usize) -> usize {
    const SIZE: usize = 50_000_000;
    let mut zero_position = 0;
    let mut value_after_zero = 0;
    let mut position = 0;
    use std::cmp::Ordering::*;
    for i in 1..SIZE {
        position = (position + *input + 1) % i;
        match zero_position.cmp(&position) {
            Less => (),
            Equal => value_after_zero = i,
            Greater => zero_position += 1,
        }
    }
    value_after_zero
}
