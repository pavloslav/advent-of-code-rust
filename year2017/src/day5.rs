pub fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    input.lines().map(|s| Ok(s.parse()?)).collect()
}

pub fn task<F>(input: &[i32], f: F) -> anyhow::Result<usize>
where
    F: Fn(i32) -> i32,
{
    let mut jumps = input.to_vec();
    let mut ip = 0;
    for step in 0usize.. {
        if !(0..jumps.len() as i32).contains(&ip) {
            return Ok(step);
        }
        let jmp = jumps[ip as usize];
        jumps[ip as usize] += f(jmp);
        ip += jmp;
    }
    unreachable!()
}

pub fn task1(input: &[i32]) -> anyhow::Result<usize> {
    task(input, |_| 1)
}

pub fn task2(input: &[i32]) -> anyhow::Result<usize> {
    task(input, |jmp| if jmp < 3 { 1 } else { -1 })
}
