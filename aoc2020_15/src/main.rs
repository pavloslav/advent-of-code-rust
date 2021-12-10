use std::collections::BTreeMap;

fn memory_game(s:&str, turn: u64) -> u64
{
    let mut input = s.split(',').map(|line|line.parse().unwrap());
    let mut numbers = BTreeMap::new();
    let mut last_spoken:u64 = 0;
    for i in 0..turn {
        let new = input.next()
                           .unwrap_or_else(||
            i-*numbers.get(&last_spoken).unwrap_or(&i)

        );
        //println!("{}: {:?}, {}",i, numbers, last_spoken);
        if i>0 {
            numbers.insert(last_spoken, i);
        }
        last_spoken = new;
    }
    last_spoken
}

fn task1(s:&str) -> u64
{
    memory_game(s, 2020)
}

fn task2(s:&str) -> u64
{
    memory_game(s, 30000000)
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("15","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input));
}