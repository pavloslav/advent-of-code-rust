use super::super::common::Error::TaskError;
use super::super::common::Result;

pub struct Data {
    preamble: usize,
    numbers: Vec<i64>,
}

fn find_first_wrong(data: &Data) -> i64 {
    let numbers = &data.numbers;
    let span = data.preamble;
    let mut set: std::collections::BTreeSet<_> =
        numbers.iter().take(span).collect();

    for i in span..numbers.len() {
        if !(0..span).any(|j| {
            numbers[i - j] * 2 != numbers[i]
                && set.contains(&(numbers[i] - numbers[i - j]))
        }) {
            return numbers[i];
        }
        set.remove(&(numbers[i - span]));
        set.insert(&(numbers[i]));
    }
    -1
}

fn find_span_adding(data: &Data, target: i64) -> Result<&[i64]> {
    let numbers = &data.numbers;
    let mut start = 0;
    let mut sum = numbers[0];
    for end in 1..numbers.len() {
        sum += numbers[end];
        while sum > target && start + 1 < end {
            sum -= numbers[start];
            start += 1;
        }
        if sum == target {
            return Ok(&numbers[start..end]);
        }
    }
    Err(TaskError("Not found!".to_string()))
}

pub fn parse_input(input: &str) -> Result<Data> {
    Ok(Data {
        preamble: 25,
        numbers: input
            .lines()
            .map(|x| Ok(x.parse()?))
            .collect::<Result<_>>()?,
    })
}

pub fn task1(data: &Data) -> Result<i64> {
    Ok(find_first_wrong(data))
}

//145997291 - low
//2984417418 - high
pub fn task2(data: &Data) -> Result<i64> {
    let weakness = find_first_wrong(data);
    let arr = find_span_adding(data, weakness)?;
    if let (Some(min), Some(max)) = (arr.iter().min(), arr.iter().max()) {
        Ok(min + max)
    } else {
        Err(TaskError("Should be not empty!".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task2() {
        let input1 = "35
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
        assert_eq!(
            task2(&Data {
                preamble: 5,
                numbers: input1.lines().map(|x| x.parse().unwrap()).collect(),
            })
            .unwrap(),
            62
        );
    }
}
