use super::super::common::Result;

use unicode_categories::UnicodeCategories;

type CaveName = String;

#[derive(Clone)]
pub struct CaveNode {
    ways_out: Vec<CaveName>,
    visited: Option<bool>,
}

impl CaveNode {
    fn new(name: &str, neighbor: &str) -> CaveNode {
        CaveNode {
            ways_out: vec![neighbor.to_owned()],
            visited: if name.chars().all(|c| c.is_letter_uppercase()) {
                None
            } else {
                Some(false)
            },
        }
    }
}

type CaveMap = std::collections::HashMap<CaveName, CaveNode>;

pub fn parse_input(input: &str) -> Result<CaveMap> {
    let mut map = CaveMap::new();
    for line in input.lines() {
        let (left, right) = scan_fmt::scan_fmt!(line, "{}-{}", String, String)?;

        map.entry(left.clone())
            .and_modify(|node| node.ways_out.push(right.clone()))
            .or_insert_with(|| CaveNode::new(&left, &right));
        map.entry(right.clone())
            .and_modify(|node| node.ways_out.push(left.clone()))
            .or_insert_with(|| CaveNode::new(&right, &left));
    }
    Ok(map)
}

fn count_ways(map: &mut CaveMap, start: &str, end: &str) -> usize {
    if start == end {
        return 1;
    }
    if map[start].visited == Some(true) {
        return 0;
    }
    let start = start.to_string();
    if map[&start].visited.is_some() {
        map.entry(start.to_owned())
            .and_modify(|node| node.visited = Some(true));
    }
    let result = (0..map[&start].ways_out.len())
        .map(|i| {
            let name = map[&start].ways_out[i].to_owned();
            count_ways(map, &name, end)
        })
        .sum();
    if map[&start].visited.is_some() {
        map.entry(start)
            .and_modify(|node| node.visited = Some(false));
    }
    result
}

pub fn task1(map: &CaveMap) -> Result<usize> {
    let mut map = map.clone();
    Ok(count_ways(&mut map, "start", "end"))
}

fn count_ways2(
    map: &mut CaveMap,
    start: &str,
    end: &str,
    start_visited: bool,
    small_visited_twice: bool,
) -> usize {
    if start == end {
        return 1;
    }
    if start == "start" && start_visited {
        return 0;
    }
    let in_visited_small = map[start].visited.unwrap_or(false);
    if small_visited_twice && in_visited_small {
        return 0;
    }
    let start = start.to_string();
    if map[&start].visited.is_some() {
        map.entry(start.to_owned())
            .and_modify(|node| node.visited = Some(true));
    }
    let result = (0..map[&start].ways_out.len())
        .map(|i| {
            let name = map[&start].ways_out[i].to_owned();
            count_ways2(
                map,
                &name,
                end,
                true,
                small_visited_twice || in_visited_small,
            )
        })
        .sum();
    if !in_visited_small && map[&start].visited.is_some() {
        map.entry(start)
            .and_modify(|node| node.visited = Some(false));
    }
    result
}

pub fn task2(map: &CaveMap) -> Result<usize> {
    let mut map = map.clone();
    Ok(count_ways2(&mut map, "start", "end", false, false))
}
