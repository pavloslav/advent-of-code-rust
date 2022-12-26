fn from_snafu(input: &str) -> i64 {
    input
        .chars()
        .map(|c| match c {
            '=' => -2,
            '-' => -1,
            c if c.is_ascii_digit() => c.to_digit(10).unwrap() as i64,
            _ => panic!("Wrong digit: {c}"),
        })
        .fold(0, |acc, d| acc * 5 + d)
}

fn to_snafu(input: i64) -> String {
    let mut n = input;
    let mut result = vec![];
    while n != 0 {
        let d = n % 5;
        if d <= 2 {
            result.push(char::from_digit(d as u32, 10).unwrap());
            n /= 5;
        } else {
            result.push(if d == 3 { '=' } else { '-' });
            n /= 5;
            n += 1;
        }
    }
    result.iter().rev().collect()
}

pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(from_snafu).collect()
}

pub fn task1(input: &[i64]) -> String {
    to_snafu(input.iter().sum())
}

pub fn task2(_input: &[i64]) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = &"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input(EXAMPLE)), "2=-1=0");
    }

    #[test]
    fn test_from_snafu() {
        for (snafu, n) in [
            ("0", 0),
            ("2", 2),
            ("1=", 3),
            ("1-", 4),
            ("10", 5),
            ("22", 12),
            ("1==", 13),
            ("1-12", 107),
            ("1=-0-2", 1747),
        ] {
            assert_eq!(from_snafu(snafu), n);
        }
    }

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1747), "1=-0-2");
        assert_eq!(to_snafu(107), "1-12");
        assert_eq!(to_snafu(4890), "2=-1=0");
    }
}
