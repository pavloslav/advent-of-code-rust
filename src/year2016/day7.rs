use super::aoc::*;

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

fn is_abba(s: &str) -> bool {
    let b = s.as_bytes();
    (0..(s.len() - 3))
        .any(|i| b[i] == b[i + 3] && b[i + 1] == b[i + 2] && b[i] != b[i + 1])
}

fn has_abba(ip: &str) -> bool {
    let parts: (Vec<_>, Vec<_>) = ip
        .split(|c: char| !c.is_alphanumeric())
        .enumerate()
        .map(|x| (x.0, is_abba(x.1)))
        .partition(|x| x.0 % 2 == 0);
    parts.0.iter().any(|x| x.1) && parts.1.iter().all(|x| !x.1)
}

pub fn task1(lines: &str) -> Result<usize> {
    Ok(lines.lines().filter(|line| has_abba(line)).count())
}

use std::collections::HashSet;

fn has_aba(ip: &str) -> bool {
    let mut in_supernet = true;
    let mut supernets = Vec::new();
    let mut hypernets = Vec::new();
    ip.split(|c: char| !c.is_alphanumeric())
        .map(|ip| {
            if in_supernet {
                supernets.push(ip)
            } else {
                hypernets.push(ip)
            }
            in_supernet = !in_supernet
        })
        .all(|_| true);
    let mut abs = HashSet::new();
    for s in supernets {
        for i in 0..(s.len() - 2) {
            if s[i..i + 1] == s[i + 2..i + 3] && s[i..i + 1] != s[i + 1..i + 2]
            {
                abs.insert(s[i..i + 2].to_string());
            }
        }
    }
    let mut bas = HashSet::new();
    for s in hypernets {
        for i in 0..(s.len() - 2) {
            if s[i..i + 1] == s[i + 2..i + 3] && s[i..i + 1] != s[i + 1..i + 2]
            {
                bas.insert(s[i + 1..i + 3].to_string());
            }
        }
    }

    !abs.is_disjoint(&bas)
}

pub fn task2(lines: &str) -> Result<usize> {
    Ok(lines.lines().filter(|line| has_aba(line)).count())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_abba() {
        assert!(has_abba("abba[mnop]qrst"));
        assert!(!has_abba("abcd[bddb]xyyx"));
        assert!(!has_abba("aaaa[qwer]tyui"));
        assert!(has_abba("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_has_aba() {
        assert!(has_aba("aba[bab]xyz"));
        assert!(!has_aba("xyx[xyx]xyx"));
        assert!(has_aba("aaa[kek]eke"));
        assert!(has_aba("zazbz[bzb]cdb"));
    }
}
