pub fn parse_input(input: &str) -> anyhow::Result<Vec<String>> {
    Ok(prse::try_parse!(input.replace('\n', ""), "{:,:}")?)
}

const MULT: u8 = 17;

fn hash(s: &str) -> u32 {
    let mut value = 0u8;
    for c in s.bytes() {
        value = value.overflowing_add(c).0.overflowing_mul(MULT).0;
    }
    value as u32
}

pub fn task1(input: &[String]) -> anyhow::Result<u32> {
    Ok(input.iter().map(|step| hash(step)).sum::<u32>())
}

pub fn task2(input: &[String]) -> anyhow::Result<usize> {
    let mut hash_map = vec![vec![]; 256];
    for instr in input {
        if instr.ends_with('-') {
            let label = &instr[..instr.len() - 1];
            let idx = hash(label) as usize;
            hash_map[idx].retain(|(lens_label, _): &(&str, u8)| lens_label != &label);
        } else if let Some(eq) = instr.find('=') {
            let label = &instr[..eq];
            let idx = hash(label) as usize;
            let focal_length = instr[eq + 1..].parse()?;
            if let Some(lens_idx) = hash_map[idx]
                .iter()
                .position(|(lens_label, _)| lens_label == &label)
            {
                hash_map[idx][lens_idx] = (&label, focal_length);
            } else {
                hash_map[idx].push((&label, focal_length));
            }
        }
    }
    Ok(hash_map
        .iter()
        .enumerate()
        .map(|(i, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(j, (_, focal_strength))| (i + 1) * (j + 1) * *focal_strength as usize)
                .sum::<usize>()
        })
        .sum())
}
