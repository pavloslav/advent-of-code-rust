use crate::*;

pub struct Data {
    numbers: Vec<usize>,
    size: usize,
}

pub fn parse_input(input: &str) -> Result<Data> {
    let mut size = None;
    let numbers = input
        .lines()
        .map(|line| {
            if let Some(s) = size {
                if s != line.len() {
                    return Err(task_error!(
                        "All lines should be same length {}, but one is {s}",
                        line.len()
                    ));
                }
            } else {
                size = Some(line.len());
            }
            Ok(usize::from_str_radix(line, 2)?)
        })
        .collect::<Result<_>>()?;
    let size = size.ok_or_else(|| task_error!("No line length!"))?;
    Ok(Data { numbers, size })
}

pub fn task1(data: &Data) -> Result<usize> {
    let (gamma, epsilon) = (0..data.size)
        .rev()
        .map(|i| data.numbers.iter().filter(|&n| (n >> i) & 1 == 1).count())
        .fold((0, 0), |(gamma, epsilon), count| {
            (
                (gamma << 1) | (2 * count > data.numbers.len()) as usize,
                (epsilon << 1) | (2 * count <= data.numbers.len()) as usize,
            )
        });
    Ok(gamma * epsilon)
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

pub fn task2(data: &Data) -> Result<usize> {
    let oxygen = find_by_bit_criteria(data, |digit, more_ones| {
        (digit == 1) == more_ones
    });
    let co2 = find_by_bit_criteria(data, |digit, more_ones| {
        (digit == 0) == more_ones
    });
    Ok(oxygen * co2)
}
