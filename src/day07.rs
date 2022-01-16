use std::collections::{HashSet, HashMap};

use itertools::Itertools;

pub fn part1(input: String) {
    let mut all = HashSet::new();
    let mut rules = input
        .lines()
        .fold(HashMap::new(), |mut rules, l| {
            let mut words = l.split(' ');
            let first = words.nth(1).unwrap_or_default();
            let after = words.nth(5).unwrap_or_default();
            all.insert(first);
            all.insert(after);
            let set = rules.entry(after).or_insert_with(HashSet::new);
            set.insert(first);
            rules
        });
    for step in &all {
        if !rules.contains_key(step) {
            rules.insert(step, HashSet::new());
        }
    }
    let mut string = String::new();
    while string.len() < all.len() {
        let next = rules.iter().filter(|(_, v)| v.is_empty()).sorted_by_key(|(k, _)| **k).next().map_or("", |(k, _)| k);
        string.push_str(next);
        rules.remove(next);
        for (_, after) in rules.iter_mut() {
            after.remove(next);
        }
    }
    println!("{}", string);
}

pub fn part2(input: String) {
    let mut all = HashSet::new();
    let mut rules = input
        .lines()
        .fold(HashMap::new(), |mut rules, l| {
            let mut words = l.split(' ');
            let first = words.nth(1).unwrap_or_default().bytes().next().unwrap_or_default();
            let after = words.nth(5).unwrap_or_default().bytes().next().unwrap_or_default();
            all.insert(first);
            all.insert(after);
            let set = rules.entry(after).or_insert_with(HashSet::new);
            set.insert(first);
            rules
        });
    for step in &all {
        if !rules.contains_key(step) {
            rules.insert(*step, HashSet::new());
        }
    }
    let mut workers = Vec::<(u8, u64)>::new();
    let mut time_spent = 0;
    let mut done = HashSet::new();
    while done.len() < all.len() {
        let next_steps = rules.iter().filter(|(_, a)| a.is_empty()).map(|t| t.0).sorted().collect_vec();
        for &next in next_steps {
            if workers.len() == 5 {
                break;
            }
            if done.contains(&next) {
                continue;
            }
            workers.push((next, u64::from(61 + next - b'A')));
            done.insert(next);
        }

        let min = workers.iter().map(|t| t.1).min().unwrap_or_default();
        time_spent += min;
        for worker in workers.iter_mut() {
            (*worker).1 -= min;
            if worker.1 == 0 {
                rules.remove(&worker.0);
                for (_, after) in rules.iter_mut() {
                    after.remove(&worker.0);
                }
            }
        }
        workers = workers.iter().filter(|(_, t)| t != &0).map(|x| *x).collect_vec();
    }
    time_spent += workers.iter().map(|t| t.1).sum::<u64>();
    println!("{}", time_spent);
}
