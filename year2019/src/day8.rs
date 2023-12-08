pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXELS: usize = WIDTH * HEIGHT;

pub fn task1(input: &str) -> anyhow::Result<usize> {
    if let Some([_, ones, twos]) = (0..input.len() / PIXELS)
        .map(|i| {
            let image = &input[i * PIXELS..(i + 1) * PIXELS];
            ('0'..'3')
                .map(|j| image.chars().filter(|&c| c == j).count())
                .collect::<Vec<_>>()
        })
        .min_by_key(|arr| arr[0])
        .as_deref()
    {
        Ok(ones * twos)
    } else {
        Err(anyhow::anyhow!("Impossible, 3 elements are 3 elements",))
    }
}

fn decode(c: char) -> anyhow::Result<char> {
    Ok(match c {
        '0' => ' ',
        '1' => 'X',
        '2' => '-',
        other => Err(anyhow::anyhow!("Unknown symbol '{other}'"))?,
    })
}

pub fn task2(input: &str) -> anyhow::Result<String> {
    let result = (0..input.len() / PIXELS)
        .map(|i| &input[i * PIXELS..(i + 1) * PIXELS])
        .try_fold("-".repeat(PIXELS), |acc, image| {
            acc.chars()
                .zip(image.chars())
                .map(|(a, i)| if a == '-' { decode(i) } else { Ok(a) })
                .collect::<anyhow::Result<String>>()
        })?;
    Ok((0..HEIGHT)
        .map(|i| result[WIDTH * i..WIDTH * (i + 1)].to_string() + "\n")
        .collect())
}
