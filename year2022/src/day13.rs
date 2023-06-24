use crate::*;
use serde_json::Value;

pub fn parse_input(input: &str) -> Result<Vec<Value>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect()
}

fn cmp(left: &Value, right: &Value) -> std::cmp::Ordering {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => {
            left.as_i64().unwrap().cmp(&right.as_i64().unwrap())
        }
        (Value::Number(_), Value::Array(_)) => {
            cmp(&Value::Array(vec![left.clone()]), right)
        }
        (Value::Array(_), Value::Number(_)) => {
            cmp(left, &Value::Array(vec![right.clone()]))
        }
        (Value::Array(left), Value::Array(right)) => {
            if let Some(c) = left
                .iter()
                .zip(right.iter())
                .map(|(l, r)| cmp(l, r))
                .find(|&c| c != std::cmp::Ordering::Equal)
            {
                c
            } else {
                left.len().cmp(&right.len())
            }
        }
        _ => unimplemented!(),
    }
}

pub fn task1(input: &[Value]) -> Result<usize> {
    let sum = input
        .chunks(2)
        .enumerate()
        .filter_map(|(i, pair)| {
            if cmp(&pair[0], &pair[1]) == std::cmp::Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum();
    Ok(sum)
}

pub fn task2(input: &[Value]) -> Result<usize> {
    let mut packets = input.to_vec();
    let start: Value = serde_json::from_str("[[2]]")?;
    let end: Value = serde_json::from_str("[[6]]")?;
    packets.push(start.clone());
    packets.push(end.clone());
    packets.sort_by(cmp);
    let start = packets
        .iter()
        .position(|v| v == &start)
        .ok_or_else(|| aoc_error!("start not found"))?
        + 1;
    let end = packets
        .iter()
        .position(|v| v == &end)
        .ok_or_else(|| aoc_error!("end not found"))?
        + 1;
    Ok(start * end)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let inp = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(task1(&parse_input(inp).unwrap()).unwrap(), 13);
    }
}
