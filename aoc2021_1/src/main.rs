fn task1(depths: &[u32]) -> usize
{
    depths.windows(2)
          .filter(|&w|w[0]<w[1])
          .count()
}

fn task2(depths: &[u32]) -> usize
{
    depths.windows(4)
         .filter(|&w|w[0]<w[3])
         .count()

}

fn main() {
    let input = aoc::get_input_from_ini_with_year("1","2021").unwrap();
    let depths:Vec<_> = input.lines()
                             .map(|line|line.parse::<u32>().unwrap())
                             .collect();
    println!("Result1: {}", task1(&depths));
    println!("Result2: {}", task2(&depths));
}