use std::collections::HashSet;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn reduce(chars: &mut Vec<char>) {
    loop {
        let mut to_remove = HashSet::new();
        for ((ai, a), (bi, b)) in chars.iter().enumerate().tuple_windows() {
            if a.to_ascii_uppercase() == b.to_ascii_uppercase() && 
                !to_remove.contains(&ai) && 
                (a.is_ascii_uppercase() && b.is_ascii_lowercase() || a.is_ascii_lowercase() && b.is_ascii_uppercase()) {
                    to_remove.insert(ai);
                    to_remove.insert(bi);
                }
        }
        if to_remove.len() == 0 {
            break;
        }
        for i in to_remove.iter().sorted().rev() {
            chars.remove(*i);
        }
    }
}

pub fn part1(input: String) {
    let mut chars = input.chars().collect_vec();
    reduce(&mut chars);
    println!("{}", chars.len());
}

pub fn part2(input: String) {
    let chars = input.chars().collect_vec();
    let candidates = chars.iter().map(|c| c.to_ascii_uppercase()).collect::<HashSet<_>>();
    candidates
        .iter()
        .map(|candidate| {
            let mut new_chars = chars.clone();
            new_chars = new_chars.iter().filter(|c| c.to_ascii_uppercase() != *candidate).map(|c| *c).collect();
            reduce(&mut new_chars);
            new_chars.len()
        })
        .min()
        .unwrap_or_default()
        .print();
}
