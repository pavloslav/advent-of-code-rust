use std::collections::HashMap;

struct Rules {
    map: HashMap<Vec<u8>, Vec<u8>>,
    terminals: HashMap<char, u8>,
}

fn process(s: &str) -> (Rules, std::str::Lines) {
    let mut lines = s.lines();
    let mut rules: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    let mut terminals = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        //println!("Parsing line '{}'", line);
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let key = parts.next().unwrap().parse().unwrap();
        for subrule in parts.next().unwrap().split(" | ") {
            //println!("Parsing subrule '{}'", subrule);
            if subrule.starts_with('\"') {
                terminals.insert(subrule.chars().nth(1).unwrap(), key);
            } else {
                let rule = subrule
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                rules.entry(rule).or_default().push(key);
            };
        }
    }
    println!("{} rules parsed", rules.len());
    let mut applicable: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
    let zero = rules
        .iter()
        .find(|(_, v)| v.iter().any(|&x| x == 0))
        .unwrap();
    applicable.insert(zero.0.clone(), vec![0]);
    loop {
        let size = applicable.len();
        for rule in rules.iter() {
            for &target in rule.1.iter() {
                if applicable
                    .iter()
                    .any(|(k, _)| k.iter().any(|&x| x == target))
                {
                    applicable.entry(rule.0.clone()).or_default().push(target);
                }
            }
        }
        if size == applicable.len() {
            break;
        }
    }
    println!("{} applicable rules parsed", applicable.len());

    (
        Rules {
            map: applicable,
            terminals,
        },
        lines,
    )
}

type ParseString = Vec<u8>;

impl Rules {
    fn valid(&self, string: &mut ParseString) -> bool {
        if cfg!(debug_assertions) {
            print!("Valid called for {:?} ", string);
        }
        if string.len() < 2 {
            return *string == vec![0_u8];
        }
        for i in 0..string.len() {
            for size in 1..=3.min(string.len() - i) {
                let old_values = string[i..i + size].to_owned();
                if let Some(new_values) = self.map.get(&old_values) {
                    let save = string.clone();
                    string.splice(i + 1..i + size, std::iter::empty());
                    for &new_value in new_values.iter() {
                        string[i] = new_value;
                        if cfg!(debug_assertions) {
                            println!(" going into {}", string.len());
                        }
                        if self.valid(string) {
                            return true;
                        }
                    }
                    *string = save;
                }
            }
        }
        false
    }

    fn parse_string(&self, s: &str) -> ParseString {
        s.chars().map(|c| self.terminals[&c]).collect()
    }
    fn validate(&self, messages: std::str::Lines) -> usize {
        messages
            .filter(|&message| self.valid(&mut self.parse_string(message)))
            .count()
    }
}

#[cfg(test)]
mod m {
    use super::*;
    #[test]
    fn test_task1() {
        assert_eq!(
            task1(
                "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            ),
            2
        );
    }
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s: &str) -> usize {
    let (rules, strings) = process(s);
    rules.validate(strings)
}

pub fn task2(_s: &str) -> usize {
    todo!()
}
