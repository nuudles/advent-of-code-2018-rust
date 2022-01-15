use counter::Counter;
use itertools::{Itertools, iproduct};
use regex::Regex;
use lazy_static::lazy_static;

struct Claim {
    id: u64,
    pos: (u64, u64),
    size: (u64, u64)
}

impl Claim {
    fn from(string: &str) -> Option<Claim> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)")
                    .expect("Invalid Regex");
        }

        let captures = RE.captures(string)?;
        Some(
            Claim {
                id: captures.get(1)?.as_str().parse().ok()?,
                pos: (captures.get(2)?.as_str().parse().ok()?, captures.get(3)?.as_str().parse().ok()?),
                size: (captures.get(4)?.as_str().parse().ok()?, captures.get(5)?.as_str().parse().ok()?)
            }
        )
    }
}

pub fn part1(input: String) {
    let mut counter = Counter::<(u64, u64), usize>::new();
    let claims = input.lines().flat_map(Claim::from).collect_vec();
    for claim in &claims {
        for (y, x) in iproduct!(
            claim.pos.1..claim.pos.1 + claim.size.1,
            claim.pos.0..claim.pos.0 + claim.size.0
        ) {
            counter[&(x, y)] += 1;
        }
    }
    println!("Part 1: {}", counter.iter().filter(|(_, c)| c > &&1).count());
    println!(
        "Part 2: {}",
        claims
            .iter()
            .find(|claim|
                iproduct!(
                    claim.pos.1..claim.pos.1 + claim.size.1,
                    claim.pos.0..claim.pos.0 + claim.size.0
                )
                .all(|(y, x)| counter[&(x, y)] == 1)
            )
            .map_or(0, |c| c.id)
    );
}
