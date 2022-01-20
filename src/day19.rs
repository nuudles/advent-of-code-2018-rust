use std::collections::HashMap;

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

fn noop(_map: &mut HashMap<i64, i64>, _a: i64, _b: i64, _c: i64) {
    assert!(false, "Noop was called");
}

pub fn part1(input: String) {
    let mut lines = input.lines();
    let ip: i64 = parse_nums(lines.next().unwrap_or_default()).next().unwrap_or_default();
    let mut registers = HashMap::<i64, i64>::new();
    for i in 0..6 {
        registers.insert(i, 0);
    }
    let instructions = lines.collect_vec();

    while let Ok(index) = usize::try_from(registers[&ip]) {
        if index >= instructions.len() {
            break;
        }
        let mut instruction = instructions[index].split_whitespace();
        let function = match instruction.next().unwrap_or_default() {
            "addr" => addr,
            "addi" => addi,
            "mulr" => mulr,
            "muli" => muli,
            "banr" => banr,
            "bani" => bani,
            "borr" => borr,
            "bori" => bori,
            "setr" => setr,
            "seti" => seti,
            "gtir" => gtir,
            "gtri" => gtri,
            "gtrr" => gtrr,
            "eqir" => eqir,
            "eqri" => eqri,
            "eqrr" => eqrr,
            _ => noop,
        };
        let arguments = instruction.flat_map(|n| n.parse().ok()).collect_vec();
        function(&mut registers, arguments[0], arguments[1], arguments[2]);

        registers.insert(ip, registers[&ip] + 1);
    }
    println!("{}", registers[&0]);
}

pub fn part2(_input: String) {
    /*
    // Decoded the instructions once the initialization is
    // complete to the following:
    let mut registers: [i64; 6] = [0, 0, 0, 0, 0, 0];
    registers[2] = 10551298;
    loop {
        registers[1] = 1;
        loop {
            registers[3] = 1;
            if registers[1] * registers[3] == registers[2] {
                registers[0] += registers[1];
            }
            registers[3] += 1;
            if registers[3] > registers[2] {
                break;
            }
        }
        registers[1] += 1;
        if registers[1] > registers[2] {
            break;
        }
    }
    println!("{}", registers[0]);
    */

    // Simplified it to this:
    let two = 10551298;
    let mut zero = 0;
    for i in 1..=two {
        if two % i == 0 {
            zero += i;
        }
    }
    println!("{}", zero);
}
