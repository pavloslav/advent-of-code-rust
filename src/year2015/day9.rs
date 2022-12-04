type Distances = Vec<Vec<usize>>;

use once_cell::sync::Lazy;

static SPLIT_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r" to | = ").unwrap());

pub fn parse_input(input: &str) -> Distances {
    let mut dist = Vec::new();
    let mut name_map = std::collections::HashMap::new();
    for line in input.lines() {
        let mut fields = SPLIT_REGEX.split(line);
        let size = name_map.len();
        let from = *name_map.entry(fields.next().unwrap()).or_insert(size);
        let size = name_map.len();
        let to = *name_map.entry(fields.next().unwrap()).or_insert(size);
        let distance = fields.next().unwrap().parse().unwrap();
        let size = name_map.len();
        if dist.len() < size {
            dist.resize(size, Vec::new());
        }
        if dist[from].len() < size {
            dist[from].resize(size, 0);
        }
        dist[from][to] = distance;
        if dist[to].len() <= size {
            dist[to].resize(size, 0);
        }
        dist[to][from] = distance;
    }
    dist
}

use itertools::Itertools; //for permutations

macro_rules! task {
    ($distances: expr, $func: ident) => {
        (0..$distances.len())
            .permutations($distances.len())
            .map(|comb| {
                comb.windows(2)
                    .map(|pair| {
                        if let &[i, j] = pair {
                            $distances[i][j]
                        } else {
                            0
                        }
                    })
                    .sum()
            })
            .$func()
            .unwrap()
    };
}

pub fn task1(distances: &Distances) -> usize {
    task!(distances, min)
}

pub fn task2(distances: &Distances) -> usize {
    task!(distances, max)
}
