pub fn parse_input(input: &str) -> anyhow::Result<Vec<&str>> {
    Ok(input.lines().collect())
}

pub fn task1(input: &[&str]) -> anyhow::Result<usize> {
    let mut two = 0;
    let mut three = 0;
    for code in input {
        let mut counter = [0; 26];
        for letter in code.as_bytes() {
            if letter.is_ascii_lowercase() {
                counter[(letter - b'a') as usize] += 1;
            } else {
                anyhow::bail!("Incorrect letter '{letter}'");
            }
        }
        if counter.contains(&2) {
            two += 1;
        }
        if counter.contains(&3) {
            three += 1;
        }
    }
    Ok(two * three)
}

pub fn task2(input: &[&str]) -> anyhow::Result<String> {
    for (i, line1) in input.iter().enumerate() {
        for line2 in &input[i..] {
            if line1
                .bytes()
                .zip(line2.bytes())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            {
                return Ok(line1
                    .bytes()
                    .zip(line2.bytes())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a as char)
                    .collect());
            }
        }
    }
    anyhow::bail!("Not found")
}
