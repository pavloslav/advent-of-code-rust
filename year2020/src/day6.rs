pub fn parse_input(input: &str) -> anyhow::Result<&str> {
    Ok(input)
}

pub fn task1(s: &str) -> anyhow::Result<u32> {
    let mut counter = 0_u32;
    let mut group = 0_u32;
    for line in s.lines() {
        if line.is_empty() {
            counter += group.count_ones();
            group = 0;
        }
        for c in line.chars() {
            group |= 1 << (c as u32 - 'a' as u32);
        }
    }
    Ok(counter + group.count_ones())
}

pub fn task2(s: &str) -> anyhow::Result<u32> {
    let mut counter = 0_u32;
    let mut group = 0_u32;
    let mut first = true;
    for line in s.lines() {
        if line.is_empty() {
            counter += group.count_ones();
            first = true;
        } else {
            let mut person = 0_u32;
            for c in line.chars() {
                person |= 1 << (c as u32 - 'a' as u32);
            }
            if first {
                group = person;
                first = false;
            } else {
                group &= person;
            }
        }
    }
    Ok(counter + group.count_ones())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task2() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(task2(input).unwrap(), 6);
    }
}
