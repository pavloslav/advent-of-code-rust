use super::aoc::*;
use std::collections::HashMap;
use std::collections::HashSet;

type BagMap = HashMap<String, Vec<(String, usize)>>;

fn create_map(s: &str, reverse: bool) -> Result<BagMap> {
    let mut result = BagMap::new();
    for rule in s.lines() {
        let undotted = &rule[..rule.len() - 1];
        let mut sides = undotted.split(" contain ");
        let container = sides.next().unwrap();
        let container = &container[..container.len() - 1];
        let contents = sides.next().unwrap();
        for content in contents.split(", ") {
            if content != "no other bags" {
                let number = content.split(' ').next().unwrap();
                let number_len = number.len();
                let number: usize = number.parse()?;
                let content = if number > 1 {
                    &content[number_len..content.len() - 1]
                } else {
                    &content[number_len..]
                }
                .trim()
                .to_owned();
                if reverse {
                    result
                        .entry(content)
                        .or_default()
                        .push((container.to_string(), number));
                } else {
                    result
                        .entry(container.to_string())
                        .or_default()
                        .push((content, number));
                }
            }
        }
    }
    Ok(result)
}

fn count_holders(map: &BagMap, bag: &str) -> usize {
    let mut result = HashSet::new();
    let mut to_consider = vec![bag];
    while !to_consider.is_empty() {
        let bag = to_consider.pop().unwrap();
        if let Some(bags) = map.get(bag) {
            for (container, _) in bags.iter() {
                if !result.contains(&container) {
                    result.insert(container);
                    if !to_consider.contains(&container.as_str()) {
                        to_consider.push(container.as_str());
                    }
                }
            }
        }
    }
    result.len()
}

fn count_insides(map: &BagMap, bag: &str) -> usize {
    1 + map.get(bag).map_or(0, |bags| {
        bags.iter()
            .map(|(content, count)| count_insides(map, content) * count)
            .sum()
    })
}

pub fn parse_input(input: &str) -> Result<&str> {
    Ok(input)
}

pub fn task1(s: &str) -> Result<usize> {
    let map = create_map(s, true)?;
    Ok(count_holders(&map, "shiny gold bag"))
}

pub fn task2(s: &str) -> Result<usize> {
    let map = create_map(s, false)?;
    Ok(count_insides(&map, "shiny gold bag") - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_task2() {
        let input1 =
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let input2 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        assert_eq!(task2(input1).unwrap(), 32);
        assert_eq!(task2(input2).unwrap(), 126);
    }
}
