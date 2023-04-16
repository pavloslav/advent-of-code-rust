use super::super::common::Result;
use super::Error::TaskError;

const TOTAL_SPOONS: usize = 100;
const TOTAL_CALORIES: i32 = 500;

pub struct Ingridient {
    properties: Vec<i32>,
    calories: i32,
}

impl Ingridient {
    fn try_new(input: &str) -> Result<Ingridient> {
        //Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8

        let (capacity, durability, flavor, texture, calories) = scan_fmt::scan_fmt!(input, "{*}: capacity {}, durability {}, flavor {}, texture {}, calories {}", i32, i32, i32, i32, i32)
        .map_err(|_|TaskError(format!("Incorrect line: {input}")))?;
        Ok(Ingridient {
            properties: vec![capacity, durability, flavor, texture],
            calories,
        })
    }
}

pub struct IngridientList {
    ingridients: Vec<Ingridient>,
}

pub fn parse_input(input: &str) -> Result<IngridientList> {
    Ok(IngridientList {
        ingridients: (input
            .lines()
            .map(Ingridient::try_new)
            .collect::<Result<_>>())?,
    })
}

impl IngridientList {
    fn score(&self, spoons: &Vec<usize>) -> i32 {
        assert_eq!(self.ingridients.len(), spoons.len());
        (0..spoons.len())
            .map(|i| {
                std::cmp::max(
                    0,
                    self.ingridients
                        .iter()
                        .zip(spoons.iter())
                        .map(|(ingridient, &spoon)| {
                            ingridient.properties[i] * spoon as i32
                        })
                        .sum::<i32>(),
                )
            })
            .product()
    }
    fn calories(&self, spoons: &Vec<usize>) -> i32 {
        assert_eq!(self.ingridients.len(), spoons.len());
        self.ingridients
            .iter()
            .zip(spoons.iter())
            .map(|(ing, &spoon)| ing.calories * spoon as i32)
            .sum()
    }
}

pub fn task1(ingridients: &IngridientList) -> Result<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if stack.is_empty() {
                break 'search;
            }
            *stack.last_mut().ok_or_else(|| {
                TaskError("Stack became empty".to_string())
            })? += 1;
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else {
            *stack.last_mut().ok_or_else(|| {
                TaskError("Stack became empty".to_string())
            })? += 1;
        }
        if stack.iter().sum::<usize>() <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - stack.iter().sum::<usize>());
            max = std::cmp::max(max, ingridients.score(&stack));
            stack.pop();
        }
    }
    Ok(max)
}

pub fn task2(ingridients: &IngridientList) -> Result<i32> {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if stack.is_empty() {
                break 'search;
            }
            *stack.last_mut().ok_or_else(|| {
                TaskError("Stack became empty".to_string())
            })? += 1;
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else {
            *stack.last_mut().ok_or_else(|| {
                TaskError("Stack became empty".to_string())
            })? += 1;
        }
        if stack.iter().sum::<usize>() <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - stack.iter().sum::<usize>());
            if ingridients.calories(&stack) == TOTAL_CALORIES {
                max = std::cmp::max(max, ingridients.score(&stack));
            }
            stack.pop();
        }
    }
    Ok(max)
}
