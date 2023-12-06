type Race = (u64, u64);

fn main() {
    let input = include_str!("./input");
    let races = parse_input(input);

    println!("Part 1: {}", part1(&races));
    println!("Part 2: {}", part2(&races));
}

fn parse_input(input: &str) -> Vec<Race> {
    let parsed = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                None
            } else {
                Some(
                    line.split_whitespace()
                        .skip(1)
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            }
        })
        .collect::<Vec<_>>();

    std::iter::zip(parsed[0].clone(), parsed[1].clone()).collect()
}

fn part1(races: &[Race]) -> u64 {
    races
        .iter()
        .fold(1, |acc, race| acc * find_winning_way(*race))
}

fn part2(races: &[Race]) -> u64 {
    let (time, distance) = races.iter().fold(
        (String::new(), String::new()),
        |(fixed_fst, fixed_snd), (fst, snd)| {
            (
                format!("{}{}", fixed_fst, fst),
                format!("{}{}", fixed_snd, snd),
            )
        },
    );

    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    find_winning_way((time, distance))
}

fn find_winning_way((time, distance): Race) -> u64 {
    let lower = 1;
    let upper = time - 1;
    let mid = (lower + upper) / 2;

    (lower..=mid)
        .filter_map(|x| {
            let diff = time - x;

            if diff * x > distance {
                Some(if diff == x { 1 } else { 2 })
            } else {
                None
            }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    Time:      7  15   30
    Distance:  9  40  200
    ";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(part2(&input), 71503);
    }
}
