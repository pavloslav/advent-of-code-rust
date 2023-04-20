use super::super::common::Error::TaskError;
use super::super::common::Result;

use std::collections::HashMap;

pub fn parse_input(input: &str) -> Result<Vec<HashMap<char, u32>>> {
    let mut statistics: Vec<HashMap<char, u32>> = Vec::new();
    for line in input.lines() {
        if statistics.is_empty() {
            statistics = vec![HashMap::new(); line.len()];
        }
        for (i, c) in line.chars().enumerate() {
            *statistics[i].entry(c).or_insert(0) += 1;
        }
    }

    Ok(statistics)
}

pub fn task1(statistics: &[HashMap<char, u32>]) -> Result<String> {
    statistics
        .iter()
        .map(|hashmap| {
            hashmap
                .iter()
                .max_by_key(|tuple| tuple.1)
                .map(|tuple| *tuple.0)
                .ok_or_else(|| {
                    TaskError("No elements in statistics".to_string())
                })
        })
        .collect()
}

pub fn task2(statistics: &[HashMap<char, u32>]) -> Result<String> {
    statistics
        .iter()
        .map(|hashmap| {
            hashmap
                .iter()
                .min_by_key(|tuple| tuple.1)
                .map(|tuple| *tuple.0)
                .ok_or_else(|| {
                    TaskError("No elements in statistics".to_string())
                })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_by_most() {
        let inp = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        let inp = parse_input(inp).unwrap();
        assert_eq!(task1(&inp).unwrap(), "easter");
        assert_eq!(task2(&inp).unwrap(), "advent");
    }
}
