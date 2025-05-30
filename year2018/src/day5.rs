pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input.trim())
}

fn reduce(polymer: Vec<u8>) -> usize {
    let mut polymer = polymer;
    loop {
        let mut new_polymer = Vec::with_capacity(polymer.len());
        let mut skip = false;
        for i in 0..polymer.len() - 1 {
            if !skip {
                if polymer[i].eq_ignore_ascii_case(&polymer[i + 1]) && polymer[i] != polymer[i + 1]
                {
                    skip = true;
                } else {
                    new_polymer.push(polymer[i]);
                }
            } else {
                skip = false;
            }
        }
        if !skip {
            new_polymer.push(polymer[polymer.len() - 1]);
        }
        if polymer.len() == new_polymer.len() {
            break;
        }
        polymer = new_polymer;
    }
    polymer.len()
}

pub fn task1(input: &str) -> anyhow::Result<usize> {
    Ok(reduce(input.as_bytes().to_vec()))
}

pub fn task2(input: &str) -> anyhow::Result<usize> {
    (b'a'..=b'z')
        .map(|out| {
            reduce(
                input
                    .bytes()
                    .filter(|&b| b.to_ascii_lowercase() != out)
                    .collect(),
            )
        })
        .min()
        .ok_or(anyhow::anyhow!("Empty polymer!"))
}
