use counter::Counter;
use itertools::Itertools;

pub fn part1(input: String) {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let counts = line.chars().collect::<Counter<_>>();
        if counts.iter().any(|(_, s)| s == &2) {
            twos += 1;
        }
        if counts.iter().any(|(_, s)| s == &3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
}

pub fn part2(input: String) {
    'outer: for (a, b) in input.lines().tuple_combinations() {
        let mut index: Option<usize> = None;
        for (i, (ca, cb)) in a.chars().zip(b.chars()).enumerate() {
            if ca != cb {
                if index != None {
                    continue 'outer;
                }
                index = Some(i);
            }
        }
        if let Some(index) = index {
            let mut string = a.to_string();
            string.remove(index);
            println!("{}", string);
            break;
        }
    }
}
