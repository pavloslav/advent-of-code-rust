use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Event {
    BeginsShift(usize),
    Sleep,
    Awake,
}

#[derive(Clone)]
pub struct Record {
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    event: Event,
}

pub fn parse_input(input: &str) -> Result<Vec<Record>> {
    let mut log: Vec<Record> = input
        .lines()
        .map(|line| {
            let (month, day, hour, minute, event) = scan_fmt::scan_fmt!(
                line,
                "[1518-{d}-{d} {d}:{d}] {/.*/}{e}",
                usize,
                usize,
                usize,
                usize,
                String
            )
            .map_err(|_| aoc_error!("Can't parse '{line}'"))?;
            let event = match event.as_str() {
                "falls asleep" => Event::Sleep,
                "wakes up" => Event::Awake,
                other => {
                    let guard = scan_fmt::scan_fmt!(
                        other,
                        "Guard #{} begins shift",
                        usize
                    )?;
                    Event::BeginsShift(guard)
                }
            };
            Ok(Record {
                month,
                day,
                hour,
                minute,
                event,
            })
        })
        .collect::<Result<_>>()?;
    log.sort_by_key(minutes);
    Ok(log)
}

fn minutes(record: &Record) -> usize {
    (([
        0,
        31,
        31 + 28,
        31 + 28 + 31,
        31 + 28 + 31 + 30,
        31 + 28 + 31 + 30 + 31,
        31 + 28 + 31 + 30 + 31 + 30,
        31 + 28 + 31 + 30 + 31 + 30 + 31,
        31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
        31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
        31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
        31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
    ][record.month]
        + record.day)
        * 24
        + record.hour)
        * 60
        + record.minute
}

pub fn task1(log: &[Record]) -> Result<usize> {
    let mut sleep_map = HashMap::new();
    let mut current_guard = None;
    let mut sleep_start = 0;
    for record in log {
        match record.event {
            Event::BeginsShift(guard) => current_guard = Some(guard),
            Event::Sleep => sleep_start = record.minute,
            Event::Awake => {
                if let Some(guard) = current_guard {
                    for minute in sleep_start..record.minute {
                        sleep_map.entry(guard).or_insert(vec![0; 60])
                            [minute] += 1;
                    }
                }
            }
        }
    }
    let most_sleeping_guard = sleep_map
        .iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<usize>())
        .map(|(&guard, _)| guard)
        .ok_or_else(|| aoc_error!("No log entries!"))?;
    let best_minute = sleep_map[&most_sleeping_guard]
        .iter()
        .enumerate()
        .max_by_key(|&(_, m)| m)
        .map(|(i, _)| i)
        .ok_or_else(|| aoc_error!("No minutes!"))?;
    Ok(most_sleeping_guard * best_minute)
}

pub fn task2(log: &[Record]) -> Result<usize> {
    let mut sleep_map = HashMap::new();
    let mut current_guard = None;
    let mut sleep_start = 0;
    for record in log {
        match record.event {
            Event::BeginsShift(guard) => current_guard = Some(guard),
            Event::Sleep => sleep_start = record.minute,
            Event::Awake => {
                if let Some(guard) = current_guard {
                    for minute in sleep_start..record.minute {
                        *sleep_map.entry((guard, minute)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    let (guard, minute) = sleep_map
        .iter()
        .max_by_key(|&(_, &sleep)| sleep)
        .map(|(key, _)| key)
        .ok_or_else(|| aoc_error!("No log entries!"))?;
    Ok(guard * minute)
}
