pub fn parse_input(input: &str) -> &str {
    input
}

fn slide_the_toboggan(s:&str, xshift:usize, yshift:usize) -> usize
{
	s.lines()
	 .step_by(yshift)
	 .enumerate()
	 .filter(|(y, line)|
	 	line.chars().skip((xshift*y)%line.len()).next().unwrap() == '#')
	 .count()
}

pub fn task1(s:&str) -> usize
{
	slide_the_toboggan(s, 3, 1)
}

pub fn task2(s:&str) -> usize
{
	  slide_the_toboggan(s, 1, 1)
	* slide_the_toboggan(s, 3, 1)
	* slide_the_toboggan(s, 5, 1)
	* slide_the_toboggan(s, 7, 1)
	* slide_the_toboggan(s, 1, 2)
	* slide_the_toboggan(s, 7, 1)
}
