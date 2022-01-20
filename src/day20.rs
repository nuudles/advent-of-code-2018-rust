use std::collections::{VecDeque, HashSet};

use crate::point::Point;

fn find_doors(mut pos: Point<i64>, doors: &mut HashSet<Point<i64>>, chars: &mut VecDeque<char>) -> Point<i64> {
    while let Some(char) = chars.pop_front() {
        match char {
            'N' => {
                let up = pos.up();
                doors.insert(up);
                pos = up.up();
            },
            'S' => {
                let down = pos.down();
                doors.insert(down);
                pos = down.down();
            },
            'E' => {
                let right = pos.right();
                doors.insert(right);
                pos = right.right();
            },
            'W' => {
                let left = pos.left();
                doors.insert(left);
                pos = left.left();
            },
            '(' => {
                while chars.front() != Some(&')') {
                    find_doors(pos, doors, chars);
                }
                chars.pop_front();
            },
            ')' => {
                chars.push_front(')');
                break;
            }
            '|' => {
                break;
            },
            '$' => {
                break;
            },
            _ => (),
        }
    }
    pos
}

pub fn part1(input: String) {
    let mut chars: VecDeque<_> = input.chars().collect();
    chars.pop_front();
    let mut doors = HashSet::new();
    find_doors(Point { x: 0, y: 0 }, &mut doors, &mut chars);

    let mut longest = 0;
    let mut count = 0;
    let mut next = HashSet::new();
    next.insert(Point::<i64> { x: 0, y: 0 });
    let mut seen = next.clone();
    while !next.is_empty() {
        let mut new_next = HashSet::new();
        for p in next {
            for (d, n) in [
                (p.up(), p.up().up()),
                (p.down(), p.down().down()),
                (p.left(), p.left().left()),
                (p.right(), p.right().right()),
            ] {
                if doors.contains(&d) && !seen.contains(&n) {
                    new_next.insert(n);
                    seen.insert(n);
                    if longest >= 999 {
                        count += 1;
                    }
                }
            }
        }
        longest += 1;
        next = new_next;
    }
    println!("Part 1: {}", longest - 1);
    println!("Part 2: {}", count);
}
