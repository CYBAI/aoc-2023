use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input");
    let hands = parse_input(input);

    println!("Part 1: {}", part1(&hands));
}

fn part1(hands: &Vec<Hand>) -> u64 {
    let mut hands = hands.into_iter().collect::<Vec<_>>();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, hand)| acc + hand.bid * (idx + 1) as u64)
}

const CARD_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq)]
struct Cards([char; 5]);

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            if a != b {
                let x = CARD_STRENGTH.iter().position(|&c| c == *a).unwrap();
                let y = CARD_STRENGTH.iter().position(|&c| c == *b).unwrap();

                return Some(x.cmp(&y));
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Cards,
    kind: HandKind,
    bid: u64,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.kind == other.kind
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.kind == other.kind {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(self.kind.cmp(&other.kind))
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                return None;
            }

            let splitted = line.split(' ').collect::<Vec<_>>();
            let cards = splitted[0].chars().collect::<Vec<_>>().try_into().unwrap();
            let bid = splitted[1].parse::<u64>().unwrap();
            let kind = parse_hand_kind(&cards);

            Some(Hand {
                cards: Cards(cards),
                kind,
                bid,
            })
        })
        .collect()
}

fn parse_hand_kind(cards: &[char; 5]) -> HandKind {
    let mut counts = HashMap::new();

    for card in cards {
        counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
    }

    let nums = counts.values().fold(HashMap::new(), |mut map, n| {
        if *n != 0 {
            map.entry(n).and_modify(|c| *c += 1).or_insert(1);
        }

        map
    });

    if nums.get(&5).is_some() {
        HandKind::FiveOfAKind
    } else if nums.get(&4).is_some() {
        HandKind::FourOfAKind
    } else if nums.get(&3).is_some() && nums.get(&2).is_some() {
        HandKind::FullHouse
    } else if nums.get(&3).is_some() {
        HandKind::ThreeOfAKind
    } else if let Some(2) = nums.get(&2) {
        HandKind::TwoPair
    } else if nums.get(&2).is_some() {
        HandKind::OnePair
    } else {
        HandKind::HighCard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn test_part1() {
        let hands = parse_input(EXAMPLE_INPUT);
        assert_eq!(part1(&hands), 6440);
    }
}
