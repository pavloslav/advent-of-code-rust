use crate::*;

#[derive(Default)]
struct Handful {
    red: usize,
    green: usize,
    blue: usize,
}

impl Handful {
    fn acceptable(&self, max: &Handful) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }
    fn max(&self, other: &Handful) -> Self {
        Handful {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

pub struct Game {
    id: usize,
    grabs: Vec<Handful>,
}

impl Game {
    fn possible_power(&self) -> usize {
        self.grabs
            .iter()
            .fold(Handful::default(), |acc, h| acc.max(h))
            .power()
    }
}

pub fn parse_input(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .map(|line| {
            let (id, grabs) = scan_fmt::scan_fmt!(line, "Game {}: {/.*/}{e}", usize, String)?;
            let grabs = grabs
                .split("; ")
                .map(|cubes| {
                    let mut handful = Handful::default();
                    for cube in cubes.split(", ") {
                        let (number, class) = scan_fmt::scan_fmt!(cube, "{} {}", usize, String)?;
                        match class.as_str() {
                            "red" => handful.red = number,
                            "blue" => handful.blue = number,
                            "green" => handful.green = number,
                            _ => return Err(aoc_error!("Failed to understand {cube}")),
                        }
                    }
                    Ok(handful)
                })
                .collect::<Result<_>>()?;
            Ok(Game { id, grabs })
        })
        .collect::<Result<_>>()
}

pub fn task1(input: &[Game]) -> Result<usize> {
    let max_game = Handful {
        red: 12,
        green: 13,
        blue: 14,
    };

    Ok(input
        .iter()
        .filter_map(|game| {
            game.grabs
                .iter()
                .all(|handful| handful.acceptable(&max_game))
                .then_some(game.id)
        })
        .sum())
}

pub fn task2(input: &[Game]) -> Result<usize> {
    Ok(input.iter().map(|game| game.possible_power()).sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let games = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let inp = parse_input(games);
        assert!(inp.is_ok());
        assert_eq!(task1(&inp.unwrap()).unwrap(), 8);
    }
}
