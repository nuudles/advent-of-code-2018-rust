use std::collections::{HashSet, HashMap};

use itertools::{Itertools, iproduct};
use pathfinding::prelude::astar;

use crate::{parse_nums::parse_nums, point::Point};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Equipment {
    Neither,
    ClimbingGear,
    Torch
}

pub fn part1(input: String) {
    let mut lines = input.lines();
    let depth: u64 = parse_nums(lines.next().unwrap_or_default()).next().unwrap_or_default();
    let target: (usize, usize) = parse_nums(lines.next().unwrap_or_default())
        .next_tuple()
        .unwrap_or_default();
    // Go a little extra in case the optimal path goes below or to the right of the target
    let extra = 50;
    let mut geologic_indices: Vec<Vec<u64>> = (0..=target.1 + extra)
        .map(|_| (0..=target.0 + extra).map(|_| 0).collect()).collect();
    for y in 0..=target.1 + extra {
        for x in 0..=target.0 + extra {
            geologic_indices[y][x] = if x == 0 && y == 0 {
                0
            } else if x == target.0 && y == target.1 {
                0
            } else if x == 0 {
                (y as u64) * 48271
            } else if y == 0 {
                (x  as u64) * 16807
            } else {
                ((geologic_indices[y][x - 1] + depth) % 20183) * ((geologic_indices[y - 1][x] + depth) % 20183)
            }
        }
    }
    let risks = geologic_indices.iter().map(|r| r.iter().map(|n| ((n + depth) % 20183) % 3).collect_vec()).collect_vec();
    println!("Part 1: {}", iproduct!(0..=target.1, 0..=target.0).map(|(y, x)| risks[y][x]).sum::<u64>());

    let mut equipment_map: HashMap<u64, HashSet<Equipment>> = HashMap::new();
    equipment_map.insert(0, [Equipment::Torch, Equipment::ClimbingGear].into_iter().collect());
    equipment_map.insert(1, [Equipment::Neither, Equipment::ClimbingGear].into_iter().collect());
    equipment_map.insert(2, [Equipment::Torch, Equipment::Neither].into_iter().collect());

    let target_point = Point::<i64> { x: target.0 as i64, y: target.1 as i64 };
    let path = astar(
        &(Point::<i64> { x: 0, y: 0 }, Equipment::Torch),
        |(pos, equipment)| {
            let possible_equipment = equipment_map.get(&risks[pos.y as usize][pos.x as usize]).expect("Unknown risk?");
            pos
                .neighbors()
                .into_iter()
                .filter(|p| (0..=target.0 + extra).contains(&(p.x as usize)) && (0..=target.1 + extra).contains(&(p.y as usize)))
                .fold(Vec::new(), |mut successors, neighbor| {
                    let neighbor_equipment = equipment_map.get(&risks[neighbor.y as usize][neighbor.x as usize]).expect("Unknown risk?");
                    for e in possible_equipment.intersection(neighbor_equipment) {
                        successors.push(((neighbor, e.clone()), if e == equipment { 1 } else { 8 }));
                    }
                    successors
                })
        },
        |(pos, _)| pos.manhattan_distance(&target_point),
        |(pos, equipment)| pos == &target_point && equipment == &Equipment::Torch
    );
    println!("Part 2: {}", path.expect("No path found").1);
}
