use std::collections::BTreeSet;

fn find_first_wrong(numbers:&Vec<i64>, span:usize) -> i64 {
    let mut set:BTreeSet<_> = numbers.iter().take(span).collect();

    for i in span..numbers.len() {
        if !(0..span).any(|j|
            numbers[i-j]*2!=numbers[i] && set.contains(&(numbers[i]-numbers[i-j])) 
            ) {
            return numbers[i]
        }
        set.remove(&(numbers[i-span]));
        set.insert(&(numbers[i]));
    }
    -1
}

fn find_span_adding(numbers:&Vec<i64>, target:i64) -> Option<&[i64]> {
    let mut start = 0;
    let mut sum = numbers[0];
    for end in 1..numbers.len() {
        sum += numbers[end];
        while sum>target && start+1<end {
            sum -= numbers[start];
            start += 1;
        }
        if sum==target {
            return Some(&numbers[start..end]);
        }
    }
    None
}


fn task1(s:&str) -> i64 {
    let numbers:Vec<i64> = s.lines().map(|x|x.parse().unwrap()).collect();
    find_first_wrong(&numbers, 25)
}

//145997291 - low
//2984417418 - high
fn task2(s:&str, span:usize) -> i64 {
    let numbers:Vec<i64> = s.lines().map(|x|x.parse().unwrap()).collect();
    let weakness = find_first_wrong(&numbers, span);
    //println!("Looking for two numbers with span adding to {}",weakness);
    let span = find_span_adding(&numbers, weakness);
    span.map_or(-1,|arr|arr.iter().min().unwrap()+arr.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
     use crate::task2;
    #[test]
    fn test_task2() {
        let input1 = 
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(task2(input1, 5), 62);
    }
}


fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("9","2020").unwrap();
    println!("Result1: {}",task1(&input));
    println!("Result2: {}",task2(&input, 25));
}