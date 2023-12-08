fn first_op(s: &str, precedence: bool) -> Option<usize> {
    let mut plus = None;
    let mut mult = None;
    let mut ballance = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => ballance += 1,
            ')' => ballance -= 1,
            '+' => {
                if ballance == 0 {
                    plus = Some(i);
                }
            }
            '*' => {
                if ballance == 0 {
                    mult = Some(i);
                }
            }
            _ => {}
        }
    }
    if precedence {
        mult.or(plus)
    } else {
        plus.max(mult)
    }
}

fn calculate(s: &str, precedence: bool) -> anyhow::Result<u64> {
    Ok(if let Ok(val) = s.parse() {
        val
    } else if let Some(op_idx) = first_op(s, precedence) {
        let left = s[..op_idx].trim();
        let op = s[op_idx..op_idx + 1].trim();
        let right = s[op_idx + 1..].trim();
        match op {
            "+" => calculate(left, precedence)? + calculate(right, precedence)?,
            "*" => calculate(left, precedence)? * calculate(right, precedence)?,
            _ => Err(anyhow::anyhow!(
                "failed with op='{op}' on
    s = '{s}'
    left='{left}'
    right = '{right}'"
            ))?,
        }
    } else if s.len() <= 1 {
        Err(anyhow::anyhow!("failed on s = '{s}'"))?
    } else {
        calculate(s[1..s.len() - 1].trim(), precedence)?
    })
}

pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

pub fn task1(s: &str) -> anyhow::Result<u64> {
    s.lines().map(|line| calculate(line.trim(), false)).sum()
}

pub fn task2(s: &str) -> anyhow::Result<u64> {
    s.lines().map(|line| calculate(line.trim(), true)).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_task1() {
        assert_eq!(task1("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 231);
    }
}
