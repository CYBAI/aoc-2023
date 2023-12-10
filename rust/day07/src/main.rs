use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input");

    let part1 = parse_input(input, JokerKind::Ignore);
    println!("Part 1: {}", calculate(&part1));

    let part2 = parse_input(input, JokerKind::Consider);
    println!("Part 2: {}", calculate(&part2));
}

fn calculate(hands: &Vec<Hand>) -> u64 {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JokerKind {
    Ignore,
    Consider,
}

#[derive(Debug, PartialEq, Eq)]
struct Cards {
    cards: [char; 5],
    joker_kind: JokerKind,
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                if self.joker_kind == JokerKind::Consider {
                    match (a, b) {
                        ('J', _) => {
                            return Some(Ordering::Less);
                        }
                        (_, 'J') => {
                            return Some(Ordering::Greater);
                        }
                        _ => {}
                    }
                }

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

fn parse_input(input: &str, joker_kind: JokerKind) -> Vec<Hand> {
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
            let kind = parse_hand_kind(&cards, &joker_kind);

            Some(Hand {
                cards: Cards { cards, joker_kind },
                kind,
                bid,
            })
        })
        .collect()
}

fn parse_hand_kind(cards: &[char; 5], joker_kind: &JokerKind) -> HandKind {
    let j_idx = CARD_STRENGTH.iter().position(|&c| c == 'J').unwrap();

    let mut counts: [usize; 13] = [0; 13];

    for card in cards {
        let idx = CARD_STRENGTH.iter().position(|&c| c == *card).unwrap();
        counts[idx] += 1;
    }

    // ignore zero index
    let mut nums = [0; 6];

    for (idx, count) in counts.iter().enumerate() {
        if *joker_kind == JokerKind::Consider && idx == j_idx {
            continue;
        }

        if *count != 0 {
            nums[*count] += 1;
        }
    }

    let j_count = counts[j_idx];

    if *joker_kind == JokerKind::Consider && j_count > 0 {
        match j_count {
            4 | 5 => return HandKind::FiveOfAKind,
            3 => match nums {
                [_, 0, 1, 0, 0, 0] => return HandKind::FiveOfAKind,
                [_, 2, 0, 0, 0, 0] => return HandKind::FourOfAKind,
                _ => unreachable!("invalid joker count: {:?}", nums),
            },
            2 => match nums {
                [_, 0, 0, 1, 0, 0] => return HandKind::FiveOfAKind,
                [_, 1, 1, 0, 0, 0] => return HandKind::FourOfAKind,
                [_, 3, 0, 0, 0, 0] => return HandKind::ThreeOfAKind,
                _ => unreachable!("invalid joker count: {:?}", nums),
            },
            1 => match nums {
                [_, 0, 0, 0, 1, 0] => return HandKind::FiveOfAKind,
                [_, 1, 0, 1, 0, 0] => return HandKind::FourOfAKind,
                [_, 0, 2, 0, 0, 0] => return HandKind::FullHouse,
                [_, 2, 1, 0, 0, 0] => return HandKind::ThreeOfAKind,
                [_, 4, 0, 0, 0, 0] => return HandKind::OnePair,
                _ => unreachable!("invalid joker count: {:?}", nums),
            },
            _ => unreachable!("invalid joker count: {:?}", nums),
        }
    }

    match nums {
        [_, 0, 0, 0, 0, 1] => HandKind::FiveOfAKind,
        [_, 1, 0, 0, 1, 0] => HandKind::FourOfAKind,
        [_, 0, 1, 1, 0, 0] => HandKind::FullHouse,
        [_, 2, 0, 1, 0, 0] => HandKind::ThreeOfAKind,
        [_, _, 2, 0, 0, 0] => HandKind::TwoPair,
        [_, _, 1, 0, 0, 0] => HandKind::OnePair,
        _ => HandKind::HighCard,
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
        let hands = parse_input(EXAMPLE_INPUT, JokerKind::Ignore);
        assert_eq!(calculate(&hands), 6440);
    }

    #[test]
    fn test_part2() {
        let hands = parse_input(EXAMPLE_INPUT, JokerKind::Consider);
        assert_eq!(calculate(&hands), 5905);
    }
}
