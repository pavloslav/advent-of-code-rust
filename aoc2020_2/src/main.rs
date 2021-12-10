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

fn count_valid_passwords(data: &String) -> usize
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

fn count_valid_passwords2(data: &String) -> usize
{
    data.lines()
        .filter(|&s|is_valid2(s))
        .count()
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("2","2020").unwrap();
    println!("Result1: {}",count_valid_passwords(&input));
    println!("Result2: {}",count_valid_passwords2(&input));
}