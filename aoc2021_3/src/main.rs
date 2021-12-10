fn task1(numbers: &[usize], size: usize) -> usize
{
    let (gamma, epsilon) = (0..size).rev()
                                    .map(|i|numbers.iter()
                                                   .filter(|&n|(n>>i)&1==1)
                                                   .count())
                                    .fold((0,0),|(gamma, epsilon), count|
                                    (
                                        (gamma  <<1) | (2*count> numbers.len()) as usize, 
                                        (epsilon<<1) | (2*count<=numbers.len()) as usize,
                                    ));
    gamma*epsilon
}

fn find_by_bit_criteria<F>(numbers: &[usize], size: usize, criteria: F) -> usize
    where F: Fn(usize, bool) -> bool
{
    let mut numbers = Vec::from(numbers);
    for i in (0..size).rev() {
        if numbers.len() == 1 {
            break;
        }
        let more_ones = numbers.iter()
                               .filter(|&n|(n>>i)&1==1)
                               .count()*2 >= numbers.len();
        numbers = numbers.into_iter()
                         .filter(|&n|criteria((n>>i)&1, more_ones))
                         .collect();
    }
    numbers[0]
}

fn task2(numbers: &[usize], size: usize) -> usize
{
    let oxygen = find_by_bit_criteria(numbers, size, 
        |digit, more_ones| (digit == 1) == more_ones);
    let co2 = find_by_bit_criteria(numbers, size, 
        |digit, more_ones| (digit == 0) == more_ones);
    oxygen * co2
}

fn main() {
    let input = aoc::get_input_from_ini_with_year("3","2021").unwrap();
    let mut size = 0;
    let numbers:Vec<_> = input.lines()
                              .map(|line|{size=line.len();usize::from_str_radix(line, 2).unwrap()})
                              .collect();
    println!("Result1: {}", task1(&numbers, size));
    println!("Result2: {}", task2(&numbers, size));
}