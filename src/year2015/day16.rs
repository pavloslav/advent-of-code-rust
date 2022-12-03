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

pub fn parse_input(input: &str) -> Vec<Map> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|value| {
                    let mut parts = value.split(": ");
                    (
                        parts.next().unwrap().to_owned(),
                        parts.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn task<F>(input: &[Map], check: &F) -> usize
where
    F: Fn(&Map, &Map) -> bool,
{
    let filter: Map = FILTER
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    input
        .iter()
        .enumerate()
        .filter_map(
            |(i, data)| {
                if check(data, &filter) {
                    Some(i)
                } else {
                    None
                }
            },
        )
        .next()
        .unwrap()
        + 1
}

pub fn task1(input: &[Map]) -> usize {
    task(input, &satisfy)
}

pub fn task2(input: &[Map]) -> usize {
    task(input, &satisfy2)
}
