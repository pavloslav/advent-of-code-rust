use crate::*;

const TOTAL_SPOONS: usize = 100;
const TOTAL_CALORIES: i32 = 500;

pub struct Ingridient {
    properties: Vec<i32>,
    calories: i32,
}

impl std::str::FromStr for Ingridient {
    type Err = AocError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8

        let (_, capacity, durability, flavor, texture, calories): (&str, i32, i32, i32, i32, i32) =
            prse::try_parse!(
                s,
                "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}"
            )?;
        Ok(Ingridient {
            properties: vec![capacity, durability, flavor, texture],
            calories,
        })
    }
}

pub struct IngridientList {
    ingridients: Vec<Ingridient>,
}

pub fn parse_input(input: &str) -> AocResult<IngridientList> {
    Ok(IngridientList {
        ingridients: (input.lines().map(|l| l.parse()).collect::<AocResult<_>>())?,
    })
}

impl IngridientList {
    fn score(&self, spoons: &[usize]) -> i32 {
        assert_eq!(self.ingridients.len(), spoons.len());
        (0..spoons.len())
            .map(|i| {
                self.ingridients
                    .iter()
                    .zip(spoons.iter())
                    .map(|(ingridient, &spoon)| ingridient.properties[i] * spoon as i32)
                    .sum::<i32>()
                    .max(0)
            })
            .product()
    }
    fn calories(&self, spoons: &[usize]) -> i32 {
        assert_eq!(self.ingridients.len(), spoons.len());
        self.ingridients
            .iter()
            .zip(spoons.iter())
            .map(|(ing, &spoon)| ing.calories * spoon as i32)
            .sum::<i32>()
            .max(0)
    }
}

pub fn task1(ingridients: &IngridientList) -> AocResult<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if let Some(last) = stack.last_mut() {
                *last += 1;
            } else {
                break 'search;
            }
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else if let Some(last) = stack.last_mut() {
            *last += 1;
        }
        let sum: usize = stack.iter().sum();
        if sum <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - sum);
            max = max.max(ingridients.score(&stack));
            stack.pop();
        }
    }
    Ok(max)
}

pub fn task2(ingridients: &IngridientList) -> AocResult<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if let Some(last) = stack.last_mut() {
                *last += 1;
            } else {
                break 'search;
            }
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else if let Some(last) = stack.last_mut() {
            *last += 1;
        }

        let sum: usize = stack.iter().sum();
        if sum <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - sum);
            if ingridients.calories(&stack) == TOTAL_CALORIES {
                max = max.max(ingridients.score(&stack));
            }
            stack.pop();
        }
    }
    Ok(max)
}
