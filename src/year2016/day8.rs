pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    build_lcd(6, 50, input)
}

fn collect_lcd(lcd: &[Vec<char>]) -> String {
    let mut result = String::from("\n");
    for j in 0..lcd[0].len() {
        for line in lcd {
            result.push(line[j]);
        }
        result.push('\n');
    }
    result
}

fn build_lcd(rows: usize, columns: usize, commands: &str) -> Vec<Vec<char>> {
    let mut lcd = Vec::with_capacity(columns);
    for _ in 0..columns {
        lcd.push(std::iter::repeat('.').take(rows).collect::<Vec<_>>());
    }
    for command in commands.lines() {
        let mut words = command.split(' ');
        match words.next() {
            Some("rect") => {
                let size: Vec<_> = words
                    .next()
                    .unwrap()
                    .split('x')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                for line in &mut lcd[..size[0]] {
                    line[..size[1]].fill('#');
                }
            }
            Some("rotate") => match words.next() {
                Some("column") => {
                    let x =
                        words.next().unwrap()[2..].parse::<usize>().unwrap();
                    let shift = words.nth(1).unwrap().parse::<usize>().unwrap();
                    for _ in 0..shift {
                        let temp = lcd[x][rows - 1];
                        for j in 1..rows {
                            let y = rows - j;
                            lcd[x][y] = lcd[x][y - 1];
                        }
                        lcd[x][0] = temp;
                    }
                }
                Some("row") => {
                    let y =
                        words.next().unwrap()[2..].parse::<usize>().unwrap();
                    let shift = words.nth(1).unwrap().parse::<usize>().unwrap();
                    for _ in 0..shift {
                        let temp = lcd[columns - 1][y];
                        for j in 1..columns {
                            let x = columns - j;
                            lcd[x][y] = lcd[x - 1][y];
                        }
                        lcd[0][y] = temp;
                    }
                }
                _ => panic!("wrong command afrer rotate"),
            },
            _ => panic!("wrong command"),
        }
    }
    lcd
}

pub fn task1(lcd: &[Vec<char>]) -> usize {
    lcd.iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum()
}

pub fn task2(lcd: &[Vec<char>]) -> String {
    collect_lcd(lcd)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pixels() {
        let px = [".#..#.#", "#.#....", ".#....."];

        let lcd: Vec<Vec<char>> = (0..7)
            .map(|col| {
                (0..3)
                    .map(|row| px[row].chars().nth(col).unwrap())
                    .collect()
            })
            .collect();

        let commands = "\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
        assert_eq!(build_lcd(3, 7, commands), lcd);
    }
}
