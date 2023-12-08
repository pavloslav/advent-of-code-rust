use std::ops::RangeInclusive;

pub fn parse_input(input: &str) -> anyhow::Result<RangeInclusive<usize>> {
    let (start, end) = prse::try_parse!(input, "{}-{}")?;
    Ok(start..=end)
}

pub fn task1(input: &RangeInclusive<usize>) -> anyhow::Result<usize> {
    Ok(input
        .clone()
        .filter(|&x| {
            let mut x = x;
            let mut prev = 10;
            let mut repeat = false;
            while x > 0 {
                let digit = x % 10;
                x /= 10;
                if digit > prev {
                    return false;
                }
                if digit == prev {
                    repeat = true;
                }
                prev = digit;
            }
            repeat
        })
        .count())
}

pub fn task2(input: &RangeInclusive<usize>) -> anyhow::Result<usize> {
    Ok(input
        .clone()
        .filter(|&x| {
            let mut x = x;
            let mut prev = 10;
            let mut repeat = 1;
            let mut has_2_repeated = false;
            while x > 0 {
                let digit = x % 10;
                x /= 10;
                if digit > prev {
                    return false;
                }
                if digit == prev {
                    repeat += 1;
                } else {
                    if repeat == 2 {
                        has_2_repeated = true;
                    }
                    repeat = 1;
                }
                prev = digit;
            }
            repeat == 2 || has_2_repeated
        })
        .count())
}
