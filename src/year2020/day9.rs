pub struct Data {
    preamble: usize,
    numbers: Vec<i64>,
}

fn find_first_wrong(data: &Data) -> i64 {
    let numbers = &data.numbers;
    let span = data.preamble;
    let mut set: std::collections::BTreeSet<_> = numbers.iter().take(span).collect();

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

fn find_span_adding(data: &Data, target:i64) -> Option<&[i64]> {
    let numbers = &data.numbers;
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

pub fn parse_input(input: &str) -> Data {
    Data {
        preamble: 25,
        numbers: input.lines().map(|x|x.parse().unwrap()).collect(),
    }
}

pub fn task1(data: &Data) -> i64 {
    find_first_wrong(&data)
}

//145997291 - low
//2984417418 - high
pub fn task2(data: &Data) -> i64 {
    let weakness = find_first_wrong(&data);
    //println!("Looking for two numbers with span adding to {}",weakness);
    let span = find_span_adding(&data, weakness);
    span.map_or(-1,|arr|arr.iter().min().unwrap()+arr.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
     use super::*;
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
        assert_eq!(task2( &Data {
            preamble: 5,
            numbers: input1.lines().map(|x|x.parse().unwrap()).collect(),
        }), 62);
    }
}