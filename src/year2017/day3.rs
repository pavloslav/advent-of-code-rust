pub fn parse_input(input: &str) -> u32 {
    input
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("Should be a number, got '{}'", input))
}

pub fn task1(&input: &u32) -> u32 {
    let inner = (((input - 1) as f64).sqrt() as i32 + 1) / 2;
    let start = (2 * inner - 1).pow(2);
    let step = 2 * inner;
    let location_in_side = (input as i32 - start - 1) % step;
    (inner + (1 + location_in_side - inner).abs()) as u32
}

pub fn task2(&input: &u32) -> u32 {
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
                    return new_value;
                }
                values.insert((x, y), new_value);
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&2), 1);
        assert_eq!(task1(&3), 2);
        assert_eq!(task1(&4), 1);
        assert_eq!(task1(&5), 2);
        assert_eq!(task1(&6), 1);
        assert_eq!(task1(&7), 2);
        assert_eq!(task1(&8), 1);
        assert_eq!(task1(&9), 2);
    }
}
