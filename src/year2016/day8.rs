pub fn parse_input(input: &str) -> &str {
    input
}

fn print_vec(v:&Vec<Vec<char>>) {
    for j in 0..v[0].len() {
        for i in 0..v.len() {
            print!("{}",v[i][j]);
        }
        println!("");
    }
    println!("");
}

fn build_lcd(rows:usize, columns:usize, commands:&str) -> Vec<Vec<char>> {
    let mut lcd = Vec::with_capacity(columns);
    for _ in 0..columns {
        lcd.push(std::iter::repeat('.').take(rows).collect::<Vec<_>>());
    }
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
						lcd[i][j] = '#';
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
							let temp = lcd[x][rows-1];
							println!("temp={}",temp);
							for j in 1..rows {
                                let y = rows-j;
								lcd[x][y]=lcd[x][y-1];
							}
							lcd[x][0] = temp;
						}
					},
					Some("row") => {
						let y = words.next().unwrap()[2..].parse::<usize>().unwrap();
						let shift = words.skip(1).next().unwrap().parse::<usize>().unwrap();
						for _ in 0..shift {
							let temp = lcd[columns-1][y];
							for j in 1..columns {
                                let x = columns-j;
                                lcd[x][y]=lcd[x-1][y];
							}
							lcd[0][y] = temp;
						}

					},
					_ => panic!("wrong command afrer rotate"),
				}
			},
			_ => panic!("wrong command"),
		}
		print_vec(&lcd);
	}
    print_vec(&lcd);
	lcd
}

pub fn task1(commands:&str)->usize {
	build_lcd(6,50,commands).iter()
							.map(|line|line.iter()
										   .filter(|c|**c=='#')
										   .count()
								)
	                        .sum()
}


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_pixels()
	{
		let px = [".#..#.#",
				  "#.#....",
				  ".#....."];

		let lcd: Vec<Vec<char>> = (0..7).map(|col|
				(0..3).map(|row|px[row].chars().nth(col).unwrap()).collect()
				)
				.collect();
	    
		let commands = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
		assert_eq!(build_lcd(3,7,commands), lcd);
	}
}