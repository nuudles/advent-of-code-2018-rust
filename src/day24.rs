use std::{collections::BTreeSet, cmp::max};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Group<'a> {
    units: i64,
    hit_points: i64,
    immune_to: BTreeSet<&'a str>,
    weak_to: BTreeSet<&'a str>,
    attack: i64,
    attack_type: &'a str,
    initiative: u64,
}

impl Group<'_> {
    fn from(string: &str) -> Option<Group> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"(\d+) units each with (\d+) hit points(?: \((.*)\))? with an attack that does (\d+) (\w+) damage at initiative (\d+)")
                    .expect("Invalid Regex");
        }

        let captures = RE.captures(string)?;
        let mut immune_to = BTreeSet::new();
        let mut weak_to = BTreeSet::new();
        if let Some(info) = captures.get(3).map(|x| x.as_str()) {
            for part in info.split("; ") {
                if let Some(weaknesses) = part.strip_prefix("weak to ") {
                    for weakness in weaknesses.split(", ") {
                        weak_to.insert(weakness);
                    }
                } else if let Some(immunities) = part.strip_prefix("immune to ") {
                    for immunity in immunities.split(", ") {
                        immune_to.insert(immunity);
                    }
                }
            }
        }
        Some(
            Group {
                units: captures.get(1)?.as_str().parse().ok()?,
                hit_points: captures.get(2)?.as_str().parse().ok()?,
                immune_to,
                weak_to,
                attack: captures.get(4)?.as_str().parse().ok()?,
                attack_type: captures.get(5)?.as_str(),
                initiative: captures.get(6)?.as_str().parse().ok()?,
            }
        )
    }

    fn damage_to(&self, other: &Group) -> i64 {
        if other.immune_to.contains(self.attack_type) {
            0
        } else if other.weak_to.contains(self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn take_damage_from(&mut self, other: &Group) {
        // println!("{} taking {} damage killing {} units from {}", self.units, other.damage_to(self), other.damage_to(self) / self.hit_points, other.units);
        self.units -= other.damage_to(self) / self.hit_points;
    }

    fn effective_power(&self) -> i64 {
        self.units * self.attack
    }
}

fn simulate(factions: &mut Vec<Vec<Group>>) -> (usize, i64) {
    let winning_faction: usize;
    let mut last_factions = factions.clone();
    loop {
        // Target Selection (faction, attacker, target, initiative)
        let mut targets: Vec<(usize, usize, usize, u64)> = Vec::new();
        for i in 0..=1 {
            let mut target_indices = factions[1 - i]
                .iter()
                .enumerate()
                .filter(|(_, g)| g.units > 0)
                .map(|(i, _)| i)
                .collect::<BTreeSet<_>>();
            for (index, group) in factions[i]
                .iter()
                .enumerate()
                .filter(|(_, g)| g.units > 0)
                .sorted_by(|(_, a), (_, b)| 
                    a.effective_power().cmp(&b.effective_power()).reverse()
                        .then(a.initiative.cmp(&b.initiative).reverse())
                ) {
                    let target = target_indices
                        .iter()
                        .filter(|&&t| group.damage_to(&factions[1 - i][t]) > 0)
                        .max_by(|&&a, &&b| {
                            let group_a = &factions[1 - i][a];
                            let group_b = &factions[1 - i][b];
                            let damage_a = group.damage_to(group_a);
                            let damage_b = group.damage_to(group_b);
                            damage_a.cmp(&damage_b)
                                .then(group_a.effective_power().cmp(&group_b.effective_power()))
                                .then(group_a.initiative.cmp(&group_b.initiative))
                        });
                    if let Some(&target) = target {
                        targets.push((i, index, target, group.initiative));
                        target_indices.remove(&target);
                    }
                }
        }

        // Attack
        for (faction, attacker, target, _) in targets.into_iter().sorted_by_key(|t| t.3).rev() {
            let attack_group = factions[faction][attacker].clone();
            let target_group = &mut factions[1 - faction][target];
            if attack_group.units > 0 {
                target_group.take_damage_from(&attack_group);
            }
        }

        if factions[0].iter().all(|g| g.units <= 0) {
            winning_faction = 1;
            break;
        } else if factions[1].iter().all(|g| g.units <= 0) {
            winning_faction = 0;
            break;
        } else if last_factions == *factions {
            // Stalemate
            return (2, 0);
        }
        last_factions = factions.to_vec();
    }
    (winning_faction, factions[winning_faction].iter().map(|g| max(g.units, 0)).sum::<i64>())
}

pub fn part1(input: String) {
    let mut factions = input
        .split("\n\n")
        .map(|component| {
            component
                .lines()
                .skip(1)
                .flat_map(Group::from)
                .collect_vec()
        })
        .collect_vec();
    println!("{}", simulate(&mut factions).1);
}

pub fn part2(input: String) {
    let factions = input
        .split("\n\n")
        .map(|component| {
            component
                .lines()
                .skip(1)
                .flat_map(Group::from)
                .collect_vec()
        })
        .collect_vec();
    for boost in 1.. {
        let mut boosted_factions = factions.clone();
        for group in boosted_factions[0].iter_mut() {
            group.attack += boost;
        }
        let (winning, units) = simulate(&mut boosted_factions);
        if winning == 0 {
            println!("{}", units);
            break;
        }
    }
}
