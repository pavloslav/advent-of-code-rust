pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
         .map(|line|line.parse::<u32>().unwrap())
         .collect()
}

pub fn task1(depths: &[u32]) -> usize
{
    depths.windows(2)
          .filter(|&w|w[0]<w[1])
          .count()
}

pub fn task2(depths: &[u32]) -> usize
{
    depths.windows(4)
         .filter(|&w|w[0]<w[3])
         .count()
}