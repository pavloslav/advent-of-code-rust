pub fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input.lines().map(|x| Ok(x.parse()?)).collect()
}

pub fn task1(input: &[i32]) -> anyhow::Result<i32> {
    let mut mixed = input.to_vec();
    for &x in input {
        let pos = mixed.iter().position(|&i| i == x).unwrap();
        let mut tgt = x + pos as i32;
        if tgt <= 0 {
            tgt = (tgt - 1).rem_euclid(input.len() as i32);
        }
        if tgt > input.len() as i32 {
            tgt = (tgt + 1) % input.len() as i32;
        }
        mixed.remove(pos);
        mixed.insert(tgt as usize, x);
    }
    let zero = mixed.iter().position(|&i| i == 0).unwrap();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|c| mixed[(zero + c) % mixed.len()])
        .sum())
}

pub fn task2(_input: &[i32]) -> anyhow::Result<usize> {
    todo!();
}

#[cfg(test)]
mod test {
    use super::task1;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&[1, 2, -3, 3, -2, 0, 4]).unwrap(), 3);
    }
}
