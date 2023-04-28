use super::aoc::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

type Passport<'a> = HashMap<&'a str, &'a str>;

pub fn parse_input(input: &str) -> Result<Vec<Passport>> {
    let mut passports = Vec::new();
    let mut passport = Passport::new();
    for line in input.lines() {
        if line.is_empty() {
            passports.push(passport.clone());
            passport.clear();
        } else {
            for record in line.split_whitespace() {
                let mut split = record.split(':');
                if let (Some(field), Some(value)) = (split.next(), split.next())
                {
                    passport.insert(field, value);
                }
            }
        }
    }
    if !passport.is_empty() {
        passports.push(passport);
    }
    Ok(passports)
}

fn is_valid1(passport: &Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|field| passport.contains_key(field))
}

pub fn task1(pass: &[Passport]) -> Result<usize> {
    Ok(pass.iter().filter(|p| is_valid1(p)).count())
}

static HCL_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^#[[:xdigit:]]{6}$").unwrap());

static ECL_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap());

static PID_REGEX: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^\d{9}$").unwrap());

fn is_valid2(passport: &Passport) -> bool {
    let bounded_field =
        |field, range: std::ops::RangeInclusive<usize>| -> bool {
            matches!(
                passport.get(&field).map(|val_str| val_str
                    .parse()
                    .map(|val| range.contains(&val))),
                Some(Ok(true))
            )
        };

    let height = || {
        matches!(
            passport.get("hgt").map(|hgt_str| {
                hgt_str[..hgt_str.len() - 2].parse().map(|hgt: usize| {
                    match &hgt_str[hgt_str.len() - 2..] {
                        "cm" => (150..=193).contains(&hgt),
                        "in" => (59..=76).contains(&hgt),
                        _other => false,
                    }
                })
            }),
            Some(Ok(true))
        )
    };

    let regex_check = |field, regex: &Lazy<regex::Regex>| {
        passport.get(&field).map(|value| regex.is_match(value)) == Some(true)
    };

    bounded_field("byr", 1920..=2002)
        && bounded_field("iyr", 2010..=2020)
        && bounded_field("eyr", 2020..=2030)
        && height()
        && regex_check("hcl", &HCL_REGEX)
        && regex_check("ecl", &ECL_REGEX)
        && regex_check("pid", &PID_REGEX)
}

pub fn task2(pass: &[Passport]) -> Result<usize> {
    Ok(pass.iter().filter(|p| is_valid2(p)).count())
}
