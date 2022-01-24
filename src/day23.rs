use std::cmp::{min, max};

use itertools::{Itertools, iproduct};
use pathfinding::prelude::absdiff;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn manhattan_distance(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    absdiff(a.0, b.0) + absdiff(a.1, b.1) + absdiff(a.2, b.2)
}

fn in_range(p: &(i64, i64, i64), bot: &((i64, i64, i64), i64)) -> bool {
    manhattan_distance(p, &bot.0) <= bot.1
}

pub fn part1(input: String) {
    let nanobots = input
        .lines()
        .map(|l| {
            let nums: Vec<i64> = parse_nums(l).collect();
            ((nums[0], nums[1], nums[2]), nums[3])
        })
        .collect_vec();
    let strongest = nanobots.iter().max_by_key(|(_, s)| s).unwrap();
    nanobots
        .iter()
        .filter(|n| in_range(&n.0, strongest))
        .count()
        .print();
}

pub fn part2(input: String) {
    let mut lowest = (i64::MAX, i64::MAX, i64::MAX);
    let mut highest = (i64::MIN, i64::MIN, i64::MIN);
    let nanobots = input
        .lines()
        .map(|l| {
            let nums: Vec<i64> = parse_nums(l).collect();
            lowest = (min(nums[0], lowest.0), min(nums[1], lowest.1), min(nums[2], lowest.2));
            highest = (max(nums[0], highest.0), max(nums[1], highest.1), max(nums[2], highest.2));
            ((nums[0], nums[1], nums[2]), nums[3])
        })
        .collect_vec();

    let mut size = ((highest.0 - lowest.0) / 2, (highest.1 - lowest.1) / 2, (highest.2 - lowest.2) / 2);
    loop {
        let (x, y, z) = iproduct!(
            (lowest.0..highest.0).step_by(size.0 as usize),
            (lowest.1..highest.1).step_by(size.1 as usize),
            (lowest.2..highest.2).step_by(size.2 as usize)
        )
        .max_by_key(|(x, y, z)|
            nanobots
                .iter()
                .filter(|b|
                    (*x..x + size.0).contains(&b.0.0) && (*y..y + size.1).contains(&b.0.1) && (*z..z + size.2).contains(&b.0.2) ||
                        in_range(&(*x, *y, *z), b) ||
                        in_range(&(*x + size.0, *y, *z), b) ||
                        in_range(&(*x, *y + size.1, *z), b) ||
                        in_range(&(*x + size.0, *y + size.1, *z), b) ||
                        in_range(&(*x, *y, *z + size.2), b) ||
                        in_range(&(*x + size.0, *y, *z + size.2), b) ||
                        in_range(&(*x, *y + size.1, *z + size.2), b) ||
                        in_range(&(x + size.0, y + size.1, z + size.2), b)
                )
                .count()
        )
        .unwrap_or_default();

        lowest = (x, y, z);
        highest = (x + size.0, y + size.0, z + size.0);
        if size == (1, 1, 1) {
            break;
        }
        size = (max(size.0 / 2, 1), max(size.1 / 2, 1), max(size.2 / 2, 1));
    }
    println!("{}", lowest.0 + lowest.1 + lowest.2);

    // Keeping my other attempts below for fun
    /*
    let mut counter = Counter::<Cuboid>::new();
    for bot in nanobots {
        println!("{}", counter.len());
        let cuboid = Cuboid::from(&bot);
        let overlaps = counter.keys().filter(|c| c.overlaps(&cuboid)).map(|c| c.clone()).collect_vec();
        if overlaps.is_empty() {
            counter.insert(cuboid, 1);
        } else {
            for c in overlaps {
                let intersection = cuboid.intersection(&c);
                let count = counter.remove(&c).unwrap_or_default();
                counter.insert(intersection.clone(), count + 1);
                for leftover in c.removing(&intersection) {
                    counter.insert(leftover, count);
                }
                for leftover in cuboid.removing(&intersection) {
                    counter.insert(leftover, 1);
                }
            }
        }
    }
    println!("{:?}", counter.most_common()[0]);
    println!("{:?} {:?} {:?}", lowest, highest, average)

    for x in lowest.0..=highest.0 {
        let count = nanobots.iter().filter(|b| absdiff(b.0.0, x) <= b.1).count() as i64;
        if count > best.0.0 {
            best.0.0 = count;
            best.0.1 = x;
        }
    }
    for y in lowest.1..=highest.1 {
        let count = nanobots.iter().filter(|b| absdiff(b.0.1, y) <= b.1).count() as i64;
        if count > best.1.0 {
            best.1.0 = count;
            best.1.1 = y;
        }
    }
    for z in lowest.2..=highest.2 {
        let count = nanobots.iter().filter(|b| absdiff(b.0.2, z) <= b.1).count() as i64;
        println!("{} {}", z, count);
        if count > best.2.0 {
            best.2.0 = count;
            best.2.1 = z;
        }
    }
    println!("{},{},{}", best.0.1, best.1.1, best.2.1);

    let counter = nanobots
        .iter()
        .flat_map(|bot| {
            iproduct!(bot.0.0 - bot.1..=bot.0.0 + bot.1, bot.0.1 - bot.1..=bot.0.1 + bot.1, bot.0.2 - bot.1..=bot.0.2 + bot.1)
                .filter(move |p| in_range(p, &bot))
        })
        .collect::<Counter<_>>();
    counter.most_common()[0].print();

    iproduct!(bot.0.0 - bot.1..=bot.0.0 + bot.1, bot.0.1 - bot.1..=bot.0.1 + bot.1, bot.0.2 - bot.1..=bot.0.2 + bot.1)
        .filter(|p| in_range(p, &bot))
        .collect_vec()
        .print();
    */
}
