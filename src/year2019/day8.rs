pub fn parse_input(input: &str) -> &str {
    input
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXELS: usize = WIDTH * HEIGHT;

pub fn task1(input: &str) -> usize {
    let (_, ones, twos) = (0..input.len() / PIXELS)
        .map(|i| {
            let image = &input[i * PIXELS..(i + 1) * PIXELS];
            let zeroes = image.chars().filter(|&c| c == '0').count();
            let ones = image.chars().filter(|&c| c == '1').count();
            let twos = image.chars().filter(|&c| c == '2').count();
            (zeroes, ones, twos)
        })
        .min_by_key(|&(z, _, _)| z)
        .unwrap();
    ones * twos
}

fn decode(c: char) -> char {
    match c {
        '0' => ' ',
        '1' => 'X',
        '2' => '-',
        _ => unimplemented!(),
    }
}

pub fn task2(input: &str) -> String {
    let result = (0..input.len() / PIXELS)
        .map(|i| &input[i * PIXELS..(i + 1) * PIXELS])
        .fold(
            std::iter::repeat('-').take(PIXELS).collect::<String>(),
            |acc, image| {
                acc.chars()
                    .zip(image.chars())
                    .map(|(a, i)| if a == '-' { decode(i) } else { a })
                    .collect()
            },
        );
    (0..HEIGHT)
        .map(|i| result[WIDTH * i..WIDTH * (i + 1)].to_string() + "\n")
        .collect()
}
