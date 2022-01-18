use std::{collections::{HashMap, HashSet, VecDeque}, cmp::Ordering};

use itertools::Itertools;

use crate::point::Point;

fn parse(input: String) -> (Vec<Vec<char>>, HashMap<Point<usize>, i64>) {
    let board = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut units = HashMap::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'E' || c == 'G' {
                units.insert(Point { x, y }, 200);
            }
        }
    }
    (board, units)
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        println!("{}", row.iter().collect::<String>());
    }
}

fn reading_order(a: &Point<usize>, b: &Point<usize>) -> Ordering {
    a.y.cmp(&b.y).then(a.x.cmp(&b.x))
}

fn next_step(start: &Point<usize>, goals: HashSet<Point<usize>>, board: &Vec<Vec<char>>) -> Option<Point<usize>> {
    if goals.contains(start) {
        return None;
    }

    let neighbors = start.neighbors();
    let mut next = neighbors
        .iter()
        .filter(|p| board[p.y][p.x] == '.')
        .map(|p| (*p, *p))
        .collect_vec();
    let mut seen = neighbors
        .iter()
        .map(|p| *p)
        .collect::<HashSet<_>>();
    let mut potentials = Vec::new();
    while next.len() > 0 && potentials.is_empty() {
        let mut new_next = Vec::new();
        for (p, first) in next {
            if goals.contains(&p) {
                potentials.push((p, first));
            }
            for n in p.neighbors() {
                if board[n.y][n.x] == '.' && !seen.contains(&n) {
                    new_next.push((n, first));
                    seen.insert(n);
                }
            }
        }
        next = new_next;
    }
    if let Some((_, first)) = potentials.iter().sorted_by(|(a, fa), (b, fb)| reading_order(a, b).then(reading_order(fa, fb))).next() {
        Some(*first)
    } else {
        None
    }
}

fn outcome(input: String, elf_attack: i64) -> Option<i64> {
    let (mut board, mut units) = parse(input);
    let mut rounds = 0;
    'outer: loop {
        let mut unit_positions: VecDeque<_> = units.keys().map(|p| p.clone()).sorted_by(reading_order).collect();
        while let Some(point) = unit_positions.pop_front() {
            let unit = board[point.y][point.x];
            let targets = units
                .keys()
                .filter(|p| board[p.y][p.x] != unit)
                .map(|p| p.clone())
                .collect_vec();
            if targets.is_empty() {
                break 'outer;
            }

            let attack_area: [Point<usize>; 4];
            if let Some(step) = next_step(
                &point, 
                targets
                    .iter()
                    .flat_map(|p| p.neighbors())
                    .filter(|p| board[p.y][p.x] == '.' || p == &point)
                    .collect(),
                &board
            ) {
                board[point.y][point.x] = '.';
                board[step.y][step.x] = unit;
                let hp = units.remove(&point).unwrap_or_default();
                units.insert(step, hp);

                attack_area = step.neighbors();
            } else {
                attack_area = point.neighbors();
            }

            if let Some(attack_pos) = targets
                .iter()
                .filter(|p| attack_area.contains(p))
                .sorted_by(|a, b|
                    units.get(a).cmp(&units.get(b)).then(reading_order(a, b))
                )
                .next() {
                    let target = units.get_mut(attack_pos).expect("Could not find the unit to attack?!");
                    *target -= if unit == 'E' { elf_attack } else { 3 };
                    if *target <= 0 {
                        units.remove(attack_pos);
                        board[attack_pos.y][attack_pos.x] = '.';
                        unit_positions = unit_positions.iter().filter(|p| p != &attack_pos).map(|p| *p).collect();
                        if elf_attack != 3 && unit == 'G' {
                            return None;
                        }
                    }
                }
        }
        rounds += 1;
    }

    Some(rounds * units.values().sum::<i64>())
}

pub fn part1(input: String) {
    println!("{}", outcome(input, 3).unwrap_or_default());
}

pub fn part2(input: String) {
    let mut elf_attack = 10;
    loop {
        let value = outcome(input.clone(), elf_attack);
        if let Some(value) = value {
            println!("{}", value);
            break;
        }
        elf_attack += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::outcome;

    #[test]
    fn test_highest_score() {
        assert_eq!(outcome(r"
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
            ".trim().to_string(), 3), Some(27730));
        assert_eq!(outcome(r"
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
            ".trim().to_string(), 3), Some(36334));
        assert_eq!(outcome(r"
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
            ".trim().to_string(), 3), Some(39514));
        assert_eq!(outcome(r"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
            ".trim().to_string(), 3), Some(27755));
        assert_eq!(outcome(r"
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
            ".trim().to_string(), 3), Some(28944));
        assert_eq!(outcome(r"
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
            ".trim().to_string(), 3), Some(18740));
        assert_eq!(outcome(r"
####
##E#
#GG#
####
            ".trim().to_string(), 3), Some(13400));
        assert_eq!(outcome(r"
#####
#GG##
#.###
#..E#
#.#G#
#.E##
#####
            ".trim().to_string(), 3), Some(13987));
        assert_eq!(outcome(r"
#################
##..............#
##........G.....#
####.....G....###
#....##......####
#...............#
##........GG....#
##.........E..#.#
#####.###...#####
#################
            ".trim().to_string(), 3), Some(14740));
    }
}

