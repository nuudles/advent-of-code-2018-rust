use cached::proc_macro::cached;
use itertools::iproduct;

#[cached]
fn power_level(pos: (i64, i64), serial_number: i64) -> i64 {
    let rack_id = pos.0 + 10;
    let mut power_level = rack_id * pos.1 + serial_number;
    power_level *= rack_id;
    ((power_level / 100) % 10) - 5
}

pub fn part1(input: String) {
    let serial_number = input.parse::<i64>().unwrap_or_default();
    let max = iproduct!(1..=298, 1..298)
        .max_by_key(|(x_offset, y_offset)| {
            iproduct!(0..3, 0..3)
                .map(|(x, y)| power_level((x + x_offset, y + y_offset), serial_number))
                .sum::<i64>()
        })
        .expect("Max not found");
    println!("{},{}", max.0, max.1);
}

pub fn part2(input: String) {
    let serial_number = input.parse::<i64>().unwrap_or_default();
    // Let's just brute force it!
    let max = iproduct!(3..=300, 1..=298, 1..=298)
        .filter(|(size, x_offset, y_offset)| x_offset + size <= 300 && y_offset + size <= 300)
        .max_by_key(|(size, x_offset, y_offset)| {
            iproduct!(0..*size, 0..*size)
                .map(|(x, y)| power_level((x + x_offset, y + y_offset), serial_number))
                .sum::<i64>()
        })
        .expect("Max not found");
    // Took 85 minutes to run... May revisit to optimize, but probably not
    println!("{},{},{}", max.1, max.2, max.0);
}
