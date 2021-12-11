pub fn parse_input(input: &str) -> &str {
    input
}

fn is_valid(line:&str) -> bool
{
    let mut parts = line.split_whitespace();
    let mut range = parts.next().unwrap().split('-');
    let low = range.next().unwrap().parse::<usize>().unwrap();
    let hi = range.next().unwrap().parse::<usize>().unwrap();
    let letter = parts.next().unwrap().chars().next().unwrap();
    let password = parts.next().unwrap();
    let count = password.chars().filter(|&c|c==letter).count();
    low<=count && count<=hi
}

pub fn task1(data: &str) -> usize
{
    data.lines()
        .filter(|&s|is_valid(s))
        .count()
}

fn is_valid2(line:&str) -> bool
{
    let mut parts = line.split_whitespace();
    let mut range = parts.next().unwrap().split('-');
    let low = range.next().unwrap().parse::<usize>().unwrap();
    let hi = range.next().unwrap().parse::<usize>().unwrap();
    let letter = parts.next().unwrap().chars().next().unwrap() as u8;
    let password = parts.next().unwrap().as_bytes();
    (password[low-1]==letter) != (password[hi-1]==letter)
}

pub fn task2(data: &str) -> usize
{
    data.lines()
        .filter(|&s|is_valid2(s))
        .count()
}