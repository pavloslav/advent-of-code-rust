use super::computer::Computer;

pub fn parse_input(input: &str) -> Vec<isize> {
    Computer::prepare_code(input)
}

use itertools::Itertools;

pub fn task1(code: &[isize]) -> isize {
    (0..5)
        .permutations(5)
        .map(|perm| {
            let mut data = 0;
            for phase in perm {
                let mut amplifier = Computer::new(code);
                amplifier.write(phase);
                amplifier.write(data);
                amplifier.run();
                data = amplifier.read().unwrap();
            }
            data
        })
        .max()
        .unwrap()
}

pub fn task2(code: &[isize]) -> isize {
    (5..10)
        .permutations(5)
        .map(|perm| {
            let mut amplifiers = Vec::new();
            for phase in perm {
                let mut amplifier = Computer::new(code);
                amplifier.write(phase);
                amplifiers.push(amplifier);
            }
            amplifiers[0].write(0);
            let mut changes = true;
            let mut last_data = -1;
            while changes {
                changes = false;
                for i in 0..5 {
                    amplifiers[i].run();
                    while let Some(data) = amplifiers[i].read() {
                        if i == 4 {
                            last_data = data;
                        }
                        amplifiers[(i + 1) % 5].write(data);
                        changes = true;
                    }
                }
            }
            last_data
        })
        .max()
        .unwrap()
}
