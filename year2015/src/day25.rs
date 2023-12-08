pub fn parse_input(input: &str) -> anyhow::Result<(usize, usize)> {
    Ok(prse::try_parse!(input,
        "To continue, please consult the code grid in the manual.  Enter the code at row {}, column {}."
        )?)
}

const START: usize = 20151125;
const MULT: usize = 252533;
const MODULUS: usize = 33554393;

pub fn task1((row, col): &(usize, usize)) -> anyhow::Result<usize> {
    let diag = row + col - 1;
    let start_diag = diag * (diag - 1) / 2 + 1;
    let seq_num = start_diag + col - 1;
    let mut code = START;
    for _ in 0..seq_num - 1 {
        code = code * MULT % MODULUS;
    }
    Ok(code)
}

pub fn task2((_row, _col): &(usize, usize)) -> anyhow::Result<&'static str> {
    Ok("Done!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        assert_eq!(task1(&(1, 1)).unwrap(), START);
        assert_eq!(task1(&(6, 1)).unwrap(), 33071741);
    }
}
