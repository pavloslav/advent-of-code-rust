use crate::*;

fn can_be_triangle(a: i32, b: i32, c: i32) -> bool {
    a + b > c && (a - b).abs() < c
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| Ok(s.parse()?)).collect())
        .collect()
}

pub fn task1(input: &[Vec<i32>]) -> Result<i32> {
    let mut sum = 0;
    for nums in input {
        if can_be_triangle(nums[0], nums[1], nums[2]) {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn task2(input: &[Vec<i32>]) -> Result<i32> {
    let mut sum = 0;
    for i in 0..input.len() / 3 {
        for j in 0..3 {
            if can_be_triangle(
                input[i * 3][j],
                input[i * 3 + 1][j],
                input[i * 3 + 2][j],
            ) {
                sum += 1;
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tri_test() {
        let inp = parse_input("5 10 25").unwrap();
        assert_eq!(task1(&inp).unwrap(), 0);
        let inp = parse_input("5 3 4").unwrap();
        assert_eq!(task1(&inp).unwrap(), 1);
        let inp = parse_input(
            "\
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603",
        )
        .unwrap();
        assert_eq!(task2(&inp).unwrap(), 6);
    }
}
