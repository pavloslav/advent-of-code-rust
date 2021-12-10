use std::collections::BTreeSet;

fn find_pair(data: &String) -> i64
{
    let mut set = BTreeSet::new();
    for line in data.lines() {
        let value = line.parse::<i64>().unwrap();
        if set.contains(&(2020-value)) {
        	return value*(2020-value);
        }
        set.insert(value);
    }
    -1
}

fn find_three(data: &String) -> i64
{
    let mut numbers:Vec<_> = data.lines()
                                 .map(|line|line.parse::<i64>().unwrap())  
                                 .collect();
    let set:BTreeSet<_> = numbers.iter().cloned().collect();
    numbers.sort();
    for i in 0..numbers.len()-1 {
        let first = numbers[i];
        for second in numbers.iter().skip(i+1) {
            let third = 2020-first-second;
            if set.contains(&third) {
                return first*second*third;
            }
        }
    }
    -1
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("1","2020").unwrap();
    println!("Result1: {}",find_pair(&input));
    println!("Result2: {}",find_three(&input));
}