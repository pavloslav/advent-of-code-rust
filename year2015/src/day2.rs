use crate::*;

type Present = (usize, usize, usize);

pub fn parse_input(input: &str) -> AocResult<Vec<Present>> {
    input
        .lines()
        .map(|line| Ok(prse::try_parse!(line, "{}x{}x{}")?))
        .collect()
}

fn wrapping_paper((x, y, z): &(usize, usize, usize)) -> usize {
    let side1 = x * y;
    let side2 = x * z;
    let side3 = y * z;
    2 * (side1 + side2 + side3) + side1.min(side2).min(side3)
}

fn ribbon((x, y, z): &(usize, usize, usize)) -> usize {
    2 * (x + y + z - x.max(y).max(z)) + x * y * z
}

pub fn task1(presents: &[Present]) -> AocResult<usize> {
    Ok(presents.iter().map(wrapping_paper).sum())
}

pub fn task2(presents: &[Present]) -> AocResult<usize> {
    Ok(presents.iter().map(ribbon).sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&parse_input("2x3x4").unwrap()).unwrap(), 58);
        assert_eq!(task1(&parse_input("1x1x10").unwrap()).unwrap(), 43);
    }
    #[test]
    fn test_task2() {
        assert_eq!(task2(&parse_input("2x3x4").unwrap()).unwrap(), 34);
        assert_eq!(task2(&parse_input("1x1x10").unwrap()).unwrap(), 14);
    }
}
