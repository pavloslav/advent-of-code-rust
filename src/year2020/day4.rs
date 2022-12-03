use std::collections::HashMap;

fn is_valid1(passport: &HashMap<&str, &str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|field| passport.contains_key(field))
}

fn check(val: bool) -> Option<()> {
    if val {
        Some(())
    } else {
        None
    }
}

fn is_valid2_inner(passport: &HashMap<&str, &str>) -> Option<()> {
    let bounded_field = |field, low, high| -> Option<()> {
        let &val = passport.get(field)?;
        check(val.len() == 4 && val.chars().all(|c| c.is_ascii_digit()))?;
        let val = val.parse::<i32>().ok()?;
        check(low <= val && val <= high)
    };

    bounded_field("byr", 1920, 2002)?;
    bounded_field("iyr", 2010, 2020)?;
    bounded_field("eyr", 2020, 2030)?;

    let hgt = passport.get("hgt")?;
    let cm = hgt.ends_with("cm");
    check(cm || hgt.ends_with("in"))?;
    let hgt = hgt[..hgt.len() - 2].parse::<i32>().ok()?;
    if cm {
        check((150..=193).contains(&hgt))?;
    } else {
        check((59..=76).contains(&hgt))?;
    }

    let hcl = passport.get("hcl")?;
    check(
        hcl.len() == 7
            && hcl.starts_with('#')
            && hcl[1..].chars().all(|c| c.is_ascii_hexdigit()),
    )?;

    let ecl = passport.get("ecl")?;
    check(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl))?;

    let pid = passport.get("pid")?;
    check(pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()))?;

    Some(())
}

fn is_valid2(passport: &HashMap<&str, &str>) -> bool {
    is_valid2_inner(passport).is_some()
}

fn valid_passports(
    s: &str,
    validator: fn(&HashMap<&str, &str>) -> bool,
) -> usize {
    let mut passport = HashMap::new();
    let mut count = 0;
    for line in s.lines() {
        if line.is_empty() {
            if validator(&passport) {
                count += 1;
            }
            passport.clear();
        } else {
            for record in line.split_whitespace() {
                let mut split = record.split(':');
                if let Some(field) = split.next() {
                    if let Some(value) = split.next() {
                        passport.insert(field, value);
                    }
                }
            }
        }
    }
    if is_valid1(&passport) {
        count += 1;
    }
    count
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s: &str) -> usize {
    valid_passports(s, is_valid1)
}

pub fn task2(s: &str) -> usize {
    valid_passports(s, is_valid2)
}
