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

pub fn parse_input(input: &str) -> AocResult<Vec<Record>> {
    let mut log: Vec<Record> = input
        .lines()
        .map(|line| {
            let (month, day, hour, minute, event): (usize, usize, usize, usize, &str) =
                prse::try_parse!(line, "[1518-{}-{} {}:{}] {}")?;
            let event = match event {
                "falls asleep" => Event::Sleep,
                "wakes up" => Event::Awake,
                other => {
                    let guard = prse::try_parse!(other, "Guard #{} begins shift")?;
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
        .collect::<AocResult<_>>()?;
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

pub fn task1(log: &[Record]) -> AocResult<usize> {
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
                        sleep_map.entry(guard).or_insert(vec![0; 60])[minute] += 1;
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

pub fn task2(log: &[Record]) -> AocResult<usize> {
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
