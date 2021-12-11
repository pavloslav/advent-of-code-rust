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

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s:&str) -> u64
{
    memory_game(s, 2020)
}

pub fn task2(s:&str) -> u64
{
    memory_game(s, 30000000)
}