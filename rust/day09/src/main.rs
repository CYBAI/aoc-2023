fn main() {
    let input = include_str!("./input");
    let histories = parse(input);

    println!("Part 1: {}", part1(&histories));
    println!("Part 2: {}", part2(&histories));
}

type History = Vec<i64>;

fn parse(input: &str) -> Vec<History> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                None
            } else {
                Some(
                    line.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect::<History>(),
                )
            }
        })
        .collect::<Vec<_>>()
}

fn part1(histories: &[History]) -> i64 {
    histories
        .iter()
        .map(|history| find_next_num(history))
        .sum()
}

fn part2(histories: &[History]) -> i64 {
    histories
        .iter()
        .map(|history| find_previous_num(history))
        .sum()
}

fn find_next_num(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }

    let diff = calc_diff(history);
    history.last().unwrap() + find_next_num(&diff)
}

fn find_previous_num(history: &[i64]) -> i64 {
    if history.iter().all(|n| *n == 0) {
        return 0;
    }

    let diff = calc_diff(history);
    history.first().unwrap() - find_previous_num(&diff)
}

fn calc_diff(history: &[i64]) -> History {
    let mut diff = Vec::new();

    for i in 1..history.len() {
        diff.push(history[i] - history[i - 1]);
    }

    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    ";

    #[test]
    fn test_part1() {
        let histories = parse(EXAMPLE_INPUT);
        assert_eq!(part1(&histories), 114);
    }

    #[test]
    fn test_part2() {
        let histories = parse(EXAMPLE_INPUT);
        assert_eq!(part2(&histories), 2);
    }
}
