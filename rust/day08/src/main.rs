use std::collections::HashMap;

fn main() {
    let input = include_str!("./input");
    let nav = parse(input);

    println!("Part 1: {}", part1(&nav));
    println!("Part 2: {}", part2(&nav));
}

fn find<F>(start: &str, has_finished: F, nav: &Navigation) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut count = 0;
    let mut current = start;

    let mut instructions = nav.instructions.iter().cycle();

    while !has_finished(current) {
        let node = nav.nodes.get(current).unwrap();
        let direction = instructions.next().unwrap();

        match direction {
            Direction::Left => current = node.next.0.as_str(),
            Direction::Right => current = node.next.1.as_str(),
        }

        count += 1;
    }

    count
}

fn part1(nav: &Navigation) -> u64 {
    const START: &str = "AAA";
    const END: &str = "ZZZ";
    find(START, |node| node == END, nav)
}

fn part2(nav: &Navigation) -> u64 {
    nav.nodes
        .keys()
        .filter_map(|k| {
            if k.ends_with("A") {
                Some(find(k, |node| node.ends_with("Z"), nav))
            } else {
                None
            }
        })
        .fold(1, |acc, f| lcm(acc, f))
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
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

    #[test]
    fn test_part1() {
        const EXAMPLE_INPUT: &str = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";
        let nav = parse(EXAMPLE_INPUT);
        assert_eq!(part1(&nav), 6);
    }

    #[test]
    fn test_part2() {
        const EXAMPLE_INPUT_2: &str = "
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";

        let nav = parse(EXAMPLE_INPUT_2);
        assert_eq!(part2(&nav), 6);
    }
}
