const TOTAL_SPOONS: usize = 100;
const TOTAL_CALORIES: i32 = 500;

pub struct Ingridient {
    properties: Vec<i32>,
    calories: i32,
}

impl Ingridient {
    fn try_new(input: &str) -> Option<Ingridient> {
        lazy_static::lazy_static! {
            //Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
            static ref INPUT_REGEX: regex::Regex = regex::Regex::new(r"\w: capacity (?P<capacity>[-\d]+), durability (?P<durability>[-\d]+), flavor (?P<flavor>[-\d]+), texture (?P<texture>[-\d]+), calories (?P<calories>[-\d]+)").unwrap();
        }
        if let Some(captures) = INPUT_REGEX.captures(input) {
            Some(Ingridient {
                properties: ["capacity", "durability", "flavor", "texture"]
                    .iter()
                    .map(|name| {
                        captures.name(name).unwrap().as_str().parse().unwrap()
                    })
                    .collect(),
                calories: captures
                    .name("calories")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            })
        } else {
            None
        }
    }
}

pub struct IngridientList {
    ingridients: Vec<Ingridient>,
}

pub fn parse_input(input: &str) -> IngridientList {
    IngridientList {
        ingridients: input
            .lines()
            .filter_map(|line| Ingridient::try_new(line))
            .collect(),
    }
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

pub fn task1(ingridients: &IngridientList) -> i32 {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if stack.is_empty() {
                break 'search;
            }
            *stack.last_mut().unwrap() += 1;
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else {
            *stack.last_mut().unwrap() += 1;
        }
        if stack.iter().sum::<usize>() <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - stack.iter().sum::<usize>());
            max = std::cmp::max(max, ingridients.score(&stack));
            stack.pop();
        }
    }
    max
}

pub fn task2(ingridients: &IngridientList) -> i32 {
    let mut stack: Vec<usize> = vec![];
    let mut max = 0;
    'search: loop {
        while stack.iter().sum::<usize>() > TOTAL_SPOONS {
            stack.pop();
            if stack.is_empty() {
                break 'search;
            }
            *stack.last_mut().unwrap() += 1;
        }
        if stack.len() < ingridients.ingridients.len() - 1 {
            stack.resize(ingridients.ingridients.len() - 1, 0);
        } else {
            *stack.last_mut().unwrap() += 1;
        }
        if stack.iter().sum::<usize>() <= TOTAL_SPOONS {
            stack.push(TOTAL_SPOONS - stack.iter().sum::<usize>());
            if ingridients.calories(&stack) == TOTAL_CALORIES {
                max = std::cmp::max(max, ingridients.score(&stack));
            }
            stack.pop();
        }
    }
    max
}
