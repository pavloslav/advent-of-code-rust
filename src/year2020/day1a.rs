pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(data: &str) -> i64
{
    let mut set = std::collections::BTreeSet::new();
    for line in data.lines() {
        let value = line.parse::<i64>().unwrap();
        if set.contains(&(2020-value)) {
            return value*(2020-value);
        }
        set.insert(value);
    }
    -1
}

pub fn task2(data: &str) -> i64
{
    let mut numbers:Vec<_> = data.lines()
                                 .map(|line|line.parse::<i64>().unwrap())  
                                 .collect();
    let set: std::collections::BTreeSet<_> = numbers.iter().cloned().collect();
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