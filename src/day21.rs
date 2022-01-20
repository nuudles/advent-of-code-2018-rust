use std::collections::HashSet;

pub fn part1(_input: String) {
    let mut seen = HashSet::new();

    let mut one: u64 = 0;
    let mut last_one = one;

    // Wrote the puzzle input as Rust instructions
    'outer: loop {
        let mut three = one | 65536;
        one = 10905776;

        loop {
            let mut four = three & 255;
            one = one + four;
            one = one & 16777215;
            one = one * 65899;
            one = one & 16777215;
            if 256 > three {
                if seen.is_empty() {
                    println!("Part 1: {}", one);
                }
                if seen.contains(&one) {
                    println!("Part 2: {}", last_one);
                    break 'outer;
                }
                seen.insert(one);
                last_one = one;
                continue 'outer;
            }
            four = 0;
            loop {
                let mut five = four + 1;
                five = five * 256;
                if five > three { // 65536
                    break;
                }
                four += 1;
            }
            three = four;
        }
    }
}
