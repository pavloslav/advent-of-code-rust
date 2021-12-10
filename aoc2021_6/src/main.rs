fn simulate(mut state: [usize; 9], time: usize) -> usize
{
    for _ in 0..time {
        let giving_birth = state[0];
        for i in 0..8 {
            state[i] = state[i+1];
        }
        state[6] += giving_birth;
        state[8] = giving_birth;
    }
    state.iter().sum()
}

fn task1(state: &[usize; 9]) -> usize
{
    simulate(*state, 80)
}


fn task2(state: &[usize; 9]) -> usize
{
    simulate(*state, 256)
}

fn main() {
    let input = aoc::get_input_from_ini_with_year("6","2021").unwrap();
    let mut state = [0; 9];
    for fish in input.split(',').map(|x|x.parse::<usize>().unwrap()) {
        state[fish] += 1;
    };
    println!("Result1: {}", task1(&state));
    println!("Result2: {}", task2(&state));
}