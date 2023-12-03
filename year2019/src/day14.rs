use crate::*;
use std::collections::HashMap;

fn scan_chem(element: &str) -> AocResult<(String, usize)> {
    let (chemical, amount) = prse::try_parse!(element, "{} {}")?;
    Ok((amount, chemical))
}

type Formulas = HashMap<String, (usize, HashMap<String, usize>)>;

pub fn parse_input(input: &str) -> AocResult<Formulas> {
    input
        .lines()
        .map(|line| {
            let (formula, result): (&str, &str) = prse::try_parse!(line, "{} => {}")?;
            let formula = formula
                .split(", ")
                .map(scan_chem)
                .collect::<AocResult<HashMap<_, _>>>()?;
            let (chemical, amount) = scan_chem(result)?;
            Result::Ok((chemical, (amount, formula)))
        })
        .collect()
}

pub fn task1(input: &Formulas) -> AocResult<usize> {
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
        for (item, count) in chem_formula.iter() {
            *fuel_formula.entry(item.to_string()).or_insert(0) += (to_produce + count - 1) / count;
        }
        fuel_formula.remove(&chem);
    }
    Ok(fuel_formula["ORE"])
}

pub fn task2(_input: &Formulas) -> AocResult<usize> {
    todo!();
}
