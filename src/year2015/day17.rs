pub fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

const SIZE: usize = 150;

pub fn task1(input: &Vec<usize>) -> usize {
    let mut result = 0;
    for i in 0..(1 << input.len()) {
        if input
            .iter()
            .enumerate()
            .filter_map(
                |(j, &x)| if (i >> j) & 1 == 1 { Some(x) } else { None },
            )
            .sum::<usize>()
            == SIZE
        {
            result += 1;
        }
    }
    result
}

pub fn task2(input: &Vec<usize>) -> usize {
    let mut result = 0;
    let mut best = input.len();
    for i in 0_usize..(1 << input.len()) {
        let count = i.count_ones() as usize;
        if count <= best
            && input
                .iter()
                .enumerate()
                .filter_map(
                    |(j, &x)| if (i >> j) & 1 == 1 { Some(x) } else { None },
                )
                .sum::<usize>()
                == SIZE
        {
            if count == best {
                result += 1;
            } else {
                best = count;
                result = 1;
            }
        }
    }
    result
}
