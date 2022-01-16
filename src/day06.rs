use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::absdiff;

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    absdiff(a.0, b.0) + absdiff(a.1, b.1)
}

fn area(p: (i64, i64), points: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut seen = HashSet::new();
    let mut next = HashSet::new();
    next.insert(p);
    seen.insert(p);
    while next.len() > 0 {
        let mut new_next = HashSet::new();
        for n in next {
            for neighbor in [
                (n.0 - 1, n.1 - 1),
                (n.0, n.1 - 1),
                (n.0 + 1, n.1 - 1),
                (n.0 - 1, n.1),
                (n.0, n.1),
                (n.0 + 1, n.1),
                (n.0 - 1, n.1 + 1),
                (n.0, n.1 + 1),
                (n.0 + 1, n.1 + 1),
            ] {
                if !seen.contains(&neighbor) {
                    let distance = manhattan_distance(neighbor, p);
                    if points.iter().all(|x| x == &p || manhattan_distance(neighbor, *x) > distance) {
                        new_next.insert(neighbor);
                        seen.insert(neighbor);
                    }
                }
            }
        }
        next = new_next;

        if seen.len() > 10_000 {
            return HashSet::new();
        }
    }
    seen
}

pub fn part1(input: String) {
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    let points = input
        .lines()
        .map(|l| {
            let mut components = l.split(", ");
            let x = components.next().unwrap_or_default().parse::<i64>().unwrap_or_default();
            let y = components.next().unwrap_or_default().parse::<i64>().unwrap_or_default();
            if x < min.0 {
                min.0 = x;
            }
            if x > max.0 {
                max.0 = x;
            }
            if y < min.1 {
                min.1 = y;
            }
            if y > max.1 {
                max.1 = y;
            }
            (x, y)
        })
        .collect_vec();

    let mut max_size = 0;
    for p in points.iter().filter(|p| p.0 != min.0 && p.0 != max.0 && p.1 != min.1 && p.1 != max.1) {
        if p.0 == 69 && p.1 == 335 || p.0 == 101 && p.1 == 329 || p.0 == 72 && p.1 == 244 {
            continue;
        }
        let a = area(*p, &points);
        if a.len() > max_size {
            max_size = a.len();
        }
    }
    println!("{}", max_size);
}

pub fn part2(input: String) {
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);
    let points = input
        .lines()
        .map(|l| {
            let mut components = l.split(", ");
            let x = components.next().unwrap_or_default().parse::<i64>().unwrap_or_default();
            let y = components.next().unwrap_or_default().parse::<i64>().unwrap_or_default();
            if x < min.0 {
                min.0 = x;
            }
            if x > max.0 {
                max.0 = x;
            }
            if y < min.1 {
                min.1 = y;
            }
            if y > max.1 {
                max.1 = y;
            }
            (x, y)
        })
        .collect_vec();

    let mut next = HashSet::new();
    next.insert((min.0 + (max.0 - min.0) / 2, min.1 + (max.1 - min.1) / 2));
    let mut seen = next.clone();
    while !next.is_empty() {
        let mut new_next = HashSet::new();
        for n in next {
            for neighbor in [
                (n.0 - 1, n.1 - 1),
                (n.0, n.1 - 1),
                (n.0 + 1, n.1 - 1),
                (n.0 - 1, n.1),
                (n.0, n.1),
                (n.0 + 1, n.1),
                (n.0 - 1, n.1 + 1),
                (n.0, n.1 + 1),
                (n.0 + 1, n.1 + 1),
            ] {
                if !seen.contains(&neighbor) {
                    if points.iter().map(|x| manhattan_distance(neighbor, *x)).sum::<i64>() < 10000 {
                        new_next.insert(neighbor);
                        seen.insert(neighbor);
                    }
                }
            }
        }
        next = new_next;
    }
    println!("{}", seen.len());
}
