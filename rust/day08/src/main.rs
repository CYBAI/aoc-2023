use std::collections::HashMap;

fn main() {
    let input = include_str!("./input");
    let nav = parse(input);

    println!("Part 1: {}", part1(&nav));
}

fn part1(nav: &Navigation) -> u64 {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let mut count = 0;
    let mut current = START;

    let mut instructions = nav.instructions.iter().cycle();

    while current != END {
        let node = nav.nodes.get(current).unwrap();
        let direction = instructions.next().unwrap();

        match direction {
            Direction::Left => current = node.next.0.as_str(),
            Direction::Right => current = node.next.1.as_str(),
        }

        count += 1;
    }

    count as u64
}

fn parse(input: &str) -> Navigation {
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if instructions.is_empty() {
            instructions = line.chars().map(|c| c.into()).collect();
        } else {
            let mut parts = line.splitn(2, " = ");
            let name = parts.next().unwrap().to_string();
            let next = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.replace("(", "").replace(")", ""))
                .collect::<Vec<_>>();

            nodes.insert(
                name.clone(),
                Node {
                    next: (next[0].clone(), next[1].clone()),
                },
            );
        }
    }

    Navigation {
        instructions,
        nodes,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

#[derive(Debug)]
struct Navigation {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct Node {
    next: (String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn test_part1() {
        let nav = parse(EXAMPLE_INPUT);
        assert_eq!(part1(&nav), 6);
    }
}
