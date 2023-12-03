use crate::*;

fn simulate(mut state: [usize; 9], time: usize) -> usize {
    for _ in 0..time {
        let giving_birth = state[0];
        for i in 0..8 {
            state[i] = state[i + 1];
        }
        state[6] += giving_birth;
        state[8] = giving_birth;
    }
    state.iter().sum()
}

pub fn parse_input(input: &str) -> AocResult<[usize; 9]> {
    let mut state = [0; 9];
    for fish in input.split(',').map(|x| x.parse::<usize>()) {
        state[fish?] += 1;
    }
    Ok(state)
}

const SIZE1: usize = 80;
const SIZE2: usize = 256;

pub fn task1(state: &[usize; 9]) -> AocResult<usize> {
    Ok(simulate(*state, SIZE1))
}

pub fn task2(state: &[usize; 9]) -> AocResult<usize> {
    Ok(simulate(*state, SIZE2))
}
