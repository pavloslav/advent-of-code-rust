use crate::*;
use std::collections::HashMap;

fn scan_chem(element: &str) -> Result<(String, usize)> {
    let (chemical, amount) =
        scan_fmt::scan_fmt!(element, "{} {}", usize, String)?;
    Ok((amount, chemical))
}

type Formulas = HashMap<String, (usize, HashMap<String, usize>)>;

pub fn parse_input(input: &str) -> Result<Formulas> {
    input
        .lines()
        .map(|line| {
            let (formula, result) =
                scan_fmt::scan_fmt!(line, "{} => {}{e}", String, String)?;
            let formula = formula
                .split(", ")
                .map(scan_chem)
                .collect::<Result<HashMap<_, _>>>()?;
            let (chemical, amount) = scan_chem(&result)?;
            Result::Ok((chemical, (amount, formula)))
        })
        .collect()
}

pub fn task1(input: &Formulas) -> Result<usize> {
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
    Ok(fuel_formula["ORE"])
}

pub fn task2(_input: &Formulas) -> Result<usize> {
    todo!();
}
