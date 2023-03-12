pub fn knots_hash(
    rounds: usize,
    size: usize,
    lengths: impl Iterator<Item = usize> + Clone,
) -> Vec<usize> {
    let mut knots: Vec<_> = (0..size).collect();
    let mut position = 0;
    let mut skip = 0;
    for _round in 0..rounds {
        for length in lengths.clone() {
            for i in 0..length / 2 {
                knots.swap(
                    (position + i) % size,
                    (position + length - 1 - i) % size,
                );
            }
            position = (position + length + skip) % size;
            skip += 1;
        }
    }
    knots
}

const ROUNDS: usize = 64;
pub const SIZE: usize = 256;
const CHUNK_SIZE: usize = 16;
const SUFFIX: [usize; 5] = [17, 31, 73, 47, 23];

pub fn dense_hash(input: impl Iterator<Item = usize> + Clone) -> Vec<u8> {
    knots_hash(ROUNDS, SIZE, input.chain(SUFFIX.into_iter()))
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x as u8))
        .collect()
}
