pub fn parse_input(input: &str) -> anyhow::Result<Vec<(i32, i32)>> {
    input
        .trim()
        .split(',')
        .map(|s| {
            Ok(match s {
                "nw" => (-1, 0),
                "n" => (0, -1),
                "ne" => (1, -1),
                "se" => (1, 0),
                "s" => (0, 1),
                "sw" => (-1, 1),
                other => Err(anyhow::anyhow!("Unknown direction: {other}"))?,
            })
        })
        .collect()
}

pub fn task1(input: &[(i32, i32)]) -> anyhow::Result<i32> {
    let mut position = (0, 0);
    for step in input {
        position = (position.0 + step.0, position.1 + step.1);
    }
    Ok((position.0.abs() + position.1.abs() + (position.0 + position.1).abs()) / 2)
}

pub fn task2(input: &[(i32, i32)]) -> anyhow::Result<i32> {
    let mut position = (0, 0);
    let mut max = 0;
    for step in input {
        position = (position.0 + step.0, position.1 + step.1);
        max = max.max((position.0.abs() + position.1.abs() + (position.0 + position.1).abs()) / 2)
    }
    Ok(max)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        for (inp, res) in [
            ("ne,ne,ne", 3),
            ("ne,ne,sw,sw", 0),
            ("ne,ne,s,s", 2),
            ("se,sw,se,sw,sw", 3),
        ] {
            assert_eq!(task1(&parse_input(inp).unwrap()).unwrap(), res)
        }
    }
}
