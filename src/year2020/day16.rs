enum ScanType {
    Class,
    YourTicket,
    NearbyTickets,
}

struct Field {
    name: String,
    from1: u64,
    to1: u64,
    from2: u64,
    to2: u64,
}

impl Field {
    fn accepts(&self, value: u64) -> bool {
        (self.from1 <= value && value <= self.to1)
            || (self.from2 <= value && value <= self.to2)
    }
}

use once_cell::sync::Lazy;
use regex::Regex;

fn find_error_rate(s: &str) -> u64 {
    static DELIM: Lazy<Regex> = Lazy::new(|| Regex::new(r": | or |-").unwrap());
    let mut state = ScanType::Class;
    let mut fields: Vec<Field> = Vec::new();
    let mut error_rate = 0;
    for line in s.lines() {
        match state {
            ScanType::Class => {
                if !line.is_empty() {
                    let mut parts = DELIM.split(line);
                    fields.push(Field {
                        name: parts.next().unwrap().to_owned(),
                        from1: parts.next().unwrap().parse().unwrap(),
                        to1: parts.next().unwrap().parse().unwrap(),
                        from2: parts.next().unwrap().parse().unwrap(),
                        to2: parts.next().unwrap().parse().unwrap(),
                    });
                } else {
                    state = ScanType::YourTicket;
                }
            }
            ScanType::YourTicket => {
                if !line.is_empty() {
                    //skip for now
                } else {
                    state = ScanType::NearbyTickets
                }
            }
            ScanType::NearbyTickets => {
                if line != "nearby tickets:" {
                    for part in line.split(',') {
                        let value = part.parse().unwrap();
                        if !fields.iter().any(|field| field.accepts(value)) {
                            error_rate += value;
                        }
                    }
                }
            }
        }
    }
    error_rate
}

fn process_tickets(s: &str) -> u64 {
    static DELIM: Lazy<Regex> = Lazy::new(|| Regex::new(r": | or |-").unwrap());
    let mut state = ScanType::Class;
    let mut fields: Vec<Field> = Vec::new();
    let mut your_ticket: Vec<u64> = Vec::new();
    let mut nearby_tickets = Vec::new();
    for line in s.lines() {
        match state {
            ScanType::Class => {
                if !line.is_empty() {
                    let mut parts = DELIM.split(line);
                    fields.push(Field {
                        name: parts.next().unwrap().to_owned(),
                        from1: parts.next().unwrap().parse().unwrap(),
                        to1: parts.next().unwrap().parse().unwrap(),
                        from2: parts.next().unwrap().parse().unwrap(),
                        to2: parts.next().unwrap().parse().unwrap(),
                    });
                } else {
                    state = ScanType::YourTicket;
                }
            }
            ScanType::YourTicket => {
                if line.is_empty() {
                    state = ScanType::NearbyTickets
                } else if line != "your ticket:" {
                    your_ticket =
                        line.split(',').map(|x| x.parse().unwrap()).collect();
                }
            }
            ScanType::NearbyTickets => {
                if line != "nearby tickets:" {
                    let ticket: Vec<_> =
                        line.split(',').map(|x| x.parse().unwrap()).collect();
                    if ticket.iter().all(|&value| {
                        fields.iter().any(|field| field.accepts(value))
                    }) {
                        nearby_tickets.push(ticket.clone());
                    }
                }
            }
        }
    }
    println!("{} tickets are ok", nearby_tickets.len());
    let mut possibles: Vec<Vec<bool>> = (0..fields.len())
        .map(|_| ((0..your_ticket.len()).map(|_| true).collect()))
        .collect();

    for field_idx in 0..possibles.len() {
        for place_idx in 0..possibles[field_idx].len() {
            for ticket in nearby_tickets.iter() {
                if possibles[field_idx][place_idx]
                    && !fields[field_idx].accepts(ticket[place_idx])
                {
                    possibles[field_idx][place_idx] = false;
                }
            }
        }
    }

    loop {
        let mut changed = false;
        for field_idx in 0..possibles.len() {
            if possibles[field_idx].iter().filter(|&&x| x).count() == 1 {
                let true_idx =
                    possibles[field_idx].iter().position(|&x| x).unwrap();
                for other_field_idx in 0..possibles[field_idx].len() {
                    if other_field_idx != field_idx {
                        changed |= possibles[other_field_idx][true_idx];
                        possibles[other_field_idx][true_idx] = false;
                    }
                }
            }
        }
        for place_idx in 0..possibles[0].len() {
            if possibles.iter().filter(|&x| x[place_idx]).count() == 1 {
                let true_idx =
                    possibles.iter().position(|x| x[place_idx]).unwrap();
                for other_place_idx in 0..possibles.len() {
                    if other_place_idx != place_idx {
                        changed |= possibles[true_idx][other_place_idx];
                        possibles[true_idx][other_place_idx] = false;
                    }
                }
            }
        }
        if !changed {
            println!("Nothing changed!");
            break;
        }
    }

    let mut prod = 1;
    for (i, f) in fields.iter().enumerate() {
        if f.name.starts_with("departure") {
            let idx = possibles[i].iter().position(|&x| x).unwrap();
            println!("{}: {}", f.name, your_ticket[idx]);
            prod *= your_ticket[idx];
        }
    }
    prod
}

pub fn parse_input(input: &str) -> &str {
    input
}

pub fn task1(s: &str) -> u64 {
    find_error_rate(s)
}

pub fn task2(s: &str) -> u64 {
    process_tickets(s)
}
