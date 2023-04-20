use super::super::common::Error::TaskError;
use super::super::common::Result;

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    build_lcd(6, 50, input)
}

fn collect_lcd(lcd: &[Vec<char>]) -> Result<String> {
    let mut result = String::from("\n");
    for j in 0..lcd[0].len() {
        for line in lcd {
            result.push(line[j]);
        }
        result.push('\n');
    }
    Ok(result)
}

fn build_lcd(
    rows: usize,
    columns: usize,
    commands: &str,
) -> Result<Vec<Vec<char>>> {
    let mut lcd = Vec::with_capacity(columns);
    for _ in 0..columns {
        lcd.push(std::iter::repeat('.').take(rows).collect::<Vec<_>>());
    }
    static INPUT_REGEX: once_cell::sync::Lazy<regex::Regex> =
        once_cell::sync::Lazy::new(|| {
            regex::Regex::new(
    r"(rect (?P<rect_x>\d+)x(?P<rect_y>\d+))|(rotate row y=(?P<row_y>\d+) by (?P<row_shift>\d+))|(rotate column x=(?P<col_x>\d+) by (?P<col_shift>\d+))"
    ).unwrap()
        });

    for command in commands.lines() {
        if let Some(cap) = INPUT_REGEX.captures(command) {
            if let (Some(x), Some(y)) = (cap.name("rect_x"), cap.name("rect_y"))
            {
                let x = x.as_str().parse()?;
                let y = y.as_str().parse()?;
                for line in &mut lcd[..x] {
                    line[..y].fill('#');
                }
            } else if let (Some(y), Some(shift)) =
                (cap.name("row_y"), cap.name("row_shift"))
            {
                let y = y.as_str().parse::<usize>()?;
                let shift = shift.as_str().parse()?;
                for _ in 0..shift {
                    let temp = lcd[columns - 1][y];
                    for j in 1..columns {
                        let x = columns - j;
                        lcd[x][y] = lcd[x - 1][y];
                    }
                    lcd[0][y] = temp;
                }
            } else if let (Some(x), Some(shift)) =
                (cap.name("col_x"), cap.name("col_shift"))
            {
                let x = x.as_str().parse::<usize>()?;
                let shift = shift.as_str().parse()?;

                for _ in 0..shift {
                    let temp = lcd[x][rows - 1];
                    for j in 1..rows {
                        let y = rows - j;
                        lcd[x][y] = lcd[x][y - 1];
                    }
                    lcd[x][0] = temp;
                }
            } else {
                return Err(TaskError(format!(
                    "Can't find all data in command {command}"
                )));
            }
        } else {
            return Err(TaskError(format!(
                "Failed to parse command {command}"
            )));
        }
    }
    Ok(lcd)
}

pub fn task1(lcd: &[Vec<char>]) -> Result<usize> {
    Ok(lcd
        .iter()
        .map(|line| line.iter().filter(|c| **c == '#').count())
        .sum())
}

pub fn task2(lcd: &[Vec<char>]) -> Result<String> {
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
        assert_eq!(build_lcd(3, 7, commands).unwrap(), lcd);
    }
}
