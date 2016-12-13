extern crate aoc;

use std::collections::HashMap;

fn correct_by_most(lines:&str) -> String {
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

fn correct_by_least(lines:&str) -> String {
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

fn main() {
    let input = aoc::get_input_from_ini(env!("CARGO_PKG_NAME")).unwrap();
    println!("{}",correct_by_most(&input));    
    println!("{}",correct_by_least(&input));
}

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
    assert_eq!(correct_by_most(&inp), "easter");
    assert_eq!(correct_by_least(&inp), "advent");
}