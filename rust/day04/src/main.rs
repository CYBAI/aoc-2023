use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    holds: HashSet<u32>,
}

impl Card {
    fn new() -> Self {
        Self {
            winning: HashSet::new(),
            holds: HashSet::new(),
        }
    }
}

fn main() {
    let input = include_str!("./input");
    let cards = parse(input);

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut card = Card::new();

        let nums = line
            .split(": ")
            .skip(1)
            .take(1)
            .flat_map(|nums| nums.split(" | "))
            .collect::<Vec<_>>();

        card.winning = nums[0]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        card.holds = nums[1]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        cards.push(card);
    }

    cards
}

fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| {
            let n = card.winning.intersection(&card.holds).count() as u32;

            if n == 0 {
                0
            } else {
                2_u32.pow(n - 1)
            }
        })
        .sum()
}

fn part2(cards: &[Card]) -> u32 {
    let boundary = cards.len();
    let mut ans = vec![1; boundary];

    for (id, card) in cards.iter().enumerate() {
        let n = card.winning.intersection(&card.holds).count();

        for i in (id + 1)..=(id + n) {
            ans[i] += ans[id];
        }
    }

    ans.iter().sum()
}

#[test]
fn test_part1() {
    let input = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards = parse(input);
    assert_eq!(part1(&cards), 13);
}

#[test]
fn test_part2() {
    let input = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let cards = parse(input);
    assert_eq!(part2(&cards), 30);
}
