pub fn parse_input(input: &str) -> &str {
    input.trim()
}

use crypto::digest::Digest;
use crypto::md5::Md5;

fn find_hash(init: &str, start_hash: &str) -> usize {
    for i in 0.. {
        let mut md5 = Md5::new();
        md5.input_str(&format!("{}{}", init, i));
        if md5.result_str().starts_with(start_hash) {
            return i;
        }
    }
    0
}

pub fn task1(input: &str) -> usize {
    find_hash(input, "00000")
}

pub fn task2(input: &str) -> usize {
    find_hash(input, "000000")
}
