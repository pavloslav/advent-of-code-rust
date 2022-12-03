pub struct Data {
    numbers: Vec<usize>,
    size: usize,
}

pub fn parse_input(input: &str) -> Data {
    let mut size = 0;
    Data {
        numbers: input
            .lines()
            .map(|line| {
                size = line.len();
                usize::from_str_radix(line, 2).unwrap()
            })
            .collect(),
        size,
    }
}

pub fn task1(data: &Data) -> usize {
    let (gamma, epsilon) = (0..data.size)
        .rev()
        .map(|i| data.numbers.iter().filter(|&n| (n >> i) & 1 == 1).count())
        .fold((0, 0), |(gamma, epsilon), count| {
            (
                (gamma << 1) | (2 * count > data.numbers.len()) as usize,
                (epsilon << 1) | (2 * count <= data.numbers.len()) as usize,
            )
        });
    gamma * epsilon
}

fn find_by_bit_criteria<F>(data: &Data, criteria: F) -> usize
where
    F: Fn(usize, bool) -> bool,
{
    let mut numbers = data.numbers.clone();
    for i in (0..data.size).rev() {
        if numbers.len() == 1 {
            break;
        }
        let more_ones = numbers.iter().filter(|&n| (n >> i) & 1 == 1).count()
            * 2
            >= numbers.len();
        numbers.retain(|n| criteria((n >> i) & 1, more_ones));
    }
    numbers[0]
}

pub fn task2(data: &Data) -> usize {
    let oxygen = find_by_bit_criteria(data, |digit, more_ones| {
        (digit == 1) == more_ones
    });
    let co2 = find_by_bit_criteria(data, |digit, more_ones| {
        (digit == 0) == more_ones
    });
    oxygen * co2
}
