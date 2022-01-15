use std::collections::HashSet;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .flat_map(|l| l.trim_start_matches('+').parse::<i64>().ok())
        .sum::<i64>()
        .print();
}

pub fn part2(input: String) {
    let numbers = input
        .lines()
        .flat_map(|l| l.trim_start_matches('+').parse::<i64>().ok())
        .collect_vec();
    let mut frequency = 0;
    let mut seen = HashSet::new();
    seen.insert(frequency);
    loop {
        for n in &numbers {
            frequency += n;
            if seen.contains(&frequency) {
                println!("{}", frequency);
                return;
            }
            seen.insert(frequency);
        }
    }
}
