use std::collections::HashMap;

fn scan_chem(element: &str) -> (String, usize) {
    let (chemical, amount) =
        scan_fmt::scan_fmt!(element, "{} {}", usize, String).unwrap();
    (amount, chemical)
}

type Formulas = HashMap<String, (usize, HashMap<String, usize>)>;

pub fn parse_input(input: &str) -> Formulas {
    input
        .lines()
        .map(|line| {
            let mut line = line.split(" => ");
            let formula =
                line.next().unwrap().split(", ").map(scan_chem).collect();
            let (chemical, amount) = scan_chem(line.next().unwrap());
            (chemical, (amount, formula))
        })
        .collect()
}

pub fn task1(input: &Formulas) -> usize {
    let mut formulas = input.clone();
    let mut fuel_formula = formulas.remove("FUEL").unwrap().1;
    while fuel_formula.len() > 1 || !fuel_formula.contains_key("ORE") {
        println!("{fuel_formula:?}");
        let chem = fuel_formula
            .keys()
            .find(|k| k != &"ORE")
            .unwrap()
            .to_owned();
        let (to_produce, chem_formula) = &formulas[&chem];
        //println!("{fuel_formula:?}<={to_produce} {chem_formula:?}");
        for (item, count) in chem_formula.iter() {
            *fuel_formula.entry(item.to_string()).or_insert(0) +=
                (to_produce + count - 1) / count;
        }
        fuel_formula.remove(&chem);
    }
    fuel_formula["ORE"]
}

pub fn task2(_input: &Formulas) -> usize {
    unimplemented!();
}
