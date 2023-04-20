use super::Error::TaskError;
use super::Result;

pub fn parse_input(input: &str) -> Result<u32> {
    Ok(input.trim().parse()?)
}

pub fn task1(&input: &u32) -> Result<u32> {
    let inner = (((input - 1) as f64).sqrt() as i32 + 1) / 2;
    let start = (2 * inner - 1).pow(2);
    let step = 2 * inner;
    let location_in_side = (input as i32 - start - 1) % step;
    Ok((inner + (1 + location_in_side - inner).abs()) as u32)
}

pub fn task2(&input: &u32) -> Result<u32> {
    let mut values =
        std::collections::HashMap::<(i32, i32), u32>::from([((0, 0), 1)]);
    #[rustfmt::skip]
    let neighbors = [(-1, -1), (0, -1), (1, -1),
                     (-1,  0),          (1,  0),
                     (-1,  1), (0,  1), (1,  1)];

    let mut x = 0;
    let mut y = 0;
    for level in 0.. {
        for dir in 0..4 {
            let steps = 2 * level + 1 + dir / 2;
            for _ in 0..steps {
                match dir {
                    0 => x += 1,
                    1 => y += 1,
                    2 => x -= 1,
                    3 => y -= 1,
                    _ => unreachable!(),
                }
                let new_value = neighbors
                    .iter()
                    .map(|(dx, dy)| values.get(&(x + dx, y + dy)).unwrap_or(&0))
                    .sum();
                if new_value > input {
                    return Ok(new_value);
                }
                values.insert((x, y), new_value);
            }
        }
    }
    Err(TaskError("Failed to find the answer".to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&2).unwrap(), 1);
        assert_eq!(task1(&3).unwrap(), 2);
        assert_eq!(task1(&4).unwrap(), 1);
        assert_eq!(task1(&5).unwrap(), 2);
        assert_eq!(task1(&6).unwrap(), 1);
        assert_eq!(task1(&7).unwrap(), 2);
        assert_eq!(task1(&8).unwrap(), 1);
        assert_eq!(task1(&9).unwrap(), 2);
    }
}
