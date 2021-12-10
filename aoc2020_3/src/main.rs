fn slide_the_toboggan(s:&str, xshift:usize, yshift:usize) -> usize
{
	s.lines()
	 .step_by(yshift)
	 .enumerate()
	 .filter(|(y, line)|
	 	line.chars().skip((xshift*y)%line.len()).next().unwrap() == '#')
	 .count()
}

fn slide1(s:&str) -> usize
{
	slide_the_toboggan(s, 3, 1)
}

fn slide2(s:&str) -> usize
{
	  slide_the_toboggan(s, 1, 1)
	* slide_the_toboggan(s, 3, 1)
	* slide_the_toboggan(s, 5, 1)
	* slide_the_toboggan(s, 7, 1)
	* slide_the_toboggan(s, 1, 2)
	* slide_the_toboggan(s, 7, 1)
}


fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini_with_year("3","2020").unwrap();
    println!("Result1: {}",slide1(&input));
    println!("Result2: {}",slide2(&input));
}