use super::computer::Computer;
use anyhow::Context;

pub fn parse_input(input: &str) -> anyhow::Result<Vec<isize>> {
    Computer::prepare_code(input)
}

use itertools::Itertools;

pub fn task1(code: &[isize]) -> anyhow::Result<isize> {
    (0..5)
        .permutations(5)
        .map(|perm| {
            let mut data = 0;
            for phase in perm {
                let mut amplifier = Computer::new(code);
                amplifier.write(phase);
                amplifier.write(data);
                data = amplifier.run()?.read()?;
            }
            Ok(data)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .max()
        .context("Should not be empty!")
}

pub fn task2(code: &[isize]) -> anyhow::Result<isize> {
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
                    amplifiers[i].run()?;
                    while let Ok(data) = amplifiers[i].read() {
                        if i == 4 {
                            last_data = data;
                        }
                        amplifiers[(i + 1) % 5].write(data);
                        changes = true;
                    }
                }
            }
            Ok(last_data)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .max()
        .copied()
        .context("Should not be empty!")
}
