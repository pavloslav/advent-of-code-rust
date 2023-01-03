use crypto::digest::Digest;
use crypto::md5::Md5;
use itertools::Itertools;

fn successive(s: &str, n: usize) -> Vec<char> {
    s.chars()
        .group_by(|&c| c)
        .into_iter()
        .filter_map(|(k, g)| if g.count() >= n { Some(k) } else { None })
        .collect()
}

fn stretched_hash(src: &str, additional: usize) -> String {
    let mut src = src.to_string();
    let mut md5 = Md5::new();
    for _ in 0..=additional {
        md5.input_str(&src);
        src = md5.result_str();
        md5.reset();
    }
    src
}

fn find_key(init: &str, n: usize, hash_deep: usize) -> usize {
    let mut buffer = std::collections::VecDeque::<String>::new();
    let mut results = std::collections::HashSet::new();
    for i in 0usize.. {
        let hash = stretched_hash(&format!("{init}{i}"), hash_deep);
        for fives in successive(&hash, 5) {
            for (j, old_md5) in buffer.iter().enumerate() {
                if successive(old_md5, 3).first() == Some(&fives) {
                    let result = i.saturating_sub(1000) + j;
                    results.insert(result);
                }
            }
        }
        if results.len() >= n {
            let mut results = Vec::from_iter(results.iter().copied());
            results.sort();
            return results[n - 1];
        }
        buffer.push_back(hash);
        if buffer.len() > 1000 {
            buffer.pop_front();
        }
    }
    unreachable!()
}

pub fn parse_input(input: &str) -> &str {
    input.trim()
}

pub fn task1(input: &str) -> usize {
    find_key(input, 64, 0)
}

pub fn task2(input: &str) -> usize {
    find_key(input, 64, 2016)
}
