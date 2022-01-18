use std::collections::{HashSet, HashMap, VecDeque};

fn run(input: &str, count: usize) -> HashSet<i64> {
    let mut plants = input
        .lines()
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .nth(2)
        .unwrap_or_default()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| if c == '#' { Some(i as i64) } else { None })
        .collect::<HashSet<_>>();
    let rules = input
        .lines()
        .skip(2)
        .map(|l| {
            let mut components = l.split(" => ");
            let pattern = components.next().unwrap_or_default().chars().map(|c| c == '#').collect::<VecDeque<_>>();
            let result = components.next().unwrap_or_default() == "#";
            (pattern, result)
        })
        .collect::<HashMap<_, _>>();
    let mut min = 0;
    let mut max = 100;
    for _ in 0..count {
        let mut new_plants = HashSet::new();
        let mut deque = VecDeque::new();
        deque.push_back(false);
        deque.push_back(false);
        deque.push_back(false);
        deque.push_back(false);
        deque.push_back(false);
        for i in min - 2..=max + 2 {
            deque.pop_front();
            deque.push_back(plants.contains(&i));
            if rules.get(&deque).map(|x| *x).unwrap_or_default() {
                new_plants.insert(i - 2);
            }
        }
        plants = new_plants;
        min -= 2;
        max += 2;
    }
    plants
}

pub fn part1(input: String) {
    println!("{}", run(&input, 20).iter().sum::<i64>());
}

pub fn part2(input: String) {
    // After about 200 iterations the pattern stays the same, but all the numbers just shift to the right
    // one position per iteration
    let pots = run(&input, 200);
    println!("{}", pots.iter().map(|p| p + 50000000000 - 200).sum::<i64>());
}
