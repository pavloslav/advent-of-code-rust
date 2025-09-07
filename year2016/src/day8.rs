pub fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<char>>> {
    build_lcd(6, 50, input)
}

fn collect_lcd(lcd: &[Vec<char>]) -> anyhow::Result<String> {
    let mut result = String::from("\n");
    for j in 0..lcd[0].len() {
        for line in lcd {
            result.push(line[j]);
        }
        result.push('\n');
    }
    Ok(result)
}

fn build_lcd(rows: usize, columns: usize, commands: &str) -> anyhow::Result<Vec<Vec<char>>> {
    let mut lcd = Vec::with_capacity(columns);
    for _ in 0..columns {
        lcd.push(std::iter::repeat_n('.', rows).collect::<Vec<_>>());
    }
    for command in commands.lines() {
        if let Ok((x, y)) = prse::try_parse!(command, "rect {}x{}") {
            for line in &mut lcd[..x] {
                line[..y].fill('#');
            }
        } else if let Ok((y, shift)) = prse::try_parse!(command, "rotate row y={} by {}") {
            let y: usize = y; //prse::try_parse! hint
            for _ in 0..shift {
                let temp = lcd[columns - 1][y];
                for j in 1..columns {
                    let x = columns - j;
                    lcd[x][y] = lcd[x - 1][y];
                }
                lcd[0][y] = temp;
            }
        } else if let Ok((x, shift)) = prse::try_parse!(command, "rotate column x={} by {}") {
            let x: usize = x; //prse::try_parse! hint
            for _ in 0..shift {
                let temp = lcd[x][rows - 1];
                for j in 1..rows {
                    let y = rows - j;
                    lcd[x][y] = lcd[x][y - 1];
                }
                lcd[x][0] = temp;
            }
        } else {
            anyhow::bail!("Can't find all data in command {command}");
        }
    }
    Ok(lcd)
}

pub fn task1(lcd: &[Vec<char>]) -> anyhow::Result<usize> {
    Ok(lcd
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum())
}

pub fn task2(lcd: &[Vec<char>]) -> anyhow::Result<String> {
    collect_lcd(lcd)
}

#[cfg(test)]
mod test {
    use super::build_lcd;
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
        assert_eq!(build_lcd(3, 7, commands).unwrap(), lcd);
    }
}
