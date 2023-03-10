pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

pub fn task1(input: &[i32]) -> usize {
    let mut jumps = Vec::from(input);
    let mut ip = 0;
    for step in 0usize.. {
        if !(0..jumps.len() as i32).contains(&ip) {
            return step;
        }
        let jmp = jumps[ip as usize];
        jumps[ip as usize] += 1;
        ip += jmp;
    }
    unreachable!()
}

pub fn task2(input: &[i32]) -> usize {
    let mut jumps = Vec::from(input);
    let mut ip = 0;
    for step in 0usize.. {
        if !(0..jumps.len() as i32).contains(&ip) {
            return step;
        }
        let jmp = jumps[ip as usize];
        jumps[ip as usize] += if jmp < 3 { 1 } else { -1 };
        ip += jmp;
    }
    unreachable!()
}
