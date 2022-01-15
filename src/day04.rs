use std::collections::HashMap;

use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum LogType {
    BeginShift(u64),
    FallAsleep,
    WakeUp
}

#[derive(Debug)]
struct Entry {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    min: u64,
    log: LogType
}

impl Entry {
    fn from(string: &str) -> Option<Entry> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})] (?:(?:Guard #(\d+) begins shift)|(falls asleep)|(wakes up))")
                    .expect("Invalid Regex");
        }

        let captures = RE.captures(string)?;
        Some(
            Entry {
                year: captures.get(1)?.as_str().parse().ok()?,
                month: captures.get(2)?.as_str().parse().ok()?,
                day: captures.get(3)?.as_str().parse().ok()?,
                hour: captures.get(4)?.as_str().parse().ok()?,
                min: captures.get(5)?.as_str().parse().ok()?,
                log: if let Some(id) = captures.get(6).and_then(|n| n.as_str().parse().ok()) {
                    LogType::BeginShift(id)
                } else if captures.get(7) != None {
                    LogType::FallAsleep
                } else {
                    LogType::WakeUp
                }
            }
        )
    }
}

pub fn part1(input: String) {
    let mut counters = HashMap::<u64, Counter<u64>>::new();
    let mut guard_id = 0;
    let mut asleep_time = 0;
    for entry in input
        .lines()
        .flat_map(Entry::from)
        .sorted_by(|a, b| a.year.cmp(&b.year)
            .then(a.month.cmp(&b.month))
            .then(a.day.cmp(&b.day))
            .then(a.hour.cmp(&b.hour))
            .then(a.min.cmp(&b.min))
        ) {
            match entry.log {
                LogType::BeginShift(id) => guard_id = id,
                LogType::FallAsleep => asleep_time = entry.min,
                LogType::WakeUp => {
                    let counter = counters.entry(guard_id).or_insert_with(Counter::new);
                    for i in asleep_time..entry.min {
                        counter[&i] += 1;
                    }
                },
            }
        }

    if let Some((id, counter)) = counters.iter().max_by_key(|(_, c)| c.iter().map(|(_, s)| s).sum::<usize>()) {
        let minute = counter.iter().max_by_key(|(_, s)| **s).map_or(0, |(m, _)| *m);
        println!("Part 1: {}", id * minute);
    }
    if let Some((id, counter)) = counters.iter().max_by_key(|(_, c)| c.iter().map(|(_, s)| *s).max().unwrap_or_default()) {
        let minute = counter.iter().max_by_key(|(_, s)| **s).map_or(0, |(m, _)| *m);
        println!("Part 2: {}", id * minute);
    }
}
