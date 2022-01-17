use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

pub fn part1(input: String) {
    let regex = Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
        .expect("Invalid Regex");
    let points = input
        .lines()
        .flat_map(|l| {
            regex
                .captures(l)
                .map(|c| {
                    (
                        (
                            c.get(1).unwrap().as_str().parse::<i64>().unwrap_or_default(),
                            c.get(2).unwrap().as_str().parse::<i64>().unwrap_or_default(),
                        ),
                        (
                            c.get(3).unwrap().as_str().parse::<i64>().unwrap_or_default(),
                            c.get(4).unwrap().as_str().parse::<i64>().unwrap_or_default(),
                        )
                    )
                })
        })
        .collect_vec();

    // Eyeballed an appropriate range by looking at the input
    let multiplier = (10000..11000)
        .min_by_key(|mul| {
            let mut min = (i64::MAX, i64::MAX);
            let mut max = (i64::MIN, i64::MIN);
            for (mut p, v) in &points {
                p.0 += v.0 * mul;
                p.1 += v.1 * mul;
                if p.0 < min.0 {
                    min.0 = p.0;
                }
                if p.0 > max.0 {
                    max.0 = p.0;
                }
                if p.1 < min.1 {
                    min.1 = p.1;
                }
                if p.1 > max.1 {
                    max.1 = p.1;
                }
            }
            (max.0 - min.0) * (max.1 - min.1)
        })
        .unwrap_or_default();

    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    let set = points
        .iter()
        .map(|(mut p, v)| {
            p.0 += v.0 * multiplier;
            p.1 += v.1 * multiplier;
            if p.0 < min.0 {
                min.0 = p.0;
            }
            if p.0 > max.0 {
                max.0 = p.0;
            }
            if p.1 < min.1 {
                min.1 = p.1;
            }
            if p.1 > max.1 {
                max.1 = p.1;
            }
            p
        })
        .collect::<HashSet<_>>();
    for y in min.1..=max.1 {
        let mut string = String::new();
        for x in min.0..=max.0 {
            if set.contains(&(x, y)) {
                string.push('█');
            } else {
                string.push(' ');
            }
        }
        println!("{}", string);
    }
    println!("Part 2: {}", multiplier);
}
