use std::collections::{HashSet, HashMap};

use itertools::Itertools;

fn changed_direction(direction: (i64, i64), index: usize) -> (i64, i64) {
    match index {
        0 => (direction.1, -direction.0),
        1 => direction,
        _ => (-direction.1, direction.0),
    }
}

pub fn part1(input: String) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let mut carts: Vec<((i64, i64), (i64, i64), usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .flat_map(move |(x, c)| {
                    match c {
                        'v' => Some(((x as i64, y as i64), (0, 1), 0)),
                        '^' => Some(((x as i64, y as i64), (0, -1), 0)),
                        '<' => Some(((x as i64, y as i64), (-1, 0), 0)),
                        '>' => Some(((x as i64, y as i64), (1, 0), 0)),
                        _ => None,
                    }
                })
        })
        .collect_vec();

    let mut is_first_crash = true;
    while carts.len() > 1 {
        let mut positions = carts
            .iter()
            .enumerate()
            .map(|(i, c)| (c.0, i))
            .collect::<HashMap<_, _>>();
        let mut to_remove = HashSet::new();
        for (i, cart) in carts.iter_mut().enumerate().sorted_by(|(_, (a, _, _)), (_, (b, _, _))| a.1.cmp(&b.1).then(a.0.cmp(&b.0))) {
            positions.remove(&cart.0);
            cart.0.0 += cart.1.0;
            cart.0.1 += cart.1.1;
            if let Some(index) = positions.get(&cart.0) {
                to_remove.insert(i);
                to_remove.insert(*index);
                if is_first_crash {
                    println!("Part 1: {},{}", cart.0.0, cart.0.1);
                    is_first_crash = false;
                }
                continue;
            } else {
                positions.insert(cart.0, i);
            }
            match grid[cart.0.1 as usize][cart.0.0 as usize] {
                '+' => {
                    cart.1 = changed_direction(cart.1, cart.2);
                    cart.2 = (cart.2 + 1) % 3;
                },
                '/' if cart.1 == (0, -1) => cart.1 = (1, 0),
                '/' if cart.1 == (-1, 0) => cart.1 = (0, 1),
                '/' if cart.1 == (0, 1) => cart.1 = (-1, 0),
                '/' if cart.1 == (1, 0) => cart.1 = (0, -1),
                '\\' if cart.1 == (0, -1) => cart.1 = (-1, 0),
                '\\' if cart.1 == (-1, 0) => cart.1 = (0, -1),
                '\\' if cart.1 == (0, 1) => cart.1 = (1, 0),
                '\\' if cart.1 == (1, 0) => cart.1 = (0, 1),
                _ => (),
            }
        }
        for i in to_remove.iter().sorted().rev() {
            carts.remove(*i);
        }
    }
    println!("Part 2: {},{}", carts[0].0.0, carts[0].0.1);
}
