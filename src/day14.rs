use std::collections::VecDeque;

use itertools::Itertools;

pub fn part1(input: String) {
    let target = input.parse::<usize>().unwrap_or_default();
    let mut scores = vec![3, 7];
    let mut indices = [0, 1];
    while scores.len() < target + 10 {
        let sum = scores[indices[0]] + scores[indices[1]];
        if sum > 9 {
            scores.push(1);
            scores.push(sum % 10);
        } else {
            scores.push(sum);
        }
        indices[0] = (indices[0] + scores[indices[0]] + 1) % scores.len();
        indices[1] = (indices[1] + scores[indices[1]] + 1) % scores.len();
    }

    let mut string = String::new();
    for i in target..target + 10 {
        string.push_str(scores[i].to_string().as_str());
    }
    println!("{}", string);
}

pub fn part2(input: String) {
    // Lot of DRY issues here, but too tired to clean up
    let target = input.bytes().map(|b| usize::from(b - b'0')).collect_vec();
    let mut scores = vec![3, 7];
    let mut indices = [0, 1];
    let mut possible = VecDeque::from(target.clone());
    let mut possible_index = usize::MAX;
    let mut index = 2;
    while !possible.is_empty() {
        let sum = scores[indices[0]] + scores[indices[1]];
        if sum > 9 {
            scores.push(1);
            scores.push(sum % 10);
            if possible.front().unwrap_or(&0) == &1 {
                if possible_index == usize::MAX {
                    possible_index = index;
                }
                possible.pop_front();
            } else {
                possible = VecDeque::from(target.clone());
                possible_index = usize::MAX;
            }
            if possible.is_empty() {
                break;
            }
            index += 1;
            if possible.front().unwrap_or(&0) == &(sum % 10) {
                if possible_index == usize::MAX {
                    possible_index = index;
                }
                possible.pop_front();
            } else {
                possible = VecDeque::from(target.clone());
                possible_index = usize::MAX;
            }
            index += 1;
        } else {
            scores.push(sum);
            if possible.front().unwrap_or(&0) == &sum {
                if possible_index == usize::MAX {
                    possible_index = index;
                }
                possible.pop_front();
            } else {
                possible = VecDeque::from(target.clone());
                possible_index = usize::MAX;
            }
            index += 1;
        }
        indices[0] = (indices[0] + scores[indices[0]] + 1) % scores.len();
        indices[1] = (indices[1] + scores[indices[1]] + 1) % scores.len();
    }

    println!("{}", possible_index);
}
