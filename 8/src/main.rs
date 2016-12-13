use std::fmt;
impl fmt::Display for Vec<Vec<char>> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter()
		             .map(|v|v.iter()
		             	       .map(|c|c.to_string())
		             	       .collect::<String>()+"\n")
		             .collect::<String>()
        	)
    }
}

fn build_lcd(rows:usize, columns:usize, commands:&str) -> Vec<Vec<char>> {
	let mut lcd: Vec<Vec<char>> = std::iter::repeat(
			   std::iter::repeat('.').take(columns)
			 						 .collect()
			   ).take(rows)
	            .collect();
	for command in commands.lines() {
		println!("Executing \"{}\"...",command);		
		let mut words = command.split(' ');
		match words.next() {
			Some("rect") => {
				let size: Vec<_> = words.next()
				                .unwrap()
				                .split('x')
				                .map(|s|s.parse::<usize>().unwrap())
				                .collect();
				for i in 0..size[0] {
					for j in 0..size[1] {
						lcd[j][i] = '#';
					}
				}

			},
			Some("rotate") => {
				match words.next() {
					Some("column") => {
						let x = words.next().unwrap()[2..].parse::<usize>().unwrap();
						let shift = words.skip(1).next().unwrap().parse::<usize>().unwrap();
						println!("Rotating column {} for {}",x,shift);
						for _ in 0..shift {
							let temp = lcd[rows-1][x];
							println!("temp={}",temp);
							for j in 1..(rows-1) {
								lcd[rows-j][x]=lcd[rows-j-1][x];
							}
							lcd[0][x] = temp;
						}
					},
					Some("row") => {
						let y = words.next().unwrap()[2..].parse::<usize>().unwrap();
						let shift = words.skip(1).next().unwrap().parse::<usize>().unwrap();
						for _ in 0..shift {
							let temp = lcd[y][columns-1];
							for j in 1..(columns-1) {
								lcd[y][columns-j]=lcd[y][columns-j-1];
							}
							lcd[y][0] = temp;
						}

					},
					_ => panic!("wrong command afrer rotate"),
				}
			},
			_ => panic!("wrong command"),
		}
		println!("{}",lcd.iter()
		             .map(|v|v.iter()
		             	       .map(|c|c.to_string())
		             	       .collect::<String>()+"\n")
		             .collect::<String>());
	}
	println!("{}",lcd.iter()
		             .map(|v|v.iter()
		             	       .map(|c|c.to_string())
		             	       .collect::<String>()+"\n")
		             .collect::<String>());
	lcd
}

fn count_pixels(commands:&str)->usize {
	build_lcd(6,50,commands).iter()
							.map(|line|line.iter()
										   .filter(|c|**c=='#')
										   .count()
								)
	                        .sum()
}

fn main() {
    extern crate aoc;
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    println!("Pixels: {}", count_pixels(&input));
}


#[test]
fn test_pixels()
{
	let pixels:Vec<Vec<char>> = "\
.#..#.#
#.#....
.#.....".lines()
        .map(|line|line.chars()
        	           .collect())
        .collect();
	let commands = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
	assert_eq!(pixels, build_lcd(3,7,commands));
}