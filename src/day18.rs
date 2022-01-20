use std::collections::HashSet;

use crate::point::Point;

fn run(input: &str, steps: u64) {
    let mut open = HashSet::new();
    let mut trees = HashSet::new();
    let mut lumberyards = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let point = Point { x: x as i64, y: y as i64 };
            if c == '|' {
                trees.insert(point);
            } else if c == '#' {
                lumberyards.insert(point);
            } else {
                open.insert(point);
            }
        }
    }

    for _ in 0..steps {
        let mut new_open = HashSet::new();
        let mut new_trees = HashSet::new();
        let mut new_lumberyards = HashSet::new();

        for p in open {
            if p.neighbors_with_diagonals().iter().filter(|n| trees.contains(&n)).count() >= 3 {
                new_trees.insert(p);
            } else {
                new_open.insert(p);
            }
        }
        for p in &trees {
            if p.neighbors_with_diagonals().iter().filter(|n| lumberyards.contains(&n)).count() >= 3 {
                new_lumberyards.insert(*p);
            } else {
                new_trees.insert(*p);
            }
        }
        for p in &lumberyards {
            if p.neighbors_with_diagonals().iter().filter(|n| lumberyards.contains(&n)).count() >= 1 
            && p.neighbors_with_diagonals().iter().filter(|n| trees.contains(&n)).count() >= 1 {
                new_lumberyards.insert(*p);
            } else {
                new_open.insert(*p);
            }
        }

        open = new_open;
        trees = new_trees;
        lumberyards = new_lumberyards;
    }

    println!("{}", trees.len() * lumberyards.len());
}

pub fn part1(input: String) {
    run(&input, 10);
}

pub fn part2(input: String) {
    // For my input, starting around 441, it cycles every 28 iterations, so just find the 468th
    run(&input, 468);
}
