use super::super::common::Result;
use super::Error::TaskError;

fn can_be_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && (a - b).abs() < c
}

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input.trim())
}

pub fn task1(input: &str) -> Result<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| TaskError(format!("Can't parse input: {e}")))?;
        if can_be_triangle(nums[0], nums[1], nums[2]) {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn task2(input: &str) -> Result<i32> {
    let mut sum = 0;
    let mut part = Vec::new();
    for line in input.lines() {
        let line_ints: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| TaskError(format!("Can't parse input: {e}")))?;
        part.push(line_ints);
        if part.len() == 3 {
            for i in 0..3 {
                if can_be_triangle(part[0][i], part[1][i], part[2][i]) {
                    sum += 1;
                }
            }
            part = Vec::new();
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tri_test() {
        let inp = "5 10 25";
        assert_eq!(task1(inp).unwrap(), 0);
        let inp = "5 3 4";
        assert_eq!(task1(inp).unwrap(), 1);
        let inp = "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603";
        assert_eq!(task2(inp).unwrap(), 6);
    }
}
