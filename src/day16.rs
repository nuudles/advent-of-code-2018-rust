use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::parse_nums::parse_nums;

fn addr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] + map[&b]);
}

fn addi(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] + b);
}

fn mulr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] * map[&b]);
}

fn muli(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] * b);
}

fn banr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] & map[&b]);
}

fn bani(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] & b);
}

fn borr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] | map[&b]);
}

fn bori(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, map[&a] | b);
}

fn setr(map: &mut HashMap<i64, i64>, a: i64, _b: i64, c: i64) {
    map.insert(c, map[&a]);
}

fn seti(map: &mut HashMap<i64, i64>, a: i64, _b: i64, c: i64) {
    map.insert(c, a);
}

fn gtir(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if a > map[&b] { 1 } else { 0 });
}

fn gtri(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if map[&a] > b { 1 } else { 0 });
}

fn gtrr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if map[&a] > map[&b] { 1 } else { 0 });
}

fn eqir(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if a == map[&b] { 1 } else { 0 });
}

fn eqri(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if map[&a] == b { 1 } else { 0 });
}

fn eqrr(map: &mut HashMap<i64, i64>, a: i64, b: i64, c: i64) {
    map.insert(c, if map[&a] == map[&b] { 1 } else { 0 });
}

pub fn part1(input: String) {
    let functions = [
        addr,
        addi,
        mulr,
        muli,
        banr,
        bani,
        borr,
        bori,
        setr,
        seti,
        gtir,
        gtri,
        gtrr,
        eqir,
        eqri,
        eqrr
    ];

    let mut part1_count = 0;
    let mut components = input.split("\n\n\n\n");
    let mut opcodes = components
        .next()
        .unwrap_or_default()
        .split("\n\n")
        .fold(HashMap::<i64, HashSet<usize>>::new(), |mut map, sample| {
            let mut lines = sample.lines();
            let before: HashMap<i64, i64> = parse_nums(lines.next().unwrap_or_default())
                .enumerate()
                .map(|(k, v)| (k as i64, v))
                .collect();
            let instruction: Vec<i64> = parse_nums(lines.next().unwrap_or_default()).collect();
            let after: HashMap<i64, i64> = parse_nums(lines.next().unwrap_or_default())
                .enumerate()
                .map(|(k, v)| (k as i64, v))
                .collect();

            let indices = functions
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    let mut registers = before.clone();
                    f(&mut registers, instruction[1], instruction[2], instruction[3]);
                    registers == after
                })
                .map(|(i, _)| i)
                .collect::<HashSet<_>>();
            if indices.len() >= 3 {
                part1_count += 1;
            }
            if let Some(entry) = map.get_mut(&instruction[0]) {
                *entry = entry.intersection(&indices).map(|x| *x).collect();
            } else {
                map.insert(instruction[0], indices);
            }
            map
        });
    println!("Part 1: {}", part1_count);

    let mut mapping = HashMap::new();
    while mapping.len() < functions.len() {
        for (o, s) in opcodes.iter().filter(|(_, s)| s.len() == 1) {
            mapping.insert(*o, *s.iter().next().unwrap_or(&0));
        }
        for (_, s) in opcodes.iter_mut() {
            s.retain(|x| !mapping.values().contains(x));
        }
    }

    let mut registers = HashMap::new();
    registers.insert(0, 0);
    registers.insert(1, 0);
    registers.insert(2, 0);
    registers.insert(3, 0);
    for line in components.next().unwrap_or_default().lines() {
        let instruction: Vec<i64> = parse_nums(line).collect();
        functions[mapping[&instruction[0]]](&mut registers, instruction[1], instruction[2], instruction[3]);
    }
    println!("Part 2: {}", registers[&0]);
}
