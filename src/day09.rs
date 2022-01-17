use std::collections::VecDeque;

use itertools::Itertools;

fn highest_score(input: &str, target_multiplier: u64) -> u64 {
    let mut components = input.split_whitespace();
    let player_count = components.next().unwrap_or_default().parse::<usize>().unwrap_or_default();
    let target = components.nth(5).unwrap_or_default().parse::<u64>().unwrap_or_default();

    let mut current = 1;
    let mut marbles = VecDeque::new();
    marbles.push_front(0);
    let mut scores = (0..player_count).map(|_| 0).collect_vec();
    let mut turn = 0;
    loop {
        if current % 23 == 0 {
            marbles.rotate_right(7);
            let removed = marbles.pop_front().unwrap_or_default();
            scores[turn] += current + removed;
        } else {
            marbles.rotate_left(2 % marbles.len());
            marbles.push_front(current);
        }

        if current == target * target_multiplier {
            break;
        }

        current += 1;
        turn = (turn + 1) % player_count;
    }
    scores.iter().map(|x| *x).max().unwrap_or_default()
}

pub fn part1(input: String) {
    println!("{}", highest_score(&input, 1));
}

pub fn part2(input: String) {
    println!("{}", highest_score(&input, 100));
}

#[cfg(test)]
mod tests {
    use super::highest_score;

    #[test]
    fn test_highest_score() {
        assert_eq!(highest_score("9 players; last marble is worth 32 points", 1), 32);
        assert_eq!(highest_score("10 players; last marble is worth 1618 points", 1), 8317);
        assert_eq!(highest_score("13 players; last marble is worth 7999 points", 1), 146373);
        assert_eq!(highest_score("21 players; last marble is worth 6111 points", 1), 54718);
        assert_eq!(highest_score("30 players; last marble is worth 5807 points", 1), 37305);
    }
}
