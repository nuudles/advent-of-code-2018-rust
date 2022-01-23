use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::absdiff;

fn manhattan_distance(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    absdiff(a[0], b[0]) + absdiff(a[1], b[1]) + absdiff(a[2], b[2])+ absdiff(a[3], b[3])
}

pub fn part1(input: String) {
    let mut points = input
        .lines()
        .map(|l| l.split(",").flat_map(|n| n.parse::<i64>()).collect_vec())
        .collect_vec();
    let mut count = 0;
    while !points.is_empty() {
        let mut constellation = HashSet::new();
        constellation.insert(points.pop().unwrap());
        while let Some(index) = points
            .iter()
            .enumerate()
            .filter(|(_, p)| constellation.iter().any(|c| manhattan_distance(p, c) <= 3))
            .map(|(i, _)| i)
            .next() {
                let removed = points.swap_remove(index);
                constellation.insert(removed);
            }
        count += 1;
    }
    println!("{}", count);
}
