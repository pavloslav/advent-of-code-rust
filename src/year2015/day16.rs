use super::super::common::Error::TaskError;
use super::aoc::*;

type Map = std::collections::HashMap<String, usize>;

const FILTER: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

fn satisfy(data: &Map, filter: &Map) -> bool {
    data.iter().all(|(key, val)| filter.get(key) == Some(val))
}

fn satisfy2(data: &Map, filter: &Map) -> bool {
    let cmp_filter: std::collections::HashMap<String, std::cmp::Ordering> = [
        ("cats".to_owned(), std::cmp::Ordering::Greater),
        ("trees".to_owned(), std::cmp::Ordering::Greater),
        ("pomeranians".to_owned(), std::cmp::Ordering::Less),
        ("goldfish".to_owned(), std::cmp::Ordering::Less),
    ]
    .into_iter()
    .collect();
    data.iter().all(|(key, data_val)| match filter.get(key) {
        Some(filter_val) => {
            data_val.cmp(filter_val)
                == *cmp_filter.get(key).unwrap_or(&std::cmp::Ordering::Equal)
        }
        _ => true,
    })
}

pub fn parse_input(input: &str) -> Result<Vec<Map>> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|value| {
                    Ok(scan_fmt::scan_fmt!(value, "{}: {}", String, usize)?)
                })
                .collect::<Result<Map>>()
        })
        .collect()
}

fn task<F>(input: &[Map], check: &F) -> Result<usize>
where
    F: Fn(&Map, &Map) -> bool,
{
    let filter: Map = FILTER
        .lines()
        .map(|line| Ok(scan_fmt::scan_fmt!(line, "{}: {}", String, usize)?))
        .collect::<Result<_>>()?;

    Ok(input
        .iter()
        .enumerate()
        .find(|(_i, data)| check(data, &filter))
        .ok_or_else(|| TaskError("No suitable answer".to_string()))?
        .0
        + 1)
}

pub fn task1(input: &[Map]) -> Result<usize> {
    task(input, &satisfy)
}

pub fn task2(input: &[Map]) -> Result<usize> {
    task(input, &satisfy2)
}
