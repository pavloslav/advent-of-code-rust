use crate::*;

type Counter = std::collections::HashMap<char, usize>;

type Polymer = Vec<char>;
type PolymerRules = std::collections::HashMap<(char, char), char>;

#[derive(Clone)]
pub struct PolymerData {
    polymer: Polymer,
    rules: PolymerRules,
    counters: std::collections::HashMap<(char, char, usize), Counter>,
}

impl PolymerData {
    fn from_str(input: &str) -> Result<PolymerData> {
        let mut lines = input.lines();
        let mut polymer_data = PolymerData {
            polymer: lines
                .next()
                .ok_or_else(|| aoc_error!("Empty input!"))?
                .chars()
                .collect(),
            rules: std::collections::HashMap::new(),
            counters: std::collections::HashMap::new(),
        };

        for line in lines.skip(1) {
            let (left1, left2, right) =
                scan_fmt::scan_fmt!(line, "{/./}{/./} -> {/./}", char, char, char)?;
            polymer_data.rules.insert((left1, left2), right);
        }
        Ok(polymer_data)
    }

    fn composition_recursive(&mut self, polymer: (char, char), steps: usize) -> Result<Counter> {
        let (left, right) = polymer;

        #[allow(clippy::map_entry)]
        if !self.counters.contains_key(&(left, right, steps)) {
            let counters = if steps == 0 {
                [(right, 1)].into()
            } else {
                match self.rules.get(&(left, right)) {
                    Some(&v) => {
                        let mut left = self.composition_recursive((left, v), steps - 1)?;
                        for (element, count) in self.composition_recursive((v, right), steps - 1)? {
                            *left.entry(element).or_insert(0) += count;
                        }
                        left
                    }
                    None => [(right, 1)].into(),
                }
            };
            self.counters.insert((left, right, steps), counters);
        }
        Ok(self
            .counters
            .get(&(left, right, steps))
            .ok_or_else(|| aoc_error!("Empty counters!"))?
            .clone())
    }

    fn composition(&mut self, steps: usize) -> Result<usize> {
        let mut counter: Counter = [(self.polymer[0], 1)].into();
        for i in 0..self.polymer.len() - 1 {
            for (element, count) in
                self.composition_recursive((self.polymer[i], self.polymer[i + 1]), steps)?
            {
                *counter.entry(element).or_insert(0) += count;
            }
        }

        let min = counter
            .values()
            .min()
            .ok_or_else(|| aoc_error!("Empty counter!"))?;
        let max = counter
            .values()
            .max()
            .ok_or_else(|| aoc_error!("Empty counter!"))?;
        Ok(max - min)
    }
}

pub fn parse_input(input: &str) -> Result<PolymerData> {
    PolymerData::from_str(input)
}

fn task(data: &PolymerData, steps: usize) -> Result<usize> {
    let mut data = data.clone();
    data.composition(steps)
}

pub fn task1(data: &PolymerData) -> Result<usize> {
    task(data, 10)
}

pub fn task2(data: &PolymerData) -> Result<usize> {
    task(data, 40)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task1() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(task1(&parse_input(input).unwrap()).unwrap(), 1588);
    }
}
