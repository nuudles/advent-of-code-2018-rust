use itertools::Itertools;

struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>
}

impl Node {
    fn from(numbers: &Vec<usize>, index: &mut usize) -> Node {
        let child_count = numbers[*index];
        *index += 1;
        let metadata_count = numbers[*index];
        *index += 1;
        let children = (0..child_count).map(|_| Node::from(numbers, index)).collect_vec();
        let metadata = (0..metadata_count).map(|_| {
            let number = numbers[*index];
            *index += 1;
            number
        }).collect_vec();
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
    let numbers = input.split_whitespace().flat_map(|n| n.parse().ok()).collect_vec();
    let mut index = 0;
    let node = Node::from(&numbers, &mut index);
    println!("{}", node.total_metadata());
}

pub fn part2(input: String) {
    let numbers = input.split_whitespace().flat_map(|n| n.parse().ok()).collect_vec();
    let mut index = 0;
    let node = Node::from(&numbers, &mut index);
    println!("{}", node.value());
}
