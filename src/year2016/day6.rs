use std::collections::HashMap;

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(lines:&str) -> String {
    let mut statistics:Vec<HashMap<char,u32>> = Vec::new();
    for line in lines.lines() {
        if statistics.len() == 0 {
            statistics = vec![HashMap::new(); line.len()];
        }
        for (i, c) in line.chars().enumerate() {
            *statistics[i].entry(c).or_insert(0) += 1;
        }
    }

    statistics.iter()
              .map( |hashmap| hashmap.iter()
                                     .max_by_key(|tuple|tuple.1)
                                     .map(|tuple|*tuple.0)
                                     .unwrap() )
              .collect()
}

pub fn task2(lines:&str) -> String {
    let mut statistics:Vec<HashMap<char,u32>> = Vec::new();
    for line in lines.lines() {
        if statistics.len() == 0 {
            statistics = vec![HashMap::new(); line.len()];
        }
        for (i, c) in line.chars().enumerate() {
            *statistics[i].entry(c).or_insert(0) += 1;
        }
    }

    statistics.iter()
              .map( |hashmap| hashmap.iter()
                                     .min_by_key(|tuple|tuple.1)
                                     .map(|tuple|*tuple.0)
                                     .unwrap() )
              .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_correct_by_most() {
        let inp ="\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
        assert_eq!(task1(&inp), "easter");
        assert_eq!(task2(&inp), "advent");
    }
}