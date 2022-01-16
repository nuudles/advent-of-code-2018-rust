use std::collections::VecDeque;

use itertools::Itertools;

struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>
}

impl Node {
    fn from(numbers: &mut VecDeque<usize>) -> Node {
        let child_count = numbers.pop_front().unwrap_or_default();
        let metadata_count = numbers.pop_front().unwrap_or_default();
        let children = (0..child_count).map(|_| Node::from(numbers)).collect_vec();
        let metadata = (0..metadata_count).map(|_| numbers.pop_front().unwrap_or_default()).collect_vec();
        Node { metadata, children }
    }

    fn total_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.children.iter().map(|c| c.total_metadata()).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata
                .iter()
                .map(|i| self.children.get(*i - 1).map_or(0, |c| c.value()))
                .sum::<usize>()
        }
    }
}

pub fn part1(input: String) {
    let mut numbers = input.split_whitespace().flat_map(|n| n.parse().ok()).collect::<VecDeque<_>>();
    let node = Node::from(&mut numbers);
    println!("{}", node.total_metadata());
}

pub fn part2(input: String) {
    let mut numbers = input.split_whitespace().flat_map(|n| n.parse().ok()).collect::<VecDeque<_>>();
    let node = Node::from(&mut numbers);
    println!("{}", node.value());
}
