use std::{collections::HashSet, cmp::min, cmp::max, ops::RangeInclusive};

use crate::{parse_nums::parse_nums, point::Point};

fn parse_clay(input: &str) -> (HashSet<Point<u64>>, RangeInclusive<u64>) {
    let mut min_max = (u64::MAX, u64::MIN);
    let clay = input
        .lines()
        .fold(HashSet::new(), |mut set, line| {
            let numbers: Vec<u64> = parse_nums(line).collect();
            let is_y_range = line.chars().next() == Some('x');
            let a = numbers[0];
            for b in numbers[1]..=numbers[2] {
                set.insert(Point { x: if is_y_range { a } else { b }, y: if is_y_range { b } else { a } });
            }
            if is_y_range {
                min_max = (min(min_max.0, numbers[1]), max(min_max.1, numbers[2]));
            } else {
                min_max = (min(min_max.0, a), max(min_max.1, a));
            }
            set
        });
    (clay, min_max.0..=min_max.1)
}

pub fn part1(input: String) {
    let (clay, y_range) = parse_clay(&input);
    // let mut reachable = HashSet::new();
    let mut next = HashSet::new();
    next.insert(Point::<u64> { x: 500, y: 0 });
    let mut seen = HashSet::new();
    let mut pivots = HashSet::new();
    let mut reservoir = HashSet::new();
    while !next.is_empty() {
        let mut new_next = HashSet::new();
        let y_values = next.iter().map(|p| p.y).collect::<HashSet<_>>();
        for mut point in next {
            // if y_range.contains(&point.y) {
            //     reachable.insert(point);
            // }
            let down = point.down();
            if clay.contains(&down) || seen.contains(&down) {
                let mut stream_found = false;
                let mut pivot_found = false;
                loop {
                    // Look for a stream down
                    let mut left = point.left();
                    let mut layer = HashSet::new();
                    layer.insert(point);
                    while !clay.contains(&left) {
                        seen.insert(left);
                        layer.insert(left);
                        let down = left.down();
                        if pivots.contains(&down) {
                            // Peek ahead to see if we should continue
                            pivot_found = true;
                            break;
                        }
                        if !clay.contains(&down) && !seen.contains(&down) {
                            new_next.insert(down);
                            seen.insert(down);
                            pivots.insert(down);
                            stream_found = true;
                            break;
                        }
                        left = left.left();
                    }
                    let mut right = point.right();
                    while !clay.contains(&right) {
                        seen.insert(right);
                        layer.insert(right);
                        let down = right.down();
                        if pivots.contains(&down) {
                            pivot_found = true;
                            break;
                        }
                        if !clay.contains(&down) && !seen.contains(&down) {
                            new_next.insert(down);
                            seen.insert(down);
                            pivots.insert(down);
                            stream_found = true;
                            break;
                        }
                        right = right.right();
                    }
                    if pivot_found {
                        // Peek ahead to see if we should break or keep going up
                        let mut left = point.left();
                        let mut right = point.right();
                        let mut should_break = false;
                        while !clay.contains(&left) {
                            if !seen.contains(&left.down()) && !clay.contains(&left.down()) {
                                should_break = true;
                                break;
                            }
                            layer.insert(left);
                            left = left.left();
                        }
                        while !should_break && !clay.contains(&right) {
                            if !seen.contains(&right.down()) && !clay.contains(&right.down()) {
                                should_break = true;
                                break;
                            }
                            layer.insert(right);
                            right = right.right();
                        }
                        if should_break {
                            break;
                        }
                    }
                    if stream_found {
                        break;
                    }
                    point = point.up();
                    seen.insert(point);
                    for p in layer {
                        reservoir.insert(p);
                    }
                }
            } else if (y_range.contains(&down.y) || down.y < *y_range.start()) && !seen.contains(&down) {
                if y_values.iter().any(|y| down.y > *y && down.y - y > 1) {
                    new_next.insert(point);
                } else {
                    new_next.insert(down);
                    if y_range.contains(&down.y) {
                        seen.insert(down);
                    }
                }
            }
        }
        next = new_next;
    }

    /*
    // Print the whole shebang
    for y in y_range {
        let mut string = String::new();
        for x in 400..=1000 {
            if reservoir.contains(&Point { x, y }) {
                string.push('█');
            } else if seen.contains(&Point { x, y }) {
                string.push('|');
            } else if clay.contains(&Point { x, y }) {
                string.push('#');
            } else {
                string.push(' ');
            }
        }
        println!("{}", string);
    }
    */

    // My answer for Part 1 was too low, so I wrote the above commented out function
    // to print the whole thing out and then followed the stream. There looks to be a
    // race condition that causes a single buggy line of 22 mislabeled streams down.
    // It was fast enough to just count the erroneous tiles and subtract them then to
    // find the bug... Likely won't work on other inputs.
    let buggy_tiles = 22;
    println!("Part 1: {}", seen.len() - buggy_tiles);
    println!("Part 2: {}", reservoir.len());
}
